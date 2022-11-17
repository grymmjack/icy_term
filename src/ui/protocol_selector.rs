use super::Message;
use crate::protocol::ProtocolType;
use iced::widget::{button, column, horizontal_space, row, text, Row, Column};
use iced::{Element, Length};

const button_width: u16 = 120;
const space: u16 = 8;
const left: u16 = 20;


fn create_button_row<'a>(msg: Message, title: &'static str, descr: &'static str)  -> Element<'a, Message>  {
    Row::new()
        .push(horizontal_space(Length::Units(left)))
        .push(button(title)
            .on_press(msg)
            .width(Length::Units(button_width)))
        .push(horizontal_space(Length::Units(space)))
        .push(text(descr))
        .into()
}

pub fn view_protocol_selector<'a>(download: bool) -> Element<'a, Message> {
    
    let header = Row::new()
    .push(button("Cancel").on_press(Message::Back))
    .padding(4)
            .spacing(8);
    Column::new() 
    .push(header)
    .push(text(format!("Select {} protocol", if download { "download" } else { "upload" } )).size(40))
    .push(
        Column::new()
        .push(create_button_row(Message::SelectProtocol(ProtocolType::ZModem, download), "Zmodem", "The standard protocol"))
        .push(create_button_row(Message::SelectProtocol(ProtocolType::ZedZap, download), "ZedZap", "8k Zmodem"))
        .push(create_button_row(Message::SelectProtocol(ProtocolType::XModem, download), "Xmodem", "Outdated protocol"))
        .push(create_button_row(Message::SelectProtocol(ProtocolType::XModem1k, download), "Xmodem 1k", "Rarely used anymore"))
        .push(create_button_row(Message::SelectProtocol(ProtocolType::XModem1kG, download), "Xmodem 1k-G", "Does that even exist?"))
        .push(create_button_row(Message::SelectProtocol(ProtocolType::YModem, download), "Ymodem", "Ok but Zmodem is better"))
        .push(create_button_row(Message::SelectProtocol(ProtocolType::YModemG, download), "Ymodem-G", "A fast Ymodem variant"))
        .spacing(8))
    .into()
}
