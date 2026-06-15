---
title: 5 Project Keren yang Bisa Dibuat dengan Armbian di Board ARM
description: >-
  Temukan 5 project praktis menggunakan Armbian di board ARM: NAS pribadi,
  Pi-hole ad blocker, Home Assistant, Jellyfin media server, dan web server
  dengan Docker.
date: 2026-06-16T03:57:00.000Z
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
# 5 Project Keren yang Bisa Dibuat dengan Armbian di Board ARM

## Pendahuluan

Setelah berhasil menginstall Armbian di board ARM seperti Orange Pi, NanoPi, atau Rock Pi, banyak pengguna bertanya-tanya: *"Lalu, mau diapain?"*

Board ARM yang menjalankan Armbian sebenarnya adalah **mini komputer Linux** yang sangat powerful. Dengan konsumsi daya rendah (biasanya 5-15 watt), board ini bisa berjalan 24/7 tanpa khawatir tagihan listrik membengkak. Berikut adalah **5 project praktis** yang bisa Anda bangun dengan Armbian.

---

## Project 1: Personal NAS (Network Attached Storage)

Jadikan board ARM Anda sebagai cloud storage pribadi di rumah.

### Mengapa NAS?

- **Akses file** dari mana saja dalam jaringan rumah
- **Backup otomatis** dari laptop dan smartphone
- **Biaya rendah** dibanding beli NAS komersial (Synology, QNAP)

### Tools yang Digunakan

| Software | Fungsi |
| --- | --- |
| **Samba** | Sharing folder ke Windows/Mac/Linux |
| **OpenMediaVault (OMV)** | Web-based NAS management |
| **Syncthing** | Sinkronisasi file antar device |
| **Nextcloud** | Cloud storage dengan web interface |

### Langkah Singkat

```bash
# Install Samba
sudo apt install samba samba-common-bin -y

# Setup shared folder
sudo mkdir -p /mnt/nas-storage
sudo chmod 777 /mnt/nas-storage

# Edit konfigurasi Samba
sudo nano /etc/samba/smb.conf

# Tambahkan di bagian bawah:
[nas-share]
path = /mnt/nas-storage
writeable = yes
create mask = 0777
directory mask = 0777
public = yes

# Restart Samba
sudo systemctl restart smb
```

> **Tips**: Tambahkan hard disk USB 3.0 eksternal untuk kapasitas besar. Format dengan `ext4` agar lebih cepat dan stabil.\

## **Project 2: Pi-hole — Ad Blocker untuk Seluruh Jaringan**

Blokir iklan dan tracker di semua device rumah Anda dengan sekali setup.

### **Cara Kerja Pi-hole**

Pi-hole bertindak sebagai **DNS server** di jaringan Anda. Setiap kali device meminta akses ke domain iklan (misalnya `googleadservices.com`), Pi-hole akan memblokirnya sebelum iklan terdownload.

### **Keuntungan**

- Blokir iklan di **semua device** (HP, laptop, TV smart, bahkan console game)
- Blokir **tracker dan malware** domain
- **Lihat statistik** website apa saja yang diakses di rumah
- Hemat **bandwidth** dan baterai HP

### **Install Pi-hole di Armbian**

**bash**

```bash
# One-liner install Pi-hole
curl -sSL https://install.pi-hole.net | bash
```

Setelah install, arahkan DNS router Anda ke IP board ARM. Atau set DNS manual di setiap device:

- **Primary DNS**: IP board ARM Anda
- **Secondary DNS**: `1.1.1.1` (Cloudflare) sebagai backup

> **Akses Dashboard**: Buka `http://IP-BOARD/admin` di browser.

---

## **Project 3: Home Assistant — Smart Home Server**

Kendalikan lampu, AC, kipas, dan sensor IoT dari satu dashboard.

### **Apa itu Home Assistant?**

Home Assistant adalah platform **open-source home automation** yang mendukung ribuan device dari berbagai merek: Xiaomi, Philips Hue, Tuya, Sonoff, ESP8266, dan banyak lagi.

### **Kenapa Jalankan di Armbian?**

- **Privasi** — data smart home Anda tidak dikirim ke cloud pihak ketiga
- **Lokal** — berjalan 100% di jaringan rumah, internet mati pun tetap jalan
- **Gratis** — tidak perlu berlangganan layanan cloud

### **Install Home Assistant Container**

**bash**

```bash
# Install Docker terlebih dahulu
sudo apt install docker.io docker-compose -y
sudo systemctl enable docker

# Jalankan Home Assistant
sudo docker run -d \
  --name homeassistant \
  --privileged \
  --restart=unless-stopped \
  -e TZ=Asia/Jakarta \
  -v /home/$(whoami)/homeassistant:/config \
  --network=host \
  ghcr.io/home-assistant/home-assistant:stable
```

