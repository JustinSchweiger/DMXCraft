# OBS-Anbindung

DMXCraft steuert OBS (ab Version 28) über dessen eingebauten WebSocket-Server
(obs-websocket 5). OBS spielt Videos selbst ab – DMXCraft sagt nur, welche
Datei, ab welcher Stelle und in welchem Tempo.

## Einrichten

1. In OBS: *Werkzeuge → WebSocket-Server-Einstellungen*, Server aktivieren,
   Port (Standard 4455) und Passwort notieren.
2. In DMXCraft: *Bearbeiten → Einstellungen → OBS*, „Mit OBS verbinden“ einschalten, Adresse
   (`localhost`, wenn OBS auf demselben Rechner läuft), Port und Passwort
   eintragen. Die Daten werden mit der Show gespeichert.
3. Der Verbindungsstatus steht in den Einstellungen und als „OBS“ in der Titelleiste
   (grün = verbunden, rot = getrennt; Klick öffnet die Einstellungen). DMXCraft
   verbindet sich bei Abbruch alle paar Sekunden neu.

## Videoblöcke

* Videodatei (MP4, MOV, MKV, WebM …) in die Zeitleiste einer SUPER SCENE
  ziehen oder im Kontextmenü der Spuren *Video hinzufügen …* wählen. Die
  Länge wird aus MP4/MOV gelesen.
* Erreicht die Wiedergabe den Block, setzt DMXCraft die Datei in die
  Medienquelle (Standard „DMXCraft Video“, je Block änderbar) und startet sie
  an der passenden Stelle. Fehlt die Quelle in OBS, wird sie in der
  OBS-Szene des Blocks (sonst in der aktuellen Szene) angelegt.
* Optional wechselt OBS beim Start des Blocks in eine Szene
  (Block-Eigenschaften → OBS-Szene).
* Pause, Springen mit dem Lineal, Geschwindigkeit und Pegel (Lautstärke der
  Quelle) folgen der Zeitleiste; Abweichungen über ca. 0,3 s werden
  nachgestellt. Am Ende des Blocks wird die Quelle gestoppt.
* Kürzen von links/rechts verschiebt wie bei Audio den Beginn in der Datei.

## OBS-Aktionen

Kontextmenü der Spuren → *OBS-Aktion hinzufügen*. Beim Erreichen des Blocks
(nur bei laufender Wiedergabe):

* **Szene wechseln** – z. B. zum Countdown in die Video-Szene springen.
* **Quelle einblenden / ausblenden** – Quelle in einer Szene schalten.

## Tipps

* Bildausgabe auf einen Beamer: in OBS Rechtsklick auf die Vorschau →
  *Vollbild-Projektor*.
* Der Ton des Videos läuft über den Audiomixer von OBS und landet damit auch
  in der Aufnahme. Für stumme Videos den Pegel des Blocks auf 0 ziehen.
