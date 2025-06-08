//! Contains the rendering logic for the [`crate::layout::EditorPanels::Inspector`] pane.

use crate::layout::EditorLayout;
use bevy_inspector_egui::bevy_inspector::ui_for_world;
use egui::Ui;

/// Handles the rendering of the [`crate::layout::EditorPanels::Inspector`] tab.
pub(super) fn render(viewer: &mut EditorLayout, ui: &mut Ui) {
    ui_for_world(viewer.world, ui);
}
