use bevy::prelude::{Component, Rect, Transform, Visibility};

/// Represents a project in DungeonRS, which can be compared to a workspace or folder.
///
/// A project is the marker component for the ECS hierarchy that contains everything you want saved
/// when the user "saves" a map.
///
/// Typically, a [Project] will contain [crate::components::Layer]s which in turn contain the elements that the
/// user sees (images, paths, patterns, ...).
#[derive(Component)]
#[component(immutable)]
#[require(Transform::from_xyz(0.0, 0.0, 0.0), Visibility::default())]
pub struct Project {
    pub name: String,
    pub size: Rect,
}

impl Project {
    pub fn new(name: impl Into<String>, size: Rect) -> Self {
        Self {
            name: name.into(),
            size,
        }
    }
}
