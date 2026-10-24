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

### Instalación y descargas

Obtenga la última versión prediseñadas de [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Archivo portátil (`Mercy-v*.zip`)** — Paquete independiente. Extraiga en cualquier lugar y ejecute `mercy.exe`. Incluye paquetes de sonido incluidos listos para usar en `soundpacks/`.
---
### Destacados

- **Huella de recursos ultraligera**: consume entre 25 y 40 MB de RAM en comparación con los 150 a 300+ MB de Mechvibes basados en electrones.
- **Motor de audio dedicado de baja latencia**: la síntesis y mezcla de audio se ejecutan en un hilo de sistema aislado, logrando tiempos de respuesta inferiores a 10 ms sin interrupciones de audio durante ráfagas de escritura de más de 100 palabras por minuto.
- **Impacto cero FPS y seguridad anti-trampas**: captura eventos de entrada a través de la entrada sin formato nativa de Windows en lugar de bloquear los ganchos de teclado globales (`WH_KEYBOARD_LL`). Seguro junto con Vanguard, VAC, Faceit y EAC.
- **Paisajes sonoros de teclado y mouse sincronizados**: asignación de paquete de sonido independiente, control de volumen y panorámica para pulsaciones de teclas y clics del mouse.
- **Polifonía de 32 voces**: reproducción de sonido simultánea sin estallidos, recortes ni interrupciones de voz.
- **Compatibilidad universal de Soundpack**: analizador nativo para formatos heredados de Mechvibes v1 y v2 (admite WAV, MP3, OGG y FLAC).
- **100 % sin conexión y privado**: cero análisis, rastreadores de telemetría (se eliminó Aptabase) y solicitudes de red.
---
### Gestión de paquetes de sonido

1. **Arrastrar y soltar**: arrastre cualquier carpeta de paquete de sonido directamente a la ventana de la aplicación o navegue hasta **Configuración → Paquetes de sonido → Importar paquete de sonido**.
2. **Resolución de directorio**: los paquetes de sonido del usuario se leen desde `%APPDATA%\Mechvibes\soundpacks` o el directorio local `./soundpacks` al lado del binario.
---
### Preguntas frecuentes

#### ¿En qué se diferencia Mercy de los Mechvibes originales?
Mercy es una reescritura completa de Rust de Mechvibes. El pesado tiempo de ejecución de Chromium/Node.js se reemplaza con llamadas nativas al sistema y subprocesos de audio de alto rendimiento, lo que reduce el consumo de RAM en aproximadamente un 85 % y elimina la latencia de entrada.

#### ¿El software antitrampas marcará a Mercy?
No. Mercy lee las pulsaciones de teclas a través de Windows Raw Input (`WM_INPUT`), el mismo mecanismo de entrada estándar utilizado por los ratones y teclados para juegos. Nunca inyecta código en los procesos de destino, engancha `WH_KEYBOARD_LL` ni toca la memoria del juego.

#### ¿Dónde se almacenan mis paquetes de sonido?
Los paquetes de sonido residen en `%APPDATA%\Mechvibes\soundpacks` o directamente junto al ejecutable portátil en `soundpacks/`. Se admiten las estructuras de carpetas Mechvibes v1 y v2 y las especificaciones `config.json`.

#### ¿Qué atajos globales están disponibles?
- `Ctrl + Alt + M`: alterna silenciar/activar audio globalmente.
- El botón Cerrar minimiza directamente a la bandeja del sistema para una operación en segundo plano discreta.
---
### Documentación y arquitectura

- [Arquitectura del sistema] (docs/system-architecture.md): modelo de subproceso, canal de sondeo de entrada y gráfico de mezcla de audio
- [Configuración de Soundpack v2](docs/soundpack-config-v2.md) — Esquema de configuración y estructura de directorios de soundpack
- [Guía de implementación] (docs/deployment-guide.md): instrucciones de compilación cruzada y empaquetado
- [Diagrama interactivo](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Descripción general de la arquitectura visual
---
### Construir desde la fuente

Requisitos previos: cadena de herramientas Rust (1.88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

El binario compilado se colocará en `target/release/mercy.exe`.
---
### Licencia

[MIT](LICENCIA) — Basado originalmente en [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) de Hai Nguyen.