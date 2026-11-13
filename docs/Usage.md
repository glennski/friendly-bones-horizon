# Usage

[Wiki home](Home.md) · [Companion behaviour](Companion-Behaviour.md)

## Getting a companion

There are three ways to get a companion.

### Skeleton ritual

Build a column of two **bone blocks** and place a **jack-o'-lantern** on top. The lantern must be the last block you place.

- Column from the bottom up: bone block, bone block, jack-o'-lantern.
- The two bone blocks and the lantern are used up, and a Friendly Skeleton appears.
- The player who placed the lantern becomes the owner.

### Wither ritual

Build the same column, but finish it with a **wither skeleton skull** instead of a jack-o'-lantern. This spawns a Friendly Wither Skeleton.

> Status: implemented, not yet verified in game. See [Validation](Validation.md).

### Spawn egg

The **Friendly Skeleton Spawn Egg** is in the creative tab "Friendly Bones". Using it on a block spawns a Friendly Skeleton owned by you. In survival the egg is used up; in creative it is not.

## Claiming and owning

A companion without an owner is claimed by right-clicking it. The companion then belongs to you, gets your name over its head ("<name>'s Bones") and opens its menu.

- Only the owner can open the menu and change settings.
- The owner must be within 12 blocks of the companion to right-click it. Other players are refused.
- Companions remember their owner, mode and settings across restarts.

## The menu

Right-click an owned companion to open the **Friendly Bones** menu.

| Control | What it does |
| --- | --- |
| Roam / Follow / Guard | Switches the mode. The companion confirms in chat. |
| Roam radius | Distance the companion may wander from its anchor. |
| Stay within | In follow mode, how close the companion stays to you. |
| Catch up at | In follow mode, the distance at which it starts running to you. |
| Combat radius | In guard mode, how far it may leave its post to fight. |
| Set here | Moves the anchor (roam centre or guard post) to where you stand. |
| Done | Closes the menu and sends the changes to the server. |

Changes are checked by the server before they are stored. Values outside the allowed ranges are clamped; see [Configuration](Configuration.md).

### Controls

- **Mouse:** click mode buttons and "Set here"/"Done", drag sliders.
- **Keyboard:** Tab and Shift+Tab move the focus, arrow keys change the focused slider or mode, Enter or Space activates the focused button, Escape closes the menu.
- **Controller:** not verified.

## Feedback

- Each mode change shows a short status message in chat (can be turned off with `announceStatusChat`).
- Mode changes show particles around the companion.
