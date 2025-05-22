use crate::AssetPack;
use bevy::prelude::{BevyError, Resource};
use std::fs;
use std::path::PathBuf;
use tantivy::schema::{Schema, STORED, TEXT};
use tantivy::{Index, IndexReader, IndexWriter};
use toml_edit::{value, ArrayOfTables, DocumentMut};

#[derive(Resource)]
pub struct AssetLibrary {
    /// The name of the library.
    name: String,
    /// Where the library metadata will be stored on disk.
    /// This **must** refer to a folder.
    root: PathBuf,
    /// All the [`AssetPack`]s that are included in this library.
    packs: Vec<AssetPack>,
    /// The [Schema] used to index this asset library.
    schema: Schema,
    /// The [Index] that contains all assets for this library.
    index: Index,
    /// The [`IndexWriter`] to insert documents in the [Index].
    writer: IndexWriter,
    /// The [`IndexReader`] to read documents from the [Index].
    reader: IndexReader,
    /// Internal TOML document used to store the library metadata.
    config: DocumentMut,
}

impl AssetLibrary {
    /// Adds an [`AssetPack`] to the [`AssetLibrary`].
    ///
    /// Note that this is not immediately persisted nor indexed.
    pub fn add_pack(&mut self, pack: AssetPack) -> Result<(), BevyError> {
        let packs = self
            .config
            .entry("packs")
            .or_insert(ArrayOfTables::default().into())
            .as_array_of_tables_mut()
            .unwrap();

        packs.push(pack.clone().try_into()?);
        self.packs.push(pack);
        Ok(())
    }

    /// Create a new [AssetLibrary] without any [AssetPack]s.
    ///
    /// This creates the folder (if it doesn't exist yet), an empty index and the config file.
    pub fn create(name: String, path: impl Into<PathBuf>) -> Result<Self, BevyError> {
        let path = path.into();
        let index_path = path.join("index");
        fs::create_dir_all(index_path.clone())?;

        let mut builder = Schema::builder();
        builder.add_text_field("name", TEXT | STORED);
        builder.add_text_field("categories", TEXT | STORED);

        let schema = builder.build();
        let index = Index::builder()
            .schema(schema.clone())
            .create_in_dir(index_path)?;
        let writer = index.writer(100_000_000)?;
        let reader = index.reader()?;
        let mut document = DocumentMut::new();
        document["name"] = value(name.clone());
        fs::write(path.join("config.toml"), document.to_string())?;

        Ok(Self {
            name,
            root: path,
            packs: vec![],
            schema,
            index,
            writer,
            reader,
            config: document,
        })
    }

    /// Attempts to open an already created [AssetLibrary] and all dependent [AssetPack]s.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, BevyError> {
        let path = path.into();
        let index_path = path.join("index");

        let config_path = if path.is_dir() {
            path.join("config.toml")
        } else {
            path.clone()
        };

        let index = Index::open_in_dir(index_path)?;
        let schema = index.schema();
        let writer = index.writer(100_000_000)?;
        let reader = index.reader()?;
        let config = fs::read_to_string(config_path)?;
        let mut config = config.parse::<DocumentMut>()?;
        let name = config["name"]
            .as_str()
            .ok_or("Manifest does not contain a name")?
            .to_string();
        let packs = config
            .entry("packs")
            .or_insert(ArrayOfTables::default().into())
            .as_array_of_tables_mut()
            .ok_or("packs entry does not contain a valid value")?
            .iter()
            .map(AssetPack::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            name,
            root: path,
            packs,
            schema,
            index,
            writer,
            reader,
            config,
        })
    }
}
