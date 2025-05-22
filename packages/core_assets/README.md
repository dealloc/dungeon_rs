Top-level asset management for DungeonRS.

This module defines the abstractions and systems for discovering, indexing,
categorising, caching, and loading large numbers of image assets in a
non-blocking way. It is designed to integrate seamlessly with ECS and AssetServer while keeping
raw asset folders pristine and fully reusable by multiple libraries.

# Key Concepts

- **AssetLibrary**
  A self-contained "library" residing in its own folder, containing:
  - metadata (UUID, name, pack roots, filters, rules, thumbnail size).
  - Tantivy index files.
  - last-run content hashes for delta detection.
  - on-disk thumbnail cache.

- **AssetPack**
  One root folder (absolute or relative to the library) containing image assets.
  [AssetLibrary] may reference multiple packs.

- **Rule-Driven Metadata Extraction**
  The user can define a script (written in [Rhai](https://rhai.rs) that defines how a given [AssetPack]
  should be indexed.

Progress is reported via a channel polled by a non-blocking Bevy system, which in turn dispatches
events.