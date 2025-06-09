//! Contains the rendering logic for the [`crate::layout::EditorPanels::Inspector`] pane.

use crate::layout::EditorLayout;
use egui::{DragValue, Ui, Widget};

/// Handles the rendering of the [`crate::layout::EditorPanels::Inspector`] tab.
pub(super) fn render(viewer: &mut EditorLayout, ui: &mut Ui) {
    ui.label("X:");

    DragValue::new(&mut viewer.entity.translation.x).ui(ui);
}
