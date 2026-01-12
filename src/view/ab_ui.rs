use crate::{
    biz::ffmpeg,
    component::{self, biz::water_mark},
};

pub struct State {
    pub water_mark: water_mark::State,
    pub ab: component::biz::ab::State,
    pub output_dir: component::base::file_dialog::State,
}

impl Default for State {
    fn default() -> Self {
        Self {
            output_dir: component::base::file_dialog::State {
                pick_type: component::base::file_dialog::PickType::Folder,
                ..Default::default()
            },
            water_mark: Default::default(),
            ab: Default::default(),
        }
    }
}

impl State {
    pub fn ui(self: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.water_mark.ui(ctx, ui);
        self.ab.ui(ctx, ui);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("输出文件夹:");
            component::base::file_dialog::component(&mut self.output_dir, ctx, ui);
            if ui.button("融帧").clicked() {
                ffmpeg::ab(self);
            }
        });
        component::base::file_dialog::file_dialog_selector_show(&mut self.output_dir, ctx, ui);
    }
}
