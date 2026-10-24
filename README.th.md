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

### การติดตั้งและดาวน์โหลด

รับบิลด์ที่สร้างไว้ล่วงหน้าล่าสุดจาก [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Portable Archive (`Mercy-v*.zip`)** — บันเดิลแบบสแตนด์อโลน แยกได้ทุกที่แล้วเปิด `mercy.exe` รวมชุดเสียงประกอบที่มาพร้อมกล่องไว้ใน `ชุดเสียง/`
---
### ไฮไลท์

- **ปริมาณทรัพยากรที่เบามาก** — ใช้ RAM ประมาณ 25–40 MB เทียบกับ 150–300+ MB บน Mechvibes ที่ใช้อิเล็กตรอน
- **Dedicated Low-Latency Audio Engine** — การสังเคราะห์เสียงและมิกซ์เสียงทำงานบนเธรดระบบแบบแยก ทำให้ได้รับเวลาตอบสนองต่ำกว่า 10 มิลลิวินาที โดยที่เสียงไม่กระตุกระหว่างการพิมพ์ต่อเนื่องเกิน 100 WPM
- **ผลกระทบเป็นศูนย์ FPS และป้องกันการโกง** — บันทึกเหตุการณ์อินพุตผ่าน Windows Raw Input ดั้งเดิม แทนที่จะบล็อกการเชื่อมต่อแป้นพิมพ์ส่วนกลาง (`WH_KEYBOARD_LL`) ปลอดภัยควบคู่ไปกับ Vanguard, VAC, Faceit และ EAC
- **ภาพเสียงของคีย์บอร์ดและเมาส์ที่ซิงโครไนซ์** — การกำหนดชุดเสียงอิสระ การควบคุมระดับเสียง และการแพนสำหรับการกดแป้นพิมพ์และการคลิกเมาส์
- **32-Voice Polyphony** — เล่นเสียงพร้อมกันโดยไม่มีเสียงแตก ถูกตัด หรือเสียงขาดหาย
- **ความเข้ากันได้ของ Universal Soundpack** — ตัวแยกวิเคราะห์ดั้งเดิมสำหรับรูปแบบ Mechvibes v1 และ v2 รุ่นเก่า (รองรับ WAV, MP3, OGG และ FLAC)
- **ออฟไลน์และเป็นส่วนตัว 100%** — ไม่มีการวิเคราะห์ เครื่องมือติดตามการวัดและส่งข้อมูลทางไกล (ลบ Aptabase ออก) และคำขอเครือข่าย
---
### การจัดการชุดเสียง

1. **ลากและวาง** — ลากโฟลเดอร์ชุดเสียงลงในหน้าต่างแอปพลิเคชันโดยตรง หรือไปที่ **การตั้งค่า → ชุดเสียง → นำเข้าชุดเสียง**
2. **การแก้ไขไดเรกทอรี** — ชุดเสียงของผู้ใช้จะถูกอ่านจาก `%APPDATA%\Mechvibes\soundpacks` หรือไดเรกทอรี `./soundpacks` ในเครื่องถัดจากไบนารี
---
### คำถามที่พบบ่อย

#### Mercy แตกต่างจาก Mechvibes ดั้งเดิมอย่างไร?
Mercy เป็นการนำ Mechvibes ที่เขียนขึ้นใหม่โดย Rust รันไทม์ Chromium/Node.js จำนวนมากถูกแทนที่ด้วยการเรียกของระบบดั้งเดิมและเธรดเสียงประสิทธิภาพสูง ลดการใช้ RAM ลงประมาณ 85% และกำจัดเวลาแฝงของอินพุต

#### ซอฟต์แวร์ป้องกันการโกงจะติดธง Mercy หรือไม่
ไม่ Mercy อ่านการกดแป้นพิมพ์ผ่าน Windows Raw Input (`WM_INPUT`) ซึ่งเป็นกลไกการป้อนข้อมูลมาตรฐานแบบเดียวกับที่ใช้โดยเมาส์และคีย์บอร์ดสำหรับเล่นเกม มันไม่เคยฉีดโค้ดเข้าไปในกระบวนการเป้าหมาย เกี่ยว `WH_KEYBOARD_LL` หรือแตะต้องหน่วยความจำเกม

#### ชุดเสียงของฉันเก็บไว้ที่ไหน?
ชุดเสียงจะอยู่ใน `%APPDATA%\Mechvibes\soundpacks` หรือติดกับไฟล์ปฏิบัติการแบบพกพาใน `ชุดเสียง/` โดยตรง รองรับทั้งโครงสร้างโฟลเดอร์ Mechvibes v1 และ v2 และข้อกำหนด `config.json`

#### มีทางลัดส่วนกลางอะไรบ้าง?
- `Ctrl + Alt + M` — สลับปิดเสียง/เปิดเสียงทั่วโลก
- ปุ่มปิดย่อเล็กสุดไปที่ถาดระบบโดยตรงเพื่อการทำงานเบื้องหลังที่ไม่เกะกะ
---
### เอกสารและสถาปัตยกรรม

- [สถาปัตยกรรมระบบ](docs/system-architecture.md) — โมเดลเธรด ไปป์ไลน์การโพลอินพุต และกราฟการผสมเสียง
- [การกำหนดค่า Soundpack v2] (docs/soundpack-config-v2.md) — สคีมาการกำหนดค่าและโครงสร้างไดเร็กทอรี soundpack
- [คู่มือการปรับใช้](docs/deployment-guide.md) — คำแนะนำในการคอมไพล์ข้ามและบรรจุภัณฑ์
- [แผนภาพแบบโต้ตอบ](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — ภาพรวมสถาปัตยกรรมภาพ
---
### สร้างจากแหล่งที่มา

สิ่งที่จำเป็นต้องมี: Rust toolchain (1.88+)

___โค้ดบล็อก_0___

ไบนารีที่คอมไพล์แล้วจะถูกวางไว้ที่ `target/release/mercy.exe`
---
### ใบอนุญาต

[MIT](LICENSE) — เดิมทีอิงจาก [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) โดย Hai Nguyen