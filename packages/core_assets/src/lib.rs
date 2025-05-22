#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic, clippy::suspicious, clippy::complexity)]

mod asset_library;
mod asset_pack;

use bevy::prelude::{App, Commands, Plugin, Startup, Result};
use crate::asset_library::AssetLibrary;
use crate::asset_pack::AssetPack;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) -> Result {
    let mut library = AssetLibrary::create("default".into(), ".library")?;
    library.add_pack(AssetPack::new(String::from("asset pack 1"), "/some/path"))?;
    // let library = AssetLibrary::open(".library")?;

    commands.insert_resource(library);
    Ok(())
}