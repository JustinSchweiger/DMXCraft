# OpenLicht

Eine freie, in Rust geschriebene DMX-Lichtsteuerung nach dem Vorbild von
Daslight 5. Sie läuft auf Linux, Windows und macOS und soll später die
vorhandene **Daslight DVC Gold** per USB ansteuern.

> Unabhängiges Projekt, nicht mit Nicolaudie/Daslight verbunden. Es wird
> kein Code, keine Gerätebibliothek und kein Grafik-Asset von Daslight
> verwendet.

## Stand (v0.1)

| Bereich | Status |
|---|---|
| Patch (Geräte, Universen, Adressen, Kollisionsprüfung) | ✅ |
| Eingebaute generische Gerätetypen (Dimmer, RGB, RGBW, Moving Head) | ✅ |
| Szenen mit Schritten, Überblend- und Haltezeit, Schleife/Einmal | ✅ |
| Zonen (Szenen einer Zone schließen sich aus) | ✅ |
| Merge: HTP für Dimmer, LTP für alles andere | ✅ |
| Programmer (Live-Fader, Farbwähler) → als Szene/Schritt speichern | ✅ |
| Master, Blackout, Speed | ✅ |
| DMX-Monitor (512 Kanäle pro Universum) | ✅ |
| Show-Datei (JSON) laden/speichern | ✅ |
| Ausgabe-Thread mit 40 Hz, automatischer Reconnect | ✅ |
| **USB-Ausgang DVC Gold** | ⏳ Platzhalter – Protokoll wird noch entschlüsselt |
| Effekt-Generator, 3D-Ansicht, MIDI, Import aus Open Fixture Library | ❌ geplant |

## Bauen und starten

Voraussetzung: Rust ≥ 1.85 (`rustup`). Unter Linux werden zur Laufzeit die
üblichen Grafikbibliotheken gebraucht (OpenGL/Mesa sowie X11 oder Wayland).
Die sind auf Desktop-Systemen normalerweise schon installiert.

```sh
cargo run --release -p openlicht-app            # startet mit Demo-Show
cargo run --release -p openlicht-app -- show.json  # lädt eine Show
cargo test --workspace                          # Tests
```

Ohne Gerät wählst du oben „Simulation (kein Gerät)“. Die Engine läuft dann
trotzdem, und im Tab **DMX-Monitor** siehst du, was an die Box gehen würde.

## Bedienung in Kürze

1. **Patch** (links): Gerätetyp wählen, Anzahl, Universum, Startadresse →
   *Hinzufügen*. Mit den Kästchen wählst du Geräte für den Programmer aus.
2. **Programmer**: Fader bewegen. Der Inhalt lässt sich als *neue Szene*
   speichern oder als *Schritt* an eine bestehende Szene anhängen.
3. **Szenen**: Name, Zone, Ablauf sowie die Zeiten jedes Schritts
   bearbeiten. Schritte kannst du in den Programmer laden und überschreiben.
4. **Live**: Szenen-Buttons nach Zone. Ein Klick schaltet die Szene ein
   oder aus.

## Aufbau

```
crates/
  core/    Datenmodell + Engine, ohne I/O, vollständig getestet
           show.rs    Patch, Zonen, Szenen, JSON
           engine.rs  Wiedergabe, Fades, HTP/LTP, Master, Programmer
  output/  DmxOutput-Trait, Ausgabe-Thread (runtime.rs), Backends
           dvc_gold.rs  ← hier kommt die USB-Anbindung hinein
  app/     Oberfläche mit egui/eframe
```

Die USB-Anbindung ist genau ein Backend: Man implementiert `DmxOutput`
(`open` = verbinden + Handshake, `send` = Frame senden) in
`crates/output/src/dvc_gold.rs`. Engine und Oberfläche bleiben dabei
unverändert. Die Schritte dafür stehen als Kommentar in der Datei.

## Show-Format

JSON, Version 1. Die verwendeten Gerätetypen werden in jede Show kopiert,
damit die Datei auf jedem Rechner identisch lädt. Universen sind in der Datei
0-basiert, DMX-Adressen 1-basiert.
