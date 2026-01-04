#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

use crate::{component::biz::water_mark, config::font::setup_chinese_fonts, view::ab_ui};

mod component;
mod config;
mod view;

fn main() -> eframe::Result {
    // 初始化 FFmpeg（自动）
    // ffmpeg::init().expect("无法初始化FFmpeg");
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "ffmpeg_egui_hlw",
        options,
        Box::new(|cc| {
            // Setup Chinese fonts - this is all you need!
            if let Err(e) = setup_chinese_fonts(&cc.egui_ctx) {
                eprintln!("Failed to load Chinese fonts: {}", e);
            }
            Ok(Box::new(MyApp::default()))
        }),
    )
}

struct MyApp {
    tab: Menu,
    ab_state: ab_ui::State,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            tab: Menu::AB,
            ab_state: ab_ui::State {
                water_mark: water_mark::State {
                    file_dialog: component::base::file_dialog::State {
                        pick_type: component::base::file_dialog::PickType::File,
                        ..Default::default()
                    },
                },
                ..Default::default()
            },
        }
    }
}

#[derive(Default, PartialEq)]
enum Menu {
    #[default]
    AB,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ctx, ui);
        });
    }
}

impl MyApp {
    fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tab, Menu::AB, "AB融帧");
        });
        match self.tab {
            Menu::AB => self.ab_state.ui(ctx, ui),
            _ => (),
        }
    }
}
