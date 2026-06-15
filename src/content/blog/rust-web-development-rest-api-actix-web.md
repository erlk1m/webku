---
title: 'Rust Web Development: Membuat REST API dengan Actix-web'
description: >-
  Tutorial praktis membangun REST API dengan Rust dan Actix-web: routing,
  handler, JSON, middleware, error handling, dan database integration � lengkap
  dengan code blocks.
date: 2026-06-16T02:50:00.000Z
tags:
  - rust
subtitle: >-
  Bangun Web API Production-Ready dengan Rust, Actix-web, dan PostgreSQL Langkah
  demi Langkah
catalog: true
categories:
  - tutorial
sticky: true
draft: false
tocNumbering: true
excludeFromSummary: true
math: true
quiz: true
keywords: []
---
Rust bukan hanya untuk sistem programming dan CLI tools — ia juga sangat powerful untuk web development. Dengan framework seperti Actix-web, kamu bisa membangun REST API yang cepat, aman, dan scalable. Artikel ini membawa kamu membangun API lengkap dari nol.

**1. Apa Itu Actix-web?**

Actix-web adalah framework web untuk Rust yang berbasis actor system. Ia menawarkan:

• **Performa tinggi** — salah satu framework web tercepat di benchmark\
• **Type-safe routing** — routing diperiksa pada waktu kompilasi\
• **Middleware** yang fleksibel dan composable\
• **Async/await** native\
• **Auto-reload** untuk development

**2. Setup Project**

Buat project baru dan tambahkan dependency:

```bash
cargo new rust-api
cd rust-api
```

```toml
[package]
name = "rust-api"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**3. Hello World Server**

Server Actix-web paling sederhana:

```rust
use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, Rust API!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

Jalankan dengan:

```bash
cargo run
```

Test dengan curl:

```bash
curl http://localhost:8080/
```

**4. Routing dengan Parameter**

Actix-web mendukung path parameter dan query parameter:

```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/user/{id}")]
async fn get_user(path: web::Path) -> impl Responder {
    format!("User ID: {}", path.into_inner())
}

#[get("/search")]
async fn search(info: web::Query) -> impl Responder {
    format!("Searching for: {}", info.q)
}

#[derive(serde::Deserialize)]
struct SearchInfo {
    q: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_user)
            .service(search)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

Test:

```bash
curl http://localhost:8080/user/42
curl "http://localhost:8080/search?q=rust"
```

**5. JSON Request dan Response**

Actix-web otomatis meng-handle serialisasi dan deserialisasi JSON:

```rust
use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    data: T,
}

