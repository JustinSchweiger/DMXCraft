//! On Windows, embeds the app icon and version info into the .exe.

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    #[cfg(windows)]
    {
        if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("assets/icon.ico");
            res.set("ProductName", "DMXCraft");
            res.set("FileDescription", "DMXCraft – DMX lighting control");
            if let Err(e) = res.compile() {
                println!("cargo:warning=Could not embed icon: {e}");
            }
        }
    }
}
