//! Defines [`Configuration`], the primary struct to access configuration throughout the application.

use crate::LogConfiguration;
use anyhow::Context;
use bevy::prelude::Resource;
use semver::{BuildMetadata, Prerelease, Version};
use serialization::{Deserialize, SerializationFormat, Serialize, deserialize, serialize_to};
use std::collections::HashMap;
use std::env::current_exe;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// Configuration for the `DungeonRS` application.
#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct Configuration {
    /// The version of the software that created the configuration file.
    pub version: Version,
    /// A list of recently opened files,
    /// used in the UI to show recently opened projects.
    pub recents: Vec<PathBuf>,
    /// A map of asset libraries currently known to the application.
    /// This is used so that assets can be referred to relative to a library, which in turn is referred
    /// to by name.
    /// The result is that if the same library is located in different paths (as is often the case
    /// across multiple devices and/or users), asset references will still work consistently.
    pub libraries: HashMap<String, PathBuf>,
    /// Controls how the application should handle logging.
    ///
    /// Note that some configuration requires additional features to be enabled to work.
    pub logging: LogConfiguration,
}

/// The filename of the configuration file.
const CONFIG_FILE_NAME: &str = "config.toml";

impl Default for Configuration {
    fn default() -> Self {
        Self {
            version: utils::version().clone(),
            recents: Vec::new(),
            libraries: HashMap::new(),
            logging: LogConfiguration::default(),
        }
    }
}

impl Configuration {
    /// Attempt to load configuration from [`CONFIG_FILE_NAME`].
    /// If the [`CONFIG_FILE_NAME`] does not exist, a default configuration is generated and returned.
    ///
    /// Note that while it creates an instance of [`Configuration`], it doesn't create the
    /// [`CONFIG_FILE_NAME`] file unless you call [`Configuration::save`] directly.
    ///
    /// # Errors
    /// The method will return an error in two scenarios:
    /// - The application failed to retrieve the path to the current executable (`current_exe`)
    /// - The config file failed to deserialise
    pub fn load() -> anyhow::Result<Self> {
        let mut path = current_exe().with_context(|| "Failed to get current executable path")?;
        path.pop(); // Remove the executable name
        path.push(CONFIG_FILE_NAME); // and add the config file name
        let Ok(mut file) = File::open(path) else {
            // If the file doesn't exist, return the default configuration.
            return Ok(Self::default());
        };

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        deserialize::<Self>(&buffer, &SerializationFormat::Toml)
            .with_context(|| "Failed to deserialize config file")
    }

    /// Attempts to save the configuration.
    ///
    /// # Errors
    /// - `Error` returned when the underlying calls to either `current_exe`
    ///   or `File::create` fails.
    /// - [`serialization::SerializationError`] Thrown when a serialisation-related error occurs.
    pub fn save(&self) -> anyhow::Result<()> {
        let mut path = current_exe().with_context(|| "Failed to get current executable path")?;
        path.pop(); // Remove the executable name
        path.push(CONFIG_FILE_NAME); // and add the config file name
        let file = File::create(path).with_context(|| "Failed to create config file")?;

        serialize_to(&self, &SerializationFormat::Toml, file)?;
        Ok(())
    }
}
