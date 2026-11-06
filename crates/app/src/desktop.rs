//! Startmenü-Eintrag unter Linux anlegen bzw. aktualisieren.
//!
//! Die Installer legen nur die Programmdatei ab. Damit DMXCraft im
//! Anwendungsmenü erscheint, schreibt das Programm beim Start eine
//! `.desktop`-Datei nach `~/.local/share/applications` – aber nur, wenn es
//! aus einer installierten Version läuft (nicht aus `target/`).

#[cfg(target_os = "linux")]
pub fn ensure_linux_menu_entry() {
    use std::path::PathBuf;

    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    if exe.components().any(|c| c.as_os_str() == "target") {
        return; // Entwickler-Build
    }
    let Some(home) = std::env::var_os("HOME").map(PathBuf::from) else {
        return;
    };
    let data = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local/share"));
    // Icon ablegen (hicolor-Theme)
    let icon_dir = data.join("icons/hicolor/256x256/apps");
    let icon_file = icon_dir.join("dmxcraft.png");
    if std::fs::read(&icon_file).ok().as_deref() != Some(crate::icon::ICON_PNG) {
        let _ = std::fs::create_dir_all(&icon_dir)
            .and_then(|_| std::fs::write(&icon_file, crate::icon::ICON_PNG));
    }
    let dir = data.join("applications");
    let file = dir.join("dmxcraft.desktop");
    let content = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=DMXCraft\n\
         Comment=DMX-Lichtsteuerung\n\
         Exec=\"{}\" %f\n\
         Icon=dmxcraft\n\
         Terminal=false\n\
         Categories=AudioVideo;Video;\n\
         Keywords=DMX;Licht;Lighting;Daslight;\n",
        exe.display()
    );
    if std::fs::read_to_string(&file).ok().as_deref() == Some(content.as_str()) {
        return;
    }
    let _ = std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(&file, content));
}

#[cfg(not(target_os = "linux"))]
pub fn ensure_linux_menu_entry() {}
