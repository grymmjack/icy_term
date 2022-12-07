use std::cmp::max;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use eframe::egui::{self, ProgressBar, RichText};
use eframe::epaint::{Color32, FontId};
use egui_extras::{Column, TableBuilder};
use gabi::BytesConfig;

use crate::protocol::TransferState;

pub fn view_file_transfer(
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
    transfer_state: &Arc<Mutex<TransferState>>,
    download: bool,
) -> bool {
    let mut open = true;
    let text_style = FontId::proportional(24.);
    let title = RichText::new(if download { "Download" } else { "Upload" }).font(text_style);

    egui::Window::new(title)
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            let state = transfer_state.lock().unwrap();
            let transfer_info = if download {
                &state.recieve_state
            } else {
                &state.send_state
            };

            let check = transfer_info.check_size.clone();
            let file_name = transfer_info.file_name.clone();
            let current_state = state.current_state.to_string();

            let bps = transfer_info.get_bps();
            let bytes_left = transfer_info
                .file_size
                .saturating_sub(transfer_info.bytes_transfered);
            let time_left = Duration::from_secs(bytes_left as u64 / max(1, bps));

            let bb = BytesConfig::default();

            let elapsed_time = SystemTime::now().duration_since(state.start_time).unwrap();
            let elapsed_time = format!(
                "{:02}:{:02}",
                elapsed_time.as_secs() / 60,
                elapsed_time.as_secs() % 60
            );
            /*
            let log = column(
                transfer_state
                    .output_log
                    .iter()
                    .rev()
                    .take(1)
                    .rev()
                    .map(|txt| row![text(txt)].align_items(Alignment::Center).into())
                    .collect(),
            )
            .spacing(10);*/

            if state.is_finished {
                ui.label("Completed");
            }
            let text_style = FontId::proportional(18.);

            let table = TableBuilder::new(ui)
                .striped(false)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .resizable(false);
            let row_height = 30.;

            table.body(|mut body| {
                body.row(row_height, |mut row| {
                    row.col(|ui| {
                        ui.label(RichText::new("Protocol:").font(text_style.clone()));
                        ui.label(
                            RichText::new(state.protocol_name.clone())
                                .font(text_style.clone())
                                .color(Color32::WHITE),
                        );
                    });

                    row.col(|ui| {
                        ui.label(RichText::new("Total errors:").font(text_style.clone()));
                        ui.label(
                            RichText::new(transfer_info.errors.to_string())
                                .font(text_style.clone())
                                .color(Color32::WHITE),
                        );
                    });
                });

                body.row(row_height, |mut row| {
                    row.col(|ui| {
                        ui.label(RichText::new("Check/size:").font(text_style.clone()));
                        ui.label(
                            RichText::new(check)
                                .font(text_style.clone())
                                .color(Color32::WHITE),
                        );
                    });

                    row.col(|ui| {
                        ui.label(RichText::new("Elapsed time:").font(text_style.clone()));
                        ui.label(
                            RichText::new(elapsed_time)
                                .font(text_style.clone())
                                .color(Color32::WHITE),
                        );
                    });
                });

                body.row(row_height, |mut row| {
                    row.col(|ui| {
                        ui.label(RichText::new("State:").font(text_style.clone()));
                        ui.label(
                            RichText::new(current_state)
                                .font(text_style.clone())
                                .color(Color32::WHITE),
                        );
                    });

                    row.col(|ui| {
                        ui.label(RichText::new("Time left:").font(text_style.clone()));
                        ui.label(
                            RichText::new(format!(
                                "{:02}:{:02}",
                                time_left.as_secs() / 60,
                                time_left.as_secs() % 60
                            ))
                            .font(text_style.clone())
                            .color(Color32::WHITE),
                        );
                    });
                });
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("File:").font(text_style.clone()));
                ui.label(
                    RichText::new(file_name)
                        .font(text_style.clone())
                        .color(Color32::WHITE),
                );
            });
            ui.add(
                ProgressBar::new(
                    transfer_info.bytes_transfered as f32 / transfer_info.file_size as f32,
                )
                .text(
                    RichText::new(format!(
                        "{}% {}/{}",
                        (transfer_info.bytes_transfered * 100) / max(1, transfer_info.file_size),
                        bb.bytes(transfer_info.bytes_transfered as u64),
                        bb.bytes(transfer_info.file_size as u64)
                    ))
                    .font(text_style.clone()),
                ),
            );
            ui.horizontal(|ui| {
                ui.label(RichText::new("transfer rate:").font(text_style.clone()));
                ui.label(
                    RichText::new(format!("{} per second", bb.bytes(bps as u64)))
                        .font(text_style.clone())
                        .color(Color32::WHITE),
                );
            });
        });
    open
}