**Akses UI**: `http://IP-BOARD:8123`

> Dengan Home Assistant, Anda bisa membuat **automasi** seperti: *"Jika sensor gerak terdeteksi di malam hari, nyalakan lampu hallway selama 5 menit."*

---

## **Project 4: Media Server dengan Jellyfin**

Streaming film, musik, dan foto koleksi pribadi ke TV, HP, atau laptop.

### **Jellyfin vs Plex**

**Table**

| **Fitur** | **Jellyfin** | **Plex** |
| --- | --- | --- |
| **Harga** | Gratis & open-source | Freemium (fitur premium berbayar) |
| **Privasi** | 100% lokal | Memerlukan akun cloud |
| **Transcoding** | Tergantung hardware board | Sama |
| **Plugin** | Aktif dikembangkan komunitas | Lebih mature |

### **Install Jellyfin**

**bash**

```bash
# Tambahkan repo Jellyfin
curl -fsSL https://repo.jellyfin.org/debian/jellyfin_team.gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/jellyfin.gpg

# Tambahkan source list
echo "deb [signed-by=/usr/share/keyrings/jellyfin.gpg] https://repo.jellyfin.org/debian $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/jellyfin.list

# Install
sudo apt update
sudo apt install jellyfin -y

# Enable service
sudo systemctl enable --now jellyfin
```

**Akses UI**: `http://IP-BOARD:8096`

> **Tips Transcoding**: Board ARM dengan GPU Mali (seperti Orange Pi 5) mendukung **hardware transcoding** via FFmpeg. Ini memungkinkan streaming 4K ke device lemah tanpa patah-patah.

---

## **Project 5: Web Server + Reverse Proxy (Nginx + Docker)**

Host website pribadi, blog, atau aplikasi web di board ARM Anda sendiri.

### **Stack yang Digunakan**

**Table**

| **Komponen** | **Fungsi** |
| --- | --- |
| **Nginx Proxy Manager** | Reverse proxy + SSL otomatis (Let's Encrypt) |
| **Docker + Portainer** | Manajemen container via GUI |
| **WordPress / Ghost** | CMS untuk blog |
| **Uptime Kuma** | Monitoring uptime website |

### **Setup dengan Docker Compose**

**bash**

```bash
# Buat folder project
mkdir ~/docker-server && cd ~/docker-server

# Buat file docker-compose.yml
nano docker-compose.yml
```

Isi `docker-compose.yml`:

**yaml**

```yaml
version: "3"

services:
  nginx-proxy-manager:
    image: jc21/nginx-proxy-manager:latest
    restart: unless-stopped
    ports:
      - "80:80"
      - "81:81"
      - "443:443"
    volumes:
      - ./npm-data:/data
      - ./letsencrypt:/etc/letsencrypt

  portainer:
    image: portainer/portainer-ce:latest
    restart: unless-stopped
    ports:
      - "9000:9000"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - ./portainer-data:/data
```

Jalankan:

**bash**

```bash
sudo docker-compose up -d
```

**Akses Nginx Proxy Manager**: `http://IP-BOARD:81`\
**Akses Portainer**: `http://IP-BOARD:9000`

> Dengan Nginx Proxy Manager, Anda bisa mendapatkan **SSL gratis** dari Let's Encrypt untuk domain Anda (misal: `blog.rumahku.duckdns.org`) dengan sekali klik.

---

## **Perbandingan Daya dan Biaya**

**Table**

| **Project** | **CPU Usage** | **RAM Usage** | **Cocok untuk Board** |
| --- | --- | --- | --- |
| NAS (Samba) | Rendah | 100-200 MB | Orange Pi Zero 2 |
| Pi-hole | Sangat rendah | 50-100 MB | NanoPi NEO |
| Home Assistant | Sedang | 300-500 MB | Orange Pi 3 LTS |
| Jellyfin | Tinggi (saat transcode) | 500 MB - 1 GB | Orange Pi 5 / RK3588 |
| Web Server | Rendah-Sedang | 200-400 MB | Banana Pi M5 |

> **Catatan**: Board dengan RAM 1GB masih cukup untuk project 1-3. Untuk Jellyfin dan web server, disarankan minimal **2GB RAM**.

---

## **Kesimpulan**

Board ARM + Armbian adalah kombinasi yang sangat **versatile** dan **hemat biaya**. Dengan investasi board seharga **Rp 200 ribu - 1 jutaan**, Anda bisa membangun:

- **Cloud storage pribadi**
- **Ad blocker jaringan**
- **Smart home server**
- **Media streaming server**
- **Web server pribadi**

Semua project ini berjalan **24/7** dengan konsumsi daya setara **1-2 lampu LED**. Jauh lebih hemat dibanding menyewa VPS cloud yang biayanya puluhan dollar per bulan.
