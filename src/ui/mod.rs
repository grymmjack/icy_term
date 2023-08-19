#![allow(unsafe_code, clippy::wildcard_imports)]

use chrono::Utc;
use i18n_embed_fl::fl;
use icy_engine::ansi::BaudEmulation;
use icy_engine::BufferParser;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use eframe::egui::Key;

use crate::features::{AutoFileTransfer, AutoLogin};
use crate::protocol::TransferState;
use crate::util::{beep, play_music, Rng};
use crate::Options;
use crate::{
    addresses::{store_phone_book, Address},
    protocol::FileDescriptor,
    TerminalResult,
};

use crate::com::Connection;

pub mod app;

pub mod buffer_view;
pub use buffer_view::*;

pub mod terminal_window;
pub use terminal_window::*;

pub mod util;
pub use util::*;

pub mod dialogs;

pub mod com_thread;

// pub mod simulate;

#[derive(PartialEq, Eq)]
pub enum MainWindowMode {
    ShowTerminal,
    ShowPhonebook,
    ShowSettings(bool),
    SelectProtocol(bool),
    FileTransfer(bool),
    ShowCaptureDialog,
    ShowIEMSI, //   AskDeleteEntry
}

pub struct MainWindow {
    pub buffer_view: Arc<eframe::epaint::mutex::Mutex<BufferView>>,
    pub buffer_parser: Box<dyn BufferParser>,

    pub connection_opt: Connection,

    pub mode: MainWindowMode,
    pub addresses: Vec<Address>,
    pub handled_char: bool,
    pub cur_addr: usize,
    pub selected_bbs: Option<usize>,
    pub phonebook_filter: dialogs::PhonebookFilter,
    pub phonebook_filter_string: String,

    pub options: Options,
    pub screen_mode: ScreenMode,
    pub auto_login: AutoLogin,
    pub capture_session: bool,
    /// debug spew prevention
    pub show_capture_error: bool,
    pub has_baud_rate: bool,

    pub rng: Rng,
    pub auto_file_transfer: AutoFileTransfer,
    // protocols
    pub current_transfer: Option<Arc<Mutex<TransferState>>>,
    pub is_alt_pressed: bool,

    pub settings_category: usize,

    file_transfer_dialog: dialogs::FileTransferDialog,
}

impl MainWindow {
    pub fn println(&mut self, str: &str) -> TerminalResult<()> {
        for ch in str.chars() {
            if ch as u32 > 255 {
                continue;
            }
            self.buffer_view
                .lock()
                .print_char(&mut self.buffer_parser, ch)?;
        }
        Ok(())
    }

    pub fn handle_result<T>(&mut self, res: TerminalResult<T>, terminate_connection: bool) {
        if let Err(err) = res {
            log::error!("{err}");

            if terminate_connection {
                self.connection_opt.disconnect().unwrap_or_default();
            }
        }
    }

    pub fn output_char(&mut self, ch: char) {
        let translated_char = self.buffer_parser.convert_from_unicode(ch);
        if self.connection_opt.is_connected() {
            let r = self.connection_opt.send(vec![translated_char as u8]);
            self.handle_result(r, false);
        } else if let Err(err) = self.print_char(translated_char as u8) {
            log::error!("{err}");
        }
    }

    pub fn output_string(&mut self, str: &str) {
        if self.connection_opt.is_connected() {
            let mut v = Vec::new();
            for ch in str.chars() {
                let translated_char = self.buffer_parser.convert_from_unicode(ch);
                v.push(translated_char as u8);
            }
            let r = self.connection_opt.send(v);
            self.handle_result(r, false);
        } else {
            for ch in str.chars() {
                let translated_char = self.buffer_parser.convert_from_unicode(ch);
                if let Err(err) = self.print_char(translated_char as u8) {
                    log::error!("{err}");
                }
            }
        }
    }

    pub fn print_char(&mut self, c: u8) -> Result<(), Box<dyn std::error::Error>> {
        let result = self
            .buffer_view
            .lock()
            .print_char(&mut self.buffer_parser, unsafe {
                char::from_u32_unchecked(c as u32)
            })?;
        match result {
            icy_engine::CallbackAction::None => {}
            icy_engine::CallbackAction::SendString(result) => {
                let r = self.connection_opt.send(result.as_bytes().to_vec());
                self.handle_result(r, false);
            }
            icy_engine::CallbackAction::PlayMusic(music) => {
                play_music(&music);
            }
            icy_engine::CallbackAction::Beep => {
                if self.options.console_beep {
                    beep();
                }
            }
            icy_engine::CallbackAction::ChangeBaudEmulation(baud_emulation) => {
                let r = self
                    .connection_opt
                    .set_baud_rate(baud_emulation.get_baud_rate());
                self.handle_result(r, false);
            }
        }
        self.buffer_view.lock().redraw_view();
        Ok(())
    }

