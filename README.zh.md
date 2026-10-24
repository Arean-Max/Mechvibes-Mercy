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

### 安装和下载

从 [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases) 获取最新的预构建版本：

- **便携式存档 (`Mercy-v*.zip`)** — 独立捆绑包。解压到任意位置并启动 `mercy.exe`。在“soundpacks/”中包含开箱即用的捆绑声音包。
---
### 亮点

- **超轻资源占用** — 消耗约 25–40 MB RAM，而基于 Electron 的 Mechvibes 则消耗 150–300+ MB RAM。
- **专用低延迟音频引擎** — 音频合成和混合在独立的系统线程上运行，在 100+ WPM 的打字突发期间实现低于 10 毫秒的响应时间，而不会出现音频卡顿。
- **零 FPS 影响和防作弊安全** - 通过本机 Windows 原始输入捕获输入事件，而不是阻止全局键盘挂钩 (`WH_KEYBOARD_LL`)。与 Vanguard、VAC、Faceit 和 EAC 并驾齐驱。
- **同步键盘和鼠标音景** — 独立的音包分配、音量控制和平移按键和鼠标点击。
- **32 复音** — 并发声音播放，不会出现爆音、削波或声音丢失的情况。
- **通用声音包兼容性** — 适用于旧版 Mechvibes v1 和 v2 格式的本机解析器（支持 WAV、MP3、OGG 和 FLAC）。
- **100% 离线和私密** — 零分析、遥测跟踪器（已删除 Aptabase）和网络请求。
---
### 声音包管理

1. **拖放** — 将任何声音包文件夹直接拖到应用程序窗口中，或导航到 **设置 → 声音包 → 导入声音包**。
2. **目录解析** — 从“%APPDATA%\Mechvibes\soundpacks”或二进制文件旁边的本地“./soundpacks”目录读取用户声音包。
---
### 常见问题解答

#### Mercy 与原始 Mechvibes 有何不同？
Mercy 是 Mechvibes 的完全 Rust 重写。繁重的 Chromium/Node.js 运行时被本机系统调用和高性能音频线程取代，将 RAM 消耗削减了约 85%，并消除了输入延迟。

#### 反作弊软件会标记 Mercy 吗？
Mercy 通过 Windows 原始输入 (“WM_INPUT”) 读取击键，这与游戏鼠标和键盘使用的标准输入机制相同。它永远不会将代码注入目标进程、挂钩“WH_KEYBOARD_LL”或触及游戏内存。

#### 我的声音包存储在哪里？
Soundpack 位于 `%APPDATA%\Mechvibes\soundpacks` 中，或者直接与可移植可执行文件一起位于 `soundpacks/` 中。支持 Mechvibes v1 和 v2 文件夹结构以及 `config.json` 规范。

#### 有哪些全局快捷键可用？
- `Ctrl + Alt + M` — 全局切换静音/取消静音音频。
- 关闭按钮直接最小化到系统托盘，以实现不显眼的后台操作。
---
### 文档和架构

- [系统架构](docs/system-architecture.md) — 线程模型、输入轮询管道和音频混合图
- [Soundpack 配置 v2](docs/soundpack-config-v2.md) — 配置架构和 soundpack 目录结构
- [部署指南](docs/deployment-guide.md) — 交叉编译和打包说明
- [交互式图](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — 视觉架构概述
---
### 从源代码构建

先决条件：Rust 工具链 (1.88+)。

___代码块_0___

编译后的二进制文件将放置在“target/release/mercy.exe”中。
---
### 许可证

[MIT]（许可证）——最初基于 Hai Nguyen 的 [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx)。