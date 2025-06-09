//! Contains the UI code for specific components (widgets).
//! This allows organising code and data structures related to a specific component together.
//!
//! Smaller widgets are in the top level module, larger widgets (that require external state, ...)
//! are kept in submodules.

mod asset_pack;

use crate::layout::EditorLayout;
use crate::state::UiState;
use bevy::prelude::ResMut;
use egui::{Context, TopBottomPanel};
use egui_dock::{DockArea, Style};

/// Render the docking layout containing the inspector and the main panels.
pub fn dock_layout(context: &mut Context, state: &mut ResMut<UiState>) {
    // construct an `EditorLayout` using our mutable world reference for rendering.
    // the `EditorLayout` struct has a strict lifetime bound to this scope and may not leak.
    let mut viewer = EditorLayout {};

    // render the `dock_state` in the `UiState` in a DockArea.
    DockArea::new(&mut state.dock_state)
        .style(Style::from_egui(context.style().as_ref()))
        .show(context, &mut viewer);
}

/// Renders the toolbar at the top of the screen with the main navigation.
pub fn toolbar(context: &mut Context) {
    TopBottomPanel::top("toolbar").show(context, move |ui| {
        ui.horizontal(|ui| {
            ui.style_mut().visuals.button_frame = false;

            let _ = ui.button("New");
            let _ = ui.button("Open");
            let _ = ui.menu_button("Window", |ui| {
                // TODO: checkboxes should reflect window state as well as add/remove them.
                ui.checkbox(&mut true, "Inspector");
                ui.checkbox(&mut true, "Assets");
                ui.checkbox(&mut true, "Levels");
                ui.checkbox(&mut true, "Layers");
            });
        });
    });
}
