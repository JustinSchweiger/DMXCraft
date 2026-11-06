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

| Token | Verwendung |
|---|---|
| `BG` | Fensterhintergrund, Canvas |
| `SURFACE` | Panels |
| `RAISED` | Karten, Fader-Spuren, Buttons |
| `RAISED_HOVER` | Hover-Zustand |
| `BORDER` | 1-px-Kanten von Karten/Panels |
| `ACCENT` | Auswahl, aktive Werte, Fokus (Cyan aus dem Logo) |
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
  Mitte „DMXCraft · Showname“; rechts Auslastungsleiste „LAST“ (Statuspunkt, Details im Tooltip),
  Sperre-Knopf, Fensterknöpfe. Keine fps- oder Geräte-/Szenenzahlen.
  Keine Bedienelemente der Wiedergabe (Master, Speed, Blackout) in der
  Titelleiste – die liegen unter STEUERN über dem Szenen-Raster.
* **Keine Statusleiste.** Kurze Meldungen (`AppState::info/error`) erscheinen
  rechts in der Titelleiste und blenden nach 5 s (Fehler 10 s) aus; Update-
  Hinweise ebenfalls dort.
* **Randlos:** Bereiche stoßen aneinander und sind nur durch 1-px-Trennlinien
  (Panel-Grenzen) getrennt. Innenabstand eines Bereichs `SP_XS`–`SP_S`,
  keine äußeren Rahmen oder Rundungen um ganze Bereiche (Canvas, Raster).
* STEUERN: oben Szenen-Raster (flexibel), unten Bühnenplan + Programmer
  (Höhe per Splitter, Standard 40 %).
* Jeder Bereich beginnt mit einem **Sektionskopf** (`widgets::section_header`):
  Titel links, Aktionen rechts. Bereichsüberschriften **immer** über
  `theme::heading_text()` (Noto Sans SemiBold, `FS_HEADING`, GROSSBUCHSTABEN) –
  nie eigene `RichText` für Überschriften, damit sie überall gleich aussehen.
* **Sperre:** Ist die Oberfläche gesperrt (`AppState::locked`), gibt es nur
  STEUERN; Einrichten, Szenen-/Zonenbearbeitung, Kontextmenüs zum Ändern,
  Einstellungen und Show öffnen sind aus. Neue Bedienelemente, die etwas am
  Patch oder an Szenen ändern, müssen `locked` beachten.
* Inhalte haben `SP_S` Innenabstand; zwischen Karten `SP_XS`.

## 4. Komponenten (`ui/widgets/`)

Wiederverwendbare Elemente sind eigene Typen mit `impl egui::Widget` oder
einer `show(self, ui) -> Response`-Methode. Sie kennen **keine** Engine und
keinen App-Zustand – nur ihre Eingabewerte.

* **Fader**: vertikale Spur, füllt die übergebene Höhe; Pegel als gefüllte
  Fläche von unten; Name oben, Wert unten in der Spur. Ziehen = setzen,
  Mausrad = ±1 (Shift: ±10), Doppelklick = 0 ↔ voll, Rechtsklick =
  Kontextmenü (Freigeben). Ungesetzt = gedämpft, im Programmer = `ACCENT`.
* **SceneCard**: Name, Typzeile (STATISCH/ABLAUF), Fortschritt als
  Leiste unten, Zonenfarbe als Rand, wenn aktiv.
* **Dialog** (`widgets::Dialog`): **jedes** Fenster läuft hierüber, nie
  `egui::Window` direkt. Kopf: Titel zentriert in Überschrift-Schrift,
  Schließen-Symbol rechts. Inhalt auf `SURFACE`. Aktionsknöpfe (Abbrechen,
  Schließen, Speichern …) **immer in der Fußzeile unten rechts**, Hauptaktion
  ganz rechts in `ACCENT` (auch per Enter), Esc schließt.
  Erstes Feld nur über das `first`-Flag fokussieren – nie jeden Frame
  `request_focus()`, sonst funktioniert Tab nicht.
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
  Symbolspalte (`plain()`). Zerstörendes (Löschen) immer als Untermenü mit
  roter Bestätigung (`danger()`). Noch nicht Verfügbares deaktiviert mit
  Tooltip „Noch nicht verfügbar“ – Klick darauf tut nichts, das Menü bleibt
  offen (`menu::context`/`menu::dropdown`, nie `resp.context_menu`).
  Kürzel-Texte aus `titlebar::keys`. „…“ am Ende heißt: es öffnet sich ein
  Dialog; Dateien immer über den Systemdialog (`file_dialog`), nie über ein
  eigenes Pfad-Feld.
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
* Keine erklärenden Hinweistexte dauerhaft in Kopfzeilen; Hilfe gehört in
  Tooltips (z. B. ein „Hilfe“-Label mit `on_hover_text`).
* Elemente, die abhängig von der Auswahl erscheinen (z. B. DIP-Schalter), dürfen
  die Höhe ihrer Zeile nicht verändern.
* Alle Bereiche einer Seite sind an ihren Trennlinien in der Größe veränderbar.
  Widgets setzen den Mauszeiger nur, wenn sie ihn wirklich brauchen (z. B.
  Greifen beim Verschieben) – nie pauschal `Default`, sonst verschwindet der
  Größenänderungs-Cursor der Trennlinien.
* Werkzeuge und Auswahlen mit wenigen festen Optionen (Formen, Anordnen)
  als Symbol-Knöpfe mit Tooltip statt Auswahllisten; Symbole immer selbst
  gezeichnet (`widgets::icons`, `stage_canvas::paint_shape_icon`).

## 6. Text & Symbole

* Sprache: Deutsch, kurz, Verben im Infinitiv („Speichern“, „Freigeben“).
* Nur Zeichen, die die eingebaute Schrift sicher darstellt: lateinische
  Buchstaben, Ziffern, gängige Satzzeichen, `·`, `–`, `„“`. **Keine Emojis
  oder Pfeil-/Symbolzeichen** (▶ ❚ ● … usw.) – Symbole werden mit dem
  Painter gezeichnet (`widgets::icons`).

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
