# DMXCraft – UI-Richtlinien

Verbindliche Regeln für jede Änderung an der Oberfläche (`crates/app`).
Ziel: ein ruhiges, dichtes Pult-Layout, das im Dunkeln bei einer Show
bedienbar ist – mit eigenem Erscheinungsbild (keine Grafiken, Icons oder
Farbschemata anderer Programme übernehmen).

## 1. Grundprinzipien

1. **Live zuerst.** Alles, was während einer Show gebraucht wird, liegt unter
   STEUERN und ist mit einem Klick erreichbar. Einrichten (Patch, Bühnenplan
   bearbeiten, Zonen) gehört unter EINRICHTEN.
2. **Flächen füllen.** Panels, Fader und Canvas nutzen den verfügbaren Platz
   vollständig (`ui.available_height()`/`available_width()`); keine festen
   Höhen, außer für Kopfzeilen und Buttons.
3. **Ein Bedienelement, eine Bedeutung.** Gleiche Aktionen sehen überall gleich
   aus (z. B. „freigeben“ immer als Rechtsklick + Kontextmenü-Eintrag).
4. **Zustand sichtbar machen.** Aktive Szenen, gesetzte Programmer-Kanäle,
   Auswahl und Ausgangsstatus sind ohne Hover erkennbar.
5. **Keine Überraschungen bei Shows.** Destruktive Aktionen nie per einfachem
   Linksklick ohne Rückfrage, nie in der Nähe häufig genutzter Buttons.

## 2. Design-Tokens (`ui/theme.rs`)

Farben, Abstände, Radien und Schriftgrößen kommen **ausschließlich** aus
`ui::theme`. Keine `Color32::from_rgb(..)`-Literale in Views.

Flächen- und Textfarben hängen vom gewählten **Theme** ab (Dunkelblau, Grau –
Einstellungen → Allgemein) und werden als Funktion gelesen: `theme::bg()`,
`theme::surface()`, `theme::text()` … Neue Themes sind ein weiterer
`Palette`-Eintrag in `theme.rs`. Signalfarben (`DANGER`, `OK`, `WARN`) und der
Akzent bleiben in allen Themes gleich erkennbar.

**Begriffe:** Die Spalten unter STEUERN heißen in der Oberfläche **Bänke**
(im Code `Zone`). Inline-Umbenennen (Chips, Patch) fokussiert das Feld nur beim
Start – nie jeden Frame `request_focus()`.

**Sprache:** Jeder sichtbare Text läuft über `t("…")` oder
`tr!("… {name}", name = x)` (deutsche Quelle, Übersetzung in
`i18n/en.rs`). Keine Texte per `format!` zusammensetzen, die übersetzt werden
müssen – Platzhalter benennen, damit die Übersetzung die Reihenfolge ändern
kann. Showdaten (Gerätenamen, Szenen, Zonen) werden nicht übersetzt.

| Token | Verwendung |
|---|---|
| `bg()` | Fensterhintergrund, Canvas |
| `surface()` | Panels |
| `raised()` | Karten, Fader-Spuren, Buttons |
| `raised_hover()` | Hover-Zustand |
| `border()` | 1-px-Kanten von Karten/Panels |
| `accent()` | Auswahl, aktive Werte, Fokus (Cyan aus dem Logo) |
| `TEXT` / `TEXT_DIM` / `TEXT_FAINT` | Primär-, Sekundär-, deaktivierter Text |
| `DANGER`, `OK`, `WARN` | Blackout/Fehler, Ausgang aktiv, Warnung |

Abstände im 4-px-Raster: `SP_XS=4`, `SP_S=8`, `SP_M=12`, `SP_L=16`.
Radien: `R_S=4` (kleine Buttons), `R_M=8` (Karten, Fader).
Schrift: `FS_CAPTION=10.5` (Sektionstitel, GROSS), `FS_BODY=13`,
`FS_TITLE=15`.

## 3. Layout

