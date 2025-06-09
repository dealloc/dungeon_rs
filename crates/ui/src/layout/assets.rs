//! Contains the rendering logic for the [`crate::layout::EditorPanels::Assets`] pane.
use crate::layout::EditorLayout;
use crate::widgets::AssetPackBuilder;
use egui::Ui;

/// Handles the rendering of the [`crate::layout::EditorPanels::Assets`] tab.
pub(super) fn render(viewer: &mut EditorLayout, ui: &mut Ui) {
    if ui.button("Add asset pack builder").clicked() {
        viewer.commands.spawn(AssetPackBuilder::default());
    }
}
