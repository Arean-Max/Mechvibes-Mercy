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

### Instalacja i pobieranie

Pobierz najnowszą, gotową wersję z [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Przenośne archiwum (`Mercy-v*.zip`)** — Pakiet samodzielny. Wypakuj gdziekolwiek i uruchom `mercy.exe`. Zawiera pakiety dźwiękowe gotowe do użycia w `pakietach dźwiękowych/`.
---
### Najważniejsze

- **Ultralekkie zasoby** — Zużywa ~25–40 MB pamięci RAM w porównaniu do 150–300+ MB w Mechvibes opartych na elektronach.
- **Dedykowany silnik audio o niskim opóźnieniu** — Synteza i miksowanie dźwięku przebiegają w izolowanym wątku systemowym, co pozwala uzyskać czas reakcji poniżej 10 ms bez zacinania się dźwięku podczas serii pisania o prędkości ponad 100 WPM.
- **Zero FPS Impact & Anti-Cheat Safe** — Przechwytuje zdarzenia wejściowe za pomocą natywnego wejścia Windows Raw zamiast blokować globalne zaczepy klawiatury (`WH_KEYBOARD_LL`). Bezpieczny obok Vanguard, VAC, Faceit i EAC.
- **Zsynchronizowane krajobrazy dźwiękowe klawiatury i myszy** — Niezależne przypisywanie pakietów dźwiękowych, regulacja głośności i panoramowanie naciśnięć klawiszy i kliknięć myszą.
- **32-głosowa polifonia** — Równoczesne odtwarzanie dźwięku bez trzaskania, przycinania i zaniku głosu.
- **Kompatybilność z Universal Soundpack** — Natywny parser dla starszych formatów Mechvibes v1 i v2 (obsługuje WAV, MP3, OGG i FLAC).
- **100% offline i prywatnie** — Zero analityki, śledzenia telemetrycznego (usunięto Aptabase) i żądań sieciowych.
---
### Zarządzanie pakietem dźwiękowym

1. **Przeciągnij i upuść** — przeciągnij dowolny folder pakietu dźwiękowego bezpośrednio do okna aplikacji lub przejdź do **Ustawienia → Pakiety dźwięków → Importuj pakiet dźwięków**.
2. **Rozdzielczość katalogów** — Pakiety dźwiękowe użytkownika są odczytywane z `%APPDATA%\Mechvibes\soundpacks` lub lokalnego katalogu `./soundpacks` obok pliku binarnego.
---
### Często zadawane pytania

#### Czym Mercy różni się od oryginalnych Mechvibes?
Mercy to kompletna przeróbka Mechvibes autorstwa Rusta. Ciężkie środowisko wykonawcze Chromium/Node.js zostało zastąpione natywnymi wywołaniami systemowymi i wysokowydajnymi wątkami audio, zmniejszając zużycie pamięci RAM o ~85% i eliminując opóźnienia wejściowe.

#### Czy oprogramowanie zapobiegające oszustwom oznaczy Mercy?
Nie. Mercy odczytuje naciśnięcia klawiszy za pośrednictwem systemu Windows Raw input („WM_INPUT”), tego samego standardowego mechanizmu wprowadzania danych, używanego przez myszy i klawiatury do gier. Nigdy nie wprowadza kodu do procesów docelowych, nie przechwytuje `WH_KEYBOARD_LL` ani nie dotyka pamięci gry.

#### Gdzie są przechowywane moje pakiety dźwiękowe?
Pakiety dźwiękowe znajdują się w `%APPDATA%\Mechvibes\soundpacks` lub bezpośrednio obok przenośnego pliku wykonywalnego w `soundpacks/`. Obsługiwane są struktury folderów Mechvibes v1 i v2 oraz specyfikacje `config.json`.

#### Jakie skróty globalne są dostępne?
- `Ctrl + Alt + M` — globalne włączanie/wyłączanie wyciszenia dźwięku.
- Przycisk Zamknij minimalizuje bezpośrednio do zasobnika systemowego, zapewniając dyskretną pracę w tle.
---
### Dokumentacja i architektura

- [Architektura systemu](docs/system-architecture.md) — Model wątku, potok odpytywania sygnału wejściowego i wykres miksowania dźwięku
- [Konfiguracja Soundpacka v2](docs/soundpack-config-v2.md) — Schemat konfiguracji i struktura katalogów Soundpacka
- [Przewodnik po wdrażaniu](docs/deployment-guide.md) — Instrukcje dotyczące kompilacji krzyżowej i pakowania
- [Diagram interaktywny](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Omówienie architektury wizualnej
---
### Kompiluj ze źródła

Wymagania wstępne: Łańcuch narzędzi Rust (1.88+).

___KODBLOK_0___

Skompilowany plik binarny zostanie umieszczony w `target/release/mercy.exe`.
---
### Licencja

[MIT](LICENCJA) — pierwotnie oparty na [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) autorstwa Hai Nguyena.