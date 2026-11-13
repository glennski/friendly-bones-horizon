# Friendly Bones nativ für Horizon entwickeln

[Projektübersicht](README.md)

## Ziel und Abgrenzung

FriendlyBones soll seine bisherigen Spielfunktionen auf Horizon erhalten und technisch an die Rust-Engine angepasst werden. Funktionale Gleichwertigkeit bedeutet, dass Spieler dieselben gewünschten Möglichkeiten erhalten. Sie verlangt nicht dieselben Fabric-Klassen, dieselbe interne Netzwerkdarstellung oder die Übernahme unbeabsichtigter Fehler.

Die aktuelle Einschätzung beruht auf Quellcodelektüre, nicht auf einem laufenden Rust-Prototyp. Die positive Machbarkeitseinschätzung ist deshalb noch keine Bestätigung vollständiger Funktionsgleichheit.

## Geprüfte Ausgangsbasis

Fabric-Projekt: `E:\UserData\carst\Projects\Minecraft Mods\friendlybones`. Lokale Versionsangabe: `1.5.0`, Unreleased. Die gemeinsame Implementierung liegt in `src/main`, die Minecraft-Anpassungen in `src/mc1213` und `src/mc12111`, die Oberfläche in `src/client`.

Horizon wurde gegen den offiziellen Hauptquellstand `441f49d57e344bd56b90fd4af5c4e443ff3937c9` vom 13. November 2026 geprüft. Das ist der analysierte Stand, kein bereits getestetes Buildziel und keine Bestätigung der lokal installierten Launcher-/Spielversion. Die Dateien des Fabric-Projekts sind in [source-inventory.json](source-inventory.json) erfasst; der Git-Arbeitsbaumstatus wurde nicht festgestellt.

## Tatsächlicher Funktionsumfang

