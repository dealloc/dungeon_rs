#![doc = include_str!("../README.md")]

use bevy::prelude::*;
use config::Configuration;
use io::IOPlugin;
use logging::log_plugin;
use ui::UIPlugin;

/// Main entry point for the editor.
///
/// # Panics
/// The application will panic when a configuration error occurs, or when Bevy panics, specific
/// circumstances for when Bevy panics can be found in Bevy's documentation.
fn main() -> AppExit {
    let config = match Configuration::load() {
        Ok(cfg) => cfg,
        Err(err) => panic!("Failed to load configuration: {err:?}"),
    };

    App::new()
        .add_plugins((
            DefaultPlugins.set(log_plugin(&config.logging)),
            IOPlugin,
            UIPlugin,
        ))
        .insert_resource(config)
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut gizmos: ResMut<Assets<GizmoAsset>>,
) {
    commands.spawn(Sprite::from_image(asset_server.load("logo.png")));

    let mut gizmo = GizmoAsset::default();
    gizmo.axes_2d(Transform::from_xyz(0., 0., 0.), 128.);
    commands.spawn(Gizmo {
        handle: gizmos.add(gizmo),
        ..default()
    });
}
