# Friendly Bones für Horizon

Native Rust-Umsetzung von FriendlyBones: freundliche Skelett- und Wither-Skelett-Begleiter für Horizon. Kompatibel mit der Nightly `2026-11-13-9f42064` (Horizon-Commit `9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`, Minecraft 26.3). Version 0.2.0.

Die Mod besteht aus zwei regulären Paketdateien: `friendlybones_0.2.0_minecraft-horizon-26.3-9f42064.hnmod` (nativer Code als Mixin-Bundle) und `friendlybones-content_0.2.0_minecraft-horizon-26.3-9f42064.hmod` (Entity-Typen, Spawn-Ei, Texte). Der Launcher baut den Code in das unveränderte Spiel ein; ein eigener Spiel-Build ist nicht nötig.

## Funktionen

- Zwei Begleiter: Friendly Skeleton und Friendly Wither Skeleton.
- Beschwören: zwei Knochenblöcke übereinander, darauf eine Kürbislaterne (Skelett) beziehungsweise ein Wither-Skelettschädel (Wither-Skelett). Alternativ das Spawn-Ei in der eigenen Creative-Gruppe.
- Besitzer: Rechtsklick auf einen besitzerlosen Begleiter übernimmt ihn; nur der Besitzer öffnet das Menü.
- Modi: Roamen, Folgen, Wache, mit einstellbaren Radien (Menü per Rechtsklick, Maus und Tastatur).
- Kampf mit Bogen und Schwert, Verteidigungsalarm für den Besitzer, Friendly Fire reduziert, kein Tageslichtbrand, kein Despawn im Friedlichen Modus.
- Konfiguration: `config/friendlybones.json` der Instanz (wird beim ersten Start angelegt).

## Installation

Voraussetzung: Horizon-Launcher mit der Nightly `2026-11-13-9f42064`.

1. `dist/friendlybones_0.2.0_minecraft-horizon-26.3-9f42064.hnmod` als `friendlybones.hnmod` in `horizon-native-mods/` der Instanz kopieren.
2. `dist/friendlybones-content_0.2.0_minecraft-horizon-26.3-9f42064.hmod` als `friendlybones.hmod` in `horizon-mods/` derselben Instanz kopieren.
3. In `horizon-mods.txt` der Instanz die Zeile `mod.friendlybones:true` eintragen (oder die Mod unter Optionen > Mods einschalten).
4. Den Quellcode in `mixins/friendlybones/` lesen (die Mod wird mit den Rechten des Benutzers kompiliert und ausgeführt) und bauen:
   ```powershell
   horizon-launcher native-build --instance "NAME" --trust-native-code
   ```

`tools/dev.ps1 install -TrustNativeCode` erledigt alle Schritte für die Testinstanz „Friendly Bones Test“. Der erste Build dauert rund sechs Minuten, spätere Builds weniger.

Der Launcher dieser Nightly (0.2.63) baut den Code aus einer kombinierten `.hmod`, installiert deren Content aber nicht in den Spielbuild; deshalb zwei Dateien. Der direkte GitHub-Import des Launchers würde nur den Code installieren; der Content muss dann zusätzlich als `.hmod` in `horizon-mods/` liegen.

## Aufbau

| Pfad | Aufgabe |
| --- | --- |
| `horizon-native.json`, `horizon-mod.toml` | Manifeste für Code und Content |
| `assets/`, `data/` | Content: Entity-Typen, Spawn-Ei, Creative-Tab, Sprachtexte |
| `mixins/friendlybones/horizon-mixin.toml` | 19 Injektionen: Hook-Punkte und drei Engine-Anpassungen |
| `mixins/friendlybones/hooks/`, `patches/` | Helferfunktionen der Injektionen |
| `mixins/friendlybones/runtime.rs` | erzeugtes Support-Modul (nicht von Hand ändern) |
| `src/core.rs` | engine-unabhängige Entscheidungen mit Tests |
| `src/runtime.rs`, `src/ui.rs` | Engine-Adapter und natives Menü |
| `tools/` | `assemble.py` (Modul erzeugen), `package.py` (`.hnmod` und Content-`.hmod`), `dev.ps1`, `plan.sh` |

Entwicklung: `src/` ändern, `tools/dev.ps1 test`, dann `tools/dev.ps1 package`.

## Technische Eigenheiten dieses Horizon-Stands

- Horizons Content-Prüfung kennt kein Skelett als Vorlage für eigene Entity-Typen. Die beiden Typen sind deshalb mit der Platzhaltervorlage `minecraft:zombie` deklariert; die Injektion `entity-rows` setzt beim Bau der Entity-Tabelle die echte Vorlage (`minecraft:skeleton` beziehungsweise `minecraft:wither_skeleton`) ein.
- Neue Begleiter starten eingefroren (Injektion `goals`); die Laufzeit installiert danach ihre Ziele. `peaceful` nimmt sie vom Peaceful-Despawn aus.
- Drei Hook-Punkte (`native_client_tick_pre`, `native_living_incoming_damage_v3`, `native_living_damage_post`) haben in diesem Commit keinen Katalognamen und sind direkt per Funktionspfad angebunden. Die Mod ist deshalb an genau diesen Commit gebunden und trägt kein `hooks`-Feld.

Lizenz: CC0-1.0 für den Mod-Code. Horizon-Quellen bleiben MIT und gehören nicht zum Paket.

Stand der Abnahme: [VALIDIERUNG.md](VALIDIERUNG.md). Ursprüngliche Funktionsanalyse: [PORTIERUNG.md](PORTIERUNG.md).
