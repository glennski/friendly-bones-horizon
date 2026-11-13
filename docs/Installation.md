# Installation

[Wiki home](Home.md)

## Requirements

- Horizon launcher with the nightly `2026-11-13-9f42064` installed. The mod is pinned to this commit and does not work on other Horizon versions yet (see [Compatibility](Compatibility.md)).
- Windows x64 has been tested. The native build uses the launcher's own Rust toolchain, so no separate Rust installation is needed.
- The first native build takes about six minutes; later builds are faster.

## Package files

The mod ships as two files in `dist/`:

| File | Goes to | Contents |
| --- | --- | --- |
| `friendlybones_0.2.0_minecraft-horizon-26.3-9f42064.hnmod` | `horizon-native-mods/` of the instance, renamed to `friendlybones.hnmod` | native code (mixin bundle) |
| `friendlybones-content_0.2.0_minecraft-horizon-26.3-9f42064.hmod` | `horizon-mods/` of the instance, renamed to `friendlybones.hmod` | content: entity types, spawn egg, creative tab, language text |

The file names follow the release naming scheme in `Agents.md` (`modname_modversion_minecraft-horizon-<game>-<commit>`).

The two files are separate because the launcher of this nightly (0.2.63) builds native code from a combined `.hmod` but does not install its content into the game build.

## Steps

1. Copy the native package into `horizon-native-mods/` of your instance and rename it to `friendlybones.hnmod`.
2. Copy the content package into `horizon-mods/` of the same instance and rename it to `friendlybones.hmod`.
3. Turn the mod on by adding this line to `horizon-mods.txt` in the instance folder (or enable it under Options > Mods):

   ```text
   mod.friendlybones:true
   ```

4. Read the source in `mixins/friendlybones/` (native mods run unsandboxed with your user rights) and build:

   ```powershell
   horizon-launcher native-build --instance "YOUR INSTANCE" --trust-native-code
   ```

5. Play the instance. The build selects the new game version for that instance.

The development script automates steps 1–4 for the test instance `Friendly Bones Test`:

```powershell
tools\dev.ps1 install -TrustNativeCode
```

## Uninstalling

Turn the mod off in `horizon-mods.txt` (`mod.friendlybones:false`), remove the two package files and rebuild the instance. Companions already in a world are not removed automatically, so back up the world first if you care about them.

## GitHub import

The launcher's "Add from GitHub" feature installs native code only. It does not install the content package, so for now install the mod from the two files above.
