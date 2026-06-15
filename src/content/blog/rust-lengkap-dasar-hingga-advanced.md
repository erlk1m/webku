---
title: 'Rust Lengkap: Dari Dasar hingga Advanced dengan Code Blocks'
description: >-
  Panduan komprehensif Rust dari nol hingga advanced: ownership, lifetimes,
  traits, generics, error handling, concurrency, testing, dan macros dengan code
  blocks lengkap.
date: 2026-06-16T02:31:00.000Z
tags:
  - rust
subtitle: Panduan Komprehensif Rust dari Nol hingga Advanced dengan Code Blocks
catalog: true
categories:
  - rust
  - tutorial
sticky: false
draft: false
tocNumbering: true
excludeFromSummary: false
math: false
quiz: false
keywords: []
---
Artikel ini adalah panduan komprehensif yang membawa kamu dari nol hingga tingkat mahir dalam Rust. Setiap konsep disertai dengan code block yang bisa langsung dicoba, sehingga kamu tidak hanya membaca teori tapi juga melihat cara kerja kode di praktik.

**1. Variabel dan Tipe Data**

Di Rust, variabel secara default immutable. Gunakan `mut` untuk membuatnya mutable.

```rust
fn main() {
    let name = "Rust";           // &str (string literal)
    let mut count = 0;           // mutable i32
    count += 1;

    let pi: f64 = 3.14159;     // eksplisit tipe
    let is_ok = true;           // boolean
    let letter = 'A';           // char (Unicode)

    let numbers: [i32; 3] = [1, 2, 3];  // array fixed-size
    let tuple = (1, "hello", 3.14);      // tuple

    println!("nama={}, count={}, pi={}, tuple={:?}", name, count, pi, tuple);
}
```

**2. Ownership dan Borrowing**

Ownership adalah fondasi Rust. Setiap nilai di heap punya satu owner.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;              // ownership berpindah
    // println!("{}", s1);    // ERROR: s1 tidak valid lagi

    let s3 = String::from("world");
    let len = get_length(&s3); // borrow — tidak mengambil ownership
    println!("{} panjangnya {}", s3, len);  // s3 masih valid
}

fn get_length(s: &String) -> usize {
    s.len()  // pinjam sementara, balikin
}
```

**3. Lifetimes**

Lifetime memastikan referensi selalu valid dan tidak dangling.

```rust
fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(&s1, s2);
    println!("String terpanjang: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Penjelasan: `'a` adalah lifetime annotation. Parameter dan return value berbagi lifetime yang sama, jadi compiler memastikan semua referensi valid selama hasil dipakai.

**4. Struct dan Enum**

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

struct Point(i32, i32, i32); // tuple struct

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let user = User {
        username: String::from("rustacean"),
        email: String::from("hi@rust.dev"),
        active: true,
    };

    let msg = Message::Move { x: 10, y: 20 };
    println!("User {} aktif: {}", user.username, user.active);
}
```

**5. Pattern Matching**

Match di Rust memastikan semua kasus ditangani, tidak ada null pointer exception.

```rust
enum Coin {
    Penny, Nickel, Dime, Quarter(String),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter dari {}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(String::from("Alaska"));
    println!("Nilai: {}", value_in_cents(coin));

    let some_number = Some(5);
    let absent_number: Option = None;

    match some_number {
        Some(n) => println!("Ada angka {}", n),
        None => println!("Tidak ada angka"),
    }

    // If let — shorthand untuk single pattern
    if let Some(5) = some_number {
        println!("Yup, itu 5!");
    }
}
```

**6. Error Handling — Result dan Option**

Rust tidak punya exception. Gunakan `Result` untuk operasi yang bisa gagal, dan `Option` untuk nilai yang mungkin tidak ada.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// Sama dengan versi di atas, tapi lebih ringkas
fn read_username_from_file_short() -> Result {
    std::fs::read_to_string("hello.txt")
}

fn main() {
    match read_username_from_file() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error: {}", e),
    }

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            println!("Gagal buka file: {:?}", error);
            panic!("File diperlukan!");
        }
    };
}
```

**7. Traits dan Generics**

Traits seperti interface di bahasa lain. Generics membuat kode reusable.

