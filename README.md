
# 🧠 Intelligent File Deduplicator

A high-performance and intelligent file deduplication tool written in **Rust**. It detects **exact duplicates** using multiple hashing algorithms, and goes beyond with **intelligent detection** of **near-duplicate text content** and **visually similar images**. Designed for speed, safety, and precision.

---

## 🚀 Features

- 🔐 **Multiple Hashing Algorithms**: Supports **SHA-256**, **Blake3**, and **xxHash** for robust hash-based deduplication.
- 🧠 **Content Similarity Detection**: Uses textual analysis to detect near-duplicates in documents.
- 🖼 **Image Similarity Matching**: Leverages image hashing to group visually similar images.
- ⚡ **Parallel Processing**: Powered by **Rayon** for multithreaded file scanning and hashing.
- 🧹 **Quarantine System**: Optionally moves detected duplicates to a safe quarantine folder for manual review.
- 📊 **Structured Reporting**: Outputs detailed reports in **JSON** format.
- 🧰 **Command-Line Interface**: Easily configurable with powerful CLI options.

---

## 🛠 How It Works

1. **Scan**: Recursively scans a target directory for files.
2. **Hashing**: Computes content hashes to detect exact duplicates.
3. **Filtering**: Optional filters based on file type, size, or name.
4. **Similarity Detection**:
   - **Text**: Compares files using token-based cosine similarity.
   - **Images**: Matches similar images using perceptual hashing.
5. **Quarantine** *(Optional)*: Moves detected duplicates to a quarantine folder for review.
6. **Reporting**: Generates a comprehensive JSON report.

---

## 📦 Installation

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (1.70+ recommended)

### Clone and Build

```bash
git clone https://github.com/buzzy-coder/file-deduplicator.git
cd file-deduplicator
cargo build --release
````

---

## ▶️ Usage

```bash
cargo run --release -- [OPTIONS] <DIRECTORY>
```

### Example

```bash
cargo run --release -- ./my_files --hash blake3 --quarantine ./quarantine --min-size 1KB --report ./report.json
```

### CLI Options

| Option                 | Description                                            |
| ---------------------- | ------------------------------------------------------ |
| `--hash`               | Select hashing algorithm: `sha256`, `blake3`, `xxhash` |
| `--quarantine`         | Path to move detected duplicates safely                |
| `--min-size`           | Minimum file size to include (e.g., `1KB`)             |
| `--report`             | Output path for JSON report                            |
| `--content-similarity` | Enable content similarity check for text files         |
| `--image-similarity`   | Enable image similarity detection                      |

---

## 🧪 Testing

Run all tests:

```bash
cargo test
```

To test individual modules (e.g., `quarantine`):

```bash
cargo test quarantine
```

---

## 📁 Output

* **Report File (JSON)**
  Structured output listing:

  * Exact duplicate groups
  * Near-similar files
  * Image similarity clusters

* **Quarantine Folder** (if enabled)
  Safe copy of all duplicate files for manual verification and deletion.

---

## 🔧 Project Structure

```
src/
├── hashing.rs            # Hashing functions for multiple algorithms
├── scanner.rs            # Directory scanning and file collection
├── filter.rs             # Filtering logic (size, extensions, etc.)
├── report.rs             # Report generation (JSON)
├── quarantine.rs         # Safe file quarantine system
├── content_similarity.rs # Text similarity detection
├── image_similarity.rs   # Image similarity matching
└── main.rs               # CLI entry point
```

---

## 📚 Roadmap

* [x] Exact duplicate detection via hashing
* [x] CLI interface with options
* [x] Parallel file scanning
* [x] Text content similarity detection
* [x] Image similarity detection
* [x] Quarantine system
* [ ] HTML Report generation
* [ ] GUI frontend (planned)
* [ ] Cloud storage deduplication support (future)

---

## 🤝 Contributing

Pull requests are welcome. For major changes, open an issue first to discuss what you'd like to change.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

## 🙋‍♀ Author

**Tanisha Mohapatra**
🦀 Rust + Blockchain Enthusiast | 👩‍💻 2nd Year CSE | ☁️ AWS & Cloud Learner
🔗 GitHub: [@buzzy-coder](https://github.com/buzzy-coder)

---

## ❤️ Acknowledgements

* [Rust Community](https://users.rust-lang.org/)
* [`rayon`](https://docs.rs/rayon/latest/rayon/) for blazing fast parallelism
* [`blake3`](https://docs.rs/blake3/latest/blake3/) hashing crate
* [`image`](https://docs.rs/image/latest/image/) for image processing
* [`cosine_similarity`](https://docs.rs) for text matching logic

```

---

Let me know if you'd like this formatted for GitHub Pages, with a banner, or if you want badges for Rust version, test status, etc.
```
