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

### Installation et téléchargements

Récupérez la dernière version prédéfinie de [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases) :

- **Archive portable (`Mercy-v*.zip`)** — Offre groupée autonome. Extrayez n'importe où et lancez `mercy.exe`. Inclut des packs de sons prêts à l'emploi dans `soundpacks/`.
---
### Faits saillants

- **Ultralight Resource Footprint** — Consomme environ 25 à 40 Mo de RAM, contre 150 à 300+ Mo sur les Mechvibes à base d'électrons.
- **Moteur audio dédié à faible latence** — La synthèse et le mixage audio s'exécutent sur un thread système isolé, atteignant des temps de réponse inférieurs à 10 ms sans bégaiement audio pendant des rafales de frappe de plus de 100 WPM.
- **Zero FPS Impact & Anti-Cheat Safe** — Capture les événements d'entrée via l'entrée native Windows Raw plutôt que de bloquer les crochets de clavier globaux (`WH_KEYBOARD_LL`). En sécurité aux côtés de Vanguard, VAC, Faceit et EAC.
- **Paysages sonores synchronisés du clavier et de la souris** — Attribution indépendante du pack de sons, contrôle du volume et panoramique pour les frappes au clavier et les clics de souris.
- **Polyphonie à 32 voix** — Lecture simultanée du son sans éclats, coupures ou coupures de voix.
- **Compatibilité Universal Soundpack** — Analyseur natif pour les anciens formats Mechvibes v1 et v2 (prend en charge WAV, MP3, OGG et FLAC).
- **100 % hors ligne et privé** – Zéro analyse, suivi de télémétrie (Aptabase supprimé) et requêtes réseau.
---
### Gestion des packs de sons

1. **Glisser-déposer** — Faites glisser n'importe quel dossier de pack de sons directement dans la fenêtre de l'application ou accédez à **Paramètres → Soundpacks → Importer un pack de sons**.
2. **Résolution du répertoire** — Les soundpacks utilisateur sont lus à partir de `%APPDATA%\Mechvibes\soundpacks` ou du répertoire local `./soundpacks` à côté du binaire.
---
###FAQ

#### En quoi Mercy est-il différent des Mechvibes originaux ?
Mercy est une réécriture complète de Rust de Mechvibes. Le lourd runtime Chromium/Node.js est remplacé par des appels système natifs et des threads audio hautes performances, réduisant la consommation de RAM d'environ 85 % et éliminant la latence d'entrée.

#### Le logiciel anti-triche signalera-t-il Mercy ?
Non. Mercy lit les frappes via Windows Raw Input (`WM_INPUT`), le même mécanisme de saisie standard utilisé par les souris et les claviers de jeu. Il n'injecte jamais de code dans les processus cibles, n'accroche jamais `WH_KEYBOARD_LL` et ne touche jamais la mémoire du jeu.

#### Où sont stockés mes packs de sons ?
Les Soundpacks résident dans `%APPDATA%\Mechvibes\soundpacks` ou directement à côté de l'exécutable portable dans `soundpacks/`. Les structures de dossiers Mechvibes v1 et v2 et les spécifications « config.json » sont prises en charge.

#### Quels raccourcis globaux sont disponibles ?
- `Ctrl + Alt + M` — Activer/désactiver le son globalement.
- Le bouton Fermer se minimise directement dans la barre d'état système pour une opération en arrière-plan discrète.
---
###Documentation et architecture

- [Architecture système](docs/system-architecture.md) — Modèle de thread, pipeline d'interrogation d'entrée et graphique de mixage audio
- [Soundpack Configuration v2](docs/soundpack-config-v2.md) — Schéma de configuration et structure du répertoire soundpack
- [Guide de déploiement](docs/deployment-guide.md) — Instructions de compilation croisée et d'empaquetage
- [Diagramme interactif](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Présentation de l'architecture visuelle
---
### Construire à partir des sources

Prérequis : chaîne d'outils Rust (1.88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

Le binaire compilé sera placé dans `target/release/mercy.exe`.
---
### Licence

[MIT](LICENCE) — Basé à l'origine sur [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) par Hai Nguyen.