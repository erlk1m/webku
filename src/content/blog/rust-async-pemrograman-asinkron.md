---
title: 'Rust Async: Pemrograman Asinkron dengan async/await'
description: >-
  Panduan lengkap asynchronous programming di Rust: futures, async/await, Tokio
  runtime, concurrent tasks, channels, dan select.
date: 2026-06-16T02:45:00.000Z
tags:
  - rust
subtitle: >-
  Memahami Futures, Runtime, dan Concurrent Tasks dalam Rust dengan Praktik
  Langsung
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
Rust mendukung pemrograman asinkron melalui `async/await`, sebuah fitur yang memungkinkan kamu menulis kode non-blocking yang terlihat seperti kode sinkron biasa. Artikel ini membawa kamu dari konsep dasar futures hingga praktik concurrent tasks dengan Tokio.

**1. Apa Itu Async di Rust?**

Di Rust, fungsi `async` tidak langsung berjalan — ia mengembalikan sebuah `Future`. Future adalah nilai yang merepresentasikan komputasi yang belum selesai. Future baru dieksekusi ketika di-`await` atau dijalankan oleh runtime.

```rust
async fn hello() {
    println!("Hello dari async!");
}

fn main() {
    let future = hello();  // Future dibuat, tapi belum berjalan
    // Kode di sini belum mencetak apa pun
    
    // Untuk menjalankan, butuh runtime atau block_on
}
```

**2. Menjalankan Future dengan block\_on**

Cara termudah menjalankan future adalah menggunakan `futures::executor::block_on`:

```rust
use futures::executor::block_on;

async fn hello() {
    println!("Hello, async world!");
}

fn main() {
    let future = hello();
    block_on(future);  // blokir sampai future selesai
}
```

Tambahkan dependency di `Cargo.toml`:

```toml
[dependencies]
futures = "0.3"
```

**3. async/await dalam Praktik**

`await` digunakan di dalam fungsi `async` lain untuk menunggu future selesai tanpa memblokir thread:

```rust
use std::time::Duration;
use async_std::task;

async fn tarea_satu() {
    task::sleep(Duration::from_secs(1)).await;
    println!("Tarea satu selesai");
}

async fn tarea_dua() {
    task::sleep(Duration::from_secs(2)).await;
    println!("Tarea dua selesai");
}

async fn run_both() {
    tarea_satu().await;
    tarea_dua().await;
}

fn main() {
    task::block_on(run_both());
}
```

Perhatikan: kode di atas berjalan **secara berurutan**, bukan paralel. Total waktu = 1 + 2 = 3 detik.

**4. Menjalankan Task Secara Concurrent**

Untuk menjalankan task secara bersamaan, gunakan `join!` atau `futures::future::join`:

```rust
use futures::join;
use std::time::Duration;
use async_std::task;

async fn fetch_user(id: u32) -> String {
    task::sleep(Duration::from_secs(1)).await;
    format!("User {}", id)
}

async fn fetch_order(id: u32) -> String {
    task::sleep(Duration::from_secs(1)).await;
    format!("Order {}", id)
}

async fn get_data() {
    let user_future = fetch_user(1);
    let order_future = fetch_order(101);
    
    let (user, order) = join!(user_future, order_future);
    println!("{} dan {} diambil bersamaan", user, order);
    // Total waktu ~1 detik, bukan 2 detik!
}

fn main() {
    async_std::task::block_on(get_data());
}
```

**5. Tokio: Runtime Produksi untuk Rust**

Tokio adalah runtime async paling populer di Rust. Ia menyediakan thread pool, timer, I/O, dan channel.

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
use tokio::time::{sleep, Duration};

async fn task_a() {
    sleep(Duration::from_secs(1)).await;
    println!("Task A selesai");
}

