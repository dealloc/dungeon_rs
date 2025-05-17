use bevy::prelude::{Component, Transform, Visibility};

/// The [Level] component represents a grouping of [crate::components::Layer]s, usually an entire
/// map, and allows a single [crate::components::Project] to contain several but related maps.
/// For example, a roadside inn might have a ground floor, upper floor and a basement.
/// We would represent each of those as a [Level] under the project, each with their own
/// [crate::components::Layer]s and entities.
#[derive(Component, Default)]
#[component(immutable)]
#[require(Transform::from_xyz(0.0, 0.0, 0.0), Visibility::default())]
pub struct Level;
