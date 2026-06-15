---
title: Cara Install Docker di Armbian untuk Pemula — Lengkap dengan Contoh Project
description: >-
  Panduan step-by-step install Docker dan Docker Compose di Armbian. Dari setup
  repository hingga menjalankan container pertama untuk Home Assistant, Pi-hole,
  dan Nginx.
date: 2026-06-16T04:02:00.000Z
tags:
  - armbian
catalog: true
categories:
  - armbian
sticky: false
draft: false
tocNumbering: true
excludeFromSummary: false
math: false
quiz: false
keywords: []
---
# Cara Install Docker di Armbian untuk Pemula — Lengkap dengan Contoh Project

## Kenapa Docker di Armbian?

**Docker** adalah platform containerization yang memungkinkan Anda menjalankan aplikasi dalam lingkungan terisolasi tanpa perlu menginstall dependency satu per satu di sistem utama.

Keuntungan menggunakan Docker di board ARM dengan Armbian:

- **Isolasi aplikasi** — tiap app punya environment sendiri
- **Mudah backup** — cukup backup volume container
- **Portability** — docker-compose.yml bisa dipindah ke board lain
- **Rollback mudah** — hapus container dan buat ulang jika error
- **Hemat resource** — lebih ringan dari VM

---

## Persiapan Sebelum Install

Pastikan sistem Armbian Anda sudah:

- ✅ Update ke versi terbaru
- ✅ Memiliki koneksi internet stabil
- ✅ Minimal **1GB RAM** (2GB lebih baik untuk multiple container)

Cek versi Armbian Anda:

```bash
cat /etc/os-releas
```

Pastikan Anda menggunakan Armbian berbasis **Debian** atau **Ubuntu** (Bookworm, Jammy, atau Bullseye).

---

## **Langkah 1: Update Sistem**

**bash**

```bash
sudo apt update && sudo apt upgrade -y
sudo apt install ca-certificates curl gnupg lsb-release -y
```

---

## **Langkah 2: Install Docker Official**

### **Tambahkan Docker GPG Key**

**bash**

```bash
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg
```

### **Tambahkan Docker Repository**

**bash**

```bash
echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
```

> **Catatan**: Jika Anda menggunakan Ubuntu-based Armbian, ganti `debian` menjadi `ubuntu` pada URL di atas.

### **Install Docker Engine**

**bash**

```bash
sudo apt update
sudo apt install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y
```

### **Verifikasi Installasi**

**bash**

```bash
sudo docker --version
sudo docker compose version
```

Output yang diharapkan:

**plain**

```plain
Docker version 27.x.x, build xxxxx
Docker Compose version v2.x.x
```

---

## **Langkah 3: Jalankan Docker Tanpa Sudo (Opsional)**

Secara default, Docker hanya bisa dijalankan dengan `sudo`. Agar user biasa bisa menjalankan Docker:

**bash**

```bash
# Tambahkan user ke grup docker
sudo usermod -aG docker $USER

# Apply perubahan (logout dan login ulang, atau reboot)
newgrp docker
```

Test:

**bash**

```bash
docker run hello-world
```

Jika berhasil, Anda akan melihat pesan `Hello from Docker!`.

---

## **Langkah 4: Install Docker Compose Standalone (Opsional)**

Meskipun Docker sudah menyertakan `docker compose` (plugin), beberapa orang lebih nyaman dengan binary `docker-compose`:

**bash**

```bash
# Download versi terbaru untuk ARM64
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-linux-aarch64" -o /usr/local/bin/docker-compose

# Beri permission execute
sudo chmod +x /usr/local/bin/docker-compose

# Verifikasi
docker-compose --version
```

---

## **Langkah 5: Project Pertama — Nginx Web Server**

Buat folder project dan jalankan container Nginx:

**bash**

```bash
mkdir ~/docker-projects && cd ~/docker-projects
mkdir nginx-test && cd nginx-test
```

Buat file `docker-compose.yml`:

**yaml**

```yaml
version: "3.8"

services:
  nginx:
    image: nginx:alpine
    container_name: nginx-web
    ports:
      - "8080:80"
    volumes:
      - ./html:/usr/share/nginx/html
    restart: unless-stopped
```

Buat folder `html` dan file `index.html`:

**bash**

```plain
mkdir html
echo "<h1>Hello from Armbian + Docker!</h1>" > html/index.html
```

