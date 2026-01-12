use std::{fmt::Display, path::PathBuf};

use crate::component::util::truncate_str_with_ellipsis;

#[derive(Default)]
pub struct State {
    pub picked_files: Option<Vec<PathBuf>>,
    pub pick_type: PickType,
    pub clear_button_visible: bool,
    pub copy_button_visible: bool,
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
    if state.clear_button_visible && ui.button("清空").clicked() {
        clear_picked_files(state);
    }
    if state.copy_button_visible && ui.button("📋").clicked() {
        if let Some(picked_files) = &state.picked_files {
            let all_paths = picked_files
                .iter()
                .map(|paths| {
                    paths
                        .iter()
                        .map(|pathbuf| pathbuf.display().to_string())
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n");
            ui.ctx().copy_text(all_paths);
        }
    }
}

pub fn file_dialog_selector_show(state: &mut State, _ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.end_row();
    ui.separator();
    state.picked_files.iter().for_each(|files| {
        for file in files {
            let pathbuf_str = file.display().to_string();
            ui.horizontal(|ui| {
                if ui.button("📋").clicked() {
                    ui.ctx().copy_text(pathbuf_str.clone());
                }
                ui.label(truncate_str_with_ellipsis(&pathbuf_str, 30))
                    .on_hover_text(pathbuf_str);
            });
            ui.end_row();
        }
    });
}

pub fn clear_picked_files(state: &mut State) {
    state.picked_files = None;
}
