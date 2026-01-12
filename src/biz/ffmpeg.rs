use crate::view;

pub fn ab(state: &view::ab_ui::State) {
    println!("ffmpeg ab view");
    println!(
        "water_mark file: {:?}",
        state.water_mark.file_dialog.picked_files
    );
}
