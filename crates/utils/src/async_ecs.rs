//! This module handles asynchronous operations in ECS.
//!
//! Most notably, it defines the `AsyncComponent` struct which tracks the state of an async operation
//! and automatically executes commands emitted on the world the component is attached to.

use bevy::ecs::world::CommandQueue;
use bevy::prelude::{BevyError, Commands, Component, Entity, Event, Query, World, warn};
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, ComputeTaskPool, IoTaskPool, Task, block_on};
pub use crossbeam_channel::{Receiver, Sender};
use crossbeam_channel::{SendError, unbounded};
use std::future::Future;

/// Represents an ongoing asynchronous operation that will be polled for progress and/or completion.
///
/// To create instances, see [`AsyncComponent::new_async`], [`AsyncComponent::new_compute`] and
/// [`AsyncComponent::new_io`].
#[derive(Component)]
pub struct AsyncComponent {
    /// The task tracking if the asynchronous operation is completed.
    task: Task<Result<(), BevyError>>,
    /// The channel through which updates are emitted.
    ///
    /// Updates are modelled through Bevy's [`CommandQueue`](https://docs.rs/bevy/latest/bevy/ecs/world/struct.CommandQueue.html),
    /// which allows async tasks to send commands back to the main World.
    receiver: Receiver<CommandQueue>,
}

impl AsyncComponent {
    /// Generates a new [`AsyncComponent`] for the `task`, scheduled on the [`AsyncComputeTaskPool`].
    /// Like the [`AsyncComputeTaskPool`], this is intended for CPU-intensive work that may span
    /// across multiple frames.
    ///
    /// Given an `async` function that takes a `Sender` this method schedules it on Bevy's
    /// `TaskPool` and handles polling progress and completion on the main schedule.
    #[must_use = "async components do nothing unless polled"]
    pub fn new_async<F, Fut>(task: F) -> Self
    where
        F: FnOnce(Sender<CommandQueue>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), BevyError>> + Send + 'static,
    {
        let (sender, receiver) = unbounded::<CommandQueue>();
        let task = AsyncComputeTaskPool::get().spawn(task(sender));
        AsyncComponent { task, receiver }
    }

    /// Generates a new [`AsyncComponent`] for the `task`, scheduled on the [`ComputeTaskPool`].
    /// Like the [`ComputeTaskPool`], this is intended for CPU-intensive work that must be completed
    /// to deliver the next frame.
    ///
    /// Given an `async` function that takes a `Sender` this method schedules it on Bevy's
    /// `TaskPool` and handles polling progress and completion on the main schedule.
    #[must_use = "async components do nothing unless polled"]
    pub fn new_compute<F, Fut>(task: F) -> Self
    where
        F: FnOnce(Sender<CommandQueue>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), BevyError>> + Send + 'static,
    {
        let (sender, receiver) = unbounded::<CommandQueue>();
        let task = ComputeTaskPool::get().spawn(task(sender));
        AsyncComponent { task, receiver }
    }

    /// Generates a new [`AsyncComponent`] for the `task`, scheduled on the [`IoTaskPool`].
    /// Like the [`IoTaskPool`], this is intended for IO-intensive work.
    ///
    /// Given an `async` function that takes a `Sender` this method schedules it on Bevy's
    /// `TaskPool` and handles polling progress and completion on the main schedule.
    #[must_use = "async components do nothing unless polled"]
    pub fn new_io<F, Fut>(task: F) -> Self
    where
        F: FnOnce(Sender<CommandQueue>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), BevyError>> + Send + 'static,
    {
        let (sender, receiver) = unbounded::<CommandQueue>();
        let task = IoTaskPool::get().spawn(task(sender));
        AsyncComponent { task, receiver }
    }
}

/// A helper function for reporting progress from a task controlled by [`AsyncComponent`].
///
/// This method is a shorthand for creating a `CommandQueue`, pushing a single command that dispatches
/// an event (of type `E`) and sends it over the `sender`.
///
/// # Example
/// ```rust
/// # use bevy::prelude::*;
/// # use utils::{AsyncComponent, report_progress};
/// #[derive(Event)]
/// struct FooEvent;
///
/// # fn main() {
/// #     let mut app = App::new();
/// #     app.add_plugins(TaskPoolPlugin::default());
/// #     app.add_event::<FooEvent>();
/// #     app.add_systems(Startup, setup);
/// #     app.run();
/// # }
/// #
/// # fn setup(mut commands: Commands) {
/// #    commands.spawn(AsyncComponent::new_async(async |sender| {
/// report_progress(&sender, FooEvent)?;
/// #        Ok(())
/// #    }));
/// # }
/// ```
/// # Errors
/// This method forwards the `Result` received from calling `sender.send(...)`.
pub fn report_progress<E>(
    sender: &Sender<CommandQueue>,
    event: E,
) -> Result<(), SendError<CommandQueue>>
where
    E: Event,
{
    let mut queue = CommandQueue::default();
    queue.push(move |world: &mut World| {
        world.send_event(event);
    });

    sender.send(queue)
}

/// Polls each [`AsyncComponent`] in the ECS tree and checks for progress and/or completion.
///
/// If an [`AsyncComponent`]'s `Receiver` contains updates, they are appended to the current world.
/// The [`AsyncComponent::task`] is polled for completion, and once completed the component is removed
/// from the world.
pub(crate) fn handle_async_components(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AsyncComponent)>,
) {
    for (entity, mut component) in &mut query {
        let queue = component.receiver.try_iter().reduce(|mut acc, mut queue| {
            acc.append(&mut queue);
            acc
        });

        if let Some(mut queue) = queue {
            commands.append(&mut queue);
        }

        if let Some(result) = block_on(future::poll_once(&mut component.task)) {
            if result.is_ok() {
                commands.entity(entity).despawn();
            } else {
                let error = result.unwrap_err();

                warn!("Background task failed: {error:?}");
            }
        }
    }
}
