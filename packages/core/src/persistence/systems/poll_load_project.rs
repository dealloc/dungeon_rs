use crate::components::Project;
use crate::persistence::save_file::SaveFile;
use crate::prelude::LoadProjectRequest;
use crate::utils;
use bevy::asset::{AssetServer, Assets};
use bevy::prelude::{
    ColorMaterial, Commands, Entity, EventReader, Mesh, Query, Res, ResMut, With, info,
};

/// System to poll for incoming [LoadProjectRequest] events.
pub fn poll_load_project(
    mut load_projects: EventReader<LoadProjectRequest>,
    mut commands: Commands,
    project: Query<Entity, With<Project>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) -> bevy::prelude::Result {
    for load_project in load_projects.read() {
        let content = std::fs::read(&load_project.path)?;
        let save: SaveFile = utils::deserialize(&content)?;

        if let Ok(project) = project.single() {
            info!("Despawning existing hierarchy");
            commands.entity(project).despawn();
        }

        save.restore(&mut commands, &mut meshes, &mut materials, &asset_server);
    }

    Ok(())
}
