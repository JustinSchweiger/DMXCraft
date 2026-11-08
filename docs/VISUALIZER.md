# 3D-Visualisierer

*Tools → 3D-Visualisierer* öffnet den Visualisierer in einem eigenen Fenster
(z. B. auf einem zweiten Bildschirm, Vollbild über den Knopf oben rechts).

## Technik

* Gezeichnet wird auf der Grafikkarte mit **wgpu** – unter Windows und Linux
  über **Vulkan** (sonst DirectX 12), unter macOS über Metal. Es wird die
  leistungsstärkste Grafikkarte gewählt; Name und Schnittstelle stehen unten
  rechts im Bild.
* Szene in ein HDR-Ziel mit 4× Mehrfachabtastung, alle Scheinwerfer als
  Spotlights mit Lichtflecken auf den Flächen, Lichtstrahlen additiv im Dunst,
  ACES-Tonemapping. Lichtstrahlen enden am ersten Hindernis.
* Schatten: Die hellsten Scheinwerfer bekommen je eine Schattenkarte
  (Tiefen-Textur-Array, weich gefiltert); Bauten werfen Schatten.
* Bloom: helle Stellen (Linsen, Lichtflecken) strahlen weich über
  (verkleinern/vergrößern in fünf Stufen).
* Qualitätsstufen: Niedrig (60 % Auflösung, ohne Kantenglättung, 32 Lichter,
  ohne Schatten/Bloom), Mittel (4×, 96 Lichter, 8 Schatten à 512 px, Bloom),
  Hoch (256 Lichter, 16 Schatten à 1024 px), Ultra (150 % Auflösung,
  32 Schatten).
* Die Werte kommen direkt aus dem DMX-Ausgang: Dimmer, RGB/W/Amber/UV (auch je
  Zelle bei Bars), Strobe, Pan/Tilt (mit Feinkanal) bei Moving Heads.
* **Wertebereiche** der Kanäle (aus OFL, QLC+ und dem Profile Builder):
  Shutter offen/zu und Strobe-Frequenz, Farbrad-Felder mit ihrer Farbe,
  Gobo-Felder. Ohne Bereiche wird angenähert (Strobe unter 10 offen,
  Farbrad mit acht typischen Farben, Gobo mit sechs Mustern: Stern, Punkte,
  Ringe, Breakup, Balken, Spirale – im Lichtfleck und als Schächte im Strahl).
  Ältere Shows bekommen die Bereiche beim Öffnen aus der Bibliothek bzw. den
  eigenen Profilen (gleicher Schlüssel und gleiche Kanalnamen).
* **Fokus**: Der Fokuskanal stellt eine Schärfeweite von 1,5 bis 20 m ein;
  Lichtfleck und Gobo sind dort scharf, wo der Strahl in dieser Entfernung
  auftrifft, sonst weich.

## Veranstaltungsort

* Gehört zur Show und lässt sich als **`.dmxvenue`** (JSON) speichern und in
  anderen Shows laden. Einheiten: Meter, y nach oben, Drehungen in Grad
  (X, dann Y, dann Z).
* Formen: Quader, Fläche, Zylinder, Traverse (Drei-/Vierpunkt), Gruppen.
* **Bauen**: Objekte hinzufügen, im Objektbaum auswählen, Lage/Maße/Farbe/
  Deckkraft/Glanz/Leuchten ändern; Objekte und Geräte mit der Maus auf dem
  Boden verschieben (Raster 5 cm, Strg: frei).
* **Aufteilung**: links Raum, Hinzufügen und der Objektbaum (Gruppen
  aufklappbar; ein Klick im Bild klappt bis zum Objekt auf) bzw. im Reiter
  *Geräte* die Geräteliste mit Suche (Strg/Umschalt für Mehrfachauswahl);
  rechts angedockt die Eigenschaften der Auswahl in aufklappbaren Abschnitten.
  Rückgängig/Wiederholen oben neben *Bauen/Live* (oder Strg+Z/Strg+Y).
* **Ordner**: Im Objektbaum sind Gruppen Ordner – mit „+“ oder Rechtsklick
  anlegen, umbenennen (Doppelklick oder Menü), Objekte per Ziehen
  einsortieren (die Lage im Raum bleibt), „Auswahl in neuen Ordner“, nach
  Namen sortieren. Die Geräteliste hat eigene Ordner (nur zum Sortieren,
  im Ort gespeichert), Ziehen in einen Ordner, Sortierung nach Adresse oder
  Name.
* **Mehrfachauswahl** (Strg/Umschalt): Griffe und Ziehen wirken auf alle
  ausgewählten Objekte und Geräte; Drehen dreht jedes an seinem Platz.
* **Bibliothek**: mitgelieferte Objekte (Podest, Bühne, Treppe, DJ-Pult,
  Lautsprecher, Subwoofer, Traversen-Turm und -Rahmen, Leinwand, Tische,
  Theke, Person) und eigene: Objekt oder Ordner auswählen, „Auswahl als
  eigenes Objekt speichern“ (samt Modellen und Bildern, im
  Konfigurationsordner unter `objects/`).
* **Bilder (Texturen)**: im Material ein mitgeliefertes Bild (Holzdielen,
  Bühnenboden, Beton, Teppich, Ziegel, Fliesen, Molton) oder ein eigenes PNG/
  JPEG wählen; Kachelgröße in Metern; bei Ordnern für alles darin. Die Farbe
  tönt das Bild. glTF-Modelle bringen ihre Bilder (Grundfarbe) mit.
* **Griffe** an der Auswahl: rote/grüne/blaue Achse verschiebt entlang x/y/z
  des Raums, die drei Ringe drehen um x/y/z des Raums (5-cm- bzw. 5°-Raster,
  Strg: frei) – auch für Objekte in gedrehten Gruppen und für Geräte.
* **Eigene 3D-Modelle** (glTF/GLB) importieren: Geometrie mit allen
  Knoten-Lagen, die Grundfarben und Bilder der Materialien werden in
  die Show eingebettet; das Modell steht mit der Unterkante auf dem Boden.
  Bis 2 Mio. Dreiecke.
* **Geräte ausrichten**: auf einen Zielpunkt richten (Zahlen oder im Bild
  anklicken), senkrecht nach unten, an die nächste Traverse hängen – jeweils
  für alle ausgewählten Geräte.
* Geräte ohne eigene 3D-Lage stehen automatisch nach dem Bühnenplan unter der
  Decke.

## Import aus Easy View 2

Erwartet die Datei als XML. Übernommen werden Raum, Bauten (Quader, Flächen),
Traversen (als eigene Traversen), Umgebung und die Lage der Geräte; Geräte
werden über Universum und Adresse den Geräten im Patch zugeordnet. Für Geräte
ohne Gegenstück schlägt DMXCraft einen Typ aus der Bibliothek vor (Hersteller
und Modell aus dem Easy-View-Bibliotheksnamen, Kanalzahl wenn angegeben,
sonst generisch) und legt sie auf Wunsch an der richtigen Adresse an. Nicht
übernommen werden Easy-View-Modelle, Texturen und die eingebettete
Daslight-Gerätebibliothek.

## Nächste Schritte

Farbfelder und Gobo-Bilder je Gerät (Profile Builder), Zellen aus den
physischen Gerätedaten, Normal-/Rauheitskarten, Kamera-Voreinstellungen.
