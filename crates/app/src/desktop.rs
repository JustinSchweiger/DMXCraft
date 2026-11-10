//! Create or update the start menu entry on Linux.
//!
//! The installers only place the executable. So that DMXCraft appears in
//! the application menu, the program writes a `.desktop` file to
//! `~/.local/share/applications` at startup – but only when it runs from an
//! installed version (not from `target/`).

#[cfg(target_os = "linux")]
pub fn ensure_linux_menu_entry() {
    use std::path::PathBuf;

    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    if exe.components().any(|c| c.as_os_str() == "target") {
        return; // Developer build
    }
    let Some(home) = std::env::var_os("HOME").map(PathBuf::from) else {
        return;
    };
    let data = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local/share"));
    // Place the icon (hicolor theme)
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
         Comment=DMX lighting control\n\
         Exec=\"{}\" %f\n\
         Icon=dmxcraft\n\
         Terminal=false\n\
         Categories=AudioVideo;Video;\n\
         Keywords=DMX;Lighting;Licht;Daslight;\n\
         MimeType=application/x-dmxcraft;\n",
        exe.display()
    );
    if std::fs::read_to_string(&file).ok().as_deref() == Some(content.as_str()) {
        return;
    }
    let _ = std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(&file, content));
    register_mime(&data);
    // Update the databases if the tools are available
    let _ = std::process::Command::new("update-desktop-database")
        .arg(&dir)
        .status();
}

/// Register the `.dmxcraft` file type (double-click opens DMXCraft).
#[cfg(target_os = "linux")]
fn register_mime(data: &std::path::Path) {
    const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
  <mime-type type="application/x-dmxcraft">
    <comment>DMXCraft-Show</comment>
    <sub-class-of type="application/json"/>
    <icon name="dmxcraft"/>
    <glob pattern="*.dmxcraft"/>
  </mime-type>
</mime-info>
"#;
    let mime = data.join("mime");
    let file = mime.join("packages/dmxcraft.xml");
    if std::fs::read_to_string(&file).ok().as_deref() == Some(XML) {
        return;
    }
    if std::fs::create_dir_all(mime.join("packages"))
        .and_then(|_| std::fs::write(&file, XML))
        .is_ok()
    {
        let _ = std::process::Command::new("update-mime-database")
            .arg(&mime)
            .status();
    }
}

#[cfg(not(target_os = "linux"))]
pub fn ensure_linux_menu_entry() {}
