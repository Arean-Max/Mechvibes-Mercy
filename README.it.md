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

### Installazione e download

Prendi l'ultima build precostruita da [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Archivio portatile (`Mercy-v*.zip`)**: pacchetto autonomo. Estrai ovunque e avvia "mercy.exe". Include pacchetti di suoni in bundle pronti all'uso in `soundpacks/`.
---
### In evidenza

- **Impronta di risorse ultraleggera**: consuma circa 25–40 MB di RAM rispetto a 150–300+ MB dei Mechvibes basati su Electron.
- **Motore audio dedicato a bassa latenza** — La sintesi audio e il missaggio vengono eseguiti su un thread di sistema isolato, ottenendo tempi di risposta inferiori a 10 ms senza interruzioni dell'audio durante picchi di digitazione di oltre 100 WPM.
- **Impatto zero FPS e sicurezza anti-cheat**: acquisisce eventi di input tramite l'input nativo di Windows Raw anziché bloccare i ganci della tastiera globale (`WH_KEYBOARD_LL`). Sicuro insieme a Vanguard, VAC, Faceit ed EAC.
- **Paesaggi sonori sincronizzati di tastiera e mouse**: assegnazione indipendente di pacchetti di suoni, controllo del volume e panoramica per sequenze di tasti e clic del mouse.
- **Polifonia a 32 voci**: riproduzione simultanea del suono senza scoppiettii, ritagli o interruzioni della voce.
- **Compatibilità universale Soundpack**: parser nativo per i formati legacy Mechvibes v1 e v2 (supporta WAV, MP3, OGG e FLAC).
- **100% offline e privato**: zero analisi, tracker di telemetria (Aptabase rimosso) e richieste di rete.
---
### Gestione dei pacchetti di suoni

1. **Trascina e rilascia**: trascina qualsiasi cartella di pacchetti di suoni direttamente nella finestra dell'applicazione o vai a **Impostazioni → Pacchetti di suoni → Importa pacchetto di suoni**.
2. **Risoluzione della directory** — I pacchetti audio utente vengono letti da `%APPDATA%\Mechvibes\soundpacks` o dalla directory locale `./soundpacks` accanto al file binario.
---
### Domande frequenti

#### In cosa Mercy è diverso dai Mechvibes originali?
Mercy è una riscrittura completa di Mechvibes in Rust. Il pesante runtime Chromium/Node.js viene sostituito con chiamate di sistema native e thread audio ad alte prestazioni, riducendo il consumo di RAM di circa l'85% ed eliminando la latenza di input.

#### Il software anti-cheat contrassegnerà Mercy?
No. Mercy legge i tasti premuti tramite Windows Raw Input (`WM_INPUT`), lo stesso meccanismo di input standard utilizzato dai mouse e dalle tastiere da gioco. Non inserisce mai codice nei processi di destinazione, non esegue l'hook di `WH_KEYBOARD_LL` o non tocca la memoria del gioco.

#### Dove sono archiviati i miei pacchetti audio?
I soundpack risiedono in "%APPDATA%\Mechvibes\soundpacks" o direttamente accanto all'eseguibile portatile in "soundpacks/". Sono supportate sia le strutture di cartelle Mechvibes v1 e v2 che le specifiche `config.json`.

#### Quali scorciatoie globali sono disponibili?
- `Ctrl + Alt + M`: attiva/disattiva l'audio a livello globale.
- Il pulsante Chiudi viene ridotto a icona direttamente nella barra delle applicazioni per operazioni in background discrete.
---
### Documentazione e architettura

- [Architettura del sistema](docs/system-architecture.md): modello di thread, pipeline di polling degli input e grafico di mixaggio audio
- [Configurazione Soundpack v2](docs/soundpack-config-v2.md) — Schema di configurazione e struttura delle directory dei soundpack
- [Guida alla distribuzione](docs/deployment-guide.md) — Istruzioni per la compilazione incrociata e il confezionamento
- [Diagramma interattivo](https://gitdiagram.com/Area-Max/Mechvibes-Mercy) — Panoramica dell'architettura visiva
---
### Compila dal sorgente

Prerequisiti: toolchain Rust (1.88+).

___BLOCCOCODICE_0___

Il binario compilato verrà posizionato in "target/release/mercy.exe".
---
### Licenza

[MIT](LICENZA) — Originariamente basato su [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) di Hai Nguyen.