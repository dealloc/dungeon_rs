mod components;
mod constants;
mod export;
mod persistence;
mod states;
mod utils;

use crate::export::ExportPlugin;
use crate::persistence::PersistencePlugin;
use crate::states::DungeonRsState;
use bevy::app::App;
use bevy::prelude::{AppExtStates, Plugin};

pub mod prelude {
    pub use crate::{
        components::*, export::events::*, persistence::events::load_project_request::*,
        persistence::events::save_project_request::*, states::*,
    };
}

#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ExportPlugin, PersistencePlugin))
            .init_state::<DungeonRsState>();

        #[cfg(feature = "dev")]
        app.add_systems(
            bevy::prelude::FixedPreUpdate,
            bevy::dev_tools::states::log_transitions::<DungeonRsState>,
        );
    }
}
