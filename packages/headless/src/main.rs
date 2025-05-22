#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic, clippy::suspicious, clippy::complexity)]

use bevy::window::ExitCondition;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use core::{CorePlugin, prelude::*};
use std::path::PathBuf;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: ExitCondition::DontExit,
                    ..default()
                })
                .set(log_plugin()),
            CorePlugin,
        ))
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[allow(clippy::needless_pass_by_value)]
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_writer: EventWriter<ExportRequest>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("logo.png")),
        Transform::from_xyz(0., 0., 0.0),
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("logo.png")),
        Transform::from_xyz(500., 0., 0.0),
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("logo.png")),
        Transform::from_xyz(1000., 1000., 0.0),
    ));

    let Ok(request) = ExportRequest::new(
        PathBuf::from("output.png"),
        Rect::from_center_size(Vec2::ZERO, Vec2::splat(600.)),
        256,
    ) else {
        return;
    };

    event_writer.write(request);
}

fn update(
    mut progress: EventReader<ExportProgress>,
    mut completed: EventReader<ExportCompleted>,
    mut app_exit: EventWriter<AppExit>,
    mut gizmos: Gizmos,
) {
    gizmos.rect_2d(
        Isometry2d::IDENTITY,
        Vec2::splat(599.),
        Color::srgb(1., 0., 0.),
    );
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,
            UVec2::splat(6),
            Vec2::splat(100.),
            Color::WHITE,
        )
        .outer_edges();

    gizmos.axes_2d(Transform::IDENTITY, 300.0);

    for progress in progress.read() {
        info!("Exporting: {:?}", progress);
    }

    for completed in completed.read() {
        info!("Export completed: {:#?}", completed);

        app_exit.write(AppExit::Success);
    }
}
