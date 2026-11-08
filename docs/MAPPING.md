# Tastatur- und MIDI-Zuordnung

Die zwei Symbole links vom Schloss in der Titelleiste schalten die
Zuordnung ein: **Tastatur** (rot) oder **MIDI** (violett). Dann passiert Folgendes:

* Alle zuordenbaren Elemente werden farbig markiert: Szenenkarten,
  Bank-Chips („Alle“ und jede Bank) und die Pause-, Zurück- und Vor-Knöpfe
  im Bankkopf. Ein Klick darauf löst nichts aus. Er wählt das Element aus
  bzw. legt eine neue Zuordnung an und wartet auf die Eingabe.
* Das Fenster zeigt diese Bereiche:
  * **Voreinstellungen**: „+“ für globale Befehle (Blackout, Master, alle
    Szenen stoppen, nächste/vorige Bank, ausgewählte Szene); Löschen,
    Import und Export als `.dmxmap`-Datei.
  * **Liste** mit Kategorie, Ziel und Taste bzw. MIDI-Eingang. Ein Klick
    auf den Kopf „Kategorie“ dreht die Sortierung um. Rechtsklick oder
    Entf löscht die Zuordnung.
  * **Einstellungen** der gewählten Zuordnung:
    * Eingabe mit dem Knopf „Neu zuweisen“.
    * Bei Szenen das **Ziel**: die Szene selbst, die ausgewählte Szene,
      Nr. n in dieser Bank oder Nr. n in der angezeigten Bank.
    * **Tastenmodus**: An/Aus oder Wert in %, z. B. ein Fader auf den Master.
    * **Flash-Taste**: aktiv nur, solange die Taste gedrückt ist.
  * Nur MIDI: Gerät, Kanal, Note oder CC und die Nummer.
  * Nur MIDI, **MIDI-Ausgang** für LEDs am Controller: Gerät, und für die
    Zustände Aus und An jeweils Kanal und Velocity. Beim APC mini mk2 zeigt
    die Kanalliste die Bedeutung, etwa Helligkeit, Pulsieren oder Blinken.

Zuordnungen werden in der Show gespeichert (`mappings`). Sie wirken auch bei
gesperrter Oberfläche. Tasten werden ignoriert, solange ein Eingabefeld den
Fokus hat. MIDI-Eingänge werden automatisch geöffnet, sobald ein Gerät
angeschlossen ist. Die Suche läuft alle 3 s.
