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

### ইনস্টলেশন এবং ডাউনলোড

[**রিলিজ**](https://github.com/Arean-Max/Mechvibes-Mercy/releases) থেকে সর্বশেষ প্রি-বিল্ট বিল্ড নিন:

- **পোর্টেবল আর্কাইভ (`Mercy-v*.zip`)** — স্বতন্ত্র বান্ডিল। যেকোনো জায়গা থেকে বের করুন এবং `mercy.exe` চালু করুন। `সাউন্ডপ্যাক/`-এ বাক্সের বাইরে বান্ডিল করা সাউন্ডপ্যাকগুলি অন্তর্ভুক্ত করে।
---
### হাইলাইট

- **আল্ট্রালাইট রিসোর্স ফুটপ্রিন্ট** — ইলেক্ট্রন-ভিত্তিক মেচভাইবসে 150-300+ MB এর তুলনায় ~25–40 MB RAM খরচ করে৷
- **ডেডিকেটেড লো-ল্যাটেন্সি অডিও ইঞ্জিন** — অডিও সংশ্লেষণ এবং মিক্সিং একটি বিচ্ছিন্ন সিস্টেম থ্রেডে চালিত হয়, 100+ WPM টাইপিং বিস্ফোরণের সময় অডিও স্টাটার ছাড়াই সাব-10ms প্রতিক্রিয়া সময় অর্জন করে।
- **জিরো এফপিএস ইমপ্যাক্ট এবং অ্যান্টি-চিট সেফ** — গ্লোবাল কীবোর্ড হুক (`WH_KEYBOARD_LL`) ব্লক করার পরিবর্তে নেটিভ উইন্ডোজ র ইনপুটের মাধ্যমে ইনপুট ইভেন্ট ক্যাপচার করে। ভ্যানগার্ড, VAC, Faceit এবং EAC এর পাশাপাশি নিরাপদ।
- **সিঙ্ক্রোনাইজড কীবোর্ড এবং মাউস সাউন্ডস্কেপ** — স্বাধীন সাউন্ডপ্যাক অ্যাসাইনমেন্ট, ভলিউম নিয়ন্ত্রণ, এবং কীস্ট্রোক এবং মাউস ক্লিকের জন্য প্যানিং।
- **32-ভয়েস পলিফোনি** — পপিং, ক্লিপিং বা ভয়েস ড্রপআউট ছাড়াই সমসাময়িক সাউন্ড প্লেব্যাক।
- **ইউনিভার্সাল সাউন্ডপ্যাক সামঞ্জস্য** — উত্তরাধিকারী Mechvibes v1 এবং v2 ফর্ম্যাটের জন্য নেটিভ পার্সার (WAV, MP3, OGG, এবং FLAC সমর্থন করে)।
- **100% অফলাইন এবং ব্যক্তিগত** — জিরো অ্যানালিটিক্স, টেলিমেট্রি ট্র্যাকার (Aptabase সরানো হয়েছে), এবং নেটওয়ার্ক অনুরোধ।
---
### সাউন্ডপ্যাক ব্যবস্থাপনা

1. **ড্র্যাগ অ্যান্ড ড্রপ** — যেকোনো সাউন্ডপ্যাক ফোল্ডারকে সরাসরি অ্যাপ্লিকেশন উইন্ডোতে টেনে আনুন, অথবা **সেটিংস → সাউন্ডপ্যাক → সাউন্ডপ্যাক আমদানি করুন**-এ নেভিগেট করুন।
2. **ডিরেক্টরি রেজোলিউশন** — ব্যবহারকারীর সাউন্ডপ্যাকগুলি `%APPDATA%\Mechvibes\soundpacks` বা বাইনারির পাশের স্থানীয় `./soundpacks` ডিরেক্টরি থেকে পড়া হয়।
---
### প্রায়শই জিজ্ঞাসিত প্রশ্ন

#### করুণা কিভাবে আসল মেচভিব থেকে আলাদা?
করুণা হল Mechvibes-এর সম্পূর্ণ মরিচা পুনঃলিখন। ভারী Chromium/Node.js রানটাইম নেটিভ সিস্টেম কল এবং উচ্চ-পারফরম্যান্স অডিও থ্রেড দিয়ে প্রতিস্থাপিত হয়, RAM খরচ ~85% কমিয়ে দেয় এবং ইনপুট লেটেন্সি দূর করে।

#### অ্যান্টি-চিট সফটওয়্যার রহমতের পতাকা দেবে?
না। মার্সি উইন্ডোজ র ইনপুট (`WM_INPUT`) এর মাধ্যমে কীস্ট্রোক পড়ে, একই স্ট্যান্ডার্ড ইনপুট মেকানিজম গেমিং মাউস এবং কীবোর্ড ব্যবহার করে। এটি কখনই টার্গেট প্রসেসে কোড ইনজেক্ট করে না, হুক করে না `WH_KEYBOARD_LL`, বা গেম মেমরি স্পর্শ করে না।

#### আমার সাউন্ডপ্যাকগুলি কোথায় সংরক্ষণ করা হয়?
সাউন্ডপ্যাকগুলি `%APPDATA%\Mechvibes\soundpacks`-এ বা সরাসরি `soundpacks/`-এ বহনযোগ্য এক্সিকিউটেবলের পাশে থাকে। Mechvibes v1 এবং v2 ফোল্ডার স্ট্রাকচার এবং `config.json` স্পেসিফিকেশন সমর্থিত।

#### কোন গ্লোবাল শর্টকাট পাওয়া যায়?
- `Ctrl + Alt + M` — বিশ্বব্যাপী অডিও মিউট / আনমিউট টগল করুন।
- ক্লোজ বোতামটি অবাধ ব্যাকগ্রাউন্ড অপারেশনের জন্য সরাসরি সিস্টেম ট্রেতে ছোট করে।
---
### ডকুমেন্টেশন এবং আর্কিটেকচার

- [সিস্টেম আর্কিটেকচার](docs/system-architecture.md) — থ্রেড মডেল, ইনপুট পোলিং পাইপলাইন এবং অডিও মিক্সিং গ্রাফ
- [সাউন্ডপ্যাক কনফিগারেশন v2](docs/soundpack-config-v2.md) — কনফিগারেশন স্কিমা এবং সাউন্ডপ্যাক ডিরেক্টরি কাঠামো
- [ডিপ্লয়মেন্ট গাইড](docs/deployment-guide.md) — ক্রস-সংকলন এবং প্যাকেজিং নির্দেশাবলী
- [ইন্টারেক্টিভ ডায়াগ্রাম](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — ভিজ্যুয়াল আর্কিটেকচার ওভারভিউ
---
### উৎস থেকে তৈরি করুন

পূর্বশর্ত: মরিচা টুলচেন (1.88+)।

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

সংকলিত বাইনারিটি `target/release/mercy.exe`-এ স্থাপন করা হবে।
---
### লাইসেন্স

[MIT](লাইসেন্স) — মূলত Hai Nguyen-এর [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) এর উপর ভিত্তি করে।