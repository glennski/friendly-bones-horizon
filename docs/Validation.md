# Validation

[Wiki home](Home.md) · [Development](Development.md)

Target: Horizon `9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`, Windows x64, Rust 1.99.1, mod version 0.2.0.

## Automated checks

- Core unit tests (`tools\dev.ps1 test`): 8 tests pass. They cover settings limits, owner rights, follow hysteresis, guard return, alarm timing, weapon switching, friendly fire and roam limits.
- `horizon-mixins plan` and `apply` on the unmodified source: 19 injections and 1 module, no conflicts.
- `cargo check` of the staged game: no errors.
- Native build with the launcher (`--profile dev`): built on the unmodified nightly, `left_out` empty. The first build took 6 min 17 s.

## In game

Tested in a creative test world (`FBTest`) in the instance `Friendly Bones Test`, using the horizon-ctl control socket.

| Area | Result | Notes |
| --- | --- | --- |
| Summon both types | Verified | Both spawn with 20 health and roam. |
| Peaceful difficulty | Verified | Both companion types survive. |
| Claiming by right-click | Verified | Name "<player>'s Bones" appears; menu opens. |
| Menu: mouse mode and sliders | Verified | Guard set, combat radius changed to 6 by click. |
| Menu: keyboard | Not verified | Implemented (Tab, arrows, Enter, Escape). |
| Chat feedback on mode change | Verified | "Your companion is now guarding its post." |
| Save, quit, restart | Verified | Owner, guard mode and combat radius 6 are still set. |
| Spawn egg (creative) | Verified | Spawns a companion with 20 health. |
| Skeleton ritual | Verified | Bone blocks and lantern disappear; companion appears. |
| Wither ritual | Not verified | |
| Combat | Partly verified | A companion took damage (20 → 13) while a zombie was nearby; the zombie was gone shortly after. Target selection and weapon switching not checked. |
| Follow mode | Not verified | |
| Guard post behaviour | Not verified | Mode switch confirmed only. |
| Defence alarm | Not verified | |
| Friendly fire | Not verified | |
| Wither effect of arrows | Not verified | |
| Daylight | Not verified | |
| Limited ammo, pick-up, equipment | Not verified | |
| Chunk unload | Not verified | |
| Survival egg use | Not verified | Only creative mode was tested. |
| Two players, foreign owner, dedicated server | Not verified | |
| Controller | Not verified | |
| Performance | Not measured | Only the `dev` profile was built. No A/B comparison. |
| Official package validator (`cargo horizon-mod check`) | Not run | The packages follow `docs/api/mod-files.md` and were read by the launcher. |

Screenshots from the test session are in `screenshots/`.

## Before a release

Complete every "Not verified" row that affects gameplay, then rebuild with the `performance` profile and test again.
