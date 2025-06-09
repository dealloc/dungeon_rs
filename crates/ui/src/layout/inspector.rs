//! Contains the rendering logic for the [`crate::layout::EditorPanels::Inspector`] pane.

use crate::layout::EditorLayout;
use egui::Ui;

/// Handles the rendering of the [`crate::layout::EditorPanels::Inspector`] tab.
pub(super) fn render(_viewer: &mut EditorLayout, ui: &mut Ui) {
    ui.label("Inspector");
}