* **Eigene Titelleiste** (`titlebar.rs`, Fenster ohne Betriebssystem-Leiste):
  links Menü (Datei, Bearbeiten, Ansicht, Tools, Mappings, Hilfe) · Modus
  Einrichten/Steuern;
  Mitte „DMXCraft · Showname“ (= Dateiname, `*` bei ungespeicherten
  Änderungen); rechts Auslastungsleiste „LAST“ (Statuspunkt, Details im
  Tooltip), Sperre-Knopf, Fensterknöpfe. Keine fps- oder Geräte-/Szenenzahlen.
* **Keine Statusleiste.** Kurze Meldungen (`AppState::info/error`) erscheinen
  rechts in der Titelleiste und blenden nach 5 s (Fehler 10 s) aus; Update-
  Hinweise ebenfalls dort.
* **Randlos:** Bereiche stoßen aneinander und sind nur durch 1-px-Trennlinien
  (Panel-Grenzen) getrennt. Innenabstand eines Bereichs `SP_XS`–`SP_S`,
  keine äußeren Rahmen oder Rundungen um ganze Bereiche (Canvas, Raster).
* STEUERN: oben Szenen-Raster (flexibel), unten Bühnenplan + Programmer
  (Höhe per Splitter, Standard 40 %).
* **Keine Bereichsüberschriften** („Bühne“, „Programmer“, „Patch“ …) – sie
  kosten Platz und sagen nichts Neues. Eine Werkzeugzeile über einem Bereich
  nutzt `widgets::section_header(ui, "", …)` (Aktionen rechts). Wo eine
  Beschriftung nötig ist (z. B. „GRUPPEN |“ vor Pillen), immer über
  `theme::heading_text()`.
* **Sperre:** Ist die Oberfläche gesperrt (`AppState::locked`), gibt es nur
  STEUERN; Einrichten, Szenen-/Zonenbearbeitung, Kontextmenüs zum Ändern,
  Einstellungen und Show öffnen sind aus. Neue Bedienelemente, die etwas am
  Patch oder an Szenen ändern, müssen `locked` beachten.
* Inhalte haben `SP_S` Innenabstand; zwischen Karten `SP_XS`.

## 4. Komponenten (`ui/widgets/`)

Wiederverwendbare Elemente sind eigene Typen mit `impl egui::Widget` oder
einer `show(self, ui) -> Response`-Methode. Sie kennen **keine** Engine und
keinen App-Zustand – nur ihre Eingabewerte.

* **Fader** (Programmer): Statuspunkt oben (Farbe des Modus, anklickbar =
  Kanal speichern/freigeben), Kanalnummer, Symbol der Kanalart (Name nur als
  Hinweis am Symbol), Wert bzw. „OFF“ (Klick = eintippen, Ziehen hoch/runter =
  ändern), Schiene mit Griff. Mausrad ±2, Doppelklick 0 ↔ voll, Rechtsklick
  Menü (Freigeben, 0/25/50/75 %/Voll). In allen Kategorien außer „Alle
  Kanäle“ steuert ein Fader die Kanalart **aller** ausgewählten Geräte.
* **SceneCard**: Name in Szenenfarbe, Typ in Großbuchstaben (STATISCH, ABLAUF,
  FX), rechts der Markier-Streifen (Klick = auswählen, Ziehen = in andere Bank
  verschieben), Hauptfläche = starten/stoppen. Leiste unten: Laufzeit bzw.
  Rampe beim Ein-/Ausblenden; kleine Symbole für Ablauf und Flash.
* **Knob** (Drehregler): Bezeichnung darunter, Wert **nur beim Drehen** als
  Hinweis am Mauszeiger; Akzent in der Farbe des Objekts (z. B. Szene).
* **Zeitfeld** (`widgets::time_field`): `00h00m00s00` (Stunden nur, wo sinnvoll,
  z. B. Dauer), jede Zahl per Ziehen hoch/runter, Hundertstel in 4er-Schritten.
* **Dialog** (`widgets::Dialog`): **jedes** Fenster läuft hierüber, nie
  `egui::Window` direkt. Kopf: Titel zentriert in Überschrift-Schrift,
  Schließen-Symbol rechts. Inhalt auf `SURFACE`. Aktionsknöpfe (Abbrechen,
  Schließen, Speichern …) **immer in der Fußzeile unten rechts**, Hauptaktion
  ganz rechts in `ACCENT` (auch per Enter), Esc schließt.
  Erstes Feld nur über das `first`-Flag fokussieren – nie jeden Frame
  `request_focus()`, sonst funktioniert Tab nicht.
