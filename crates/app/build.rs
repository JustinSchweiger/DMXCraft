//! Bettet unter Windows das App-Icon und die Versionsinfo in die .exe ein.

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    #[cfg(windows)]
    {
        if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("assets/icon.ico");
            res.set("ProductName", "DmxCraft");
            res.set("FileDescription", "DmxCraft – DMX-Lichtsteuerung");
            if let Err(e) = res.compile() {
                println!("cargo:warning=Icon konnte nicht eingebettet werden: {e}");
            }
        }
    }
}
