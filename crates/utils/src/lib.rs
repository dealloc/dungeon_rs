#![doc = include_str!("../README.md")]

mod async_ecs;
mod plugin;
mod version;

pub use async_ecs::*;
pub use plugin::CorePlugin;
pub use utils_macros::*;
pub use version::version;