| Bereich | Verhalten im vorhandenen Code | Grundlage |
| --- | --- | --- |
| Mobarten | Friendly Skeleton und Friendly Wither Skeleton, mit eigenen Entity-IDs | `ModEntities`, die beiden Entity-Klassen |
| Skelett-Ritual | zwei übereinander stehende Knochenblöcke und darüber eine Jack-o-Lantern; die drei Blöcke werden verbraucht | `FriendlyBonesMod.onInitialize` |
| Wither-Ritual | Knochenblock-Säule mit Wither-Skeleton-Schädel | `FriendlyBonesMod.onInitialize` |
| Spawn-Ei | eigenes Skelett-Spawn-Ei in der Creative-Gruppe | `ModItems`, `FriendlyBonesMod` |
| Besitzer | beim Ritual nächster Spieler innerhalb von acht Blöcken; sonst Übernahme beim ersten passenden Rechtsklick; Besitzername wird angezeigt | Spawnpfad und `FriendlyLogic` |
| Rechte | nur der Besitzer öffnet das Menü; Server lehnt Änderungen durch andere Spieler oder jenseits von zwölf Blöcken ab | `FriendlyMenu`, `interactMob` |
| Roam | Umherwandern um einen Mittelpunkt, Zielsuche im Gebiet und Rückkehr nach zu weiter Verfolgung | `RoamGoal`, `allowsTarget`, `roamTick` |
| Follow | Besitzer folgen, mit getrennten Start-/Stoppabständen; normaler Kampf kann Folgen unterbrechen | `followOwnerTick` |
| Guard | Posten halten, im konfigurierten Kampfradius bewegen und anschließend zum Posten zurückkehren | `guardTick` |
| Menü | drei Modustasten, situationsabhängige Regler, Mittelpunkt-/Postenwahl und Fertig; Übertragung beim Schließen | `FriendlyMenuScreen`, `MenuPayload` |
| Reglerschranken | Roam 2–32, Follow-Stopp 2–12, Follow-Start 4–32, Guard 3–24 Blöcke; Start mindestens zwei Blöcke über Stopp | `FriendlySettings` |
| Zielwahl | feindliche Monster, keine Spieler, keine Friendly Bones, keine Endermen; passive Tiere werden vor der Auswahl ausgefiltert | `FriendlyLogic.initGoals`, `isThreat`, `canTarget` |
| Fernkampf | Bogen, konfigurierbarer Angriffsradius, Feuerrate und Pfeilschadensfaktor | `FriendlyLogic`, `shootAt` |
| Nahkampf | Wechsel zum Eisenschwert unter drei Blöcken, Rückwechsel zum Bogen ab fünf; Zwischenbereich vermeidet ständiges Umschalten | `updateCombatEquipment` |
| Wither-Effekt | Wither-Skelettpfeile erhalten einen Effekt für 200 Ticks; außerdem gelten geerbte Eigenschaften des Wither-Skeletts | `createArrowProjectile` und Vanilla-Basisklasse |
| Munition | standardmäßig unbegrenzt, optional Prüfung und Verbrauch von Pfeilen aus einer Hand | `canShoot`, `onShot`, `consumeAmmo` |
| Ausrüstung | Aufheben passender Ausrüstung, Bogen, Schwert und Pfeile; Vergleich und Wechsel werden teilweise an Vanilla delegiert | Entity-Klassen und Vanilla-Verhalten |
| Tageslicht | Friendly Bones sollen keine Sonnenbrandschäden wie normale Skelette erhalten | 1.21.3-Override beziehungsweise `FriendlyDaylightMixin` |
| Verteidigungsalarm | nach erfolgreichem Monsterschaden am Besitzer reagieren eigene Bones im konfigurierten Umgebungsbereich; Angreifer priorisieren, danach weitere nahe Bedrohungen | `LivingEntityMixin`, `FriendlyAlert`, `alertTick` |
| Alarmzustand | zunächst 200 Ticks; Anlaufen zum Besitzer, frühzeitiges Ende ohne lebenden Besitzer oder Bedrohung | `AlertGoal`, `alertTick` |
| Friendly Fire | Bones gegen Spieler/Bones und Besitzer gegen eigene Bones werden standardmäßig auf Faktor 0,25 reduziert; Bones sollen einander nie gezielt angreifen | `FriendlyFire`, Zielwahl |
| Persistenz | Modus, Besitzer-UUID, Mittelpunkt und vier Reglerwerte; dazu die Zustände, die die Basisklassen speichern | `FriendlyData` |
| Konfiguration | getrennte Entity-Konfigurationen, Statuschat, Alarm und Friendly Fire; alte flache Konfiguration wird teilweise migriert | `FriendlyConfig` |
| Rückmeldung | Moduspartikel, Status-/Chatmeldungen und deutsche/englische Texte | `FriendlyLogic`, Sprachdateien |

Anmerkung zum Alarmradius: Die Java-Implementierung sucht Bones innerhalb einer erweiterten Bounding-Box. Ein Radiuswert bedeutet hier nicht exakt dieselbe Auswahl wie eine kugelförmige Distanzprüfung. Für einen Nachbau Grenzfälle bewusst festlegen.

Die Oberfläche ist überschaubar. Der größere Teil der Arbeit steckt im Zusammenspiel von Kampf, Bewegung, Besitz, Speicherung und Client-/Serverzuständen.

## Was Horizon bereits bereitstellt

