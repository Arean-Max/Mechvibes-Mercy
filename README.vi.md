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

### Cài đặt và tải xuống

Lấy bản dựng sẵn mới nhất từ [**Release**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Lưu trữ di động (`Mercy-v*.zip`)** — Gói độc lập. Giải nén ở bất cứ đâu và khởi chạy `mercy.exe`. Bao gồm các gói âm thanh đi kèm trong `soundpacks/`.
---
### Điểm nổi bật

- **Dấu chân tài nguyên siêu nhẹ** — Tiêu thụ ~25–40 MB RAM so với 150–300+ MB trên Mechvibes dựa trên Electron.
- **Công cụ âm thanh có độ trễ thấp chuyên dụng** — Quá trình tổng hợp và trộn âm thanh chạy trên một luồng hệ thống biệt lập, đạt thời gian phản hồi dưới 10 mili giây mà không bị giật âm thanh trong hơn 100 lần gõ WPM.
- **Không tác động đến FPS & An toàn chống gian lận** — Ghi lại các sự kiện đầu vào thông qua Windows Raw input gốc thay vì chặn các móc bàn phím chung (`WH_KEYBOARD_LL`). An toàn cùng với Vanguard, VAC, Faceit và EAC.
- **Âm thanh bàn phím và chuột được đồng bộ hóa** — Việc gán gói âm thanh độc lập, điều khiển âm lượng và xoay cho các lần nhấn phím và nhấp chuột.
- **Đa âm 32 giọng** — Phát lại âm thanh đồng thời mà không bị ngắt quãng, cắt bớt hoặc mất giọng.
- **Khả năng tương thích gói âm thanh chung** — Trình phân tích cú pháp gốc cho các định dạng Mechvibes v1 và v2 cũ (hỗ trợ WAV, MP3, OGG và FLAC).
- **100% ngoại tuyến & riêng tư** — Không cần phân tích, trình theo dõi đo từ xa (đã xóa Aptabase) và yêu cầu mạng.
---
### Quản lý gói âm thanh

1. **Kéo và thả** — Kéo bất kỳ thư mục soundpack nào trực tiếp vào cửa sổ ứng dụng hoặc điều hướng đến **Cài đặt → Soundpacks → Nhập Soundpack**.
2. **Độ phân giải thư mục** — Gói âm thanh của người dùng được đọc từ `%APPDATA%\Mechvibes\soundpacks` hoặc thư mục `./soundpacks` cục bộ bên cạnh tệp nhị phân.
---
###Câu hỏi thường gặp

#### Mercy khác với Mechvibe ban đầu như thế nào?
Mercy là một bản viết lại hoàn chỉnh của Rust về Mechvibes. Thời gian chạy Chrom/Node.js nặng nề được thay thế bằng lệnh gọi hệ thống gốc và luồng âm thanh hiệu suất cao, giảm mức tiêu thụ RAM xuống ~85% và loại bỏ độ trễ đầu vào.

####Phần mềm chống gian lận có gắn cờ Mercy không?
Không. Mercy đọc các lần gõ phím thông qua Windows Raw input (`WM_INPUT`), cơ chế nhập tiêu chuẩn tương tự được sử dụng bởi chuột và bàn phím chơi game. Nó không bao giờ chèn mã vào các tiến trình đích, móc `WH_KEYBOARD_LL` hoặc chạm vào bộ nhớ trò chơi.

#### Gói âm thanh của tôi được lưu trữ ở đâu?
Gói âm thanh nằm trong `%APPDATA%\Mechvibes\soundpacks` hoặc trực tiếp bên cạnh tệp thực thi di động trong `soundpacks/`. Cả cấu trúc thư mục Mechvibes v1 và v2 cũng như thông số kỹ thuật `config.json` đều được hỗ trợ.

#### Có những phím tắt chung nào?
- `Ctrl + Alt + M` — Chuyển đổi tắt tiếng/bật tiếng âm thanh trên toàn cầu.
- Nút đóng thu nhỏ trực tiếp xuống khay hệ thống để hoạt động ở chế độ nền không phô trương.
---
### Tài liệu & Kiến trúc

- [System Architecture](docs/system-architecture.md) — Mô hình luồng, đường dẫn thăm dò đầu vào và biểu đồ trộn âm thanh
- [Soundpack Configuration v2](docs/soundpack-config-v2.md) — Lược đồ cấu hình và cấu trúc thư mục soundpack
- [Hướng dẫn triển khai](docs/deployment-guide.md) — Hướng dẫn biên dịch chéo và đóng gói
- [Sơ đồ tương tác](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Tổng quan về kiến trúc trực quan
---
### Xây dựng từ nguồn

Điều kiện tiên quyết: Chuỗi công cụ Rust (1.88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

Tệp nhị phân đã biên dịch sẽ được đặt tại `target/release/mercy.exe`.
---
### Giấy phép

[MIT](LICENSE) — Ban đầu dựa trên [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) của Hải Nguyên.