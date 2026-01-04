use egui::CollapsingHeader;

use crate::component::base::file_dialog;

#[derive(Default)]
pub struct State {
    pub file_dialog: file_dialog::State,
}

impl State {
    pub fn ui(self: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) {
        CollapsingHeader::new("添加水印")
            .default_open(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("选择水印:");
                    file_dialog::component(&mut self.file_dialog, ctx, ui);
                });
                file_dialog::file_dialog_selector_show(&mut self.file_dialog, ctx, ui);
            });
    }
}
