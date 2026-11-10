//! Update check and installation via GitHub releases.
//!
//! The check always asks the GitHub API directly for the latest release and
//! compares it with the own version – regardless of how DMXCraft was
//! installed. Installation depends on the kind of install:
//!
//! * Shell/PowerShell installer: axoupdater (the installers leave an
//!   “install receipt” that tells location and source).
//! * MSI (Windows): download the new MSI and start `msiexec`; it updates
//!   the existing installation.
//! * otherwise (e.g. developer build): open the release page in the browser.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use axoupdater::AxoUpdater;

const APP_NAME: &str = "dmxcraft";
const REPO: &str = "JustinSchweiger/DMXCraft";
#[cfg(windows)]
const MSI_ASSET: &str = "dmxcraft-x86_64-pc-windows-msvc.msi";

/// How an available update is installed.
#[derive(Clone, Debug, PartialEq)]
pub enum InstallKind {
    /// axoupdater (Installation per Shell-/PowerShell-Installer)
    Updater,
    /// Download and start the MSI
    #[cfg_attr(not(windows), allow(dead_code))]
    Msi(String),
    /// Open the release page
    Browser,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Release {
    pub version: String,
    pub url: String,
    pub install: InstallKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UpdateState {
    Idle,
    Checking,
    UpToDate,
    Available(Release),
    Installing,
    /// New version installed – restart needed.
    Installed(String),
    /// MSI installer started – close DMXCraft.
    InstallerStarted(String),
    Error(String),
}

impl UpdateState {
    pub fn busy(&self) -> bool {
        matches!(self, UpdateState::Checking | UpdateState::Installing)
    }
}

#[derive(Clone)]
pub struct Updater {
    state: Arc<Mutex<UpdateState>>,
    /// Result of a manually started check (text, error?) for the status
    /// message.
    notice: Arc<Mutex<Option<(String, bool)>>>,
}

impl Default for Updater {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(UpdateState::Idle)),
            notice: Arc::new(Mutex::new(None)),
        }
    }
}

impl Updater {
    pub fn state(&self) -> UpdateState {
        self.state
            .lock()
            .map(|s| s.clone())
            .unwrap_or(UpdateState::Idle)
    }

    fn set(&self, s: UpdateState) {
        if let Ok(mut g) = self.state.lock() {
            *g = s;
        }
    }

    fn notify(&self, text: String, err: bool) {
        if let Ok(mut g) = self.notice.lock() {
            *g = Some((text, err));
        }
    }

    /// Collect the message of a manually started check.
    pub fn take_notice(&self) -> Option<(String, bool)> {
        self.notice.lock().ok()?.take()
    }

    pub fn current_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Checks in the background whether a newer version is on GitHub.
    /// `manual`: also report the result when there is nothing new.
    pub fn check(&self, ctx: eframe::egui::Context, manual: bool) {
        if self.state().busy() {
            return;
        }
        self.set(UpdateState::Checking);
        let me = self.clone();
        std::thread::spawn(move || {
            let state = match latest_release() {
                Ok((tag, url, assets)) => {
                    if is_newer(&tag, Self::current_version()) {
                        let version = tag.trim_start_matches('v').to_string();
                        if manual {
                            me.notify(
                                crate::tr!("New version v{v} available", v = version.clone()),
                                false,
                            );
                        }
                        UpdateState::Available(Release {
                            version,
                            url,
                            install: install_kind(&assets),
                        })
                    } else {
                        if manual {
                            me.notify(
                                crate::tr!(
                                    "DMXCraft is up to date (v{v})",
                                    v = Self::current_version()
                                ),
                                false,
                            );
                        }
                        UpdateState::UpToDate
                    }
                }
                Err(e) => {
                    if manual {
                        me.notify(crate::tr!("Update check failed: {e}", e = e.clone()), true);
                    }
                    UpdateState::Error(e)
                }
            };
            me.set(state);
            ctx.request_repaint();
        });
    }

    /// Installs the update found (or opens the release page).
    pub fn install(&self, ctx: eframe::egui::Context) {
        let UpdateState::Available(rel) = self.state() else {
            return;
        };
        if rel.install == InstallKind::Browser {
            ctx.open_url(eframe::egui::OpenUrl::new_tab(&rel.url));
            return;
        }
        self.set(UpdateState::Installing);
        let me = self.clone();
        std::thread::spawn(move || {
            let state = match &rel.install {
                InstallKind::Msi(url) => match run_msi(url) {
                    Ok(()) => UpdateState::InstallerStarted(rel.version.clone()),
                    Err(e) => UpdateState::Error(e),
                },
                _ => run_updater(),
            };
            if let UpdateState::Error(e) = &state {
                me.notify(crate::tr!("Update failed: {e}", e = e.clone()), true);
            }
            me.set(state);
            ctx.request_repaint();
        });
    }
}

