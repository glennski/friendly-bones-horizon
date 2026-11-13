# Configuration

[Wiki home](Home.md) · [Companion behaviour](Companion-Behaviour.md)

The file is `config/friendlybones.json` in the game folder of the instance. It is created with the defaults below on first start. An invalid file is replaced by defaults in memory; the file itself is kept so you can fix it.

## Example (defaults)

```json
{
  "skeleton": {
    "maxHealth": 20.0,
    "speed": 0.25,
    "followRange": 32.0,
    "attackRange": 36.0,
    "damageMult": 1.0,
    "rateOfFire": 20,
    "unlimitedAmmo": true
  },
  "wither_skeleton": {
    "maxHealth": 20.0,
    "speed": 0.25,
    "followRange": 32.0,
    "attackRange": 36.0,
    "damageMult": 1.0,
    "rateOfFire": 20,
    "unlimitedAmmo": true
  },
  "alertIfPlayerAttacked": true,
  "alertRadius": 64.0,
  "friendlyFireDamageMultiplier": 0.25,
  "announceStatusChat": true
}
```

The file is read when a world is opened. Reopen the world after editing it.

## Per-type settings

`skeleton` applies to Friendly Skeletons. `wither_skeleton` applies to Friendly Wither Skeletons. Both have the same keys.

| Key | Default | Allowed range | Meaning |
| --- | --- | --- | --- |
| `maxHealth` | 20 | 1 – 10000 | Maximum health |
| `speed` | 0.25 | 0.01 – 2 | Movement speed |
| `followRange` | 32 | 4 – 128 | Distance at which the companion notices targets |
| `attackRange` | 36 | 3 – 128 | Maximum bow range |
| `damageMult` | 1.0 | 0 – 100 | Damage multiplier for the companion's attacks |
| `rateOfFire` | 20 | 1 – 1200 | Ticks between shots (20 ticks = 1 second) |
| `unlimitedAmmo` | true | true / false | If false, arrows are used up |

## Global settings

| Key | Default | Allowed range | Meaning |
| --- | --- | --- | --- |
| `alertIfPlayerAttacked` | true | true / false | Enables the defence alarm |
| `alertRadius` | 64 | 1 – 256 | Radius of the alarm area around the owner |
| `friendlyFireDamageMultiplier` | 0.25 | 0 – 1 | Damage factor for friendly fire |
| `announceStatusChat` | true | true / false | Show mode changes in chat |

Values outside the allowed range are clamped, and non-numeric values fall back to the default.

## Menu limits (not in the file)

The menu enforces these limits per companion. They are not configurable yet.

| Setting | Range | Default |
| --- | --- | --- |
| Roam radius | 2 – 32 blocks | 12 |
| Stay within | 2 – 12 blocks | 6 |
| Catch up at | 4 – 32 blocks, always at least 2 blocks more than "Stay within" | 16 |
| Combat radius | 3 – 24 blocks | 10 |
