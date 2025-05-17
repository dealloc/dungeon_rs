//! The persistence module handles saving and restoring DungeonRS' state from (and to) disk.
//!
//! This module contains copies of a lot of data structures found in Core, but this is intentional,
//! the copies contained in this module are intended (and optimised) for serialisation.
//! These copies can be found under [crate::persistence::entities].
//!
//! Serialisation logic itself is implemented using [Serde](https://serde.rs/), keeping the format
//! to which we serialise flexible.
//! Currently, large datasets (like projects) will be serialised using [MessagePack](https://msgpack.org/)
//! while smaller files intended to be edited by users (like config) will be serialised using JSON or [Ron](https://docs.rs/ron).

pub(super) mod entities;
pub(super) mod events;
pub(super) mod save_file;
mod systems;

use crate::persistence::events::load_project_request::LoadProjectRequest;
use crate::prelude::SaveProjectRequest;
use crate::states::DungeonRsState;
use bevy::app::App;
use bevy::prelude::{FixedPostUpdate, IntoScheduleConfigs, Plugin, in_state, not};

#[derive(Default)]
pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveProjectRequest>()
            .add_event::<LoadProjectRequest>();
        app.add_systems(
            FixedPostUpdate,
            (systems::poll_save_project, systems::poll_load_project)
                .run_if(not(in_state(DungeonRsState::Loading))),
        );
    }
}
