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

### Instalacija i preuzimanja

Preuzmite najnoviju unaprijed izgrađenu verziju sa [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Portable Archive (`Mercy-v*.zip`)** — Samostalni paket. Izvucite bilo gdje i pokrenite `mercy.exe`. Uključuje pakete zvuka iz kutije u `soundpacks/`.
---
### Highlights

- **Ultralight Resource Footprint** — Troši ~25–40 MB RAM-a u poređenju sa 150–300+ MB na Mechvibes baziranim na Electron-u.
- **Namjenski audio motor sa niskim kašnjenjem** — Audio sinteza i miksanje se odvijaju na izolovanoj sistemskoj niti, postižući vrijeme odziva ispod 10 ms bez zastoja zvuka tokom 100+ WPM rafova kucanja.
- **Zero FPS Impact & Anti-Cheat Safe** — Snima ulazne događaje putem izvornog Windows Raw Inputa umjesto da blokira globalne zakačke na tastaturi (`WH_KEYBOARD_LL`). Sigurno uz Vanguard, VAC, Faceit i EAC.
- **Sinhronizovani zvučni pejzaži tastature i miša** — Nezavisno dodeljivanje zvučnog paketa, kontrola jačine zvuka i pomeranje za pritiskanje tastera i klikove mišem.
- **32-glasovna polifonija** — Istovremena reprodukcija zvuka bez iskakanja, isjecanja ili prekida glasa.
- **Kompatibilnost sa univerzalnim zvučnim paketom** — Izvorni parser za stare Mechvibes v1 i v2 formate (podržava WAV, MP3, OGG i FLAC).
- **100% van mreže i privatno** — Nema analitike, telemetrijskih tragova (Aptabase je uklonjen) i mrežnih zahtjeva.
---
### Upravljanje zvučnim paketom

1. **Drag & Drop** — Prevucite bilo koju fasciklu sa zvučnim paketom direktno u prozor aplikacije ili idite na **Podešavanja → Paketi zvuka → Uvezi zvučni paket**.
2. **Rezolucija direktorija** — Korisnički zvučni paketi se čitaju iz `%APPDATA%\Mechvibes\soundpacks` ili lokalnog `./soundpacks` direktorija pored binarne datoteke.
---
### FAQ

#### Kako se Mercy razlikuje od originalnih Mechvibesa?
Mercy je potpuni Rust rewrite Mechvibesa. Teško Chromium/Node.js vrijeme izvođenja je zamijenjeno izvornim sistemskim pozivima i audio nitima visokih performansi, smanjujući potrošnju RAM-a za ~85% i eliminirajući ulaznu latencije.

#### Hoće li softver protiv varanja označiti Mercy?
Ne. Mercy čita pritiske tipki putem Windows Raw Inputa (`WM_INPUT`), istog standardnog ulaznog mehanizma koji koriste gaming miševi i tastature. Nikada ne ubacuje kod u ciljne procese, zakači `WH_KEYBOARD_LL`, niti dodiruje memoriju igre.

#### Gdje su pohranjeni moji zvučni paketi?
Zvučni paketi se nalaze u `%APPDATA%\Mechvibes\soundpacks` ili direktno uz prenosivi izvršni fajl u `soundpacks/`. I Mechvibes v1 i v2 strukture foldera i specifikacije `config.json` su podržane.

#### Koje su globalne prečice dostupne?
- `Ctrl + Alt + M` — Globalno isključi/uključi zvuk.
- Dugme za zatvaranje minimizira direktno u sistemsku paletu za nenametljiv rad u pozadini.
---
### Dokumentacija i arhitektura

- [Arhitektura sistema](docs/system-architecture.md) — Model niti, cevovod za ispitivanje ulaza i graf miješanja zvuka
- [Soundpack Configuration v2](docs/soundpack-config-v2.md) — Šema konfiguracije i struktura direktorija soundpack-a
- [Vodič za implementaciju](docs/deployment-guide.md) — Uputstva za unakrsnu kompilaciju i pakovanje
- [Interaktivni dijagram](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Vizualni pregled arhitekture
---
### Napravi iz izvora

Preduvjeti: Rust toolchain (1.88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

Prevedeni binarni fajl će biti smešten na `target/release/mercy.exe`.
---
### Licenca

[MIT](LICENCA) — Izvorno zasnovan na [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) od Hai Nguyena.