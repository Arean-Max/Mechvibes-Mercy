<p align="center"></p>
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

### Installation und Downloads

Holen Sie sich den neuesten vorgefertigten Build von [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Portables Archiv (`Mercy-v*.zip`)** – Standalone-Paket. Entpacken Sie es irgendwo und starten Sie „mercy.exe“. Enthält gebündelte Soundpacks ab Werk in „soundpacks/“.
---
### Highlights

- **Ultrageringer Ressourcenverbrauch** – Verbraucht ca. 25–40 MB RAM im Vergleich zu 150–300+ MB bei Electron-basierten Mechvibes.
- **Dedizierte Audio-Engine mit geringer Latenz** – Audiosynthese und -mischung laufen in einem isolierten System-Thread und erreichen Reaktionszeiten von weniger als 10 ms ohne Audio-Ruckler bei Tippstößen mit mehr als 100 WPM.
- **Keine FPS-Auswirkungen und Anti-Cheat-sicher** – Erfasst Eingabeereignisse über die native Windows-Roheingabe, anstatt globale Tastatur-Hooks („WH_KEYBOARD_LL“) zu blockieren. Sicher neben Vanguard, VAC, Faceit und EAC.
- **Synchronisierte Tastatur- und Maus-Soundscapes** – Unabhängige Soundpack-Zuweisung, Lautstärkeregelung und Schwenkung für Tastenanschläge und Mausklicks.
- **32-stimmige Polyphonie** – Gleichzeitige Tonwiedergabe ohne Knackgeräusche, Übersteuerungen oder Stimmaussetzer.
- **Universelle Soundpack-Kompatibilität** – Nativer Parser für ältere Mechvibes v1- und v2-Formate (unterstützt WAV, MP3, OGG und FLAC).
- **100 % offline und privat** – Keine Analysen, Telemetrie-Tracker (Aptabase entfernt) und Netzwerkanfragen.
---
### Soundpack-Management

1. **Drag & Drop** – Ziehen Sie einen beliebigen Soundpack-Ordner direkt in das Anwendungsfenster oder navigieren Sie zu **Einstellungen → Soundpacks → Soundpack importieren**.
2. **Verzeichnisauflösung** – Benutzer-Soundpacks werden aus „%APPDATA%\Mechvibes\soundpacks“ oder dem lokalen Verzeichnis „./soundpacks“ neben der Binärdatei gelesen.
---
### FAQ

#### Wie unterscheidet sich Mercy von den ursprünglichen Mechvibes?
Mercy ist eine komplette Rust-Neufassung von Mechvibes. Die umfangreiche Chromium/Node.js-Laufzeit wird durch native Systemaufrufe und leistungsstarke Audio-Threads ersetzt, wodurch der RAM-Verbrauch um etwa 85 % gesenkt und die Eingabelatenz beseitigt wird.

#### Wird Anti-Cheat-Software Mercy markieren?
Nein. Mercy liest Tastatureingaben über Windows Raw Input („WM_INPUT“), den gleichen Standardeingabemechanismus, der auch von Gaming-Mäusen und -Tastaturen verwendet wird. Es fügt niemals Code in Zielprozesse ein, hakt „WH_KEYBOARD_LL“ ein oder berührt den Spielspeicher.

#### Wo werden meine Soundpacks gespeichert?
Soundpacks befinden sich in „%APPDATA%\Mechvibes\soundpacks“ oder direkt neben der tragbaren ausführbaren Datei in „soundpacks/“. Es werden sowohl die Ordnerstrukturen von Mechvibes v1 und v2 als auch die Spezifikationen „config.json“ unterstützt.

#### Welche globalen Verknüpfungen sind verfügbar?
- „Strg + Alt + M“ – Audio global stummschalten bzw. die Stummschaltung aufheben.
- Die Schaltfläche „Schließen“ wird direkt in die Taskleiste minimiert, um einen unauffälligen Hintergrundbetrieb zu ermöglichen.
---
### Dokumentation & Architektur

– [Systemarchitektur](docs/system-architecture.md) – Thread-Modell, Eingabeabfrage-Pipeline und Audio-Mischdiagramm
- [Soundpack-Konfiguration v2](docs/soundpack-config-v2.md) – Konfigurationsschema und Soundpack-Verzeichnisstruktur
- [Bereitstellungshandbuch](docs/deployment-guide.md) – Anweisungen zur übergreifenden Kompilierung und Verpackung
- [Interaktives Diagramm](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) – Übersicht über die visuelle Architektur
---
### Aus dem Quellcode erstellen

Voraussetzungen: Rust-Toolchain (1.88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

Die kompilierte Binärdatei wird unter „target/release/mercy.exe“ abgelegt.
---
### Lizenz

[MIT](LIZENZ) – Ursprünglich basierend auf [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) von Hai Nguyen.