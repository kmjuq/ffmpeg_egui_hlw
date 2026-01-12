use egui::CollapsingHeader;

use crate::component::base::{
    file_dialog, file_selector_collapsing_header::file_selector_collapsing_header,
};

pub struct State {
    pub file_dialog: file_dialog::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            file_dialog: file_dialog::State {
                pick_type: file_dialog::PickType::Files,
                clear_button_visible: true,
                copy_button_visible: true,
                ..Default::default()
            },
        }
    }
}

impl State {
    pub fn ui(self: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) {
        file_selector_collapsing_header("融帧设置", "选择多文件:", &mut self.file_dialog, ctx, ui);
    }
}
