# 3D-Visualisierer

Reiter **3D** (oder *Tools → 3D-Visualisierer* für ein eigenes Fenster, z. B.
auf einem zweiten Bildschirm).

## Technik

* Gezeichnet wird auf der Grafikkarte mit **wgpu** – unter Windows und Linux
  über **Vulkan** (sonst DirectX 12), unter macOS über Metal. Es wird die
  leistungsstärkste Grafikkarte gewählt; Name und Schnittstelle stehen unten
  rechts im Bild.
* Szene in ein HDR-Ziel mit 4× Mehrfachabtastung, alle Scheinwerfer als
  Spotlights mit Lichtflecken auf den Flächen, Lichtstrahlen additiv im Dunst,
  ACES-Tonemapping. Lichtstrahlen enden am ersten Hindernis.
* Qualitätsstufen: Niedrig (60 % Auflösung, ohne Kantenglättung, 32 Lichter),
  Mittel (4×, 96 Lichter), Hoch (4×, 256 Lichter), Ultra (150 % Auflösung).
* Die Werte kommen direkt aus dem DMX-Ausgang: Dimmer, RGB/W/Amber/UV (auch je
  Zelle bei Bars), Strobe, Pan/Tilt (mit Feinkanal) bei Moving Heads.

## Veranstaltungsort

* Gehört zur Show und lässt sich als **`.dmxvenue`** (JSON) speichern und in
  anderen Shows laden. Einheiten: Meter, y nach oben, Drehungen in Grad
  (X, dann Y, dann Z).
* Formen: Quader, Fläche, Zylinder, Traverse (Drei-/Vierpunkt), Gruppen.
* **Bauen**: Objekte hinzufügen, im Objektbaum auswählen, Lage/Maße/Farbe/
  Deckkraft/Glanz/Leuchten ändern; Objekte und Geräte mit der Maus auf dem
  Boden verschieben (Raster 5 cm, Strg: frei).
* Geräte ohne eigene 3D-Lage stehen automatisch nach dem Bühnenplan unter der
  Decke.

## Import aus Easy View 2

Erwartet die Datei als XML. Übernommen werden Raum, Bauten (Quader, Flächen),
Traversen (als eigene Traversen), Umgebung und die Lage der Geräte; Geräte
werden über Universum und Adresse den Geräten im Patch zugeordnet. Nicht
übernommen werden Easy-View-Modelle, Texturen und die eingebettete
Daslight-Gerätebibliothek.

## Nächste Schritte

Greifer zum Drehen/Skalieren, Geräte auf Traversen hängen und ausrichten,
Schatten und Bloom (Qualität Hoch/Ultra), physische Gerätedaten
(Abstrahlwinkel, Pan/Tilt-Bereich, Zellen), Farbrad/Gobos, Zuordnung fehlender
Geräte beim Import, eigene 3D-Modelle (glTF).
