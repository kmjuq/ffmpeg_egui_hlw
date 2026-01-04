use crate::component::biz::water_mark;

#[derive(Default)]
pub struct State {
    pub water_mark: water_mark::State,
}

impl State {
    pub fn ui(self: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.water_mark.ui(ctx, ui);
    }
}
