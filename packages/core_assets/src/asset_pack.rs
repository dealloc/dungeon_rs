use std::path::PathBuf;
use toml_edit::{Table, value};

/// A single folder in a library containing assets and subfolders.
///
/// Everything under this folder is considered an asset.
#[derive(Debug, Clone)]
pub struct AssetPack {
    pub name: String,
    pub root: PathBuf,
}

impl AssetPack {
    pub fn new(name: String, root: impl Into<PathBuf>) -> Self {
        Self {
            name,
            root: root.into(),
        }
    }
}

impl TryInto<Table> for AssetPack {
    type Error = &'static str;

    fn try_into(self) -> Result<Table, Self::Error> {
        let mut metadata = Table::default();
        metadata["name"] = value(self.name);
        metadata["root"] = value(
            self.root
                .to_str()
                .ok_or("Failed to convert 'root' path to UTF-8 String")?
                .to_string(),
        );

        Ok(metadata)
    }
}

impl TryFrom<&Table> for AssetPack {
    type Error = &'static str;

    fn try_from(value: &Table) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value["name"]
                .as_str()
                .ok_or("AssetPack manifest did not contain a 'name'.")?
                .to_string(),
            root: PathBuf::from(
                value["root"]
                    .as_str()
                    .ok_or("AssetPack manifest did not contain a 'root'.")?,
            ),
        })
    }
}