    fn start_transfer_thread(
        &mut self,
        protocol_type: crate::protocol::TransferType,
        download: bool,
        files_opt: Option<Vec<FileDescriptor>>,
    ) {
        self.mode = MainWindowMode::FileTransfer(download);
        let state = Arc::new(Mutex::new(TransferState::default()));
        self.current_transfer = Some(state.clone());
        let res =
            self.connection_opt
                .start_file_transfer(protocol_type, download, state, files_opt);
        self.handle_result(res, true);
    }

    /*

                                    let mut protocol = protocol_type.create();
                                match protocol.initiate_send(com, files, &self.current_transfer.unwrap()) {
                                    Ok(state) => {
                                        self.mode = MainWindowMode::FileTransfer(download);
    //                                    let a =(protocol, )));

    self.current_transfer = Some(Arc::new(Mutex::new(state)));
    }
                                    Err(error) => {
                                        log::error!("{}", error);
                                    }
                                }

        */
    pub(crate) fn initiate_file_transfer(
        &mut self,
        protocol_type: crate::protocol::TransferType,
        download: bool,
    ) {
        self.mode = MainWindowMode::ShowTerminal;
        if self.connection_opt.is_disconnected() {
            return;
        }
        if download {
            self.start_transfer_thread(protocol_type, download, None);
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let files = rfd::FileDialog::new().pick_files();
                if let Some(path) = files {
                    let fd = FileDescriptor::from_paths(&path);
                    if let Ok(files) = fd {
                        self.start_transfer_thread(protocol_type, download, Some(files));
                    }
                }
            }
        }
    }

    pub fn set_screen_mode(&mut self, mode: ScreenMode) {
        self.screen_mode = mode;
        mode.set_mode(self);
    }

    pub fn show_terminal(&mut self) {
        self.mode = MainWindowMode::ShowTerminal;
    }

    pub fn show_phonebook(&mut self) {
        self.mode = MainWindowMode::ShowPhonebook;
    }

    pub fn get_address_mut(&mut self, uuid: Option<usize>) -> &mut Address {
        if uuid.is_none() {
            return &mut self.addresses[0];
        }

        let uuid = uuid.unwrap();
        for (i, adr) in self.addresses.iter().enumerate() {
            if adr.id == uuid {
                return &mut self.addresses[i];
            }
        }

        &mut self.addresses[0]
    }

    pub fn call_bbs_uuid(&mut self, uuid: Option<usize>) {
        if uuid.is_none() {
            self.call_bbs(0);
            return;
        }

        let uuid = uuid.unwrap();
        for (i, adr) in self.addresses.iter().enumerate() {
            if adr.id == uuid {
                self.call_bbs(i);
                return;
            }
        }
    }

    pub fn call_bbs(&mut self, i: usize) {
        self.mode = MainWindowMode::ShowTerminal;
        let mut adr = self.addresses[i].address.clone();
        if !adr.contains(':') {
            adr.push_str(":23");
        }
        self.addresses[i].number_of_calls += 1;
        self.addresses[i].last_call = Some(Utc::now());
        store_phone_book(&self.addresses).unwrap_or_default();

        let call_adr = self.addresses[i].clone();
        self.auto_login = AutoLogin::new(&call_adr.auto_login);
        self.auto_login.disabled = self.is_alt_pressed;
        self.buffer_view.lock().buf.clear();
        self.cur_addr = i;
        self.set_screen_mode(call_adr.screen_mode);
        self.buffer_parser = self.addresses[i].get_terminal_parser(&call_adr);
        self.has_baud_rate = self.addresses[i].baud_emulation != BaudEmulation::Off;

        self.buffer_view
            .lock()
            .buf
            .terminal_state
            .set_baud_rate(self.addresses[i].baud_emulation);

        self.buffer_view.lock().redraw_font();
        self.buffer_view.lock().redraw_palette();
        self.buffer_view.lock().redraw_view();
        self.buffer_view.lock().clear();

        self.println(&fl!(
            crate::LANGUAGE_LOADER,
            "connect-to",
            address = call_adr.address.clone()
        ))
        .unwrap_or_default();

        let timeout = self.options.connect_timeout;
        let window_size = self.screen_mode.get_window_size();

        self.connection_opt
            .Connect(call_adr, timeout, window_size)
            .unwrap_or_default();
    }

    pub fn select_bbs(&mut self, uuid: Option<usize>) {
        self.selected_bbs = uuid;
    }

    pub fn delete_selected_address(&mut self) {
        if let Some(uuid) = self.selected_bbs {
            for (i, adr) in self.addresses.iter().enumerate() {
                if adr.id == uuid {
                    self.addresses.remove(i);
                    break;
                }
            }
        }
        let res = store_phone_book(&self.addresses);
        self.handle_result(res, true);
    }

    pub fn update_state(&mut self) -> TerminalResult<()> {
        if self.connection_opt.is_disconnected() {
            return Ok(());
        }
        let data_opt = if self.connection_opt.is_data_available()? {
            Some(self.connection_opt.read_buffer())
        } else {
            None
        };

        if let Some(data) = data_opt {
            if self.capture_session {
                if let Ok(mut data_file) = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.options.capture_filename)
                {
                    if let Err(err) = data_file.write_all(&data) {
                        if !self.show_capture_error {
                            self.show_capture_error = true;
                            log::error!("{err}");
                        }
                    }
                }
            }

            for ch in data {
                if self.options.iemsi_autologin {
                    if let Some(adr) = self.addresses.get(self.cur_addr) {
                        if let Err(err) = self.auto_login.try_login(
                            &mut self.connection_opt,
                            adr,
                            ch,
                            &self.options,
                        ) {
                            log::error!("{err}");
                        }
                    }
                }

                /*
                match ch {
                    b'\\' => print!("\\\\"),
                    b'\n' => println!("\\n"),
                    b'\r' => print!("\\r"),
                    b'\"' => print!("\\\""),
                    _ => {
                        if ch < b' ' || ch == b'\x7F' {
                            print!("\\x{ch:02X}");
                        } else if ch > b'\x7F' {
                            print!("\\u{{{ch:02X}}}");
                        } else {
                            print!("{}", char::from_u32(ch as u32).unwrap());
                        }
                    }
                }*/

                if let Err(err) = self.print_char(ch) {
                    log::error!("{err}");
                }

                if let Some((protocol_type, download)) = self.auto_file_transfer.try_transfer(ch) {
                    self.initiate_file_transfer(protocol_type, download);
                    return Ok(());
                }
            }
        }

        self.auto_login.disabled |= self.is_alt_pressed;
        if self.options.iemsi_autologin {
            if let Some(adr) = self.addresses.get(self.cur_addr) {
                if self.connection_opt.is_connected() {
                    if let Err(err) = self.auto_login.run_autologin(&mut self.connection_opt, adr) {
                        log::error!("{err}");
                    }
                }
            }
        }

        Ok(())
    }

    pub fn hangup(&mut self) {
        self.connection_opt.disconnect().unwrap_or_default();
        self.mode = MainWindowMode::ShowPhonebook;
    }

    pub fn send_login(&mut self) {
        let user_name = self.addresses.get(self.cur_addr).unwrap().user_name.clone();
        let password = self.addresses.get(self.cur_addr).unwrap().password.clone();
        let mut cr: Vec<u8> = [self.buffer_parser.convert_from_unicode('\r') as u8].to_vec();
        for (k, v) in self.screen_mode.get_input_mode().cur_map() {
            if *k == Key::Enter as u32 {
                cr = v.to_vec();
                break;
            }
        }
        self.output_string(&user_name);
        let r = self.connection_opt.send(cr.clone());
        self.handle_result(r, false);
        self.output_string(&password);
        let r = self.connection_opt.send(cr);
        self.handle_result(r, false);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn update_title(&self, frame: &mut eframe::Frame) {
        if let MainWindowMode::ShowPhonebook = self.mode {
            frame.set_window_title(&crate::DEFAULT_TITLE);
        } else {
            let str = if self.connection_opt.is_connected() {
                let d = Instant::now().duration_since(self.connection_opt.get_connection_time());
                let sec = d.as_secs();
                let minutes = sec / 60;
                let hours = minutes / 60;
                let cur = &self.addresses[self.cur_addr];
                let t = format!("{:02}:{:02}:{:02}", hours, minutes % 60, sec % 60);
                let s = if cur.system_name.is_empty() {
                    cur.address.clone()
                } else {
                    cur.system_name.clone()
                };

                fl!(
                    crate::LANGUAGE_LOADER,
                    "title-connected",
                    version = crate::VERSION,
                    time = t,
                    name = s
                )
            } else {
                fl!(
                    crate::LANGUAGE_LOADER,
                    "title-offline",
                    version = crate::VERSION
                )
            };
            frame.set_window_title(str.as_str());
        }
    }

    pub(crate) fn show_settings(&mut self, in_phonebook: bool) {
        self.mode = MainWindowMode::ShowSettings(in_phonebook);
    }
}
