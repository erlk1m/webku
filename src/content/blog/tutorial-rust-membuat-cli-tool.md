---
title: 'Tutorial Rust: Membuat CLI Tool dari Nol'
description: >-
  Tutorial praktis membuat command-line tool dengan Rust: inisialisasi project,
  parsing argument, membaca file, dan menampilkan output � lengkap dengan code
  blocks.
date: 2026-06-16T02:35:00.000Z
tags:
  - rust
  - cli
subtitle: >-
  Project-Based Learning: Bangun CLI Tool Pertamamu dengan Rust Langkah demi
  Langkah
catalog: true
categories:
  - tutorial
sticky: false
draft: false
tocNumbering: true
excludeFromSummary: true
math: false
quiz: false
keywords: []
---
Dalam tutorial ini, kamu akan membuat sebuah CLI tool sederhana dengan Rust: aplikasi command-line yang membaca file teks dan menghitung statistik seperti jumlah baris, kata, dan karakter. Kita mulai dari instalasi Rust hingga aplikasi yang bisa dijalankan langsung.

**Prasyarat**

Pastikan kamu sudah menginstal Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
cargo --version
```

**Langkah 1: Inisialisasi Project**

Buat project baru dengan Cargo:

```bash
cargo new wordcount

cd wordcount

cargo run
```

Struktur folder yang dihasilkan:

```text
wordcount/
├── Cargo.toml
├── src/
│   └── main.rs
└── target/
    └── debug/
```

**Langkah 2: Membaca File**

Edit `src/main.rs` untuk membaca file dari command-line:

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec = env::args().collect();

    if args.len() < 2 {
        eprintln!("Penggunaan: wordcount <file>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Tidak bisa membaca file");

    println!("Isi file:
{}", contents);
}
```

Jalankan dengan:

```bash
echo "Hello Rust" > test.txt
cargo run -- test.txt
```

**Langkah 3: Menghitung Statistik**

Tambahkan fungsi untuk menghitung jumlah baris, kata, dan karakter:

```rust
fn count_stats(text: &str) -> (usize, usize, usize) {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    (lines, words, chars)
}
```

Ganti `main` untuk menggunakan fungsi tersebut:

```rust
use std::env;
use std::fs;

fn count_stats(text: &str) -> (usize, usize, usize) {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    (lines, words, chars)
}

fn main() {
    let args: Vec = env::args().collect();

    if args.len() < 2 {
        eprintln!("Penggunaan: wordcount <file>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Tidak bisa membaca file");

    let (lines, words, chars) = count_stats(&contents);

    println!("File: {}", filename);
    println!("  Baris : {}", lines);
    println!("  Kata  : {}", words);
    println!("  Karakter: {}", chars);
}
```

Output:

```text
File: test.txt
  Baris : 1
  Kata  : 2
  Karakter: 11
```

**Langkah 4: Error Handling yang Lebih Baik**

Gunakan `Result` untuk penanganan error yang lebih robust:

```rust
use std::env;
use std::fs;
use std::process;

fn count_stats(text: &str) -> (usize, usize, usize) {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    (lines, words, chars)
}

fn main() {
    let args: Vec = env::args().collect();

    if args.len() < 2 {
        eprintln!("Penggunaan: wordcount <file>");
        process::exit(1);
    }

    let filename = &args[1];

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error membaca '{}': {}", filename, err);
            process::exit(1);
        }
    };

    let (lines, words, chars) = count_stats(&contents);

    println!("File: {}", filename);
    println!("  Baris  : {}", lines);
    println!("  Kata   : {}", words);
    println!("  Karakter: {}", chars);
}
```

**Langkah 5: Parsing Argumen dengan Struct**

Buat struct untuk menangani argument parsing dengan lebih terstruktur:

```rust
use std::env;
use std::fs;
use std::process;

struct Config {
    filename: String,
    show_lines: bool,
    show_words: bool,
    show_chars: bool,
}

impl Config {
    fn new(args: &[String]) -> Result {
        if args.len() < 2 {
            return Err("Penggunaan: wordcount <file> [options]");
        }

        let filename = args[1].clone();
        let mut show_lines = true;
        let mut show_words = true;
        let mut show_chars = true;

        for arg in &args[2..] {
            match arg.as_str() {
                "--lines-only" => { show_words = false; show_chars = false; }
                "--words-only" => { show_lines = false; show_chars = false; }
                "--chars-only" => { show_lines = false; show_words = false; }
                _ => {}
            }
        }

        Ok(Config { filename, show_lines, show_words, show_chars })
    }
}

fn count_stats(text: &str) -> (usize, usize, usize) {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    (lines, words, chars)
}

fn main() {
    let args: Vec = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let contents = match fs::read_to_string(&config.filename) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error membaca '{}': {}", config.filename, err);
            process::exit(1);
        }
    };

    let (lines, words, chars) = count_stats(&contents);

    println!("File: {}", config.filename);
    if config.show_lines { println!("  Baris  : {}", lines); }
    if config.show_words  { println!("  Kata   : {}", words); }
    if config.show_chars { println!("  Karakter: {}", chars); }
}
```

Jalankan dengan opsi:

```bash
cargo run -- test.txt --words-only
cargo run -- test.txt --lines-only
```

**Langkah 6: Unit Testing**

Tambahkan test untuk memastikan fungsi `count_stats` bekerja dengan benar:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_stats() {
        let text = "Hello Rust
Tutorial CLI
";
        let (lines, words, chars) = count_stats(text);
        assert_eq!(lines, 2);
        assert_eq!(words, 4);
        assert_eq!(chars, 24);
    }

    #[test]
    fn test_count_stats_empty() {
        let text = "";
        let (lines, words, chars) = count_stats(text);
        assert_eq!(lines, 0);
        assert_eq!(words, 0);
        assert_eq!(chars, 0);
    }
}
```

Jalankan test:

```bash
cargo test
```

**Langkah 7: Build untuk Release**

Setelah selesai, build aplikasi untuk release:

```bash
cargo build --release
```

Binary hasil ada di `target/release/wordcount`. Bisa langsung dijalankan:

```bash
./target/release/wordcount test.txt
```

**Kesimpulan**

Kamu baru saja membuat CLI tool pertama dengan Rust! Dari tutorial ini kamu belajar:

• Inisialisasi project dengan Cargo\
• Membaca argument dari command-line\
• Membaca dan memproses file\
• Error handling dengan `Result`\
• Membuat struct dan method\
• Unit testing\
• Build untuk release

Lanjutkan eksplorasi dengan menambahkan fitur seperti: membaca multiple file, output ke file, progress bar, atau parsing dengan library seperti `clap` untuk argument parsing yang lebih powerful. Selamat coding! 🦀
