# Friendly Bones for Horizon

Friendly Bones adds two companion mobs to [Horizon](https://github.com/horizon-rust/horizon): a **Friendly Skeleton** and a **Friendly Wither Skeleton**. Summon them with a ritual or a spawn egg, claim them, and set them to roam, follow or guard. They fight with a bow and a sword and defend their owner.

- **Version:** 0.2.0
- **Target:** Horizon nightly `2026-11-13-9f42064` (commit `9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`), Minecraft 26.3
- **Licence:** CC0-1.0 for the mod code

The full documentation is in the [wiki](docs/Home.md):

- [Installation](docs/Installation.md)
- [Usage](docs/Usage.md)
- [Companion behaviour](docs/Companion-Behaviour.md)
- [Configuration](docs/Configuration.md)
- [Compatibility](docs/Compatibility.md)
- [Architecture](docs/Architecture.md)
- [Development](docs/Development.md)
- [Validation](docs/Validation.md)
- [Porting notes](docs/Porting-Notes.md)

## Quick start

1. Copy `dist/friendlybones_0.2.0_minecraft-horizon-26.3-9f42064.hnmod` to `horizon-native-mods/` in your instance as `friendlybones.hnmod`.
2. Copy `dist/friendlybones-content_0.2.0_minecraft-horizon-26.3-9f42064.hmod` to `horizon-mods/` as `friendlybones.hmod`.
3. Add `mod.friendlybones:true` to `horizon-mods.txt` in the instance folder.
4. Build with `horizon-launcher native-build --instance "YOUR INSTANCE" --trust-native-code`.

Native mods run unsandboxed with your user rights. Read the source in `mixins/friendlybones/` before building.

## Development

```powershell
tools\dev.ps1 test      # payload check and core unit tests
tools\dev.ps1 package   # build the two release files in dist/
tools\dev.ps1 install -TrustNativeCode -Profile dev   # package, install into the test instance, build
```

Project layout: `src/` holds the source, `mixins/friendlybones/` the native bundle, `assets/` and `data/` the content. See [Architecture](docs/Architecture.md).
