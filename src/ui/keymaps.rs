use eframe::egui::Key;

pub const CTRL_MOD: u32 = 0b1000_0000_0000_0000_0000;
pub const SHIFT_MOD: u32 = 0b0100_0000_0000_0000_0000;

pub static ANSI_KEY_MAP: &[(u32, &[u8])] = &[
    (Key::Escape as u32, &[0x1B]),
    (Key::Home as u32, b"\x1b[H"),
    (Key::Insert as u32, b"\x1b[@"),
    (Key::Backspace as u32, &[8]),
    (Key::Enter as u32, &[b'\r']),
    (Key::Tab as u32, &[9]),
    (Key::Tab as u32 | SHIFT_MOD, b"\x1b[Z"),
    (Key::Delete as u32, &[127]),
    (Key::Insert as u32, b"\x1b[@"),
    (Key::End as u32, b"\x1b[K"),
    (Key::PageUp as u32, b"\x1b[V"),
    (Key::PageDown as u32, b"\x1b[U"),
    (Key::F1 as u32, b"\x1b[OP"),
    (Key::F2 as u32, b"\x1b[OQ"),
    (Key::F3 as u32, b"\x1b[OR"),
    (Key::F4 as u32, b"\x1b[OS"),
    (Key::F5 as u32, b"\x1b[OT"),
    (Key::F6 as u32, b"\x1b[17~"),
    (Key::F7 as u32, b"\x1b[18~"),
    (Key::F8 as u32, b"\x1b[19~"),
    (Key::F9 as u32, b"\x1b[20~"),
    (Key::F10 as u32, b"\x1b[21~"),
    (Key::F11 as u32, b"\x1b[23~"),
    (Key::F12 as u32, b"\x1b[24~"),
    (Key::ArrowUp as u32, b"\x1b[A"),
    (Key::ArrowDown as u32, b"\x1b[B"),
    (Key::ArrowRight as u32, b"\x1b[C"),
    (Key::ArrowLeft as u32, b"\x1b[D"),
    (Key::A as u32 | CTRL_MOD, &[1]),
    (Key::B as u32 | CTRL_MOD, &[2]),
    (Key::C as u32 | CTRL_MOD, &[3]),
    (Key::D as u32 | CTRL_MOD, &[4]),
    (Key::E as u32 | CTRL_MOD, &[5]),
    (Key::F as u32 | CTRL_MOD, &[6]),
    (Key::G as u32 | CTRL_MOD, &[7]),
    (Key::H as u32 | CTRL_MOD, &[8]),
    (Key::I as u32 | CTRL_MOD, &[9]),
    (Key::J as u32 | CTRL_MOD, &[10]),
    (Key::K as u32 | CTRL_MOD, &[11]),
    (Key::L as u32 | CTRL_MOD, &[12]),
    (Key::M as u32 | CTRL_MOD, &[13]),
    (Key::N as u32 | CTRL_MOD, &[14]),
    (Key::O as u32 | CTRL_MOD, &[15]),
    (Key::P as u32 | CTRL_MOD, &[16]),
    (Key::Q as u32 | CTRL_MOD, &[17]),
    (Key::R as u32 | CTRL_MOD, &[18]),
    (Key::S as u32 | CTRL_MOD, &[19]),
    (Key::T as u32 | CTRL_MOD, &[20]),
    (Key::U as u32 | CTRL_MOD, &[21]),
    (Key::V as u32 | CTRL_MOD, &[22]),
    (Key::W as u32 | CTRL_MOD, &[23]),
    (Key::X as u32 | CTRL_MOD, &[24]),
    (Key::Y as u32 | CTRL_MOD, &[25]),
    (Key::Z as u32 | CTRL_MOD, &[26]),
    (Key::Num2 as u32 | CTRL_MOD, &[0]),
    (Key::Num3 as u32 | CTRL_MOD, &[0x1B]),
    (Key::Num4 as u32 | CTRL_MOD, &[0x1C]),
    (Key::Num5 as u32 | CTRL_MOD, &[0x1D]),
    (Key::Num6 as u32 | CTRL_MOD, &[0x1E]),
    (Key::Num7 as u32 | CTRL_MOD, &[0x1F]),
];

