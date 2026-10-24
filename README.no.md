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

### Installasjon og nedlastinger

Ta den siste forhåndsbygde versjonen fra [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Portable Archive (`Mercy-v*.zip`)** — Frittstående pakke. Pakk ut hvor som helst og start `mercy.exe`. Inkluderer medfølgende lydpakker ut av esken i `soundpacks/`.
---
### Høydepunkter

- **Ultralight Resource Footprint** — Forbruker ~25–40 MB RAM sammenlignet med 150–300+ MB på elektronbaserte Mechvibes.
- **Dedikert lydmotor med lav latens** — Lydsyntese og miksing kjøres på en isolert systemtråd, og oppnår responstider på under 10 ms uten lydstamming under 100+ WPM-skriveserier.
- **Zero FPS Impact & Anti-Cheat Safe** — Fanger inndatahendelser via native Windows Raw Input i stedet for å blokkere globale tastaturkroker (`WH_KEYBOARD_LL`). Trygg sammen med Vanguard, VAC, Faceit og EAC.
- **Synkronisert tastatur- og mus-lydlandskap** — Uavhengig lydpakketilordning, volumkontroll og panorering for tastetrykk og museklikk.
- **32-stemmers polyfoni** — Samtidig lydavspilling uten sprett, klipping eller stemmefall.
- **Universal Soundpack-kompatibilitet** — Native parser for eldre Mechvibes v1 og v2-formater (støtter WAV, MP3, OGG og FLAC).
- **100 % frakoblet og privat** - Null analyser, telemetrisporere (Aptabase fjernet) og nettverksforespørsler.
---
### Soundpack Management

1. **Dra og slipp** — Dra en hvilken som helst lydpakkemappe direkte inn i programvinduet, eller naviger til **Innstillinger → Lydpakker → Importer lydpakke**.
2. **Katalogoppløsning** — Brukerlydpakker leses fra `%APPDATA%\Mechvibes\soundpacks` eller den lokale `./soundpacks`-katalogen ved siden av binærfilen.
---
### Vanlige spørsmål

#### Hvordan er Mercy forskjellig fra de originale Mechvibes?
Mercy er en komplett Rust-omskriving av Mechvibes. Den tunge Chromium/Node.js-kjøretiden er erstattet med native systemanrop og høyytelses lydtråder, noe som reduserer RAM-forbruket med ~85 % og eliminerer inngangsforsinkelse.

#### Vil anti-jukseprogramvare flagge Mercy?
Nei. Mercy leser tastetrykk via Windows Raw Input (`WM_INPUT`), den samme standardinndatamekanismen som brukes av spillmus og tastaturer. Den injiserer aldri kode i målprosesser, kobler "WH_KEYBOARD_LL" eller berører spillminnet.

#### Hvor er lydpakkene mine lagret?
Lydpakker ligger i `%APPDATA%\Mechvibes\soundpacks` eller direkte ved siden av den bærbare kjørbare i `soundpacks/`. Både Mechvibes v1 og v2 mappestrukturer og 'config.json'-spesifikasjoner støttes.

#### Hvilke globale snarveier er tilgjengelige?
- `Ctrl + Alt + M` – Slå av/på lyd globalt.
- Lukk-knappen minimerer direkte til systemstatusfeltet for diskré bakgrunnsoperasjon.
---
### Dokumentasjon og arkitektur

– [System Architecture](docs/system-architecture.md) – Trådmodell, input-polling-pipeline og lydmiksingsgraf
- [Soundpack Configuration v2](docs/soundpack-config-v2.md) — Konfigurasjonsskjema og lydpakkekatalogstruktur
– [Deployment Guide](docs/deployment-guide.md) – Krysskompilerings- og pakkeinstruksjoner
– [Interaktivt diagram](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) – Visuell arkitekturoversikt
---
### Bygg fra kilden

Forutsetninger: Rustverktøykjede (1,88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

Den kompilerte binære filen vil bli plassert på `target/release/mercy.exe`.
---
### Lisens

[MIT](LISENS) — Opprinnelig basert på [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) av Hai Nguyen.