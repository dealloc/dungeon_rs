//! This module defines the building blocks for building the layout of the editor.
//!
//! The current implementation builds an `egui_dock` `TabViewer` and delegates the specific layout
//! to that. We contain the different panels and how to construct them in this module.

mod assets;
mod inspector;
mod layers;
mod levels;

use crate::state::UiState;
use crate::widgets;
use bevy::prelude::{Mut, Query, Sprite, Transform, With};
use bevy::{ecs::error::BevyError, prelude::ResMut};
use bevy_egui::EguiContexts;
use egui::{Ui, Widget, WidgetText};
use egui_dock::TabViewer;

/// The different panels that can be shown in the editor UI.
/// If a new panel needs to be available for the user in the UI it needs to be added here,
/// if it needs to be shown by default, make sure to add it in [`UiState::default`] as well.
#[derive(Debug)]
pub enum EditorPanels {
    /// The "main" view that shows the underlying Bevy rendered world.
    Editor,
    /// The view that shows the properties of the current entity to edit.
    Inspector,
    /// Shows the assets available in the currently selected libraries.
    Assets,
    /// Shows the layers available in the currently selected level.
    Layers,
    /// Shows the levels available in the currently selected project.
    Levels,
}

/// Contains the data structures that are available to the [`TabViewer`] when rendering the editor layout.
/// See [`EditorLayout::ui`] in particular.
pub struct EditorLayout<'a> {
    pub(crate) entity: &'a mut Mut<'a, Transform>,
}

impl TabViewer for EditorLayout<'_> {
    type Tab = EditorPanels;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        format!("{tab:?}").into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            EditorPanels::Editor => {
                // we don't render anything in the editor view.
                // Instead, in `clear_background` we make the background transparent so the underlying
                // Bevy render is visible.

                // Later on, we'd want to get the rectangle that this pane is shown in and then update
                // the Bevy camera to only render to this. That would prevent the camera shifting around
                // when we move the pane.
            }
            EditorPanels::Inspector => inspector::render(self, ui),
            EditorPanels::Assets => assets::render(self, ui),
            EditorPanels::Layers => layers::render(self, ui),
            EditorPanels::Levels => levels::render(self, ui),
        }
    }

    fn closeable(&mut self, tab: &mut Self::Tab) -> bool {
        !matches!(tab, EditorPanels::Editor)
    }

    fn allowed_in_windows(&self, tab: &mut Self::Tab) -> bool {
        !matches!(tab, EditorPanels::Editor)
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab, EditorPanels::Editor)
    }
}

#[utils::bevy_system]
pub fn render_editor_layout(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    mut query: Query<&mut Transform, With<Sprite>>,
) -> Result<(), BevyError> {
    let Some(context) = contexts.try_ctx_mut() else {
        return Ok(());
    };

    let mut transform = query.single_mut()?;

    widgets::toolbar(context);
    widgets::dock_layout(context, &mut state, &mut transform);
    Ok(())
}
