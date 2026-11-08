# Tastatur- und MIDI-Zuordnung

## Zuordnung einschalten

Die Zuordnung lässt sich auf zwei Wegen einschalten:

* über die zwei Symbole links vom Schloss in der Titelleiste,
* über das Menü **Mappings**.

Es gibt eine Zuordnung für die **Tastatur** (rot) und eine für **MIDI** (violett).

## Zuordenbare Elemente

Alle zuordenbaren Elemente werden farbig markiert:

* **Szenen:** Szenenkarten, Bank-Chips („Alle“ und jede Bank) sowie die Knöpfe Pause, Zurück und Vor im Bankkopf.
* **Szenen-Seitenleiste:**
  * die Reiter,
  * Abspielen/Pause,
  * die Regler Dimmer, Speed, Phase und Size,
  * Anfang/Ende, vorwärts/rückwärts, die Wiederholungsart,
  * die Schalter „Am Ende freigeben“ und „Flash-Modus“.
* **Programmer:**
  * die Knöpfe Edit, Blind und Live,
  * Beams aus, an und zentrieren,
  * die zwei Zurücksetzen-Knöpfe,
  * die Kategorien,
  * jeder Fader,
  * die Farbvorgaben.
* **Bühne:** jedes Gerät im Plan (schaltet die Auswahl um), die Gruppen und der FX-Knopf.
* **Globale Befehle** über „+“ im Fenster: Blackout, Master, Alle Szenen stoppen, nächste/vorige Bank, ausgewählte Szene.

Ein Klick auf ein markiertes Element löst nichts aus. Er wählt das Element aus bzw. legt eine neue Zuordnung an und wartet auf die Eingabe.

Zwei Arten von Elementen verhalten sich verschieden:

* **Szenen, Bänke und globale Befehle** wirken immer.
* **Elemente der Oberfläche** (Programmer, Seitenleiste, Bühne) wirken wie ein Klick bzw. eine Reglerbewegung. Das geht nur, solange das Element angezeigt wird. Der Dimmer-Fader des Programmers wirkt also auf die gerade ausgewählten Geräte.

Regler (Fader, Drehregler) folgen einem MIDI-Regler (CC) direkt. Eine Taste schaltet einen Regler zwischen 0 und voll um.

## Fenster

**Voreinstellungen:**

* Löschen,
* Import und Export als `.dmxmap`-Datei.

**Liste:**

* Spalten Kategorie, Ziel und Taste bzw. MIDI-Eingang.
* Ein Klick auf den Kopf „Kategorie“ dreht die Sortierung um.
* Rechtsklick oder Entf löscht die Zuordnung.

**Einstellungen** der gewählten Zuordnung:

* Eingabe mit dem Knopf „Neu zuweisen“.
* Bei Szenen das Ziel: die Szene, die ausgewählte Szene, Nr. n in dieser Bank oder Nr. n in der angezeigten Bank.
* Tastenmodus und Flash-Taste.
* Bei MIDI: Gerät, Kanal, Note oder CC und die Nummer.

**MIDI-Ausgang** (LEDs):

* Gerät,
* für die Zustände Aus und An jeweils Kanal und Velocity.
* Bei bekannten Controllern zeigt die Kanalliste die Bedeutung, und die Velocity lässt sich als Farbe aus der Palette wählen.

## MIDI-Controller einrichten (Einstellungen → MIDI)

Alle angeschlossenen MIDI-Eingänge werden automatisch geöffnet. Die Suche läuft alle 3 s. Unter „+“ lässt sich ein Gerät einrichten. Typ und LED-Ausgang werden dabei am Namen erkannt:

* **Typ:** allgemeiner Controller oder **Akai APC mini mk2**.
  * Für den APC kennt DMXCraft die Farbtabelle aus Akais veröffentlichtem Kommunikationsprotokoll: 128 Farben, die Velocity wählt die Farbe.
  * Der MIDI-Kanal wählt das Leuchtverhalten: 1–7 Helligkeit 10–100 %, 8–11 Pulsieren, 12–16 Blinken.
* **LED-Ausgang:** Neu gelernte Tasten bekommen die Rückmeldung auf diesen Ausgang. Die „An“-Farbe ist die Szenenfarbe, sofern sie bekannt ist.
* **LEDs löschen beim Verbinden:** Beim Programmstart und beim Einstecken werden alle LEDs ausgeschaltet. Danach gehen die aktuellen Zustände raus. „Jetzt löschen“ macht dasselbe sofort.

Zuordnungen werden in der Show gespeichert (`mappings`), Controller in den Programmeinstellungen. Zuordnungen wirken auch bei gesperrter Oberfläche. Tasten werden ignoriert, solange ein Eingabefeld den Fokus hat.

## Neue Elemente zuordenbar machen

Ein neues Element meldet sich mit `mapping::button(ui, &resp, "schlüssel", an, || (kategorie, ziel))` an. Ein Regler nutzt `mapping::fader(…)`. Für Drehregler gibt es `Knob::map(…)`, für Schalter `Toggle::map(…)`.

Der Schlüssel muss fest bleiben, denn er wird in der Show gespeichert.
