# Mercy

Lightweight mechanical keyboard and mouse sound simulator written in Rust.

Fork of [Mechvibes](https://github.com/hainguyents13/mechvibes-dx), focused on minimal resource usage, zero audio latency, and completely offline operation.

[Русская версия](README.md)

## Features

* **Low memory footprint** — ~25-40 MB RAM instead of 150-300+ MB on Electron.
* **Low audio latency** — dedicated background audio thread with sub-10ms response time during fast typing.
* **0% FPS / anti-cheat impact** — captures input via native Windows Raw Input without blocking global hooks (`WH_KEYBOARD_LL`).
* **Keyboard & mouse sounds** — independent volume controls and soundpacks for keystrokes and mouse clicks.
* **32-voice polyphony** — clean playback without pops, clipping, or cutoffs.
* **100% FOSS & no telemetry** — all trackers, network telemetry, and analytics (Aptabase) removed.
* **Universal soundpack support** — loads Mechvibes v1 and v2 soundpacks (WAV, MP3, OGG, FLAC).
* **Hotkeys** — toggle mute with `Ctrl+Alt+M`, minimize to system tray.

## Download & Run

Prebuilt packages are available on the [Releases](https://github.com/Arean-Max/Mechvibes-Mercy/releases) page:

* Download `Mercy-v0.8.3-windows-x64.zip`.
* Extract it anywhere and run `mercy.exe`.

Default soundpacks are included out-of-the-box in the `soundpacks/` folder.

## Adding Soundpacks

1. Drag and drop any soundpack folder directly into the app window, or go to **Settings -> Soundpacks -> Import Soundpack**.
2. Custom soundpacks are stored in `%APPDATA%\Mechvibes\soundpacks` or in the local `soundpacks/` folder next to the executable.

## Building from Source

Requires [Rust](https://rustup.rs/) (1.88+):

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

The compiled binary will be located at `target/release/mercy.exe`.

## License

Released under the [MIT License](LICENSE). Based on [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) by Hai Nguyen.
