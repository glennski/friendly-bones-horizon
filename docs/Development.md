# Development

[Wiki home](Home.md) · [Architecture](Architecture.md) · [Validation](Validation.md)

## Requirements

- Horizon launcher 0.2.63 with the nightly `2026-11-13-9f42064` installed.
- Python 3 on the PATH (used by the tools).
- PowerShell 5.1 or newer for `tools/dev.ps1`.
- The launcher's Rust toolchain (`horizon-launcher rust install`). The scripts take it from the launcher, not from the system.

## Commands

All commands run from the project root.

| Command | What it does |
| --- | --- |
| `tools\dev.ps1 test` | Checks that the generated payload is current, then runs the core unit tests |
| `tools\dev.ps1 package` | Regenerates the payload and writes both release files to `dist/` |
| `tools\dev.ps1 install -TrustNativeCode` | Packages, copies both files into the test instance, enables the mod and runs the native build |

`install` takes `-Profile performance|dev|release` (default `performance`). Use `dev` while iterating: it rebuilds faster but the game runs about twice as slow.

`tools/plan.sh <horizon checkout>` runs `horizon-mixins plan` against a pristine Horizon checkout. Use it after changing injections to see errors before building.

## Typical workflow

1. Change code in `src/` (game logic, menu) or in `mixins/friendlybones/` (injections, helpers).
2. `tools\dev.ps1 test` for the core logic.
3. `tools\dev.ps1 install -TrustNativeCode -Profile dev` to build and test in game.
4. Test in the `Friendly Bones Test` instance. Check the game log with `horizon-ctl logs` if something fails.

## Changing language text

Language files are in `assets/friendlybones/lang/`. The mod ships English only (`en_us.json`). Add a new file for another language, for example `de_de.json`, with the same keys. Changing text requires `tools\dev.ps1 package` and a reinstall of the content package.

## Versioning and release files

- Bump `version` in `horizon-native.json`, `horizon-mixin.toml` and `horizon-mod.toml` together. `package.py` refuses to build if they disagree. Also update the `VERSION` file by hand; the scripts do not read it.
- File names follow `Agents.md` section 7: `modname_modversion_minecraft-horizon-<game>-<commit>.<ext>`.
- Upload only the two release files. Do not upload sources or build caches.

## Tests

- The core tests (`src/core.rs`) cover settings limits, owner rights, follow hysteresis, guard return, alarm timing, weapon switching, friendly fire and roam limits.
- Engine behaviour is tested in game only. There are no automated game tests yet.

## Troubleshooting

| Symptom | Likely cause |
| --- | --- |
| `payload check` fails | `runtime.rs` is stale. Run `python tools/assemble.py`. |
| Plan error `E_TARGET_NOT_FOUND` | An injection target changed or the pin is wrong. Check the commit. |
| Plan error `E_HOOK_VERSION` | Hook contract changed. Update the injection and `hook_version`. |
| `E_ATOMIC_STAGE` when re-staging | Remove the old stage folder first. |
| Build succeeds, mod missing | Check `native-mods.json` in the version folder under `left_out`. |
| Companion not spawned from ritual | The lantern or skull must be placed last, directly on the bone column. |