| Bedarf | Nachgewiesene Grundlage im analysierten Horizon-Stand | Konsequenz |
| --- | --- | --- |
| eigene KI-Ziele | `native_register_goals`, `Hooks::register_goals`, `Goal`, `GoalSelector` | Ziele der Vorlage gezielt ersetzen beziehungsweise ergänzen |
| Bewegung und Navigation | `Mob::nav_move_to`, vorhandene Navigation und Wegsuche | kein eigenes allgemeines Pathfinding schreiben |
| vorhandene Verhaltensbausteine | unter anderem `Float`, `Stroll`, `LookAtPlayer`, `MeleeAttack`, `NearestTarget` | Standardverhalten wiederverwenden; passende Ziel- und Waffenbedingungen ergänzen |
| Zustand pro Entity | Attachments mit optionalem Codec, Persistenz und Synchronisierung | Besitzer und Moduseinstellungen an stabile Entity-Identität binden |
| Rechtsklick | `native_interact_entity` | serverseitige Besitzerprüfung und Menüanforderung |
| Client-Oberfläche | eigener Mod-Screen sowie Render-, Maus- und Tastatur-Hooks | Menü nativ neu aufbauen; Reglerinteraktion und Fokus selbst anbinden |
| Mod-Netzwerk | Nachrichten zwischen Server- und Clienthälfte | serverseitig geprüfte Einstellungen übertragen |
| Schaden | native Schaden-Hooks | Friendly Fire und Alarm an passende Ereignisse anbinden; Zeitpunkte unterscheiden |

