# Architecture

[Wiki home](Home.md) · [Development](Development.md) · [Compatibility](Compatibility.md)

## Package layout

```text
horizon-native.json            native package manifest (game, horizon and mixins commits)
horizon-mod.toml               content manifest
assets/friendlybones/          items, language text
data/friendlybones/horizon/    entity types, item, creative tab
mixins/friendlybones/
  horizon-mixin.toml           mod manifest: module, injections
  runtime.rs                   GENERATED support module (do not edit)
  hooks/*.rs                   helpers for hook-point injections
  patches/*.rs                 helpers for engine adjustments
src/core.rs                    engine-independent decisions, unit tests
src/runtime.rs                 engine adapter: spawning, AI goals, saving, rituals, combat
src/ui.rs                      client menu: drawing, mouse, keyboard, network
tools/                         assemble.py, package.py, dev.ps1, plan.sh
```

## Source of truth

- `src/*.rs` is the source. `tools/assemble.py` combines `src/core.rs`, `src/runtime.rs` and `src/ui.rs` into `mixins/friendlybones/runtime.rs`, which is then compiled into the game as a support module.
- `tools/assemble.py --check` fails when the generated file is stale. `package.py` runs this check first.
- `core.rs` has no engine dependencies, so its tests run on the host with `cargo test`.

## Hook points

Most injections attach to named hook points from `native-hooks.json`, which gives a stable contract:

| Injection | Hook point | Purpose |
| --- | --- | --- |
| `world` | `world_lifecycle` | Reset state when a world opens |
| `tick-pre`, `tick-post` | `level_tick_pre`, `level_tick_post` | Companion update, rituals, persistence |
| `use-item` | `use_item_on` | Spawn egg, ritual heads |
| `interact` | `interact_entity` | Claiming and opening the menu |
| `net-client`, `net-server` | `net_from_client`, `net_from_server` | Menu synchronisation |
| `loot` | `loot_drops` | Keep the stowed weapon's components |
| `ui-render`, `ui-press`, `ui-release`, `ui-key`, `ui-closing` | `screen_*` | Menu drawing and input |
| `ui-tick` | `native_client_tick_pre` (by path) | Send pending menu updates |
| `damage-in`, `damage-post` | `native_living_incoming_damage_v3`, `native_living_damage_post` (by path) | Friendly fire, alarm trigger |
| `goals` | `native_register_goals` | Freeze new companions until initialised |
| `peaceful` | `check_despawn` (expression) | Exempt companions from Peaceful despawn |
| `entity-rows` | `Plan::entity_types_json` (by path) | Real template for the two entity types |

## Data flow

1. Setup: the planner (`horizon-mixins plan`) checks every injection against the pinned source and writes a plan. `apply` stages the modified source. The launcher then compiles the game.
2. Runtime: hooks call into `runtime.rs`. Companion state lives in an attachment on the entity and in a per-world map.
3. Menu: the client sends a packet with the new settings. The server checks owner and distance, applies the values with clamping, and sends back the state.

## Attachments

Each companion stores its state as JSON in the attachment `friendlybones:companion`. The schema is versioned (`"schema": 1`). Unknown or broken data makes that companion skip initialisation and log an error; other companions are not affected.
