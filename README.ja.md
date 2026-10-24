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

### インストールとダウンロード

[**リリース**](https://github.com/Arean-Max/Mechvibes-Mercy/releases) から最新のビルド済みビルドを取得します。

- **ポータブル アーカイブ (`Mercy-v*.zip`)** — スタンドアロン バンドル。任意の場所に解凍して「mercy.exe」を起動します。 `soundpacks/` には、すぐに使えるバンドルされたサウンドパックが含まれています。
---
### ハイライト

- **超軽量のリソース フットプリント** — Electron ベースの Mechvibes では 150 ～ 300 MB 以上の RAM を消費しますが、最大 25 ～ 40 MB の RAM を消費します。
- **専用の低遅延オーディオ エンジン** - オーディオ合成とミキシングは独立したシステム スレッドで実行され、100 以上の WPM タイピング バースト中に音声が途切れることなく 10 ミリ秒未満の応答時間を実現します。
- **FPS 影響ゼロおよびアンチチートセーフ** — グローバル キーボード フック (「WH_KEYBOARD_LL」) をブロックするのではなく、ネイティブ Windows Raw 入力を介して入力イベントをキャプチャします。 Vanguard、VAC、Faceit、EAC と並んで安全です。
- **同期されたキーボードとマウスのサウンドスケープ** — 独立したサウンドパックの割り当て、ボリューム コントロール、キーストロークとマウス クリックのパン。
- **32 音声ポリフォニー** — ポップ、クリッピング、または音声のドロップアウトのないサウンドの同時再生。
- **ユニバーサル サウンドパック互換性** — レガシー Mechvibes v1 および v2 形式のネイティブ パーサー (WAV、MP3、OGG、および FLAC をサポート)。
- **100% オフラインおよびプライベート** — 分析、テレメトリ トラッカー (Aptabase は削除)、およびネットワーク リクエストはゼロです。
---
### サウンドパックの管理

1. **ドラッグ アンド ドロップ** — サウンドパック フォルダーをアプリケーション ウィンドウに直接ドラッグするか、**設定 → サウンドパック → サウンドパックのインポート** に移動します。
2. **ディレクトリ解決** — ユーザー サウンドパックは、`%APPDATA%\Mechvibes\soundpacks` またはバイナリの隣にあるローカルの `./soundpacks` ディレクトリから読み取られます。
---
### よくある質問

#### Mercy はオリジナルの Mechvibes とどう違うのですか?
Mercy は Mechvibes を Rust で完全に書き直したものです。重い Chromium/Node.js ランタイムはネイティブ システム コールと高性能オーディオ スレッドに置き換えられ、RAM 消費量が最大 85% 削減され、入力遅延が解消されます。

#### アンチチート ソフトウェアは Mercy にフラグを立てますか?
いいえ、Mercy は、ゲーム用マウスやキーボードで使用されるのと同じ標準入力メカニズムである Windows Raw 入力 (「WM_INPUT」) 経由でキーストロークを読み取ります。ターゲット プロセスにコードを挿入したり、「WH_KEYBOARD_LL」をフックしたり、ゲーム メモリにアクセスしたりすることはありません。

#### サウンドパックはどこに保存されますか?
サウンドパックは `%APPDATA%\Mechvibes\soundpacks` に存在するか、`soundpacks/` 内のポータブル実行可能ファイルのすぐ隣に存在します。 Mechvibes v1 と v2 の両方のフォルダー構造と `config.json` 仕様がサポートされています。

#### どのようなグローバル ショートカットが利用可能ですか?
- `Ctrl + Alt + M` — オーディオのミュート/ミュート解除をグローバルに切り替えます。
- 閉じるボタンは、目立たないバックグラウンド操作のためにシステム トレイに直接最小化されます。
---
### ドキュメントとアーキテクチャ

- [システム アーキテクチャ](docs/system-architecture.md) — スレッド モデル、入力ポーリング パイプライン、およびオーディオ ミキシング グラフ
- [Soundpack Configuration v2](docs/soundpack-config-v2.md) — 構成スキーマとサウンドパックのディレクトリ構造
- [導入ガイド](docs/deployment-guide.md) — クロスコンパイルおよびパッケージ化の手順
- [インタラクティブダイアグラム](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — ビジュアルアーキテクチャの概要
---
### ソースからビルドする

前提条件: Rust ツールチェーン (1.88 以降)。

___コードブロック_0___

コンパイルされたバイナリは「target/release/mercy.exe」に配置されます。
---
### ライセンス

[MIT](ライセンス) — 元は Hai Nguyen による [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) に基づいています。