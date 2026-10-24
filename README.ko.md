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

### 설치 및 다운로드

[**릴리스**](https://github.com/Arean-Max/Mechvibes-Mercy/releases)에서 사전 빌드된 최신 빌드를 다운로드하세요.

- **이동식 아카이브(`Mercy-v*.zip`)** — 독립형 번들. 아무데나 압축을 풀고 `mercy.exe`를 실행하세요. `soundpacks/`에 기본 제공되는 번들 사운드팩이 포함되어 있습니다.
---
### 하이라이트

- **초경량 리소스 공간** — Electron 기반 Mechvibe의 150~300MB RAM에 비해 ~25~40MB RAM을 소비합니다.
- **전용 저지연 오디오 엔진** — 오디오 합성 및 믹싱은 격리된 시스템 스레드에서 실행되어 100WPM 이상의 타이핑 버스트 중에 오디오 끊김 없이 10ms 미만의 응답 시간을 달성합니다.
- **제로 FPS 영향 및 치트 방지 안전** — 전역 키보드 후크(`WH_KEYBOARD_LL`)를 차단하는 대신 기본 Windows 원시 입력을 통해 입력 이벤트를 캡처합니다. Vanguard, VAC, Faceit 및 EAC와 함께 사용하면 안전합니다.
- **동기화된 키보드 및 마우스 사운드스케이프** — 독립적인 사운드팩 할당, 볼륨 제어, 키 입력 및 마우스 클릭 패닝.
- **32-Voice Polyphony** — 팝핑, 클리핑 또는 음성 끊김 없이 사운드를 동시에 재생합니다.
- **범용 사운드팩 호환성** — 레거시 Mechvibes v1 및 v2 형식에 대한 기본 파서(WAV, MP3, OGG 및 FLAC 지원)
- **100% 오프라인 및 비공개** — 분석, 원격 측정 추적기(Aptabase 제거) 및 네트워크 요청이 없습니다.
---
### 사운드팩 관리

1. **드래그 앤 드롭** — 사운드팩 폴더를 응용 프로그램 창으로 직접 드래그하거나 **설정 → 사운드팩 → 사운드팩 가져오기**로 이동합니다.
2. **디렉터리 해상도** — 사용자 사운드팩은 `%APPDATA%\Mechvibes\soundpacks` 또는 바이너리 옆의 로컬 `./soundpacks` 디렉토리에서 읽습니다.
---
### FAQ

#### Mercy는 기존 Mechvibe와 어떻게 다릅니까?
Mercy는 Mechvibes를 완전히 Rust로 재작성한 것입니다. 무거운 Chromium/Node.js 런타임은 기본 시스템 호출 및 고성능 오디오 스레드로 대체되어 RAM 소비를 최대 85%까지 줄이고 입력 대기 시간을 제거합니다.

#### 치트 방지 소프트웨어가 Mercy를 표시합니까?
아니요. Mercy는 게임용 마우스 및 키보드에 사용되는 것과 동일한 표준 입력 메커니즘인 Windows 원시 입력(`WM_INPUT`)을 통해 키 입력을 읽습니다. 대상 프로세스에 코드를 삽입하거나, 'WH_KEYBOARD_LL'을 연결하거나, 게임 메모리를 건드리지 않습니다.

#### 내 사운드팩은 어디에 저장되어 있나요?
사운드팩은 `%APPDATA%\Mechvibes\soundpacks`에 있거나 `soundpacks/`의 휴대용 실행 파일 바로 옆에 있습니다. Mechvibes v1 및 v2 폴더 구조와 `config.json` 사양이 모두 지원됩니다.

#### 어떤 전역 단축키를 사용할 수 있나요?
- `Ctrl + Alt + M` — 전역적으로 오디오 음소거/음소거 해제를 전환합니다.
- 닫기 버튼은 눈에 띄지 않는 백그라운드 작업을 위해 시스템 트레이로 직접 최소화됩니다.
---
### 문서 및 아키텍처

- [시스템 아키텍처](docs/system-architecture.md) — 스레드 모델, 입력 폴링 파이프라인 및 오디오 믹싱 그래프
- [Soundpack Configuration v2](docs/soundpack-config-v2.md) — 구성 스키마 및 사운드팩 디렉터리 구조
- [배포 가이드](docs/deployment-guide.md) — 크로스 컴파일 및 패키징 지침
- [인터랙티브 다이어그램](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — 시각적 아키텍처 개요
---
### 소스에서 빌드

전제조건: Rust 툴체인(1.88+).

____코드블록_0___

컴파일된 바이너리는 `target/release/mercy.exe`에 배치됩니다.
---
### 라이센스

[MIT](LICENSE) — 원래 Hai Nguyen의 [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx)를 기반으로 했습니다.