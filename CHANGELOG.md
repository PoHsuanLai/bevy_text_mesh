# Changelog

[git_tag_comparison]: https://github.com/blaind/bevy_text_mesh/compare/v0.9.0...main

## Unreleased

### Changed (Breaking)

- **Replaced C-based ttf2mesh with pure Rust fontmesh library**
  - Removes C compiler build prerequisites
  - Enables WASM platform support (resolves #11)
  - Improves cross-platform compatibility
  - Uses [fontmesh 0.2.0](https://crates.io/crates/fontmesh) for font-to-mesh conversion
- **Migrated to Bevy 0.18-dev** (from Bevy 0.12)
  - Updated asset loading to async API
  - Replaced `PbrBundle` with component tuples `(MeshMaterial3d, Mesh3d)`
  - Updated mesh API (`set_indices` → `insert_indices`)
  - Updated event system (`EventReader` → `MessageReader`)
- **Updated Bevy dependency** from path to git (github.com/bevyengine/bevy)
  - Added critical rendering features: `bevy_sprite_render`, `default_font`

### Fixed

- Fixed 2d_text example by adding missing Bevy features for sprite rendering

### Removed

- Removed requirement for `apt-get install build-essential patch` on Linux
- Removed ttf2mesh C library dependency

## Version 0.9.0 (2023-11-21)

[Compare changelog](https://github.com/blaind/bevy_text_mesh/compare/v0.8.0...v0.9.0)

### Changed

- [Breaking: reverted the `#mesh` suffix for font loading - the suffix is required again][34]

## Version 0.8.0 (2023-11-19)

[Compare changelog](https://github.com/blaind/bevy_text_mesh/compare/v0.7.0...v0.8.0)

### Changed

- [Upgrade bevy to 0.12.0][32]
- Breaking: font must no longer be loaded with `#mesh` suffix

## Version 0.7.0 (2023-08-05)

[Compare changelog](https://github.com/blaind/bevy_text_mesh/compare/v0.6.0...v0.7.0)

This version was never published to crates.io.

### Changed

- [Upgrade bevy to 0.11.0][29]

## Version 0.6.0 (2023-04-11)

[Compare changelog](https://github.com/blaind/bevy_text_mesh/compare/v0.5.0...v0.6.0)

### Changed

- Upgrade bevy to 0.10.0
- Upgrade bitflags to 2.1.0

## Version 0.5.0 (2022-11-13)

[Compare changelog](https://github.com/blaind/bevy_text_mesh/compare/v0.4.0...v0.5.0)

### Changed

- Upgrade bevy to 0.9.0

## Version 0.4.0 (2022-10-24)

[Compare changelog](https://github.com/blaind/bevy_text_mesh/compare/v0.3.0...v0.4.0)

### Changed

- [Breaking: use #mesh label for font loading to allow interop with bevy `.ttf` AssetLoader][15]

## Version 0.3.0 (2022-08-25)

[Compare changelog](https://github.com/blaind/bevy_text_mesh/compare/v0.2.0...v0.3.0)

### Changed

- Upgrade bevy to 0.8.0

## Version 0.2.0 (2022-04-17)

[Compare changelog](https://github.com/blaind/bevy_text_mesh/compare/v0.1.0...v0.2.0)

### Changed

- Upgrade bevy to 0.7.0

[15]: https://github.com/blaind/bevy_text_mesh/pull/15
[29]: https://github.com/blaind/bevy_text_mesh/pull/29
[32]: https://github.com/blaind/bevy_text_mesh/pull/32
[34]: https://github.com/blaind/bevy_text_mesh/pull/34