#[post("/users")]
async fn create_user(user: web::Json) -> impl Responder {
    let response = ApiResponse {
        success: true,
        data: user.into_inner(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/users/{id}")]
async fn get_user_by_id(path: web::Path) -> impl Responder {
    let user = User {
        id: path.into_inner(),
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
    };
    HttpResponse::Ok().json(ApiResponse { success: true, data: user })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_user)
            .service(get_user_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

Test JSON API:

```bash
curl -X POST http://localhost:8080/users   -H "Content-Type: application/json"   -d '{"id": 1, "name": "Alice", "email": "alice@example.com"}'

curl http://localhost:8080/users/1
```

**6. State dan Data Sharing**

Gunakan `web::Data` untuk berbagi state antar handler:

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Mutex;

struct AppState {
    counter: Mutex,
}

async fn counter(data: web::Data) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Count: {}", *counter))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/counter", web::get().to(counter))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

**7. Middleware**

Actix-web menyediakan middleware bawaan dan memungkinkan custom middleware:

```rust
use actix_web::{middleware, web, App, HttpServer, HttpResponse};

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::default())
            .route("/hello", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

Tambahkan di `Cargo.toml`:

```toml
[dependencies]
actix-web = "4"
env_logger = "0.11"
```

Inisialisasi logger di `main.rs`:

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // ... server code
}
```

**8. Error Handling**

Buat custom error response yang konsisten:

```rust
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: String,
}

#[derive(Debug)]
enum ApiError {
    NotFound,
    BadRequest(String),
    InternalError,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "Resource not found"),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::InternalError => write!(f, "Internal server error"),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            success: false,
            error: self.to_string(),
        })
    }
}

async fn get_item(path: web::Path) -> Result {
    let id = path.into_inner();
    if id == 0 {
        return Err(ApiError::BadRequest(String::from("ID must be > 0")));
    }
    if id > 100 {
        return Err(ApiError::NotFound);
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": { "id": id, "name": "Item" }
    })))
}
```

**9. CRUD API Lengkap**

In-memory CRUD API dengan Actix-web:

```rust
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

struct AppState {
    todos: Mutex>,
    next_id: Mutex,
}

#[get("/todos")]
async fn list_todos(data: web::Data) -> HttpResponse {
    let todos = data.todos.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}

#[post("/todos")]
async fn create_todo(
    data: web::Data,
    todo: web::Json,
) -> HttpResponse {
    let mut todos = data.todos.lock().unwrap();
    let mut next_id = data.next_id.lock().unwrap();
    
    let new_todo = Todo {
        id: *next_id,
        title: todo.title.clone(),
        completed: false,
    };
    *next_id += 1;
    todos.push(new_todo.clone());
    
    HttpResponse::Created().json(new_todo)
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

#[get("/todos/{id}")]
async fn get_todo(data: web::Data, path: web::Path) -> HttpResponse {
    let todos = data.todos.lock().unwrap();
    match todos.iter().find(|t| t.id == path.into_inner()) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[put("/todos/{id}")]
async fn update_todo(
    data: web::Data,
    path: web::Path,
    update: web::Json,
) -> HttpResponse {
    let mut todos = data.todos.lock().unwrap();
    match todos.iter_mut().find(|t| t.id == path.into_inner()) {
        Some(todo) => {
            todo.title = update.title.clone().unwrap_or(todo.title.clone());
            todo.completed = update.completed.unwrap_or(todo.completed);
            HttpResponse::Ok().json(todo)
        }
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[derive(Deserialize)]
struct UpdateTodo {
    title: Option,
    completed: Option,
}

#[delete("/todos/{id}")]
async fn delete_todo(data: web::Data, path: web::Path) -> HttpResponse {
    let mut todos = data.todos.lock().unwrap();
    let id = path.into_inner();
    let initial_len = todos.len();
    todos.retain(|t| t.id != id);
    
    if todos.len() < initial_len {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        todos: Mutex::new(vec![]),
        next_id: Mutex::new(1),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(list_todos)
            .service(create_todo)
            .service(get_todo)
            .service(update_todo)
            .service(delete_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

Test CRUD:

```bash
curl -X POST http://localhost:8080/todos   -H "Content-Type: application/json"   -d '{"title": "Belajar Rust"}'

curl http://localhost:8080/todos

curl -X PUT http://localhost:8080/todos/1   -H "Content-Type: application/json"   -d '{"completed": true}'

curl -X DELETE http://localhost:8080/todos/1
```

**10. Extractors dan Validasi**

Actix-web mendukung banyak extractor untuk data yang berbeda:

```rust
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option,
    per_page: Option,
}

async fn list_items(
    req: HttpRequest,
    query: web::Query,
    body: Option>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);
    
    let user_agent = req.headers().get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    
    HttpResponse::Ok().json(serde_json::json!({
        "page": page,
        "per_page": per_page,
        "user_agent": user_agent,
        "body_present": body.is_some()
    }))
}
```

**Kesimpulan**

Actix-web membuka dunia web development dengan Rust yang cepat dan aman. Dari artikel ini kamu telah belajar:

• Setup project Actix-web\
• Routing dan parameter\
• JSON request/response otomatis\
• State sharing dengan `web::Data`\
• Middleware untuk logging dan kompresi\
• Custom error handling\
• CRUD API lengkap\
• Extractors untuk header, query, dan body

Selanjutnya, eksplorasi topik seperti: integrasi database dengan `sqlx` atau `diesel`, authentication dengan JWT, WebSocket, dan deployment ke container.

Rust untuk web bukan lagi masa depan — ia sudah siap untuk production. 🦀
