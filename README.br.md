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

### Instalação e downloads

Obtenha a versão pré-construída mais recente em [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Arquivo Portátil (`Mercy-v*.zip`)** — Pacote independente. Extraia em qualquer lugar e execute `mercy.exe`. Inclui pacotes de som prontos para uso em `soundpacks/`.
---
### Destaques

- **Pegada de recursos ultraleve** — Consome aproximadamente 25–40 MB de RAM em comparação com 150–300+ MB em Mechvibes baseados em Electron.
- **Mecanismo de áudio dedicado de baixa latência** — A síntese e mixagem de áudio são executadas em um thread de sistema isolado, alcançando tempos de resposta inferiores a 10 ms sem interrupções de áudio durante rajadas de digitação de mais de 100 WPM.
- **Impacto zero de FPS e segurança anti-cheat** — Captura eventos de entrada por meio do Windows Raw Input nativo em vez de bloquear ganchos de teclado globais (`WH_KEYBOARD_LL`). Seguro ao lado de Vanguard, VAC, Faceit e EAC.
- **Paisagens sonoras de teclado e mouse sincronizadas** — Atribuição independente de pacote de som, controle de volume e panorâmica para pressionamentos de teclas e cliques do mouse.
- **Polifonia de 32 vozes** — Reprodução de som simultânea sem estalos, cortes ou interrupções na voz.
- **Compatibilidade Universal Soundpack** — Analisador nativo para formatos legados Mechvibes v1 e v2 (suporta WAV, MP3, OGG e FLAC).
- **100% off-line e privado** — Zero análises, rastreadores de telemetria (Aptabase removido) e solicitações de rede.
---
### Gerenciamento de pacote de som

1. **Arrastar e Soltar** — Arraste qualquer pasta de soundpack diretamente para a janela do aplicativo ou navegue até **Configurações → Soundpacks → Importar Soundpack**.
2. **Resolução de diretório** — Os pacotes de som do usuário são lidos em `%APPDATA%\Mechvibes\soundpacks` ou no diretório local `./soundpacks` próximo ao binário.
---
### Perguntas frequentes

#### Qual a diferença entre Mercy e Mechvibes original?
Mercy é uma reescrita completa de Mechvibes em Rust. O pesado tempo de execução do Chromium/Node.js foi substituído por chamadas de sistema nativas e threads de áudio de alto desempenho, reduzindo o consumo de RAM em aproximadamente 85% e eliminando a latência de entrada.

#### O software anti-cheat sinalizará Mercy?
O Mercy lê as teclas digitadas por meio do Windows Raw Input (`WM_INPUT`), o mesmo mecanismo de entrada padrão usado por mouses e teclados para jogos. Ele nunca injeta código em processos de destino, conecta `WH_KEYBOARD_LL` ou toca na memória do jogo.

#### Onde meus pacotes de som são armazenados?
Soundpacks residem em `%APPDATA%\Mechvibes\soundpacks` ou diretamente ao lado do executável portátil em `soundpacks/`. Ambas as estruturas de pastas Mechvibes v1 e v2 e especificações `config.json` são suportadas.

#### Quais atalhos globais estão disponíveis?
- `Ctrl + Alt + M` — Alternar entre ativar e desativar o áudio globalmente.
- O botão Fechar minimiza diretamente na bandeja do sistema para operação discreta em segundo plano.
---
### Documentação e Arquitetura

- [Arquitetura do sistema](docs/system-architecture.md) — Modelo de thread, pipeline de pesquisa de entrada e gráfico de mixagem de áudio
- [Configuração do Soundpack v2](docs/soundpack-config-v2.md) — Esquema de configuração e estrutura de diretórios do soundpack
- [Guia de implantação](docs/deployment-guide.md) — Instruções de compilação cruzada e empacotamento
- [Diagrama interativo](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Visão geral da arquitetura visual
---
### Construir a partir da fonte

Pré-requisitos: Conjunto de ferramentas Rust (1.88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

O binário compilado será colocado em `target/release/mercy.exe`.
---
### Licença

[MIT](LICENSE) — Originalmente baseado em [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) de Hai Nguyen.