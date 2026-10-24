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

### التثبيت والتنزيلات

احصل على أحدث إصدار تم إنشاؤه مسبقًا من [**الإصدارات**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **الأرشيف المحمول (`Mercy-v*.zip`)** — حزمة مستقلة. قم بالاستخراج في أي مكان ثم قم بتشغيل `mercy.exe`. يتضمن حزم صوتية مجمعة جاهزة في `حزم الصوت/`.
---
### أبرز

- **بصمة موارد خفيفة جدًا** — تستهلك حوالي 25–40 ميجابايت من ذاكرة الوصول العشوائي مقارنة بـ 150–300+ ميجابايت على Mechvibes المستندة إلى الإلكترون.
- **محرك صوتي مخصص منخفض زمن الاستجابة** — يتم تشغيل عملية التوليف الصوتي والمزج على خيط نظام معزول، مما يحقق أوقات استجابة أقل من 10 مللي ثانية دون تلعثم في الصوت أثناء دفعات الكتابة التي تزيد عن 100 WPM.
- ** Zero FPS Impact & Anti-Cheat Safe ** - يلتقط أحداث الإدخال عبر Windows Raw Input الأصلي بدلاً من حظر خطافات لوحة المفاتيح العامة (`WH_KEYBOARD_LL`). آمن إلى جانب Vanguard وVAC وFaceit وEAC.
- **مقاطع صوتية متزامنة للوحة المفاتيح والماوس** — تعيين حزمة صوتية مستقلة، والتحكم في مستوى الصوت، والتحريك لضغطات المفاتيح ونقرات الماوس.
- **32-تعدد الأصوات الصوتية** — تشغيل الصوت المتزامن دون فرقع أو قطع أو انقطاع الصوت.
- **التوافق العالمي مع Soundpack** — محلل أصلي لتنسيقات Mechvibes v1 وv2 القديمة (يدعم WAV، وMP3، وOGG، وFLAC).
- **خاص وغير متصل بالإنترنت بنسبة 100%** — لا يوجد أي تحليلات أو أدوات تتبع القياس عن بعد (تمت إزالة Aptabase) أو طلبات الشبكة.
---
### إدارة Soundpack

1. **السحب والإفلات** — اسحب أي مجلد Soundpack مباشرة إلى نافذة التطبيق، أو انتقل إلى **Settings → Soundpacks → Import Soundpack**.
2. **تحليل الدليل** — تتم قراءة حزم صوت المستخدم من `%APPDATA%\Mechvibes\soundpacks` أو الدليل `./soundpacks` المحلي بجوار الملف الثنائي.
---
### الأسئلة الشائعة

#### كيف تختلف Mercy عن Mechvibes الأصلية؟
Mercy هي إعادة كتابة Rust كاملة لـ Mechvibes. يتم استبدال وقت تشغيل Chromium/Node.js الثقيل باستدعاءات النظام الأصلية وسلاسل الصوت عالية الأداء، مما يقلل من استهلاك ذاكرة الوصول العشوائي بنسبة 85% تقريبًا ويزيل زمن انتقال الإدخال.

#### هل ستشير برامج مكافحة الغش إلى Mercy؟
لا، يقرأ Mercy ضغطات المفاتيح عبر Windows Raw Input (`WM_INPUT`)، وهي نفس آلية الإدخال القياسية التي تستخدمها أجهزة الماوس ولوحات المفاتيح المخصصة للألعاب. لا يقوم أبدًا بإدخال التعليمات البرمجية في العمليات المستهدفة، أو ربط `WH_KEYBOARD_LL`، أو لمس ذاكرة اللعبة.

#### أين يتم تخزين حزم الصوت الخاصة بي؟
توجد حزم الصوت في `%APPDATA%\Mechvibes\soundpacks` أو مباشرة بجانب الملف القابل للتنفيذ المحمول في `soundpacks/`. يتم دعم كل من بنيات مجلدات Mechvibes v1 وv2 ومواصفات `config.json`.

#### ما هي الاختصارات العامة المتوفرة؟
- `Ctrl + Alt + M` - تبديل كتم الصوت / إلغاء كتم الصوت على مستوى العالم.
- يتم تصغير زر الإغلاق مباشرة إلى علبة النظام لتشغيل الخلفية بشكل غير مزعج.
---
### التوثيق والهندسة المعمارية

- [بنية النظام](docs/system-architecture.md) - نموذج الموضوع، وخط أنابيب استقصاء المدخلات، ورسم بياني لخلط الصوت
- [Soundpack Configuration v2](docs/soundpack-config-v2.md) — مخطط التكوين وبنية دليل soundpack
- [دليل النشر](docs/deployment-guide.md) - تعليمات التجميع والتعبئة المشتركة
- [المخطط التفاعلي](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) - نظرة عامة على البنية المرئية
---
### البناء من المصدر

المتطلبات الأساسية: سلسلة أدوات الصدأ (1.88+).

___كود بلوك_0___

سيتم وضع الملف الثنائي المترجم في "target/release/mercy.exe".
---
### الترخيص

[MIT] (الترخيص) - يعتمد في الأصل على [mechvibes-dx] (https://github.com/hainguyents13/mechvibes-dx) بواسطة Hai Nguyen.