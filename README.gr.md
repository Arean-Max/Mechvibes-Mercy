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

### Εγκατάσταση & Λήψεις

Αποκτήστε την πιο πρόσφατη προκατασκευασμένη έκδοση από το [**Releases**](https://github.com/Arean-Max/Mechvibes-Mercy/releases):

- **Φορητό αρχείο (`Mercy-v*.zip`)** — Αυτόνομο πακέτο. Κάντε εξαγωγή οπουδήποτε και εκκινήστε το "mercy.exe". Περιλαμβάνει συσκευασμένα πακέτα ήχου εκτός συσκευασίας στα `soundpacks/`.
---
### Χαρακτηριστικά

- **Ultralight Resource Footprint** — Καταναλώνει ~25–40 MB RAM σε σύγκριση με 150–300+ MB σε Mechvibes που βασίζονται σε Ηλεκτρονικά.
- **Dedicated Low-Latency Audio Engine** — Η σύνθεση και η μίξη ήχου εκτελούνται σε ένα απομονωμένο νήμα συστήματος, επιτυγχάνοντας χρόνους απόκρισης κάτω των 10 ms χωρίς τραύλισμα ήχου κατά τη διάρκεια ριπών πληκτρολόγησης 100+ WPM.
- **Μηδενικό FPS Impact & Anti-Cheat Safe** — Καταγράφει συμβάντα εισόδου μέσω της εγγενούς εισόδου Raw των Windows αντί να αποκλείει τα καθολικά άγκιστρα πληκτρολογίου (`WH_KEYBOARD_LL`). Ασφαλές μαζί με τους Vanguard, VAC, Faceit και EAC.
- **Συγχρονισμένα τοπία ήχου πληκτρολογίου και ποντικιού** — Ανεξάρτητη αντιστοίχιση ηχητικού πακέτου, έλεγχος έντασης και μετατόπιση για πατήματα πλήκτρων και κλικ του ποντικιού.
- **32-Voice Polyphony** — Ταυτόχρονη αναπαραγωγή ήχου χωρίς σκάσιμο, απόκομμα ή διακοπή φωνής.
- **Συμβατότητα Universal Soundpack** — Εγγενής αναλυτής για παλαιού τύπου μορφές Mechvibes v1 και v2 (υποστηρίζει WAV, MP3, OGG και FLAC).
- **100% Εκτός σύνδεσης & Ιδιωτικό** — Μηδενικά αναλυτικά στοιχεία, ιχνηλάτες τηλεμετρίας (καταργήθηκε το Aptabas) και αιτήματα δικτύου.
---
### Διαχείριση Soundpack

1. **Drag & Drop** — Σύρετε οποιονδήποτε φάκελο soundpack απευθείας στο παράθυρο της εφαρμογής ή μεταβείτε στο **Settings → Soundpacks → Import Soundpack**.
2. **Ανάλυση καταλόγου** — Τα πακέτα ήχου χρήστη διαβάζονται από το «%APPDATA%\Mechvibes\soundpacks» ή τον τοπικό κατάλογο «./soundpacks» δίπλα στο δυαδικό.
---
### Συχνές ερωτήσεις

#### Σε τι διαφέρει το Mercy από το αρχικό Mechvibes;
Το Mercy είναι μια πλήρης επανεγγραφή Rust του Mechvibes. Ο μεγάλος χρόνος εκτέλεσης του Chromium/Node.js αντικαθίσταται με εγγενείς κλήσεις συστήματος και νήματα ήχου υψηλής απόδοσης, μειώνοντας την κατανάλωση RAM κατά ~85% και εξαλείφοντας τον λανθάνοντα χρόνο εισόδου.

#### Το λογισμικό anti-cheat θα επισημάνει το Mercy;
Όχι. Το Mercy διαβάζει πατήματα πλήκτρων μέσω εισόδου Windows Raw (`WM_INPUT`), του ίδιου τυπικού μηχανισμού εισαγωγής που χρησιμοποιείται από ποντίκια και πληκτρολόγια gaming. Ποτέ δεν εισάγει κώδικα σε διεργασίες-στόχους, δεν συνδέει το "WH_KEYBOARD_LL" ή δεν αγγίζει τη μνήμη του παιχνιδιού.

#### Πού αποθηκεύονται τα πακέτα ήχου μου;
Τα Soundpacks βρίσκονται στο `%APPDATA%\Mechvibes\soundpacks` ή απευθείας δίπλα στο φορητό εκτελέσιμο στο `soundpacks/`. Υποστηρίζονται και οι δύο δομές φακέλων Mechvibes v1 και v2 και οι προδιαγραφές «config.json».

#### Ποιες καθολικές συντομεύσεις είναι διαθέσιμες;
- `Ctrl + Alt + M` — Εναλλαγή σίγασης / κατάργησης σίγασης ήχου παγκοσμίως.
- Το κουμπί Κλείσιμο ελαχιστοποιείται απευθείας στο δίσκο συστήματος για απρόσκοπτη λειτουργία στο παρασκήνιο.
---
### Τεκμηρίωση & Αρχιτεκτονική

- [System Architecture](docs/system-architecture.md) — Μοντέλο νήματος, αγωγός ψηφοφορίας εισόδου και γράφημα μίξης ήχου
- [Soundpack Configuration v2](docs/soundpack-config-v2.md) — Σχήμα διαμόρφωσης και δομή καταλόγου soundpack
- [Οδηγός ανάπτυξης](docs/deployment-guide.md) — Οδηγίες πολλαπλής συλλογής και συσκευασίας
- [Διαδραστικό διάγραμμα](https://gitdiagram.com/Arean-Max/Mechvibes-Mercy) — Επισκόπηση οπτικής αρχιτεκτονικής
---
### Δημιουργία από πηγή

Προαπαιτούμενα: Εργαλειοθήκη Rust (1,88+).

```bash
git clone https://github.com/Arean-Max/Mechvibes-Mercy.git
cd Mechvibes-Mercy
cargo build --release
```

Το μεταγλωττισμένο δυαδικό αρχείο θα τοποθετηθεί στο `target/release/mercy.exe`.
---
### Άδεια

[MIT](ΑΔΕΙΑ) — Αρχικά βασίστηκε στο [mechvibes-dx](https://github.com/hainguyents13/mechvibes-dx) του Hai Nguyen.