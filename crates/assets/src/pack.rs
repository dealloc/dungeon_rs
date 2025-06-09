//! Defines the [`AssetPack`] data structure and related logic.
use semver::Version;
use serialization::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tantivy::doc;
use utils::AsyncComponent;

// const META_DIR: &str = ".metadata";
// const INDEX_DIR: &str = "index";
// const SCRIPT_DIR: &str = "scripts";
// const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct AssetPack {
    version: Version,
    name: String,
    scripts: HashMap<String, String>,
}

impl Default for AssetPack {
    fn default() -> Self {
        Self {
            version: utils::version().clone(),
            name: String::default(),
            scripts: HashMap::new(),
        }
    }
}

impl AssetPack {
    #[must_use]
    pub fn create(_root: impl AsRef<Path>) -> AsyncComponent {
        AsyncComponent::new_io(async move |_queue| Ok(()))
    }

    // /// Builds the Tantivy schema for the index.
    // fn schema() -> Schema {
    //     let mut builder = Schema::builder();
    //     builder.add_text_field("name", TEXT);
    //     builder.add_date_field("last_indexed", DateOptions::default());
    //
    //     builder.build()
    // }
    //
    // /// Create a `Walkdir` iterator that skips hidden folders and unsupported file types.
    // fn recursive_files_and_folders(
    //     root: impl AsRef<Path>,
    // ) -> impl Iterator<Item = walkdir::Result<DirEntry>> {
    //     let walker = WalkDir::new(root);
    //
    //     walker.into_iter().filter_entry(|entry| {
    //         let filename = entry.file_name().to_string_lossy();
    //         if entry.file_type().is_dir() && filename.starts_with('.') {
    //             return false;
    //         }
    //
    //         !filename.ends_with(".webm")
    //     })
    // }
}
