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
use bevy::prelude::{With, World};
use bevy::window::PrimaryWindow;
use bevy::{
    ecs::error::BevyError,
    prelude::{Commands, ResMut, info},
};
use bevy_egui::{EguiContext, EguiContexts};
use egui::panel::Side;
use egui::{
    Color32, Frame, ProgressBar, Rect, ScrollArea, Separator, SidePanel, Stroke, TextEdit,
    TopBottomPanel, Ui, Widget, WidgetText, Window,
};
use egui_dock::{DockArea, Style, TabViewer};
use egui_file::FileDialog;
use utils::AsyncComponent;

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
    pub world: &'a mut World,
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

/// Handles rendering the [`EditorLayout`] in the `World`.
#[utils::bevy_system]
pub fn render_editor_layout(world: &mut World) {
    let Ok(mut context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
    else {
        return;
    };

    let mut context = context.clone();
    world.resource_scope::<UiState, _>(|world, mut state| {
        let context = context.get_mut();
        if let Some(dialog) = &mut state.open_asset_folder {
            dialog.show(context);

            if dialog.selected() {
                info!("{:?}", dialog.path());
            }
        }

        widgets::toolbar(context);
        widgets::dock_layout(world, context, state);
    });

    // Window::new("Create asset library")
    //     .open(&mut true)
    //     .show(context, |ui| {
    //         ui.horizontal(|ui| {
    //             ui.label("Name: ");
    //             ui.text_edit_singleline(&mut String::new());
    //         });
    //         ui.horizontal(|ui| {
    //             ui.label("Path: ");
    //             ui.text_edit_singleline(&mut String::new());
    //             if ui.button("...").clicked() {
    //                 let mut file_dialog = FileDialog::select_folder(None);
    //                 file_dialog.open();
    //
    //                 state.open_asset_folder = Some(file_dialog);
    //             }
    //         });
    //     });
}
