use bevy::prelude::{AssetServer, Resource};
use bevy_egui::EguiContexts;
use egui::load::SizedTexture;

/// Global UI state and resources.
/// Contains things like the logo.
#[derive(Resource)]
pub(crate) struct UiState {
    pub logo: SizedTexture,
}

impl UiState {
    /// Generate a new `UiState` with the given logo.
    /// This method uses a weak clone of the given image.
    pub fn new(contexts: &mut EguiContexts, asset_server: &AssetServer) -> Self {
        let logo = asset_server.load("logo.png");
        let texture = contexts.add_image(logo);

        UiState {
            logo: SizedTexture::new(texture, [28., 28.]),
        }
    }
}
