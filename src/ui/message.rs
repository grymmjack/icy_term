use iced::{keyboard::{KeyCode, Modifiers}, mouse::ScrollDelta};

use crate::{protocol::ProtocolType, address::{Terminal, ConnectionType}};

use super::{screen_modes::ScreenMode, selection::Selection};

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    InitiateFileTransfer(bool),
    SendLogin,
    Back,
    Hangup,
    Copy,
    Paste,
    CharacterReceived(char),
    KeyPressed(KeyCode, Modifiers),
    WheelScrolled(ScrollDelta),
    FontSelected(String),
    ScreenModeSelected(ScreenMode),
    SelectProtocol(ProtocolType, bool),
    OpenURL(String),
    CancelTransfer,
    
    SetSelection(Option<Selection>),

    // Phonebook
    ShowPhonebook,
    QuickConnectChanged(String),
    CallBBS(usize),

    // Edit BBS 
    EditBBS(usize),
    EditBbsSystemNameChanged(String),
    EditBbsAddressChanged(String),
    EditBbsUserNameChanged(String),
    EditBbsPasswordChanged(String),
    EditBbsCommentChanged(String),
    EditBbsTerminalTypeSelected(Terminal),
    EditBbsScreenModeSelected(ScreenMode),
    EditBbsAutoLoginChanged(String),
    EditBbsSaveChanges(usize),
    EditBbsDeleteEntry(usize),
    EditBbsConnectionType(ConnectionType)
}