async fn task_b() {
    sleep(Duration::from_secs(1)).await;
    println!("Task B selesai");
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    
    tokio::join!(task_a(), task_b());
    
    println!("Waktu total: {:?}", start.elapsed());
    // ~1 detik, bukan 2 detik!
}
```

**6. Spawning Task**

`tokio::spawn` membuat task baru yang berjalan secara independen:

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle1 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Spawn 1 selesai");
        42
    });
    
    let handle2 = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("Spawn 2 selesai");
        "hello"
    });
    
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();
    
    println!("Hasil: {} dan {}", result1, result2);
}
```

**7. Channels untuk Komunikasi antar Task**

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100); // buffer 100
    
    let tx2 = tx.clone();
    
    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(format!("Pesan dari A: {}", i)).await.unwrap();
        }
    });
    
    tokio::spawn(async move {
        for i in 5..10 {
            tx2.send(format!("Pesan dari B: {}", i)).await.unwrap();
        }
    });
    
    drop(tx); // tutup sender agar receiver tahu kapan selesai
    
    while let Some(msg) = rx.recv().await {
        println!("Diterima: {}", msg);
    }
    
    println!("Channel selesai");
}
```

**8. Select: Menunggu Banyak Future**

`tokio::select!` menunggu beberapa future dan menangani yang selesai pertama:

```rust
use tokio::time::{sleep, Duration};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();
    
    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        tx.send("Data dari task").unwrap();
    });
    
    tokio::select! {
        result = rx => {
            match result {
                Ok(msg) => println!("Diterima: {}", msg),
                Err(_) => println!("Sender dropped"),
            }
        }
        _ = sleep(Duration::from_secs(3)) => {
            println!("Timeout!");
        }
    }
}
```

**9. Concurrent Stream Processing**

```rust
use tokio::time::{sleep, Duration};

async fn download_file(id: u32) -> String {
    sleep(Duration::from_millis(500)).await;
    format!("File-{}.pdf", id)
}

#[tokio::main]
async fn main() {
    let ids = vec![1, 2, 3, 4, 5];
    
    // Sequential (lambat)
    for id in &ids {
        let file = download_file(*id).await;
        println!("Downloaded: {}", file);
    }
    
    println!("--- Sekarang concurrent ---");
    
    // Concurrent dengan FuturesUnordered (cepat)
    let mut tasks = tokio::task::JoinSet::new();
    for id in ids {
        tasks.spawn(download_file(id));
    }
    
    while let Some(res) = tasks.join_next().await {
        println!("Downloaded: {}", res.unwrap());
    }
}
```

**10. Async Trait**

Trait dengan method async memerlukan crate tambahan seperti `async-trait`:

```toml
[dependencies]
async-trait = "0.1"
```

```rust
use async_trait::async_trait;

#[async_trait]
trait Fetcher {
    async fn fetch(&self, url: &str) -> String;
}

struct HttpFetcher;

#[async_trait]
impl Fetcher for HttpFetcher {
    async fn fetch(&self, url: &str) -> String {
        format!("Fetched from {}", url)
    }
}

#[tokio::main]
async fn main() {
    let fetcher = HttpFetcher;
    let result = fetcher.fetch("https://rust-lang.org").await;
    println!("{}", result);
}
```

**Kesimpulan**

Async programming di Rust menawarkan performa tinggi dengan overhead minimal. Dengan `async/await`, kamu bisa menulis kode non-blocking yang bersih dan mudah dibaca. Tokio menyediakan ekosistem lengkap: runtime, I/O, timer, channel, dan utility untuk concurrent programming.

Hal yang perlu diingat:

• `async fn` mengembalikan `Future`, bukan langsung berjalan\
• `.await` menunggu future di dalam konteks async\
• `tokio::join!` untuk menjalankan task concurrent\
• `tokio::spawn` untuk task yang berjalan independen\
• `tokio::select!` untuk menangani event yang selesai pertama\
• Channel untuk komunikasi antar task

Mulailah dengan contoh sederhana, lalu tingkatkan ke aplikasi yang lebih kompleks. Async Rust sangat powerful untuk web server, proxy, game server, dan sistem real-time. Selamat eksplorasi! 🦀