* **Fenster mit Reitern** (Vorbild: `settings_window.rs`): `Dialog` mit
  `.flush()` und fester `.height()`, Reiterleiste links (`BG`, aktiver Reiter
  `SURFACE` mit Akzentstreifen, Symbol + Text), Inhalt rechts scrollbar. Das
  Fenster behält beim Reiterwechsel seine Größe.
* **Formulare** (`widgets::form` + `Form::row`): Beschriftung links
  (`TEXT_DIM`, feste Spaltenbreite, ohne Doppelpunkt), Bedienelement rechts
  in voller Breite, alle Zeilen gleich hoch (`INPUT_H`).
* **Eingabefelder** immer über `widgets::text_field()` (Text vertikal mittig,
  feste Höhe, Hintergrund `BG`), nie `TextEdit::singleline` direkt.
* **Menüs** (`widgets::menu`): Hauptmenü und **alle** Kontextmenüs nur mit
  `MenuItem` (Symbolspalte, Text, Kürzel rechts gedämpft, Pfeil bei
  Untermenüs), `menu::item`, `menu::sub`, `menu::separator`, `menu::title`;
  jedes Menü beginnt mit `menu::begin`. Nie `ui.button`/`ui.menu_button` in
  Menüs. Symbole, wo sie helfen; die oberste Ebene des Hauptmenüs ohne
  Symbolspalte (`plain()`). Löschen direkt und rot (`danger()`), ohne
  Rückfrage – Strg+Z macht es rückgängig; Geräte zusätzlich per Entf. Noch nicht Verfügbares deaktiviert mit
  Tooltip „Noch nicht verfügbar“ – Klick darauf tut nichts, das Menü bleibt
  offen (`menu::context`/`menu::dropdown`, nie `resp.context_menu`).
  Kürzel-Texte aus `titlebar::keys`. „…“ am Ende heißt: es öffnet sich ein
  Dialog; Dateien **immer** über den Systemdialog (`rfd`, eigener Thread),
  nie über ein eigenes Pfad-Feld.
* **Toggle** (Schiebeschalter): für alle An/Aus-Optionen – **keine Checkboxen**.
* **RangeSlider**: senkrechter Regler mit Unter- und Obergrenze (z. B. Dimmer-Limit).
* **LimitPad**: XY-Feld mit Griffen für Bereiche (Pan/Tilt-Grenzen).
* **SegmentedTabs**: für Modus- und Kategorie-Umschalter.
* **XyPad**, **Swatch** (Farbfeld), **StageCanvas** (Bühnenplan in den
  Modi Auswahl/Bearbeiten).

## 5. Interaktion

* Jeder Befehl ist über das Hauptmenü erreichbar; häufige zusätzlich direkt
  in der Ansicht oder per Tastenkürzel (Strg+S, Strg+O, F11, B).
* Rechtsklick auf Geräte öffnet überall dasselbe Menü (`views/fixture_menu.rs`).

* Hover: `RAISED_HOVER`; gedrückt/aktiv: `ACCENT` gedämpft; Auswahl:
  2-px-Rand in `ACCENT`.
* Mehrfachauswahl: Strg/Cmd oder Shift + Klick, Aufziehen eines Rahmens.
* Ziehen beginnt erst nach der egui-Schwelle; Treffer werden am
  **Druckpunkt** (`press_origin`) ermittelt, nicht an der aktuellen Position.
* Kontextmenüs für seltene Aktionen (Bearbeiten, Löschen, Freigeben).
* **Keine Hinweis- oder Leer-Texte** („Gerät auswählen …“, „Keine Szene
  ausgewählt“, „Geräte auswählen und mit + …“, „keine Kanäle in dieser
  Kategorie“): ist nichts da, bleibt die Fläche leer. Hilfe gehört in Tooltips,
  bei Bedarf an einem Info-Symbol (`Icon::Info`).
* Elemente, die abhängig von der Auswahl erscheinen (z. B. DIP-Schalter), dürfen
  die Höhe ihrer Zeile nicht verändern.
