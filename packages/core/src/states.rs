//! For more information on Bevy's state machine implementation read
//! [their documentation](https://docs.rs/bevy/latest/bevy/state).
//!
//! DungeonRS states are mainly used to coordinate between systems that may need to run conditionally.
//! For example, we don't want the UI to be shown during an export (as it would be rendered as well)
//! or we want to display loading messages/screens while the application is performing long-running
//! operations.
//!
//! Important is to remember we don't use the state machine to communicate information between systems,
//! that's what we use the event system for, state machine should be treated as a way to conditionally
//! turn certain functionality on/off (e.g. UI, interactivity, ...) based on the state of other systems.

use bevy::prelude::States;

/// Represents the root level state of DungeonRS.
/// Every other state should be defined as a substate of this [DungeonRsState].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum DungeonRsState {
    /// The initial [DungeonRsState], the software doesn't have anything loaded and is idle.
    /// This is primarily used by the editor when launching, and nothing is being opened / created.
    #[default]
    Splash,

    /// The editor is loading and should disable anything that may interfere with the loading process.
    /// While there are many things that can be "loaded", this one should be reserved for things like
    /// loading, saving or exporting maps where the UI may be disabled or even removed entirely.
    ///
    /// All other states should consider a substate of [DungeonRsState::Loaded].
    Loading,

    /// The editor is fully operational and can be worked with, or is already working with tasks.
    /// This is the "main" state of the editor; after a map has loaded or created, the user should find
    /// themselves here.
    /// Systems that want to indicate loading that doesn't require locking out the user entirely
    /// (as [DungeonRsState::Loading] would) should consider making substates for this state.
    Loaded,
}
