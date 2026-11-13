# Compatibility

[Wiki home](Home.md) · [Architecture](Architecture.md)

## Supported version

| Item | Value |
| --- | --- |
| Horizon nightly | `2026-11-13-9f42064` |
| Horizon commit | `9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0` |
| horizon-mixins | `4342a0bfbe71d4d14185528d64e496d6022f16a1` (the launcher's own copy, version 0.6.0) |
| Minecraft | 26.3 |
| Launcher | 0.2.63 |
| Platform | Windows x64 (tested) |

The mod declares this commit in `horizon-native.json` and `[compatibility] horizon_revision` in `mixins/friendlybones/horizon-mixin.toml`. The launcher builds the mod on this commit.

**Other Horizon versions are not supported.** Hook points and generated code change between commits. A new version requires re-running `horizon-mixins plan`, checking every injection and updating the pins.

## Limitations of this Horizon version

### No skeleton content template

Horizon's content check lists only snowball, villager, wandering trader and zombie as templates for mod entity types. Skeleton and wither skeleton are missing.

Workaround:

- The two entity type files in `data/` declare `minecraft:zombie` as the template, so the content check passes.
- The injection `entity-rows` replaces that template with the real one (`minecraft:skeleton`, `minecraft:wither_skeleton`) when the entity table is built.

This relies on the internal method `entity_types_json`. A Horizon update that changes it breaks the injection, and the planner will say so.

### Three hook points without catalogue names

`native_client_tick_pre`, `native_living_incoming_damage_v3` and `native_living_damage_post` have no entry in `native-hooks.json` at this commit. They are attached by function path (`target = ...`), not by hook name. Such injections can break on any change to these functions.

Because of this, the bundle does not declare a `hooks` field. It is tied to this exact commit.

### Engine adjustments

Three injections change engine behaviour for the two companion types:

- `goals`: new companions start frozen and without goals, until the runtime installs its own.
- `peaceful`: the Peaceful despawn check exempts companions.
- `entity-rows`: see above.

## Save data from earlier versions

Saves made with the old Fabric version are not migrated. The old `fb_*` fields are not read. Companions from the Fabric version do not appear in Horizon.

## Multiplayer

Not verified. Companions are server-side entities. The menu sends changes to the server and the server checks the owner and the distance before accepting them. A dedicated server has not been tested.