* Alle Bereiche einer Seite sind an ihren Trennlinien in der Größe veränderbar.
  Widgets setzen den Mauszeiger nur, wenn sie ihn wirklich brauchen (z. B.
  Greifen beim Verschieben) – nie pauschal `Default`, sonst verschwindet der
  Größenänderungs-Cursor der Trennlinien.
* Werkzeuge und Auswahlen mit wenigen festen Optionen (Formen, Anordnen,
  Wiederholung) als Symbol-Knöpfe mit Tooltip statt Auswahllisten.
* **Entf** wirkt auf das zuletzt angeklickte Element (`AppState::focus`:
  Szenen, Bank, Effekt) – nie auf etwas anderes, das zufällig auch markiert ist.
* **Umbenennen inline:** Feld erscheint an Ort und Stelle, der ganze Name ist
  markiert (`dialog::select_all`, auch nach einem Doppelklick), Enter
  übernimmt, Esc bricht ab. Neue Objekte (Szene, Bank, Gruppe) starten direkt
  im Umbenennen.
* **Drag & Drop**, wo es naheliegt: Szenen am Streifen in andere Bänke,
  Geräte aus der Bibliothek ins Patch. Ziel zeigt Einfügemarke bzw. frei/belegt
  (grün/rot), am Mauszeiger eine Beschriftung ohne Umbruch.
* **Seitenleisten** (z. B. Szenen-Eigenschaften): Beschriftung links,
  Bedienelement **rechtsbündig**; Auswahllisten mit Ein-Wort-Optionen
  (Aus/Alle/Bank …). Lange Listen zum Auswählen (Szenen, Bänke) öffnen ein
  eigenes Fenster mit Spalten je Bank und Mehrfachauswahl.
* Bedienelemente einer Szene nutzen die **Szenenfarbe** als Akzent
  (Ausnahme: Play/Pause).
* Werte, die man ablesen muss (Zeiten, Laufzeit), groß genug und kontrastreich;
  aktuelle Zeit links in Objektfarbe, Gesamtzeit rechts in Textfarbe.
* Bühnenplan: Geräte ohne Licht sind schwarz (wie Dimmer zu), Strobe blinkt,
  jede Geräteform ist unterscheidbar.
* Größen von Fenster und Bereichen werden gespeichert (eframe-Persistenz).

## 6. Text & Symbole

* Sprache: Deutsch, kurz, Verben im Infinitiv („Speichern“, „Freigeben“).
* Nur Zeichen, die die eingebaute Schrift sicher darstellt: lateinische
  Buchstaben, Ziffern, gängige Satzzeichen, `·`, `–`, `„“`. **Keine Emojis
  oder Pfeil-/Symbolzeichen** (▶ ❚ ● … usw.) im Text.
* **Symbole** kommen einheitlich aus der Phosphor-Icon-Schrift (MIT,
  `assets/fonts/Phosphor*.ttf`) über `widgets::Icon` – neue Symbole dort als
  Variante mit passender Glyphe ergänzen. Symbole müssen zur Funktion passen
  (z. B. Zahnrad für Einstellungen, nicht Lupe). Selbst gezeichnet nur, wenn
  es keine passende Glyphe gibt.
* Fachbegriffe aus dem Lichtbereich (EDIT, LIVE, BLIND, FX, STEPS …) dürfen
  englisch bleiben.

## 7. Architektur der Oberfläche

* Jede Ansicht ist ein Typ, der das Trait `views::View` implementiert und
  ihren eigenen UI-Zustand als Felder hält.
* Gemeinsamer Zustand (Auswahl, Meldungen, Pfade) liegt in `state::AppState`;
  Ansichten bekommen ihn über `ViewCtx` (`engine`, `state`, `frame`).
* Views ändern die Show nur über `Engine`/`Show`-Methoden aus `dmxcraft-core`;
  Logik, die nicht nach UI aussieht, gehört in den Core und bekommt Tests.

## 8. Prüfen vor jedem Commit

1. `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`.
2. App unter Xvfb starten, betroffene Ansichten per Screenshot prüfen
   (Füllen die Fader die Höhe? Abgeschnittene Texte? Fehlende Glyphen?).
3. `graphify update .` ausführen, damit der Code-Graph aktuell bleibt.