Jalankan container:

**bash**

```bash
docker compose up -d
```

Buka browser dan akses: `http://IP-BOARD:8080`

---

## **Langkah 6: Project Kedua — Pi-hole dengan Docker**

**bash**

```bash
cd ~/docker-projects
mkdir pihole && cd pihole
```

File `docker-compose.yml`:

**yaml**

```yaml
version: "3.8"

services:
  pihole:
    image: pihole/pihole:latest
    container_name: pihole
    ports:
      - "53:53/tcp"
      - "53:53/udp"
      - "8081:80/tcp"
    environment:
      TZ: 'Asia/Jakarta'
      WEBPASSWORD: 'passwordadmin'
    volumes:
      - './etc-pihole:/etc/pihole'
      - './etc-dnsmasq.d:/etc/dnsmasq.d'
    restart: unless-stopped
```

Jalankan:

**bash**

```bash
docker compose up -d
```

Akses dashboard: `http://IP-BOARD:8081/admin`

---

## **Langkah 7: Project Ketiga — Home Assistant**

**bash**

```bash
cd ~/docker-projects
mkdir homeassistant && cd homeassistant
```

File `docker-compose.yml`:

**yaml**

```yaml
version: "3.8"

services:
  homeassistant:
    image: ghcr.io/home-assistant/home-assistant:stable
    container_name: homeassistant
    privileged: true
    restart: unless-stopped
    environment:
      - TZ=Asia/Jakarta
    volumes:
      - ./config:/config
    network_mode: host
```

Jalankan:

**bash**

```plain
docker compose up -d
```

Akses UI: `http://IP-BOARD:8123`

**Perintah Docker yang Sering Digunakan**

| Perintah | Fungsi |
| --- | --- |
| `docker ps` | Lihat container yang berjalan |
| `docker ps -a` | Lihat semua container (aktif & mati) |
| `docker logs CONTAINER` | Lihat log container |
| `docker stop CONTAINER` | Hentikan container |
| `docker start CONTAINER` | Jalankan container |
| `docker rm CONTAINER` | Hapus container |
| `docker images` | Lihat image yang tersedia |
| `docker compose down` | Hentikan dan hapus container + network |
| `docker compose pull` | Download image terbaru |
| `docker compose up -d` | Jalankan container di background |
| `docker exec -it CONTAINER bash` | Masuk ke shell container |

## **Troubleshooting**

### **Docker Service Tidak Berjalan**

**bash**

```bash
sudo systemctl status docker
sudo systemctl start docker
sudo systemctl enable docker
```

### **Permission Denied**

Pastikan user sudah ditambahkan ke grup `docker` dan sudah logout-login ulang.

### **Container Exit Terus**

Cek log container untuk error:

**bash**

```bash
docker logs --tail 50 CONTAINER_NAME
```

### **Port Sudah Digunakan**

Cek port yang aktif:

**bash**

```bash
sudo ss -tlnp | grep :PORT
```

Ubah port mapping di `docker-compose.yml` jika bentrok.

---

## **Tips Optimasi Docker di Board ARM**

### **1. Gunakan Image Alpine atau Slim**

Image berbasis Alpine lebih kecil dan hemat RAM:

- `nginx:alpine` vs `nginx:latest`
- `python:3.11-alpine` vs `python:3.11`

### **2. Limit Resource Container**

**yaml**

```yaml
services:
  app:
    image: some-image
    deploy:
      resources:
        limits:
          cpus: '1.0'
          memory: 512M
```

### **3. Enable Docker Log Rotation**

Edit `/etc/docker/daemon.json`:

**JSON**

```json
{
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "10m",
    "max-file": "3"
  }
}
```

Restart Docker:

**bash**

```bash
sudo systemctl restart docker
```

---

## **Kesimpulan**

Dengan Docker di Armbian, Anda bisa:

- Menjalankan **banyak aplikasi** dalam satu board tanpa konflik dependency
- **Mudah backup** dan restore project
- **Eksperimen aman** — salah konfigurasi? Hapus container dan ulang
- **Deploy cepat** — cukup `docker compose up -d`

Dari web server sederhana hingga smart home server, Docker membuat manajemen aplikasi di board ARM menjadi jauh lebih mudah dan terstruktur.

Selamat mencoba! 🐳
