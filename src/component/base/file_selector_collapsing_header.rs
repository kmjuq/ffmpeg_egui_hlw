use crate::component::base::file_dialog;

/// 通用的带文件选择的折叠面板UI组件
///
/// # 参数
/// * `header_title`: 折叠面板的标题
/// * `label_text`: 文件选择前的标签文本
/// * `file_dialog_state`: 文件对话框的状态
/// * `ctx`: egui 上下文
/// * `ui`: egui UI 上下文
pub fn file_selector_collapsing_header(
    header_title: &str,
    label_text: &str,
    file_dialog_state: &mut file_dialog::State,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
) {
    egui::CollapsingHeader::new(header_title)
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(label_text);
                file_dialog::component(file_dialog_state, ctx, ui);
            });
            file_dialog::file_dialog_selector_show(file_dialog_state, ctx, ui);
        });
}
