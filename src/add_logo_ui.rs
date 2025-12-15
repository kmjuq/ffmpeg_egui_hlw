use crate::component::file_dialog;

#[derive(Default)]
pub struct State {
    pub file_dialog: file_dialog::State,
}

impl State {
    pub fn ui(self: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Grid::new("my_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Logo路径:");
                ui.text_edit_singleline(&mut "请输入logo路径");
                ui.end_row();
                file_dialog::component(&mut self.file_dialog, ctx, ui);
                ui.end_row();
            });
    }
}
