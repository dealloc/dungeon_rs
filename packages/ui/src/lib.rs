#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic, clippy::suspicious, clippy::complexity)]

mod controls;
mod editor_layout;
mod ui_state;
mod widgets;

use crate::ui_state::UiState;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::{EguiContextPass, EguiContexts, EguiPlugin};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

/// The UI plugin handles registering and setting up all user interface elements of the editor.
/// This module is unavailable in headless mode and should not contain any actual functionality,
/// just build the user interface.
#[derive(Default)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_systems(Startup, setup)
        .add_systems(EguiContextPass, editor_layout::editor_layout)
        .add_systems(EguiContextPass, editor_layout::inspector_layout)
        .add_systems(Update, controls::camera::camera);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut contexts: EguiContexts) {
    commands.insert_resource(UiState::new(&mut contexts, &asset_server));
}