Quellen: [Mob-Ziele und Pathfinding](https://github.com/horizon-rust/horizon/blob/441f49d57e344bd56b90fd4af5c4e443ff3937c9/docs/native-mods.md#mob-goals), [Goal-Implementierungen](https://github.com/horizon-rust/horizon/blob/441f49d57e344bd56b90fd4af5c4e443ff3937c9/crates/horizon-client/src/mobs/goals.rs), [native Referenz](https://github.com/horizon-rust/horizon/blob/441f49d57e344bd56b90fd4af5c4e443ff3937c9/docs/native-mods.md).

Diese APIs stehen zum Teil unter experimentellen beziehungsweise noch vorläufig dokumentierten Verträgen. Ein passender Name in der Dokumentation reicht nicht: die Signatur und Verfügbarkeit im tatsächlichen Zielbuild prüfen.

## Die wichtigste bekannte Integrationslücke

Horizons Content-Registrierung führt in `ENTITY_TEMPLATES` nur Snowball, Villager, Wandering Trader und Zombie. Die lebenden Vorlagen enthalten also **weder Skeleton noch Wither Skeleton**. Das ist im Paketleser selbst bestätigt und nicht nur eine unvollständige Beispielauflistung. [Quellcode der Vorlagenliste](https://github.com/horizon-rust/horizon/blob/441f49d57e344bd56b90fd4af5c4e443ff3937c9/crates/horizon-mods/src/content.rs#L452)

Ein eigenes Skelett lässt sich daher nicht einfach mit einer JSON-Zeile aus der Standardregistrierung erstellen. Die naheliegende Zielrichtung ist, die passenden Skelettvorlagen über eine kleine native Source-Ergänzung oder eine Upstream-Erweiterung verfügbar zu machen und anschließend die vorhandene Engine-Darstellung und Kampfmechanik zu verwenden. Wie weit diese Ergänzung reicht, ist durch einen Build nachzuweisen.

Ein gewöhnliches Vanilla-Skelett nur mit einem Marker zu versehen, kann ein früher Techniktest sein. Für vollständige Gleichwertigkeit müssen aber eigene Identität, Peaceful-/Despawnverhalten, Tageslichtschutz, Darstellung und Speicherung stimmen. Ebenso wäre ein als Skelett gezeichneter Wandering Trader ohne weitere Nachweise kein vollständiger Ersatz.

Auch die GUI ist kein direkter Ersatz für Fabric-Widgets: Die native Screen-API bietet Zeichnung und Eingabe, aber nicht automatisch den vorhandenen Java-Slider samt Fokus, Drag-Verhalten und Netzwerkprotokoll. Controller- und Tastaturbedienung gesondert prüfen; die dokumentierte eigene Mod-Screen-API nennt Einschränkungen für Gamepad-Klicks.

## Gezielt von Rust und der Engine profitieren

### Engine Ereignisse statt nachträglicher Sucharbeit

Die Fabric-Implementierung sucht in jedem serverseitigen Mob-Tick nach jungen eigenen Pfeilen im Umkreis und skaliert deren Schaden nachträglich. Das ist ein konkreter Ansatzpunkt: Bei der nativen Umsetzung zuerst einen passenden Erzeugungs- oder Schadenspfad prüfen und den Faktor direkt dort anwenden.

Ein anderer Zeitpunkt darf das Ergebnis nicht unbeabsichtigt ändern: Pfeil-Basisschaden, Rüstung, Effekte und Friendly-Fire-Faktor sind unterschiedliche Schritte. Ihre Reihenfolge muss im Vergleichstest bestätigt werden. Falls kein geeigneter vorhandener Hook existiert, ist ein kleiner gezielter Adapter möglich; weniger Sucharbeit ist hier zunächst ein Entwurfsziel, noch kein gemessener Gewinn.

### Zustände mit ihren notwendigen Daten modellieren

Rust-Enums erlauben, einen Guard-Modus unmittelbar mit seinem Posten und Radius und einen Roam-Modus mit seinem Mittelpunkt zu beschreiben. Ein Alarm ist eine zeitlich begrenzte Überlagerung mit Rückkehr zum vorherigen Verhalten, kein versehentlich dauerhaft ersetzter Grundmodus.

Das vermeidet intern Zustände wie „Wache ohne Posten“, soweit das Datenmodell diese Kombination ausschließt. Eingelesene Dateien und Netzwerkdaten müssen trotzdem validiert werden: Rust erkennt keine falschen Besitzerrechte oder unzulässigen Distanzen allein durch Kompilierung.

### Die vorhandene Engine arbeiten lassen

Navigation, Kollision, Animation und Standardangriffe nach Möglichkeit wiederverwenden. FriendlyBones liefert Entscheidungen wie „dieser Spieler ist mein Besitzer“, „dieses Ziel ist erlaubt“ oder „ich kehre zum Posten zurück“. Es soll keine zweite komplette Bewegungssimulation neben Horizon entstehen.

Der bestehende Abstand von drei beziehungsweise fünf Blöcken für den Waffenwechsel ist bereits eine sinnvolle Hysterese und wird erhalten. Nicht jede bestehende Lösung muss allein wegen der neuen Sprache ersetzt werden.

### Weniger Arbeit im Tick

Einstellungen und Besitzerzustand nur bei tatsächlichen Änderungen neu aufbereiten. Wege in kontrollierten Abständen beziehungsweise bei verändertem Ziel aktualisieren. Für einen Alarm die notwendigen eigenen geladenen Begleiter bestimmen; keinen dauernden Vollweltscan einführen. Erst messen, bevor zusätzliche Indizes, Threads oder ein eigener Scheduler gebaut werden.

Rust kann dabei günstige Datenrepräsentationen und Wiederverwendung von Speicher ermöglichen. Der messbare Gewinn hängt aber stärker von Häufigkeit und Umfang der Arbeit ab als von der Sprachwahl allein. Die aktuelle Mod verwendet schon begrenzte Suchbereiche; daraus keinen angeblich vorhandenen Vollweltscan machen.

### Gemeinsame Regeln auf Server und Client

Dieselben Wertebereiche und Datentypen für Menü und Spielzustand verwenden, soweit die Architektur das zulässt. Der Server bleibt maßgeblich für Besitzer, Entity-Zuordnung, Distanz und akzeptierte Werte. Nach Änderungen kann er den bestätigten Zustand zurücksenden. Das verbessert die Verlässlichkeit bei alten Menüzuständen oder manipulierten Nachrichten.

## Bestehende Eigenheiten bewusst entscheiden

1. **Ritual und Dimension:** Die aktuelle Warteschlange speichert nur `BlockPos` und wird von Welt-Ticks verbraucht. Für die native Umsetzung die Dimension und den auslösenden Spieler mitführen. Das vermeidet eine mögliche Zuordnung zur falschen Welt und erlaubt eine bewusste Besitzerregel.
2. **Besitzerwahl:** Das aktuelle Ritual nimmt den nächstgelegenen Spieler, nicht ausdrücklich den Erbauer. Änderung zum tatsächlichen Auslöser wäre eine sichtbare Verbesserung und sollte als solche dokumentiert werden.
3. **Spawn-Ei und Ritual:** Der Ritualpfad initialisiert Ausrüstung und konfigurierbare Attribute explizit in `spawn`. Das Spawn-Ei erstellt den Entity-Typ über einen anderen Pfad. Ein gemeinsamer Initialisierungspunkt sollte für beide Wege nachgewiesen werden.
4. **Begrenzte Munition:** Der Code hält normalerweise Bogen und Schwert in beiden Händen, erwartet für begrenzte Munition aber einen Pfeilstapel in einer Hand. Die gewünschte Munitionsverwaltung muss deshalb fachlich geklärt und vollständig getestet werden.
5. **Spielstandmigration:** Gleiches zukünftiges Verhalten ist ein anderes Ziel als die Übernahme alter Fabric-Entities samt NBT. Die alten `fb_*`-Felder können als Migrationsgrundlage dienen; ein funktionierender Weltimport ist nicht automatisch zugesichert.

## Implementierungsreihenfolge mit vollständigem Zielumfang

1. Zielcommit und kompatiblen Launcher festlegen. Eigene Skelettregistrierung, Darstellung und Save/Load als technischen Kern nachweisen.
2. Besitzer, Tageslicht-/Despawnregeln, Ritual und Spawn-Ei über einen gemeinsamen Initialisierungspfad ergänzen.
3. Roam, Follow und Guard auf vorhandene Navigation aufsetzen; Bogen/Schwert, erlaubte Ziele und Friendly Fire prüfen.
4. Wither-Variante, Effekte, Ausrüstungsverhalten, Konfiguration und Verteidigungsalarm vervollständigen.
5. Rechtsklickmenü mit Reglern, Rechteprüfung, Serverbestätigung und Sprachtexten integrieren.
6. Normalen Paket-/Repository-Import und vollständige Abnahme durchführen.

Das trennt technische Unsicherheit von Routinearbeit. Die Reihenfolge verkleinert nicht den vereinbarten gewünschten Funktionsumfang.

## Nachweis der Funktionsgleichheit

Die vorhandenen GameTests liefern bereits gute Ausgangsfälle: gespeicherte Besitzer-/Modusdaten, Zielwahl neben passiven Mobs, Wechsel zwischen Nah- und Fernkampf, begrenzte Menüwerte, reduzierter Friendly Fire, Alarm nur für eigene Bones und begrenztes Roaming. Der Clienttest prüft Menüklicks, Reglerübertragung und einen Verteidigungsalarm mit realem Client und integriertem Server. Diese Tests wurden hier nicht ausgeführt.

Zusätzlich für Horizon prüfen:

- beide Spawnrituale und das Spawn-Ei mit identischer Grundinitialisierung;
- zwei Spieler, fremder Besitzer und zu große Menüdistanz;
- neue Welt, Speichern/Laden, Chunk-Unload und Neustart;
- Friedlich-Modus, Tageslicht, Wither-Effekt und Ausrüstungswechsel;
- Grenzen der drei Modi und Alarmende einschließlich Rückkehr zum vorherigen Verhalten;
- begrenzte Munition, keine Pfeil-/Itemduplikation und korrekte Schadensreihenfolge;
- vollständige Menübedienung per Maus und Tastatur sowie dokumentierte Controller-Unterstützung;
- A/B-Messung mit derselben Welt, demselben Buildprofil und definierter Begleiterzahl.

Die Java-GameTests lassen sich nicht als Fabric-Code in Horizon ausführen. Ihre Szenarien und überprüfbaren Erwartungen werden übernommen. Ein erfolgreicher Compilelauf allein genügt nicht für einen vollständigen Port.
