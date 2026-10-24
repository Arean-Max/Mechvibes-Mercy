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

### Встановлення та завантаження

Візьміть останню готову збірку з [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Портативний архів (`Mercy-v*.zip`)** — автономний пакет. Розпакуйте куди завгодно та запустіть `mercy.exe`. Включає пакети звукових пакетів із коробки в `soundpacks/`.
---
### Основні моменти

- **Надлегкий ресурсний слід** — споживає ~25–40 МБ оперативної пам’яті порівняно зі 150–300+ МБ у Mechvibes на базі Electron.
- **Спеціальний механізм аудіо з низькою затримкою** — синтез і мікшування аудіо виконуються в ізольованому системному потоці, досягаючи часу відгуку менше 10 мс без затримок аудіо під час пакетів набору тексту зі швидкістю 100+ WPM.
- **Zero FPS Impact & Anti-Cheat Safe** — фіксує події введення через рідний Windows Raw Input замість блокування глобальних перехоплень клавіатури (`WH_KEYBOARD_LL`). Безпечно разом із Vanguard, VAC, Faceit і EAC.
- **Синхронізовані звукові сцени клавіатури та миші** — незалежне призначення звукового пакету, регулювання гучності та панорамування для натискань клавіш і клацань миші.
- **32-голосна поліфонія** — одночасне відтворення звуку без вискакування, обрізання чи пропадання голосу.
- **Універсальна сумісність із звуковим пакетом** — рідний аналізатор для застарілих форматів Mechvibes v1 і v2 (підтримує WAV, MP3, OGG і FLAC).
- **100% офлайн і конфіденційність** — відсутність аналітики, телеметричних трекерів (Aptabase видалено) і мережевих запитів.
---
### Керування звуковим пакетом

1. **Перетягніть і опустіть** — перетягніть будь-яку папку звукового пакету безпосередньо у вікно програми або перейдіть до **Налаштування → Звукові пакети → Імпортувати звуковий пакет**.
2. **Розділення каталогу** — звукові пакети користувача зчитуються з `%APPDATA%\Mechvibes\soundpacks` або локального каталогу `./soundpacks` поруч із двійковим файлом.
---
### FAQ

#### Чим Mercy відрізняється від оригінальних Mechvibes?
Mercy — це повний перепис Mechvibes на Rust. Важке середовище виконання Chromium/Node.js замінено власними системними викликами та високопродуктивними аудіопотоками, що зменшує споживання оперативної пам’яті на ~85% і усуває затримку введення.

#### Чи позначатиме програмне забезпечення для захисту від читів Mercy?
Ні. Mercy зчитує натискання клавіш за допомогою Windows Raw Input (`WM_INPUT`), того самого стандартного механізму введення, який використовують ігрові миші та клавіатури. Він ніколи не впроваджує код у цільові процеси, не перехоплює `WH_KEYBOARD_LL` і не торкається пам'яті гри.

#### Де зберігаються мої звукові пакети?
Звукові пакети знаходяться в `%APPDATA%\Mechvibes\soundpacks` або безпосередньо поряд із портативним виконуваним файлом у `soundpacks/`. Підтримуються структури папок Mechvibes v1 і v2, а також специфікації `config.json`.

#### Які глобальні ярлики доступні?
- `Ctrl + Alt + M` — глобальне перемикання/ввімкнення звуку.
- Кнопка «Закрити» згортається безпосередньо до системного лотка для ненав’язливої ​​роботи у фоновому режимі.
---
### Документація та архітектура

- [Системна архітектура] (docs/system-architecture.md) — модель потоку, вхідний конвеєр опитування та графік мікшування звуку
- [Конфігурація звукового пакету v2](docs/soundpack-config-v2.md) — Схема конфігурації та структура каталогу звукового пакету
- [Посібник із розгортання](docs/deployment-guide.md) — Інструкції з крос-компіляції та пакування
- [Інтерактивна діаграма](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Огляд візуальної архітектури
---
### Збірка з джерела

Необхідні умови: ланцюжок інструментів Rust (1.88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

Зкомпільований двійковий файл буде розміщено в `target/release/mercy.exe`.
---
### Ліцензія

[MIT](ЛІЦЕНЗІЯ) — спочатку засновано на [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) від Hai Nguyen.