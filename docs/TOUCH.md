# Touch-Oberfläche

Im Reiter **Touch** (oben neben „Einrichten“ und „Steuern“) liegt eine frei gestaltbare Bedienoberfläche aus Seiten mit einem Raster. Darunter bleiben Bühnenplan und Programmer sichtbar. Die Oberfläche wird in der Show gespeichert (`touch`) und ist für eine spätere mobile App gedacht.

## Elemente hinzufügen

* **Alt + Klick** auf ein Bedienelement legt es auf der aktuellen Touch-Seite an:
  * Szenen, Bank-Chips, Pause/Schritt im Bankkopf,
  * Gruppen, Geräte,
  * Programmer-Fader,
  * Knöpfe der Seitenleiste.

  Gibt es noch keine Seite, entsteht „Seite 1“.
  * **Programmer-Fader** steuern danach fest die Geräte, die beim Anlegen ausgewählt waren.
  * **Farbfeld bzw. XY-Feld des Programmers** (Alt + Klick) legen ein Farbrad bzw. XY-Feld für die ausgewählten Geräte an.
* **Kontextmenü** einer Szene oder eines Geräts im Plan: „Zur Touch-Oberfläche hinzufügen“.
* **Bearbeiten → Bedienelemente hinzufügen:**
  * Beschriftung, Schaltfläche, Fader, Drehregler,
  * Farbrad und XY-Feld, beide für die aktuelle Auswahl.

  Eine neue Schaltfläche, ein Fader oder ein Drehregler hat noch kein Ziel (roter Punkt). Ist sie gewählt, gibt ihr Alt + Klick auf ein Bedienelement dieses Ziel.

## Bearbeiten

**Bearbeiten** oben rechts schaltet den Bearbeiten-Modus um:

* **Seiten:**
  * hinzufügen mit „+“,
  * umbenennen per Doppelklick,
  * löschen per Rechtsklick.
* **Elemente:**
  * auf freie Zellen ziehen,
  * mit dem Griff unten rechts die Größe ändern,
  * Rechtsklick: Duplizieren, Löschen, Farbe,
  * Entf löscht das gewählte Element.
* **Eigenschaften:**
  * Beschriftung, Textgröße, Größe, Farbe,
  * Flash-Taste bei Schaltflächen,
  * „Auswahl übernehmen“ bei Farbrad und XY-Feld.

Die Lupen ändern die Zellgröße.

## Bedienen

* **Schaltflächen** lösen ihr Ziel aus wie eine Taste der Tastatur- und MIDI-Zuordnung (siehe `MAPPING.md`). Aktive Szenen und gewählte Gruppen haben einen weißen Rahmen.
* **Fader und Drehregler** setzen den Wert direkt, z. B. Master oder feste Kanäle.
* **Farbrad:** Farbton nach rechts, Sättigung nach unten. Es setzt Rot, Grün und Blau der Geräte als Live-Werte.
* **XY-Feld:** setzt Pan und Tilt als Live-Werte.

Knöpfe des Programmers oder der Seitenleiste wirken nur, solange das jeweilige Element angezeigt wird.
