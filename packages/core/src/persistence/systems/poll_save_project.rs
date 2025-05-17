use crate::components::{Layer, Level, Project, Texture};
use crate::persistence::save_file::SaveFile;
use crate::prelude::SaveProjectRequest;
use crate::utils;
use bevy::asset::Assets;
use bevy::prelude::{
    Children, ColorMaterial, EventReader, Mesh2d, MeshMaterial2d, Name, Query, Res, Transform,
    With, info,
};
use std::fs::write;

/// System to poll for incoming [SaveProjectRequest] events.
#[allow(clippy::too_many_arguments)]
pub fn poll_save_project(
    mut save_projects: EventReader<SaveProjectRequest>,
    project_query: Query<(&Project, &Name, &Children), With<Project>>,
    level_query: Query<(&Name, &Children), With<Level>>,
    layer_query: Query<(&Transform, &Name, &Children), With<Layer>>,
    mesh_query: Query<(&Texture, Option<&Name>), With<Mesh2d>>,
    transform_query: Query<&Transform>,
    material_query: Query<&MeshMaterial2d<ColorMaterial>>,
    materials: Res<Assets<ColorMaterial>>,
) -> bevy::prelude::Result {
    for save_project in save_projects.read() {
        let save = SaveFile::new(
            project_query,
            level_query,
            layer_query,
            mesh_query,
            transform_query,
            material_query,
            &materials,
        )?;

        write(save_project.path.as_path(), utils::serialize(&save)?).expect("FAILED TO SAVE");

        info!("Saved to {}", save_project.path.display());
    }

    Ok(())
}
