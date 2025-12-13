#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use ffmpeg_next as ffmpeg;

use crate::font::setup_chinese_fonts;

mod file_dialog;
mod font;

fn main() -> eframe::Result {
    // 初始化 FFmpeg（自动）
    ffmpeg::init().expect("无法初始化FFmpeg");
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

#[derive(Default)]
struct MyApp {
    tab: Menu,
    show_log: bool,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

#[derive(Default, PartialEq)]
enum Menu {
    #[default]
    AddLogo,
    AB,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ctx,ui);
        });
    }
}

impl MyApp {

    fn ui(&mut self, ctx: &egui::Context,ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tab, Menu::AddLogo, "Menu::AddLogo");
            ui.selectable_value(&mut self.tab, Menu::AB, "Menu::AB");
        });
        file_dialog::file_dialog(self, ctx, ui);
    }
}