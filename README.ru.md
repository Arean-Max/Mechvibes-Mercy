<p align="center">
  <a href="https://github.com/Arean-Max/Mechvibes-Mercy">
    <picture>
      <source srcset="docs/logo-dark.png" media="(prefers-color-scheme: dark)">
      <source srcset="docs/logo-light.png" media="(prefers-color-scheme: light)">
      <img src="docs/logo.png" alt="Mercy logo" width="560">
    </picture>
  </a>
</p>

<p align="center">Высокопроизводительный симулятор звуков механической клавиатуры и мыши на Rust с задержкой звука менее 10 мс и полным отсутствием телеметрии.</p>

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

### Установка и запуск

Загрузите актуальную сборку со страницы [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Портативный архив (`Mercy-v*.zip`)** — Автономная сборка. Распакуйте в любую директорию и запустите `mercy.exe`. Включает набор звуковых паков по умолчанию в папке `soundpacks/`.

---

### Особенности

- **Минимальное потребление памяти** — ~25–40 МБ ОЗУ вместо 150–300+ МБ у оригинального Mechvibes на базе Electron.
- **Низкая задержка звука** — Аудио синтезируется в изолированном системном потоке с откликом менее 10 мс даже при высокоскоростном наборе текста.
- **0% влияния на FPS и безопасность для античитов** — Перехват ввода осуществляется через нативный Windows Raw Input без блокирующих глобальных хуков (`WH_KEYBOARD_LL`). Безопасно для Vanguard, VAC, Faceit и EAC.
- **Раздельные профили клавиатуры и мыши** — Индивидуальная настройка звуковых паков, громкости и панорамирования для клавиш и кликов мыши.
- **32-голосная полифония** — Одновременное воспроизведение звуков без щелчков, клиппинга и выпадения аудио.
- **Универсальная поддержка паков** — Чтение паков Mechvibes v1 и v2 (поддержка форматов WAV, MP3, OGG и FLAC).
- **100% приватность и оффлайн** — Полностью вырезана аналитика, трекеры (Aptabase) и любые сетевые запросы.

---

### Добавление звуковых паков

1. **Drag & Drop** — Перетащите папку со звуковым паком прямо в окно программы или перейдите в **Settings &rarr; Soundpacks &rarr; Import Soundpack**.
2. **Директории поиска** — Пользовательские паки считываются из `%APPDATA%\Mechvibes\soundpacks` или из локального каталога `soundpacks/` рядом с программой.

---

### FAQ

#### Чем Mercy отличается от оригинального Mechvibes?
Mercy представляет собой полную переработку Mechvibes на языке Rust. Тяжелый runtime Chromium/Node.js заменен системными вызовами WinAPI и оптимизированными аудиопотоками, что сократило потребление памяти на ~85% и устранило задержки звука.

#### Безопасен ли Mercy для античитов в играх?
Да. Mercy считывает нажатия через Windows Raw Input (`WM_INPUT`), стандартный механизм ввода операционной системы. Программа не внедряет DLL, не использует глобальные хуки `WH_KEYBOARD_LL` и не обращается к памяти игр.

#### Где хранятся звуковые паки?
В каталоге `%APPDATA%\Mechvibes\soundpacks` либо в папке `soundpacks/` рядом с бинарным файлом. Поддерживаются форматы конфигураций Mechvibes v1 и v2 (`config.json`).

#### Горячие клавиши:
- `Ctrl + Alt + M` — Включить / отключить звук программы.
- Кнопка закрытия окна сворачивает приложение в системный трей.

---

### Документация и архитектура

- [Архитектура системы](docs/system-architecture.md) — Потоковая модель, пайплайн ввода и аудио-граф
- [Спецификация конфигов звуковых паков v2](docs/soundpack-config-v2.md) — Схема конфигураций и файловая структура
- [Руководство по развертыванию](docs/deployment-guide.md) — Инструкция по компиляции и сборке дистрибутивов
- [Интерактивная диаграмма](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Граф архитектуры на GitDiagram

---

### Сборка из исходников

Требования: компилятор Rust (версия 1.88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

Скомпилированный файл будет находиться по пути `target/release/mercy.exe`.

---

### Лицензия

[MIT](LICENSE) — Форк проекта [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) от Hai Nguyen.
