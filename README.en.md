<div align="center">

# ⌨️ Mercy

**Next-generation ultra-low latency mechanical keyboard and mouse sound simulator**

[![Rust](https://img.shields.io/badge/Rust-1.88+-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/GUI-Dioxus_0.7-blue.svg?style=flat-square)](https://dioxuslabs.com/)
[![License](https://img.shields.io/badge/license-MIT-green.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg?style=flat-square)](#)

[Русская версия](README.md) • [Download](#-download) • [Features](#-features) • [Building](#-building)

</div>

---

## 🚀 About

**Mercy** is a modern, lightweight mechanical keyboard sound simulator rewritten in pure **Rust** with native **Dioxus** UI.

The legacy Electron-based Mechvibes suffered from bloated memory consumption (150–300+ MB), audio micro-delays during fast typing, and risks of anti-cheat false positives caused by intrusive low-level keyboard hooks. **Mercy** solves all of these architectural bottlenecks, providing native responsiveness and 0% FPS impact in games.

---

## ✨ Features

- **⚡ 0% FPS & CPU Impact**: Utilizes native **Windows Raw Input** without blocking `WH_KEYBOARD_LL` hooks, preventing input lag and anti-cheat conflicts.
- **🛡️ 100% FOSS & Zero Telemetry**: Truly open-source without third-party trackers, analytics, or network pings. Fully offline and private.
- **🪶 Minimal RAM Footprint**: Consumes only **25–40 MB RAM**, compared to 200+ MB for Electron/Chromium apps.
- **🎧 Ultra-Low Latency Audio**: Dedicated real-time OS audio thread powered by lock-free channels. Sounds trigger in <5–10 ms.
- **🔊 32-Voice Polyphony**: Crisp, simultaneous playback during fast typing without clipping, pops, or stutters.
- **🖱️ Keyboard & Mouse Sounds**: Granular volume controls and soundpack selection for both keyboard keystrokes and mouse clicks.
- **📁 Universal Soundpack Support**: Compatible with classic Mechvibes (V1) and modern V2 formats, supporting WAV, OGG, MP3, and FLAC.
- **🎛️ Convenient Controls**: Instant mute hotkey (`Ctrl+Alt+M`), system tray integration, themes, and logo customization.

---

## 📥 Download

Prebuilt release binaries are available under [Releases](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

| Platform | Binary | Notes |
| :--- | :--- | :--- |
| **Windows x64** | [**`Mercy-v0.8.3-windows-x64.zip`**](https://github.com/Arean-Max/Mechvibes-Mercy/releases/latest) | Portable archive (extract and run `mercy.exe`) |

---

## 🛠️ Building from Source

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (1.88 or newer)
- Windows SDK / Visual Studio C++ Build Tools (on Windows)

### Build Command
```bash
# Clone the repository
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy

# Build optimized release binary
cargo build --release
```

The compiled binary will be located at:
```
target/release/mercy.exe
```

---

## 🎹 Adding Soundpacks

Import soundpacks by dragging and dropping the folder into the app window, or via settings:
1. Open **Settings** -> **Soundpacks** -> **Import Soundpack**.
2. Select the soundpack directory.
3. Mercy will automatically validate and load the soundpack.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
