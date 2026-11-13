# Porting notes

[Wiki home](Home.md) · [Architecture](Architecture.md)

These notes summarise the analysis made before the port. They are kept for context; the current behaviour is described in [Companion behaviour](Companion-Behaviour.md). The original analysis was written in German against the Fabric version 1.5.0 of FriendlyBones.

## Feature mapping

| Area | Fabric original | Horizon port |
| --- | --- | --- |
| Companion types | two Java entity classes | two entity types copying skeleton and wither skeleton |
| Rituals | bone blocks with lantern or skull | same; owner is the player who places the head |
| Spawn egg | own item in creative tab | same |
| Ownership | nearest player at ritual, else first right-click | player who places the head, else first right-click |
| Rights | only owner opens menu, distance 12 blocks | same |
| Roam / follow / guard | Java goals | Horizon goals; navigation reused from the engine |
| Combat | bow at range, sword up close | same, with the same distance gap |
| Alarm | scan of nearby companions after monster hit | same, in a bounding box |
| Friendly fire | multiplier on companion-related damage | same, with the multiplier applied at the damage hook |
| Persistence | Java NBT fields | JSON attachment with schema version |
| Menu | Java sliders and buttons | native screen, drawn and handled in `ui.rs` |
| Configuration | JSON file | same file layout, same key names |

## Decisions

- **Ritual owner:** the original takes the nearest player. The port uses the player who placed the head, which is what most players expect. This is a deliberate change.
- **Native hooks first:** each injection uses a named hook point where one exists. Only three functions have no catalogue name and are bound by path.
- **No second movement simulation:** navigation and standard attacks come from the engine. The mod only decides targets, owners and return points.
- **Arrow damage:** the original scales arrows after the fact in a per-tick search. The port keeps the damage factor at the damage hook and does not scan the world each tick.
- **Attack hysteresis:** the 3 and 5 block switch distances of the original are kept.

## Known differences

- Java game tests cannot run in Horizon. Their scenarios are kept as a checklist in [Validation](Validation.md).
- The menu is drawn natively. The original's slider widget, focus handling and network protocol are not reused.
- Controller support is not implemented or verified.
- Migration of Fabric saves is not implemented.

## Open questions

- Ammo: the original keeps bow and sword in both hands but expects an arrow stack in one hand for limited ammo. The port needs a decision on how limited ammo should work.
- Alarm radius: the original uses an extended bounding box, not a sphere. The port keeps the box.
- Dedicated server: the port has not been tested on a server. Multiplayer behaviour is an open item.
