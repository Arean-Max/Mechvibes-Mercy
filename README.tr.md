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

### Kurulum ve İndirmeler

[**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases) adresinden en son önceden oluşturulmuş yapıyı edinin:

- **Taşınabilir Arşiv (`Mercy-v*.zip`)** — Bağımsız paket. Herhangi bir yere çıkartın ve 'mercy.exe'yi başlatın. 'Soundpacks/' içinde kutudan çıkan ses paketlerini içerir.
---
### Öne Çıkanlar

- **Ultra Hafif Kaynak Alanı** — Elektron tabanlı Mechvibe'lerdeki 150–300+ MB ile karşılaştırıldığında ~25–40 MB RAM tüketir.
- **Özel Düşük Gecikmeli Ses Motoru** — Ses sentezi ve miksajı yalıtılmış bir sistem iş parçacığı üzerinde yürütülür ve 100'den fazla WPM yazma patlamaları sırasında seste takılma olmadan 10 ms'nin altında yanıt süreleri elde edilir.
- **Sıfır FPS Etkisi ve Hileye Karşı Güvenli** — Genel klavye kancalarını (`WH_KEYBOARD_LL`) engellemek yerine, yerel Windows Ham Giriş yoluyla giriş olaylarını yakalar. Vanguard, VAC, Faceit ve EAC ile birlikte güvenli.
- **Senkronize Klavye ve Fare Ses Ortamları** — Bağımsız ses paketi ataması, ses seviyesi kontrolü ve tuş vuruşları ve fare tıklamaları için kaydırma.
- **32-Ses Çok Sesliliği** — Patlama, kesilme veya ses kesintisi olmadan eşzamanlı ses çalma.
- **Evrensel Ses Paketi Uyumluluğu** — Eski Mechvibes v1 ve v2 formatları için yerel ayrıştırıcı (WAV, MP3, OGG ve FLAC'ı destekler).
- **%100 Çevrimdışı ve Özel** — Sıfır analiz, telemetri izleyicileri (Aptabase kaldırıldı) ve ağ istekleri.
---
### Ses Paketi Yönetimi

1. **Sürükle ve Bırak** — Herhangi bir ses paketi klasörünü doğrudan uygulama penceresine sürükleyin veya **Ayarlar → Ses Paketleri → Ses Paketini İçe Aktar** seçeneğine gidin.
2. **Dizin Çözünürlüğü** — Kullanıcı ses paketleri, ikili dosyanın yanındaki "%APPDATA%\Mechvibes\soundpacks" veya yerel "./soundpacks" dizininden okunur.
---
### SSS

#### Mercy'nin orijinal Mechvibe'lerden farkı nedir?
Mercy, Mechvibes'in tamamen Rust tarafından yeniden yazılmış halidir. Ağır Chromium/Node.js çalışma zamanı, yerel sistem çağrıları ve yüksek performanslı ses iş parçacıklarıyla değiştirilerek RAM tüketimini ~%85 oranında azaltır ve giriş gecikmesini ortadan kaldırır.

#### Hile karşıtı yazılım Mercy'i işaretleyecek mi?
Hayır. Mercy, tuş vuruşlarını oyun fareleri ve klavyeleri tarafından kullanılan standart giriş mekanizmasının aynısı olan Windows Ham Girişi (`WM_INPUT`) aracılığıyla okur. Hiçbir zaman hedef süreçlere kod enjekte etmez, "WH_KEYBOARD_LL"yi bağlamaz veya oyun belleğine dokunmaz.

#### Ses paketlerim nerede saklanıyor?
Ses paketleri "%APPDATA%\Mechvibes\soundpacks" konumunda veya doğrudan "soundpacks/" içindeki taşınabilir yürütülebilir dosyanın yanında bulunur. Mechvibes v1 ve v2 klasör yapıları ve `config.json` spesifikasyonları desteklenmektedir.

#### Hangi genel kısayollar mevcut?
- `Ctrl + Alt + M` — Sesi genel olarak kapatma / açma arasında geçiş yapın.
- Kapatma düğmesi, arka planda dikkat çekmeyen işlemler için doğrudan sistem tepsisine küçültülür.
---
### Dokümantasyon ve Mimari

- [Sistem Mimarisi](docs/system-architecture.md) — İş parçacığı modeli, giriş yoklama hattı ve ses karıştırma grafiği
- [Soundpack Configuration v2](docs/soundpack-config-v2.md) — Yapılandırma şeması ve ses paketi dizin yapısı
- [Dağıtım Kılavuzu](docs/deployment-guide.md) — Çapraz derleme ve paketleme talimatları
- [Etkileşimli Diyagram](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Görsel mimariye genel bakış
---
### Kaynaktan derle

Önkoşullar: Rust takım zinciri (1.88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

Derlenen ikili dosya 'target/release/mercy.exe' konumuna yerleştirilecektir.
---
### Lisans

[MIT](LİSANS) — Orijinal olarak Hai Nguyen tarafından yazılan [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) temel alınmıştır.