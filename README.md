<p align="center">
  <a href="https://github.com/Arean-Max/Mechvibes-Mercy">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Mercy logo" width="560">
    </picture>
  </a>
</p>

<p align="center">A high-performance mechanical keyboard and mouse sound simulator written in Rust with sub-10ms audio latency and zero telemetry.</p>

<p align="center">
  <a href="https://github.com/Arean-Max/Mechvibes-Mercy/releases"><img alt="Releases" src="https://img.shields.io/github/v/release/Arean-Max/Mechvibes-Mercy?style=flat-square&color=emerald" /></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" /></a>
  <a href="https://microsoft.com/windows"><img alt="Platform" src="https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20%7C%20Linux-informational?style=flat-square" /></a>
  <a href="https://deepwiki.com/Arean-Max/Mechvibes-Mercy"><img alt="Docs" src="https://img.shields.io/badge/Docs-DeepWiki-blueviolet?style=flat-square" /></a>
  <a href="https://gitdiagram.com/Arean-Max/Mechvibes-Mercy"><img alt="Architecture" src="https://img.shields.io/badge/Diagram-GitDiagram-orange?style=flat-square" /></a>
</p>

<p align="center">
  <a href="README.md">English</a> |
  <a href="README.zh.md">简体中文</a> |
  <a href="README.zht.md">繁體中文</a> |
  <a href="README.ko.md">한국어</a> |
  <a href="README.de.md">Deutsch</a> |
  <a href="README.es.md">Español</a> |
  <a href="README.fr.md">Français</a> |
  <a href="README.it.md">Italiano</a> |
  <a href="README.da.md">Dansk</a> |
  <a href="README.ja.md">日本語</a> |
  <a href="README.pl.md">Polski</a> |
  <a href="README.ru.md">Русский</a> |
  <a href="README.bs.md">Bosanski</a> |
  <a href="README.ar.md">العربية</a> |
  <a href="README.no.md">Norsk</a> |
  <a href="README.br.md">Português (Brasil)</a> |
  <a href="README.th.md">ไทย</a> |
  <a href="README.tr.md">Türkçe</a> |
  <a href="README.uk.md">Українська</a> |
  <a href="README.bn.md">বাংলা</a> |
  <a href="README.gr.md">Ελληνικά</a> |
  <a href="README.vi.md">Tiếng Việt</a>
</p>

<p align="center">
  <a href="https://github.com/Arean-Max/Mechvibes-Mercy/releases">
    <img src="docs/screenshots/preview.png" alt="Mercy Preview" width="340" />
  </a>
</p>

---

### Installation & Downloads

Grab the latest prebuilt build from [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Portable Archive (`Mercy-v*.zip`)** — Standalone bundle. Extract anywhere and launch `mercy.exe`. Includes bundled soundpacks out of the box in `soundpacks/`.

---

### Highlights

- **Ultralight Resource Footprint** — Consumes ~25–40 MB RAM compared to 150–300+ MB on Electron-based Mechvibes.
- **Dedicated Low-Latency Audio Engine** — Audio synthesis and mixing run on an isolated system thread, achieving sub-10ms response times without audio stutter during 100+ WPM typing bursts.
- **Zero FPS Impact & Anti-Cheat Safe** — Captures input events via native Windows Raw Input rather than blocking global keyboard hooks (`WH_KEYBOARD_LL`). Safe alongside Vanguard, VAC, Faceit, and EAC.
- **Synchronized Keyboard & Mouse Soundscapes** — Independent soundpack assignment, volume control, and panning for keystrokes and mouse clicks.
- **32-Voice Polyphony** — Concurrent sound playback without popping, clipping, or voice dropouts.
- **Universal Soundpack Compatibility** — Native parser for legacy Mechvibes v1 and v2 formats (supports WAV, MP3, OGG, and FLAC).
- **100% Offline & Private** — Zero analytics, telemetry trackers (Aptabase removed), and network requests.

---

### Soundpack Management

1. **Drag & Drop** — Drag any soundpack folder directly into the application window, or navigate to **Settings &rarr; Soundpacks &rarr; Import Soundpack**.
2. **Directory Resolution** — User soundpacks are read from `%APPDATA%\Mechvibes\soundpacks` or the local `./soundpacks` directory next to the binary.

---

### FAQ

#### How is Mercy different from the original Mechvibes?
Mercy is a complete Rust rewrite of Mechvibes. The heavy Chromium/Node.js runtime is replaced with native system calls and high-performance audio threads, slashing RAM consumption by ~85% and eliminating input latency.

#### Will anti-cheat software flag Mercy?
No. Mercy reads keystrokes via Windows Raw Input (`WM_INPUT`), the same standard input mechanism used by gaming mice and keyboards. It never injects code into target processes, hooks `WH_KEYBOARD_LL`, or touches game memory.

#### Where are my soundpacks stored?
Soundpacks reside in `%APPDATA%\Mechvibes\soundpacks` or directly alongside the portable executable in `soundpacks/`. Both Mechvibes v1 and v2 folder structures and `config.json` specifications are supported.

#### What global shortcuts are available?
- `Ctrl + Alt + M` — Toggle mute / unmute audio globally.
- Close button minimizes directly to system tray for unobtrusive background operation.

---

### Documentation & Architecture

- [System Architecture](docs/system-architecture.md) — Thread model, input polling pipeline, and audio mixing graph
- [Soundpack Configuration v2](docs/soundpack-config-v2.md) — Configuration schema and soundpack directory structure
- [Deployment Guide](docs/deployment-guide.md) — Cross-compilation and packaging instructions
- [Interactive Diagram](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Visual architecture overview

---

### Build from source

Prerequisites: Rust toolchain (1.88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

The compiled binary will be placed at `target/release/mercy.exe`.

---

### License

[MIT](LICENSE) — Originally based on [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) by Hai Nguyen.
