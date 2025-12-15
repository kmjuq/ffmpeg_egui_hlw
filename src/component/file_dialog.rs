#[derive(Default)]
pub struct State {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

pub fn component(state: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("logo图片");
        if ui.button("Open file…").clicked()
            && let Some(path) = rfd::FileDialog::new().pick_file()
        {
            state.picked_path = Some(path.display().to_string());
        }
        ui.end_row();
    });

    if let Some(picked_path) = &state.picked_path {
        ui.horizontal(|ui| {
            ui.label("Picked file:");
            ui.monospace(picked_path);
        });
    }

    // Show dropped files (if any):
    if !state.dropped_files.is_empty() {
        ui.group(|ui| {
            ui.label("Dropped files:");

            for file in &state.dropped_files {
                let mut info = if let Some(path) = &file.path {
                    path.display().to_string()
                } else if !file.name.is_empty() {
                    file.name.clone()
                } else {
                    "???".to_owned()
                };

                let mut additional_info = vec![];
                if !file.mime.is_empty() {
                    additional_info.push(format!("type: {}", file.mime));
                }
                if let Some(bytes) = &file.bytes {
                    additional_info.push(format!("{} bytes", bytes.len()));
                }
                if !additional_info.is_empty() {
                    info += &format!(" ({})", additional_info.join(", "));
                }

                ui.label(info);
            }
        });
    }

    preview_files_being_dropped(ctx);

    // Collect dropped files:
    ctx.input(|i| {
        if !i.raw.dropped_files.is_empty() {
            state.dropped_files.clone_from(&i.raw.dropped_files);
        }
    });
}

/// Preview hovering files:
pub fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::{Align2, Color32, Id, LayerId, Order, TextStyle};
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let content_rect = ctx.content_rect();
        painter.rect_filled(content_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            content_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
