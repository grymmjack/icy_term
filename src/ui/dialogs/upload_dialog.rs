use eframe::egui::{self};
use egui_file::FileDialog;

use crate::{
    check_error,
    protocol::FileDescriptor,
    ui::{MainWindow, MainWindowMode},
};

#[derive(Default)]
pub struct DialogState {
    open_file_dialog: Option<FileDialog>,
    protocol_type: crate::protocol::TransferType,
}

impl MainWindow {
    pub fn init_upload_dialog(&mut self, protocol_type: crate::protocol::TransferType) {
        let mut dialog: FileDialog = FileDialog::open_file(None);
        dialog.open();
        self.upload_dialog.open_file_dialog = Some(dialog);
        self.upload_dialog.protocol_type = protocol_type;
        self.set_mode(MainWindowMode::ShowUploadDialog);
    }

    pub fn show_upload_dialog(&mut self, ctx: &egui::Context) {
        if ctx.input(|i: &egui::InputState| i.key_down(egui::Key::Escape)) {
            self.set_mode(MainWindowMode::ShowTerminal);
        }

        if let Some(dialog) = &mut self.upload_dialog.open_file_dialog {
            if dialog.show(ctx).selected() {
                if let Some(path) = dialog.path() {
                    if matches!(
                        self.upload_dialog.protocol_type,
                        crate::protocol::TransferType::Text
                    ) {
                        match std::fs::read(path) {
                            Ok(bytes) => {
                                let r = self.connection.as_mut().unwrap().send(bytes);
                                check_error!(self, r, true);
                            }
                            r => {
                                check_error!(self, r, true);
                            }
                        }
                        self.set_mode(MainWindowMode::ShowTerminal);
                        return;
                    }

                    let fd = FileDescriptor::from_paths(&vec![path.to_path_buf()]);
                    if let Ok(files) = fd {
                        self.start_file_transfer(
                            self.upload_dialog.protocol_type,
                            false,
                            Some(files),
                        );
                    }
                }
            }
        }
    }
}