pub static C64_KEY_MAP: &[(u32, &[u8])] = &[
    (Key::Escape as u32, &[0x1B]),
    (Key::Home as u32, &[0x13]),
    (Key::Enter as u32, &[b'\r']),
    (Key::Insert as u32, &[0x94]),
    (Key::Backspace as u32, &[0x14]),
    (Key::Delete as u32, &[0x14]),
    (Key::F1 as u32, &[0x85]),
    (Key::F2 as u32, &[0x86]),
    (Key::F3 as u32, &[0x87]),
    (Key::F4 as u32, &[0x88]),
    (Key::F5 as u32, &[0x89]),
    (Key::F6 as u32, &[0x8A]),
    (Key::F7 as u32, &[0x8B]),
    (Key::F8 as u32, &[0x8C]),
    (Key::ArrowUp as u32, &[0x91]),
    (Key::ArrowDown as u32, &[0x11]),
    (Key::ArrowRight as u32, &[0x1D]),
    (Key::ArrowLeft as u32, &[0x9D]),
];

pub static ATASCII_KEY_MAP: &[(u32, &[u8])] = &[
    (Key::Escape as u32, &[0x1B]),
    (Key::Enter as u32, &[155]),
    (Key::Backspace as u32, &[0x1b, 0x7e]),
    (Key::End as u32, &[0x1b, 0x9b]),
    (Key::ArrowUp as u32, &[0x1b, 0x1c]),
    (Key::ArrowDown as u32, &[0x1b, 0x1d]),
    (Key::ArrowRight as u32, &[0x1b, 0x1f]),
    (Key::ArrowLeft as u32, &[0x1b, 0x1e]),
    (Key::A as u32 | CTRL_MOD, &[1]),
    (Key::B as u32 | CTRL_MOD, &[2]),
    (Key::C as u32 | CTRL_MOD, &[3]),
    (Key::D as u32 | CTRL_MOD, &[4]),
    (Key::E as u32 | CTRL_MOD, &[5]),
    (Key::F as u32 | CTRL_MOD, &[6]),
    (Key::G as u32 | CTRL_MOD, &[7]),
    (Key::H as u32 | CTRL_MOD, &[8]),
    (Key::I as u32 | CTRL_MOD, &[9]),
    (Key::J as u32 | CTRL_MOD, &[10]),
    (Key::K as u32 | CTRL_MOD, &[11]),
    (Key::L as u32 | CTRL_MOD, &[12]),
    (Key::M as u32 | CTRL_MOD, &[13]),
    (Key::N as u32 | CTRL_MOD, &[14]),
    (Key::O as u32 | CTRL_MOD, &[15]),
    (Key::P as u32 | CTRL_MOD, &[16]),
    (Key::Q as u32 | CTRL_MOD, &[17]),
    (Key::R as u32 | CTRL_MOD, &[18]),
    (Key::S as u32 | CTRL_MOD, &[19]),
    (Key::T as u32 | CTRL_MOD, &[20]),
    (Key::U as u32 | CTRL_MOD, &[21]),
    (Key::V as u32 | CTRL_MOD, &[22]),
    (Key::W as u32 | CTRL_MOD, &[23]),
    (Key::X as u32 | CTRL_MOD, &[24]),
    (Key::Y as u32 | CTRL_MOD, &[25]),
    (Key::Z as u32 | CTRL_MOD, &[26]),
//    (Key::Period as u32 | CTRL_MOD, &[96]),
//    (Key::Colon as u32 | CTRL_MOD, &[13]),
];

