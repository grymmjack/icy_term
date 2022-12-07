use std::fmt::Display;

use icy_engine::{
    AnsiParser, AtasciiParser, AvatarParser, BitFont, PETSCIIParser, Palette, ViewdataParser,
    ATARI_DEFAULT_PALETTE, C64_DEFAULT_PALETTE, VIEWDATA_PALETTE,
};
use serde_derive::{Deserialize, Serialize};

use super::{main_window::MainWindow, BufferInputMode};

//use super::{BufferInputMode, BufferView};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "name", content = "par")]
pub enum ScreenMode {
    DOS(i32, i32),
    C64,
    C128(i32),
    Atari,
    AtariXep80,
    VT500,
    Viewdata,
}

pub const DEFAULT_MODES: [ScreenMode; 22] = [
    ScreenMode::DOS(80, 25),
    ScreenMode::DOS(80, 28),
    ScreenMode::DOS(80, 30),
    ScreenMode::DOS(80, 43),
    ScreenMode::DOS(80, 50),
    ScreenMode::DOS(80, 60),
    ScreenMode::DOS(132, 37),
    ScreenMode::DOS(132, 52),
    ScreenMode::DOS(132, 25),
    ScreenMode::DOS(132, 28),
    ScreenMode::DOS(132, 30),
    ScreenMode::DOS(132, 34),
    ScreenMode::DOS(132, 43),
    ScreenMode::DOS(132, 50),
    ScreenMode::DOS(132, 60),
    ScreenMode::C64,
    ScreenMode::C128(40),
    ScreenMode::C128(80),
    ScreenMode::Atari,
    ScreenMode::AtariXep80,
    ScreenMode::VT500,
    ScreenMode::Viewdata,
];

impl Display for ScreenMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScreenMode::DOS(w, h) => write!(f, "{}x{}", w, h),
            ScreenMode::C64 => write!(f, "C64"),
            ScreenMode::C128(col) => write!(f, "C128 ({} col)", col),
            ScreenMode::Atari => write!(f, "Atari"),
            ScreenMode::AtariXep80 => write!(f, "Atari XEP80"),
            ScreenMode::VT500 => write!(f, "VT500"),
            ScreenMode::Viewdata => write!(f, "Viewdata"),
        }
    }
}

impl ScreenMode {
    pub fn get_input_mode(&self) -> BufferInputMode {
        match self {
            ScreenMode::DOS(_, _) => BufferInputMode::CP437,
            ScreenMode::C64 | ScreenMode::C128(_) => BufferInputMode::PETSCII,
            ScreenMode::Atari | ScreenMode::AtariXep80 => BufferInputMode::ATASCII,
            ScreenMode::VT500 => BufferInputMode::VT500,
            ScreenMode::Viewdata => BufferInputMode::VIEWDATA,
        }
    }

    pub fn set_mode(&self, main_window: &mut MainWindow) {
        let buf = &mut main_window.buffer_view.lock().buf;
        match self {
            ScreenMode::DOS(w, h) => {
                buf.set_buffer_width(*w);
                buf.set_buffer_height(*h);

                buf.font_table.clear();
                buf.font_table.push(
                    BitFont::from_name(if *h >= 50 { "IBM VGA50" } else { "IBM VGA" }).unwrap(),
                );

                main_window.buffer_parser = Box::new(AvatarParser::new(true));
                buf.palette = Palette::new();
            }
            ScreenMode::C64 => {
                buf.set_buffer_width(40);
                buf.set_buffer_height(25);
                buf.font_table.clear();
                buf.font_table
                    .push(BitFont::from_name("C64 PETSCII unshifted").unwrap());
                buf.font_table
                    .push(BitFont::from_name("C64 PETSCII shifted").unwrap());
                main_window.buffer_parser = Box::new(PETSCIIParser::new());
                buf.palette = Palette {
                    colors: C64_DEFAULT_PALETTE.to_vec(),
                };
            }
            ScreenMode::C128(col) => {
                buf.set_buffer_width(*col);
                buf.set_buffer_height(25);
                buf.font_table.clear();
                buf.font_table
                    .push(BitFont::from_name("C64 PETSCII unshifted").unwrap());
                buf.font_table
                    .push(BitFont::from_name("C64 PETSCII shifted").unwrap());
                main_window.buffer_parser = Box::new(PETSCIIParser::new());
                buf.palette = Palette {
                    colors: C64_DEFAULT_PALETTE.to_vec(),
                };
            }
            ScreenMode::Atari => {
                buf.set_buffer_width(40);
                buf.set_buffer_height(24);
                buf.font_table.clear();
                buf.font_table
                    .push(BitFont::from_name("Atari ATASCII").unwrap());

                main_window.buffer_parser = Box::new(AtasciiParser::new());
                buf.palette = Palette {
                    colors: ATARI_DEFAULT_PALETTE.to_vec(),
                };
            }
            ScreenMode::AtariXep80 => {
                buf.set_buffer_width(80);
                buf.set_buffer_height(25);
                buf.font_table.clear();
                buf.font_table
                    .push(BitFont::from_name("Atari ATASCII").unwrap());
                main_window.buffer_parser = Box::new(AtasciiParser::new());
                buf.palette = Palette {
                    colors: ATARI_DEFAULT_PALETTE.to_vec(),
                };
            }
            ScreenMode::VT500 => {
                buf.set_buffer_width(80);
                buf.set_buffer_height(25);
                buf.font_table.clear();
                buf.font_table.push(BitFont::from_name("IBM VGA").unwrap());
                main_window.buffer_parser = Box::new(AnsiParser::new());
                buf.palette = Palette::new();
            }
            ScreenMode::Viewdata => {
                buf.set_buffer_width(40);
                buf.set_buffer_height(24);
                buf.font_table.clear();
                buf.font_table.push(BitFont::from_name("Viewdata").unwrap());
                main_window.buffer_parser = Box::new(ViewdataParser::new());
                buf.palette = Palette {
                    colors: VIEWDATA_PALETTE.to_vec(),
                };
            }
        }
        buf.clear();
    }
}
