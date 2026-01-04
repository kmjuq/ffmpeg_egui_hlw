use std::{fmt::Display, path::PathBuf};

use crate::component::util::truncate_str_with_ellipsis;

#[derive(Default)]
pub struct State {
    pub picked_files: Option<Vec<PathBuf>>,
    pub pick_type: PickType,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum PickType {
    #[default]
    File,
    Files,
    Folder,
    Folders,
}

impl Display for PickType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PickType::File => write!(f, "文件"),
            PickType::Files => write!(f, "多文件"),
            PickType::Folder => write!(f, "文件夹"),
            PickType::Folders => write!(f, "多文件夹"),
        }
    }
}

pub fn component(state: &mut State, _ctx: &egui::Context, ui: &mut egui::Ui) {
    if ui.button(format!("选择{}", state.pick_type)).clicked() {
        match state.pick_type {
            PickType::File => {
                if let Some(file) = rfd::FileDialog::new().pick_file() {
                    state.picked_files = Some(vec![file]);
                }
            }
            PickType::Files => {
                if let Some(files) = rfd::FileDialog::new().pick_files() {
                    state.picked_files = Some(files);
                }
            }
            PickType::Folder => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    state.picked_files = Some(vec![folder]);
                }
            }
            PickType::Folders => {
                if let Some(folders) = rfd::FileDialog::new().pick_folders() {
                    state.picked_files = Some(folders);
                }
            }
        }
    }
}

pub fn file_dialog_selector_show(state: &mut State, _ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.end_row();
    ui.separator();
    state.picked_files.iter().for_each(|files| {
        for file in files {
            let pathbuf_str = file.display().to_string();
            ui.label(truncate_str_with_ellipsis(&pathbuf_str, 30))
                .on_hover_text(pathbuf_str);
            ui.end_row();
        }
    });
}
