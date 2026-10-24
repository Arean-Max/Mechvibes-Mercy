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

### Installation og downloads

Få fat i den seneste forudbyggede build fra [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Portable Archive (`Mercy-v*.zip`)** — Standalone bundle. Udpak hvor som helst og start `mercy.exe`. Inkluderer medfølgende lydpakker ud af æsken i `soundpacks/`.
---
### Højdepunkter

- **Ultralight Resource Footprint** — Forbruger ~25–40 MB RAM sammenlignet med 150–300+ MB på elektronbaserede Mechvibes.
- **Dedikeret lydmotor med lav forsinkelse** — Lydsyntese og -miksning kører på en isoleret systemtråd, hvilket opnår en responstid på under 10 ms uden lydstimring under 100+ WPM-skrivebursts.
- **Nul FPS Impact & Anti-Cheat Safe** — Fanger inputhændelser via native Windows Raw Input i stedet for at blokere globale tastaturhooks (`WH_KEYBOARD_LL`). Sikker sammen med Vanguard, VAC, Faceit og EAC.
- **Synchronized Keyboard & Mouse Soundscapes** — Uafhængig lydpakketildeling, lydstyrkekontrol og panorering for tastetryk og museklik.
- **32-stemmers polyfoni** — Samtidig lydafspilning uden popping, klipning eller stemmeudfald.
- **Universal Soundpack-kompatibilitet** — Native parser til ældre Mechvibes v1- og v2-formater (understøtter WAV, MP3, OGG og FLAC).
- **100 % offline og privat** - Ingen analyser, telemetrisporere (Aptabase fjernet) og netværksanmodninger.
---
### Soundpack Management

1. **Træk og slip** — Træk enhver lydpakkemappe direkte ind i programvinduet, eller naviger til **Indstillinger → Lydpakker → Importer lydpakke**.
2. **Directory Resolution** — Brugerlydpakker læses fra `%APPDATA%\Mechvibes\soundpacks` eller den lokale `./soundpacks`-mappe ved siden af ​​det binære.
---
### FAQ

#### Hvordan adskiller Mercy sig fra de originale Mechvibes?
Mercy er en komplet Rust-omskrivning af Mechvibes. Den tunge Chromium/Node.js-runtime er erstattet med native systemkald og højtydende lydtråde, hvilket reducerer RAM-forbruget med ~85 % og eliminerer inputlatens.

#### Vil anti-cheat-software markere Mercy?
Nej. Mercy læser tastetryk via Windows Raw Input (`WM_INPUT`), den samme standardinputmekanisme, som bruges af gamingmus og tastaturer. Den injicerer aldrig kode i målprocesser, kobler 'WH_KEYBOARD_LL' eller rører spilhukommelsen.

#### Hvor er mine lydpakker gemt?
Lydpakker findes i `%APPDATA%\Mechvibes\soundpacks` eller direkte ved siden af ​​den bærbare eksekverbare i `soundpacks/`. Både Mechvibes v1 og v2 mappestrukturer og `config.json` specifikationer er understøttet.

#### Hvilke globale genveje er tilgængelige?
- `Ctrl + Alt + M` — Slå lyd fra/til til globalt.
- Luk-knap minimerer direkte til systembakken for diskret baggrundsbetjening.
---
### Dokumentation og arkitektur

- [Systemarkitektur](docs/system-architecture.md) - Trådmodel, input polling-pipeline og lydmiksningsgraf
- [Soundpack Configuration v2](docs/soundpack-config-v2.md) — Konfigurationsskema og soundpack-biblioteksstruktur
- [Deployment Guide](docs/deployment-guide.md) — Krydskompilering og pakningsinstruktioner
- [Interaktivt diagram](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Oversigt over visuel arkitektur
---
### Byg fra kilde

Forudsætninger: Rustværktøjskæde (1,88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

Den kompilerede binære vil blive placeret på `target/release/mercy.exe`.
---
### Licens

[MIT](LICENS) — Oprindeligt baseret på [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) af Hai Nguyen.