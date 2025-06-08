//! Defines the [`UIPlugin`] which inserts all UI related functionality into the bevy `App`.
use crate::camera::{camera_control_system, setup_ui_camera};
use crate::layout::render_editor_layout;
use crate::state::UiState;
use bevy::app::App;
use bevy::prelude::{Plugin, PostUpdate, Startup};
use bevy_egui::{EguiContextPass, EguiPlugin};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

/// A [Bevy](https://bevyengine.org/) plugin that adds UI to the app it's added to.
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(DefaultInspectorConfigPlugin);

        // Camera controls
        app.add_systems(PostUpdate, camera_control_system)
            .add_systems(Startup, setup_ui_camera);

        // editor docking layout
        app.insert_resource(UiState::default())
            .add_systems(EguiContextPass, render_editor_layout);
    }
}
