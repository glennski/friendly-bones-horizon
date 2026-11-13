# Companion behaviour

[Wiki home](Home.md) · [Usage](Usage.md) · [Configuration](Configuration.md)

This page describes what companions do. Items marked **not verified** are implemented but were not yet checked in game; see [Validation](Validation.md).

## Modes

### Roam (default)

The companion wanders around its anchor point, within the roam radius. When it is too far away, it walks back. Roam is the default for new companions.

Verified: companions move around while in roam mode.

### Follow

The companion follows its owner.

- It keeps within **Stay within** blocks of the owner.
- When the owner moves further than **Catch up at** blocks, it runs to catch up.
- Normal combat can interrupt following; the companion returns to following afterwards.

> Status: not verified.

### Guard

The companion holds its post (the anchor set with "Set here") and fights enemies within the **combat radius**. After a fight it returns to its post.

Verified: switching to guard works and is confirmed in chat. Post behaviour is not verified.

## Combat

- Targets are hostile monsters. Players, other companions, endermen and passive animals are never targeted.
- **Ranged:** companions carry a bow and shoot at targets within their attack range. Fire rate follows `rateOfFire`.
- **Melee:** a sword is equipped when a target is close, and the bow again once it is farther away. There is a gap between the two distances so the companion does not switch back and forth.
- **Wither skeleton companions** shoot arrows that apply the Wither effect for 200 ticks.
- **Ammo:** by default ammo is unlimited (`unlimitedAmmo: true`). With limited ammo, each shot uses one arrow from the companion's own ammo count.

> Status: combat was observed once (a companion took damage from a zombie). Target choice, ammo, Wither effect and sword switching are **not verified**.

## Defence alarm

When a monster damages the owner, nearby companions of that owner react for 200 ticks (10 seconds). Companions within `alertRadius` blocks of the owner, measured as a box rather than a sphere, respond. They attack the attacker first, then other nearby threats. When the alarm ends, they return to their previous mode.

> Status: not verified.

## Friendly fire

Damage between companions and the owner is reduced by `friendlyFireDamageMultiplier` (default 0.25). Companions do not target each other on purpose. Damage from companions to other players is reduced the same way.

> Status: not verified.

## Peaceful difficulty

Companions survive the Peaceful difficulty. Vanilla would remove mobs of most types in Peaceful; companions are exempt.

Verified: both companion types stayed alive in Peaceful.

## Daylight

Companions do not burn in daylight, unlike vanilla skeletons.

> Status: not verified.

## Persistence

Owner, mode, anchor, radius settings and inventory state are saved with the companion.

Verified: after saving, quitting and starting again, the owner, the guard mode and the combat radius were still set.

Companions far from a player and unloaded chunks: **not verified**.
