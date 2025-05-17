use crate::components::{Layer as LayerComponent, Level as LevelComponent, Project, Texture};
use crate::persistence::entities::{image::Image, layer::Layer, level::Level};
use crate::utils;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents the information that is written/read from/to disk.
///
/// Anything about a project that needs to be persisted should be included in this data structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveFile {
    /// The version of the software that created the save.
    pub version: semver::Version,
    /// The (semantic) name of the map (unrelated to the filename).
    pub name: String,
    /// The canvas size, while currently not enforced, determines the export region.
    pub size: Rect,
    /// A list of all levels that are currently included in the project.
    pub levels: Vec<Level>,
}

impl SaveFile {
    /// Builds a [SaveFile] based on the current ECS hierarchy.
    ///
    /// This method queries and captures the following ECS structure:
    ///
    /// [Project] <br />
    /// | -> [crate::components::Level] <br />
    /// | ----> [crate::components::Layer] <br />
    /// | --------> images, paths, patterns, ...
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        project_query: Query<(&Project, &Name, &Children), With<Project>>,
        level_query: Query<(&Name, &Children), With<LevelComponent>>,
        layer_query: Query<(&Transform, &Name, &Children), With<LayerComponent>>,
        mesh_query: Query<(&Texture, Option<&Name>), With<Mesh2d>>,
        transform_query: Query<&Transform>,
        material_query: Query<&MeshMaterial2d<ColorMaterial>>,
        materials: &Res<Assets<ColorMaterial>>,
    ) -> Result<Self, BevyError> {
        let (project, project_name, project_children) = project_query.single()?;
        let mut levels = Vec::new();

        for level_entity in project_children.iter() {
            let (level_name, level_children) = level_query.get(level_entity)?;
            let mut layers = Vec::new();

            for layer_entity in level_children.iter() {
                let (layer_transform, layer_name, layer_children) =
                    layer_query.get(layer_entity)?;

                let mut images = Vec::new();
                for entity in layer_children.iter() {
                    let (texture, name) = mesh_query.get(entity)?;
                    let transform = transform_query.get(entity)?;
                    let material_handle = material_query.get(entity)?;

                    if let Some(material) = materials.get(material_handle) {
                        if let Some(texture_handle) = &material.texture {
                            if let Some(path) = texture_handle.path() {
                                images.push(Image {
                                    name: name.map(|name| name.to_string()),
                                    path: path.path().to_path_buf(),
                                    colour: material.color,
                                    size: texture.size,
                                    transform: *transform,
                                });
                            }
                        }
                    }
                }

                layers.push(Layer::new(
                    layer_name.as_str(),
                    layer_transform.translation.z as i32,
                    images,
                ));
            }

            levels.push(Level::new(level_name.as_str(), layers));
        }

        Ok(Self {
            version: semver::Version::parse(utils::version())?,
            name: project_name.to_string(),
            size: project.size,
            levels,
        })
    }

    /// Rebuilds the ECS structure represented in this [SaveFile] back into [Commands].
    /// It automatically constructs and registers all required meshes, materials, and assets.
    pub fn restore(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) {
        let mut project = commands.spawn((
            Name::new(self.name.clone()),
            Project::new(self.name.clone(), self.size),
        ));

        project.with_children(|project| {
            for level in &self.levels {
                let mut parent = project.spawn((Name::new(level.name.clone()), LevelComponent));

                parent.with_children(|parent| {
                    for layer in &level.layers {
                        let mut child =
                            parent.spawn((Name::new(layer.name.clone()), LayerComponent));

                        let weight = layer.weight as f32;
                        child.entry::<Transform>().and_modify(move |mut transform| {
                            transform.translation.z = weight;
                        });
                        child.with_children(|grand_child| {
                            for image in &layer.images {
                                if let Some(name) = &image.name {
                                    grand_child.spawn(Name::new(name.clone()));
                                }

                                grand_child.spawn((
                                    Mesh2d(meshes.add(image.size)),
                                    MeshMaterial2d(materials.add(ColorMaterial {
                                        color: image.colour,
                                        texture: Some(asset_server.load(image.path.clone())),
                                        ..default()
                                    })),
                                    image.transform,
                                ));
                            }
                        });
                    }
                });
            }
        });
    }
}
