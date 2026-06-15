---
title: 'Rust dengan Code Preview: Belajar dari Contoh Praktis'
description: >-
  Pelajari Rust melalui contoh kode yang mudah dipahami dari Hello World hingga
  ownership, borrowing, dan struct.
date: 2026-06-16T02:11:00.000Z
tags:
  - rust
subtitle: Contoh Kode Rust untuk Pemula dengan Penjelasan Langkah demi Langkah
catalog: true
categories:
  - tutorial
sticky: false
draft: false
tocNumbering: true
excludeFromSummary: false
math: false
quiz: false
keywords: []
---
Salah satu cara terbaik untuk memahami Rust adalah dengan melihat dan mencoba kode langsung. Artikel ini membawa kamu melewati konsep-konsep fundamental Rust melalui contoh kode yang bisa dicoba langsung.

1. Hello World — Program Pertama

```rust
fn main() {
    println!("Hello, World!");
}
```

Penjelasan: `fn` mendeklarasikan fungsi. `println!` adalah macro (ditandai dengan `!`) untuk mencetak teks ke konsol. Setiap program Rust dimulai dari fungsi `main`.

2. Variabel dan Mutability

```rust
fn main() {
    let x = 5;          // immutable — tidak bisa diubah
    let mut y = 10;     // mutable — bisa diubah
    y = y + 5;
    println!("x = {}, y = {}", x, y);
}
```

Penjelasan: Secara default, variabel di Rust adalah **immutable**. Gunakan `mut` agar variabel dapat diubah nilainya. Ini mencegah perubahan tidak terduga.

3. Ownership — Konsep Inti Rust

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;          // ownership pindah ke s2
    // println!("{}", s1); // ERROR! s1 tidak lagi valid
    println!("{}", s2);  // OK
}
```

Penjelasan: `String::from` membuat `String` di heap. Saat `s1` diberikan ke `s2`, ownership berpindah. Rust tidak mengizinkan dua owner untuk satu data di heap — ini mencegah double-free.

4. Borrowing dengan Referensi

```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("Panjang '{}' adalah {}.", s, len);
}

fn calculate_length(text: &String) -> usize {
    text.len()
}
```

Penjelasan: `&s` membuat referensi terhadap `s` (borrowing). `calculate_length` meminjam `s` tanpa mengambil ownership. Setelah fungsi selesai, `s` masih valid dan bisa dipakai.

5. Struct dan Method

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Luas = {}", rect.area());
}
```

Penjelasan: `struct` mendefinisikan tipe data kustom. `impl` menambahkan method pada struct. `&self` adalah referensi ke instance struct, mirip `this` di bahasa lain.

6. Enum dan Pattern Matching

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Keluar"),
        Message::Move { x, y } => println!("Pindah ke ({}, {})", x, y),
        Message::Write(text) => println!("Tulis: {}", text),
    }
}
```

Penjelasan: `enum` di Rust bisa menyimpan data berbeda untuk setiap variant. `match` memastikan semua kemungkinan ditangani — tidak ada null pointer exception yang tidak terduga.

7. Result untuk Error Handling

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt");
    match greeting_file {
        Ok(file) => println!("File berhasil dibuka!"),
        Err(error) => println!("Gagal membuka file: {:?}", error),
    }
}
```

Penjelasan: Rust tidak punya exception. Sebagai gantinya, fungsi yang bisa gagal mengembalikan `Result<T, E>`. Kamu wajib menangani kasus `Ok` dan `Err` — compiler memastikan tidak ada error yang terlewat.

**Kesimpulan**

Rust mungkin punya kurva belajar yang curam, tetapi setiap aturan yang ada punya alasan kuat: keamanan memori, keamanan concurrency, dan performa tinggi. Mulailah dari contoh sederhana di atas, jalankan dengan `cargo run`, dan modifikasi untuk melihat apa yang terjadi. Praktik langsung adalah cara tercepat untuk menguasai Rust.