```rust
trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;

    // Default implementation
    fn summarize_detail(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.summarize())
    }
}

struct NewsArticle {
    headline: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        self.headline.clone()
    }
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        self.content.clone()
    }
    fn summarize_author(&self) -> String {
        self.username.clone()
    }
}

// Generic function dengan trait bound
fn notify(item: &T) {
    println!("Breaking! {}", item.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 1.80 Dirilis!"),
        author: String::from("Rust Team"),
    };
    let tweet = Tweet {
        username: String::from("@rustacean"),
        content: String::from("Rust itu keren!"),
    };

    notify(&article);
    notify(&tweet);
    println!("{}", article.summarize_detail());
}
```

**8. Iterators dan Closures**

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Iterator dengan map dan filter
    let doubled: Vec = v.iter()
        .map(|x| x * 2)
        .collect();
    println!("{:?}", doubled);  // [2, 4, 6, 8, 10]

    // Filter — ambil yang genap
    let evens: Vec<&i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .collect();
    println!("{:?}", evens);  // [2, 4]

    // Fold — reduce ke single value
    let sum: i32 = v.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);  // 15

    // Closure yang menangkap environment
    let threshold = 3;
    let big: Vec<&i32> = v.iter()
        .filter(|x| **x > threshold)
        .collect();
    println!("Bigger than {}: {:?}", threshold, big);

    // Chaining
    let result: Vec = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("Squares of even numbers: {:?}", result);
}
```

**9. Modules dan Crates**

Rust memiliki sistem module yang jelas untuk mengatur kode.

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("Saya pesan toast {}", meal.toast);
}

// src/main.rs
use crate::front_of_house::hosting;
fn main() { hosting::add_to_waitlist(); }
```

**10. Concurrency — Threads**

Rust membuat concurrency aman secara default berkat ownership.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();  // tunggu thread selesai

    // Message passing dengan channel
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi dari thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Diterima: {}", received);

    // Shared state dengan Mutex
    use std::sync::{Arc, Mutex};
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
```

**11. Testing**

Rust punya built-in testing framework yang sangat baik.

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

pub fn divide(a: f64, b: f64) -> Option {
    if b == 0.0 { None } else { Some(a / b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Some(5.0));
        assert_eq!(divide(10.0, 0.0), None);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Ini sengaja panic!");
    }

    #[test]
    fn result_with_question_mark() -> Result<(), String> {
        if 2 + 2 == 4 { Ok(()) } else { Err(String::from("dua plus dua bukan empat")) }
    }
}
```

Jalankan test dengan:

```bash
cargo test
cargo test -- --nocapture  # tampilkan output print
cargo test test_add        # jalankan test tertentu
```

**12. Macros**

Macros di Rust adalah metaprogramming yang powerful.

```rust
// Declarative macro dengan macro_rules!
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Kamu memanggil {:?}", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

// Procedural macro (contoh sederhana)
// #[derive(Debug)] — menambahkan implementasi trait Debug secara otomatis
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    say_hello!();
    foo();
    bar();

    let p = Person { name: String::from("Alice"), age: 30 };
    println!("{:?}", p);  // Debug output

    let p2 = p.clone();
    println!("{:?}", p2);
}

// macro_rules! dengan variadic arguments
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push(String::from($x)); )*
            temp_vec
        }
    };
}

fn use_macro() {
    let v = vec_of_strings!["a", "b", "c"];
    println!("{:?}", v);
}
```

**Kesimpulan**

Rust adalah bahasa yang menuntut disiplin dari programmer, tapi imbalannya sangat besar: kode yang aman dari bug memori, aman dari data race, dan performa setara bahasa native. Mulai dari konsep sederhana seperti variabel dan fungsi, hingga topik advanced seperti lifetimes, traits, dan concurrency — Rust menyediakan tool yang jelas dan compiler yang peduli untuk memandu kamu menulis kode berkualitas tinggi.

Jangan berkecil hati dengan pesan error yang panjang dari compiler. Pesan-pesan itu sebenarnya adalah bantuan terbaik yang pernah kamu dapatkan dari compiler mana pun. Baca pesan error dengan teliti, perbaiki, kompilasi ulang, dan teruslah praktik. Semangat rustacean! 🦀
