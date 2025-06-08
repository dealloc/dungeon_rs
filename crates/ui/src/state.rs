//! We need to share various bits and bops throughout the different UI layers, we define a singleton
//! resource that contains this code.
use crate::layout::EditorPanels;
use bevy::prelude::Resource;
use egui_dock::{DockState, NodeIndex};
use egui_file::FileDialog;

/// Holds the internal state of various UI components.
///
/// Most notably, it holds the [`DockState`] used to build the general layout docks.
#[derive(Resource)]
pub struct UiState {
    /// The [`DockState`](https://docs.rs/egui_dock/latest/egui_dock/dock_state/struct.DockState.html)
    /// that controls most of the general layout.
    pub dock_state: DockState<EditorPanels>,
    pub open_asset_folder: Option<FileDialog>,
}

impl Default for UiState {
    fn default() -> Self {
        let mut state = DockState::new(vec![EditorPanels::Editor]);
        let surface = state.main_surface_mut();
        let [_, _assets] = surface.split_below(NodeIndex::root(), 0.9, vec![EditorPanels::Assets]);
        let [_, inspector] =
            surface.split_right(NodeIndex::root(), 0.8, vec![EditorPanels::Inspector]);
        let [_, _layers] = surface.split_below(
            inspector,
            0.6,
            vec![EditorPanels::Layers, EditorPanels::Levels],
        );

        Self {
            dock_state: state,
            open_asset_folder: None,
        }
    }
}