pub static VT500_KEY_MAP: &[(u32, &[u8])] = &[
    (Key::Escape as u32, &[0x1B]),
    (Key::Home as u32, b"\x1b[1~"),
    (Key::Insert as u32, b"\x1b[2~"),
    (Key::Backspace as u32, &[8]),
    (Key::Enter as u32, &[b'\r', b'\n']),
    (Key::Tab as u32, &[9]),
    (Key::Tab as u32 | SHIFT_MOD, b"\x1b[Z"),
    (Key::Delete as u32, b"\x1b[3~"),
    (Key::End as u32, b"\x1b[4~"),
    (Key::PageUp as u32, b"\x1b[5~"),
    (Key::PageDown as u32, b"\x1b[6~"),
    (Key::F1 as u32, b"\x1b[OP"),
    (Key::F2 as u32, b"\x1b[OQ"),
    (Key::F3 as u32, b"\x1b[OR"),
    (Key::F4 as u32, b"\x1b[OS"),
    (Key::F5 as u32, b"\x1b[15~"),
    (Key::F6 as u32, b"\x1b[17~"),
    (Key::F7 as u32, b"\x1b[18~"),
    (Key::F8 as u32, b"\x1b[19~"),
    (Key::F9 as u32, b"\x1b[20~"),
    (Key::F10 as u32, b"\x1b[21~"),
    (Key::F11 as u32, b"\x1b[23~"),
    (Key::F12 as u32, b"\x1b[24~"),
    (Key::ArrowUp as u32, b"\x1b[A"),
    (Key::ArrowDown as u32, b"\x1b[B"),
    (Key::ArrowRight as u32, b"\x1b[C"),
    (Key::ArrowLeft as u32, b"\x1b[D"),
    (Key::A as u32 | CTRL_MOD, &[1]),
    (Key::B as u32 | CTRL_MOD, &[2]),
    (Key::C as u32 | CTRL_MOD, &[3]),
    (Key::D as u32 | CTRL_MOD, &[4]),
    (Key::E as u32 | CTRL_MOD, &[5]),
    (Key::F as u32 | CTRL_MOD, &[6]),
    (Key::G as u32 | CTRL_MOD, &[7]),
    (Key::H as u32 | CTRL_MOD, &[8]),
    (Key::I as u32 | CTRL_MOD, &[9]),
    (Key::J as u32 | CTRL_MOD, &[10]),
    (Key::K as u32 | CTRL_MOD, &[11]),
    (Key::L as u32 | CTRL_MOD, &[12]),
    (Key::M as u32 | CTRL_MOD, &[13]),
    (Key::N as u32 | CTRL_MOD, &[14]),
    (Key::O as u32 | CTRL_MOD, &[15]),
    (Key::P as u32 | CTRL_MOD, &[16]),
    (Key::Q as u32 | CTRL_MOD, &[17]),
    (Key::R as u32 | CTRL_MOD, &[18]),
    (Key::S as u32 | CTRL_MOD, &[19]),
    (Key::T as u32 | CTRL_MOD, &[20]),
    (Key::U as u32 | CTRL_MOD, &[21]),
    (Key::V as u32 | CTRL_MOD, &[22]),
    (Key::W as u32 | CTRL_MOD, &[23]),
    (Key::X as u32 | CTRL_MOD, &[24]),
    (Key::Y as u32 | CTRL_MOD, &[25]),
    (Key::Z as u32 | CTRL_MOD, &[26]),
    (Key::Num2 as u32 | CTRL_MOD, &[0]),
    (Key::Num3 as u32 | CTRL_MOD, &[0x1B]),
    (Key::Num4 as u32 | CTRL_MOD, &[0x1C]),
    (Key::Num5 as u32 | CTRL_MOD, &[0x1D]),
    (Key::Num6 as u32 | CTRL_MOD, &[0x1E]),
    (Key::Num7 as u32 | CTRL_MOD, &[0x1F]),
];

pub static VIDEOTERM_KEY_MAP: &[(u32, &[u8])] = &[
    (Key::Home as u32, &[0x13]),
    (Key::Enter as u32, &[b'_']),
    (Key::Insert as u32, &[0x94]),
    (Key::Backspace as u32, &[0x7F]),
    (Key::Delete as u32, &[0x7F]),
    (Key::Escape as u32, &[0x1B]),
    (Key::F1 as u32, &[b'*']),
    (Key::F2 as u32, &[0b101_1111]),

    (Key::ArrowUp as u32, &[0x0B]),
    (Key::ArrowDown as u32, &[b'\n']),
    (Key::ArrowRight as u32, &[b'\t']),
    (Key::ArrowLeft as u32, &[0x08]),
];
