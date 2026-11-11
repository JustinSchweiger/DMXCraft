## Install DMXCraft {VERSION}

| System | Download | |
|---|---|---|
| 🪟 **Windows** | [dmxcraft-x86_64-pc-windows-msvc.msi](https://github.com/JustinSchweiger/DMXCraft/releases/download/{TAG}/dmxcraft-x86_64-pc-windows-msvc.msi) | Run the installer |
| 🍎 **macOS** (Apple Silicon + Intel) | [DMXCraft-macos-universal.dmg](https://github.com/JustinSchweiger/DMXCraft/releases/download/{TAG}/DMXCraft-macos-universal.dmg) | Open, drag DMXCraft into *Applications* |
| 🐧 **Linux** (x86_64, ARM64) | shell installer below | |

**Linux** (also works on macOS without an app bundle):

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/JustinSchweiger/DMXCraft/releases/download/{TAG}/dmxcraft-installer.sh | sh
```

> [!NOTE]
> **Windows:** if SmartScreen shows *“Windows protected your PC”*, click *More info → Run anyway*. The installer is not code-signed yet, so Windows does not know the publisher – it is the same MSI that GitHub Actions builds from this repository.
>
> **macOS:** the app is not notarized by Apple. On first launch go to *System Settings → Privacy & Security → Open Anyway*, or run `xattr -dr com.apple.quarantine /Applications/DMXCraft.app`.
>
> **Already installed?** DMXCraft checks for updates on startup and offers this version in the title bar.
>
> **USB-DMX on Linux:** your user needs access to `/dev/ttyUSB*`: `sudo usermod -aG dialout $USER`, then log in again.

---
