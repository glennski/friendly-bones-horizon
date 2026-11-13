# Validierung

Ziel: Horizon `9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`, Windows x64, Rust 1.99.1, Mod-Version 0.2.0.

## Bestanden

- Acht Core-Tests (`tools/dev.ps1 test`): Wertebereiche, Besitzerrechte, Follow-Hysterese, Guard-Rückkehr, temporärer Alarm, Waffenwechsel, Friendly Fire, Roam-Grenzen.
- `horizon-mixins plan` und `apply` auf dem unveränderten Quellstand: 19 Injektionen, 1 Modul, keine Konflikte.
- `cargo check` des gestagten Spiels: fehlerfrei.
- Offizieller Launcher-Build (`horizon-launcher native-build`, Profil `dev`, Rust 1.99.1) aus dem `.hnmod` in der Testinstanz „Friendly Bones Test“: `native-9f42064-…`, gebaut auf der unveränderten Nightly (`on_base: true`, nichts ausgelassen), erster Build 6 min 17 s.

## Im Spiel geprüft (Debug-Instanz, Steuerkonsole, Creative, Testwelt `FBTest`)

- Beide Typen per `/summon` mit 20 Gesundheit; sie wandern (Roam).
- Peaceful: beide Begleiter bleiben bestehen.
- Rechtsklick öffnet das Menü; Besitzer wird übernommen („Player…'s Bones“). Moduswechsel auf Guard und Kampfradius per Mausklick, Chatbestätigung „guarding its post“.
- Speichern, Beenden, Neustart: beide Entities, Besitzer, Modus Guard und Radius 6 bleiben erhalten.
- Kampf: Ein Begleiter tötet einen Zombie (verliert dabei Gesundheit).
- Spawn-Ei erzeugt einen Begleiter mit 20 Gesundheit.
- Skelett-Ritual (zwei Knochenblöcke, Kürbislaterne): Blöcke verschwinden, Begleiter erscheint.
- Screenshots: `docs/screenshots/`.


## Nicht abgenommen

- Wither-Ritual und Wither-Effekt der Pfeile, Tageslicht, Alarm, Friendly Fire gezielt, Follow-Modus im Gelände.
- Begrenzte Munition, Aufheben und Ausrüstung ohne Verlust oder Duplikation.
- Chunk-Unload; Menü per Tastatur und Controller; zwei Spieler, fremde Besitzer, dedizierter Server.
- Performance: nur das `dev`-Profil gebaut, kein A/B-Vergleich.
- Offizieller Paketvalidator `cargo horizon-mod check`: nicht ausgeführt; die Pakete folgen `docs/api/mod-files.md` und wurden vom Launcher gelesen.
- Migration alter Fabric-Spielstände ist nicht vorgesehen.