fn client() -> Result<reqwest::blocking::Client, String> {
    reqwest::blocking::Client::builder()
        .user_agent(concat!("DMXCraft/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|e| e.to_string())
}

/// Latest release: (tag, page, names and download links of the files).
type ReleaseInfo = (String, String, Vec<(String, String)>);

fn latest_release() -> Result<ReleaseInfo, String> {
    let url = format!("https://api.github.com/repos/{REPO}/releases/latest");
    let resp = client()?
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GitHub: {}", resp.status()));
    }
    let v: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
    let tag = v["tag_name"]
        .as_str()
        .ok_or("GitHub: no tag_name")?
        .to_string();
    let page = v["html_url"]
        .as_str()
        .map(str::to_string)
        .unwrap_or_else(|| format!("https://github.com/{REPO}/releases/latest"));
    let assets = v["assets"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|x| {
                    Some((
                        x["name"].as_str()?.to_string(),
                        x["browser_download_url"].as_str()?.to_string(),
                    ))
                })
                .collect()
        })
        .unwrap_or_default();
    Ok((tag, page, assets))
}

/// Is `tag` (e.g. “v0.4.0”) above `current` (e.g. “0.3.0”)?
fn is_newer(tag: &str, current: &str) -> bool {
    fn parse(s: &str) -> Option<(u64, u64, u64, bool)> {
        let s = s.trim().trim_start_matches('v');
        let (core, pre) = match s.split_once('-') {
            Some((c, _)) => (c, true),
            None => (s, false),
        };
        let mut it = core.split('.').map(|p| p.parse::<u64>().ok());
        let a = it.next()??;
        let b = it.next().flatten().unwrap_or(0);
        let c = it.next().flatten().unwrap_or(0);
        Some((a, b, c, !pre))
    }
    match (parse(tag), parse(current)) {
        (Some(n), Some(c)) => n > c,
        _ => false,
    }
}

/// Determine how this executable was installed.
fn install_kind(_assets: &[(String, String)]) -> InstallKind {
    let mut up = AxoUpdater::new_for(APP_NAME);
    if up.load_receipt().is_ok() && up.check_receipt_is_for_this_executable().unwrap_or(false) {
        return InstallKind::Updater;
    }
    #[cfg(windows)]
    if installed_by_msi() {
        if let Some((_, url)) = _assets.iter().find(|(n, _)| n == MSI_ASSET) {
            return InstallKind::Msi(url.clone());
        }
    }
    InstallKind::Browser
}

/// Is the executable under “Program Files” (where the MSI installs)?
#[cfg(windows)]
fn installed_by_msi() -> bool {
    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    ["ProgramFiles", "ProgramW6432"]
        .iter()
        .filter_map(std::env::var_os)
        .any(|dir| exe.starts_with(std::path::Path::new(&dir)))
}

#[cfg(windows)]
fn run_msi(url: &str) -> Result<(), String> {
    let bytes = client()?
        .get(url)
        .timeout(Duration::from_secs(600))
        .send()
        .and_then(|r| r.error_for_status())
        .and_then(|r| r.bytes())
        .map_err(|e| e.to_string())?;
    let path = std::env::temp_dir().join(MSI_ASSET);
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())?;
    std::process::Command::new("msiexec")
        .arg("/i")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(not(windows))]
fn run_msi(_url: &str) -> Result<(), String> {
    Err("MSI only on Windows".into())
}

fn run_updater() -> UpdateState {
    let mut up = AxoUpdater::new_for(APP_NAME);
    match up.load_receipt() {
        Err(e) => UpdateState::Error(e.to_string()),
        Ok(up) => {
            up.disable_installer_output();
            match up.run_sync() {
                Ok(Some(r)) => UpdateState::Installed(r.new_version.to_string()),
                Ok(None) => UpdateState::UpToDate,
                Err(e) => UpdateState::Error(e.to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_compare() {
        assert!(is_newer("v0.4.0", "0.3.0"));
        assert!(is_newer("v0.10.0", "0.9.5"));
        assert!(is_newer("v1.0.0", "1.0.0-beta.1"));
        assert!(!is_newer("v0.4.0", "0.4.0"));
        assert!(!is_newer("v0.3.9", "0.4.0"));
        assert!(!is_newer("v1.0.0-beta.1", "1.0.0"));
        assert!(!is_newer("kaputt", "0.4.0"));
    }

    /// Real query to GitHub (network needed): `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn github_latest() {
        let (tag, url, assets) = latest_release().unwrap();
        eprintln!("{tag} {url} {}", assets.len());
        assert!(tag.starts_with('v'));
        assert!(!assets.is_empty());
    }
}
