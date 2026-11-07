# Meting API di Rust WebAssembly (Cloudflare Workers) 🦀⚡

API Meting berkinerja tinggi berbasis **Rust** yang dikompilasi ke **WebAssembly (WASM)** untuk dijalankan di **Cloudflare Workers**.

Project ini menggantikan ketergantungan pada API pihak ketiga publik (seperti `163.hyc.moe`) dengan instans milik Anda sendiri yang:
- **100% Gratis**: Termasuk dalam kuota gratis Cloudflare Workers (100.000 request per hari).
- **Cepat & Ringan**: Ditenagai oleh Rust WASM di edge server Cloudflare terdekat dengan pengunjung.
- **Aman (Strict HTTPS & CORS)**: Semua URL audio, gambar, dan lirik otomatis dinormalisasi ke `https://` untuk mencegah pemblokiran *Mixed Content* oleh browser modern.
- **Mendukung NetEase Cloud Music & Audius**: Mendukung playlist, lagu tunggal, lirik LRC, pencarian, gambar cover, dan streaming audio.

---

## 📁 Struktur Direktori

```
meting-worker/
├── Cargo.toml          # Konfigurasi Rust & dependencies (worker, serde, serde_json, url)
├── wrangler.toml       # Konfigurasi Cloudflare Workers & build command WASM
├── src/
│   └── lib.rs          # Kode utama implementasi Meting API dalam Rust
└── README.md           # Panduan penggunaan
```

---

## 🚀 Prasyarat

Pastikan komputer Anda sudah terpasang:
1. **Rust & Cargo** (`rustc --version`, `cargo --version`)
2. Target WebAssembly:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. Node.js (untuk menjalankan CLI Cloudflare Wrangler via `npx wrangler`)

---

## 🛠️ Uji Coba Lokal (Local Development)

Masuk ke folder `meting-worker`:

```bash
cd meting-worker
```

Jalankan server pengembangan lokal menggunakan Wrangler:

```bash
npx wrangler dev
```

Wrangler akan secara otomatis mengompilasi kode Rust ke file WebAssembly `.wasm`, mem-bundle runner JavaScript, dan menyalakan server lokal di `http://localhost:8787`.

### Tes Endpoint di Browser / cURL:
- **Informasi API**:
  `http://localhost:8787/`
- **Daftar Putar NetEase (Playlist)**:
  `http://localhost:8787/?server=netease&type=playlist&id=8676645748`
- **Lirik Lagu NetEase (LRC)**:
  `http://localhost:8787/?server=netease&type=lrc&id=3313423455`
- **Audio Stream NetEase**:
  `http://localhost:8787/?server=netease&type=url&id=3313423455`
- **Pencarian Lagu**:
  `http://localhost:8787/?server=netease&type=search&id=radwimps`

---

## 🌐 Deploy ke Cloudflare Workers

1. Login ke akun Cloudflare Anda:
   ```bash
   npx wrangler login
   ```
2. Publikasikan Worker ke Cloudflare Edge:
   ```bash
   npx wrangler deploy
   ```

Setelah deploy selesai, Wrangler akan memberikan URL publik Worker Anda, misalnya:
```
https://meting-api-worker.<username-cloudflare>.workers.dev
```

---

## 🎵 Hubungkan ke Blog Astro Anda

Setelah Worker Anda aktif, buka file `config/site.yaml` di blog Anda dan ganti konfigurasi `bgm.metingApi`:

```yaml
bgm:
  enabled: true
  metingApi: https://meting-api-worker.<username-cloudflare>.workers.dev/ # URL Worker Rust WASM Anda
  audio:
    - title: Playlist Pilihan
      list:
        - "https://music.163.com/playlist?id=8676645748"
```

Setelah itu, build atau deploy blog Anda. Seluruh pemutaran musik dan sinkronisasi lirik akan langsung dilayani secara mandiri dan cepat oleh Cloudflare Worker Rust WASM Anda sendiri!

