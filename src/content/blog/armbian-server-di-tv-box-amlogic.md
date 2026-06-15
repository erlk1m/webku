---
title: Armbian Server di TV Box Amlogic — Ubah Android TV Box Jadi Linux Server Murah
description: >-
  Panduan lengkap mengubah TV Box Amlogic (S905X, S905W, S912) menjadi server
  Linux dengan Armbian. Mulai dari flashing, boot, hingga optimasi untuk NAS,
  Docker, dan home server.
date: 2026-06-16T04:26:00.000Z
tags:
  - armbian
catalog: true
categories:
  - armbian
sticky: false
draft: false
tocNumbering: true
excludeFromSummary: true
math: false
quiz: false
keywords: []
---
# Armbian Server di TV Box Amlogic — Ubah Android TV Box Jadi Linux Server Murah

## Pendahuluan

TV Box berchipset **Amlogic** (S905X, S905W, S905X2, S912, S922X) yang beredar di pasaran dengan harga **Rp 150 ribu - 400 ribu** ternyata bisa diubah menjadi **Linux server** yang sangat capable dengan **Armbian**.

Dibanding membeli Raspberry Pi yang harganya naik drastis, TV Box Amlogic menawarkan:

- **RAM 2-4GB** dengan harga sangat terjangkau
- **Storage internal eMMC** yang lebih cepat dari SD Card
- **Port Ethernet** 100Mbps atau Gigabit
- **USB 2.0/3.0** untuk hard disk eksternal
- **HDMI** untuk troubleshooting langsung

Artikel ini akan membahas cara mengubah TV Box Amlogic Anda menjadi server Linux produktif dengan Armbian.

---

## TV Box Amlogic yang Kompatibel

### Chipset yang Didukung Armbian

{% table %}
- Spesifikasi
- TV Box Amlogic S905X2
- Raspberry Pi 4 2GB
---
- **Harga**
- Rp 250-350 ribu
- Rp 600-800 ribu
---
- **CPU**
- Quad-core Cortex-A53
- Quad-core Cortex-A72
---
- **RAM**
- 2-4GB DDR3/4
- 2-8GB LPDDR4
---
- **Storage**
- eMMC 8-16GB + MicroSD
- MicroSD only
---
- **Ethernet**
- 100M / Gigabit
- Gigabit
---
- **USB**
- USB 2.0 / 3.0
- USB 2.0 + 3.0
---
- **GPIO**
- Tidak ada
- 40-pin GPIO
---
- **Komunitas**
- Aktif (China/Internasional)
- Sangat besar
{% /table %}

### Cek Chipset TV Box Anda

Sebelum mulai, pastikan chipset TV Box Anda:

1. **Lihat stiker di bawah unit** — biasanya tertulis S905X, S912, dll
1. **Cek dengan AIDA64** — install dari Play Store, lihat bagian CPU
1. **Cek di forum Armbian** — cari model TV Box Anda di [forum.armbian.com](https://forum.armbian.com)

\> **Peringatan**: Tidak semua TV Box Amlogic bisa boot Armbian. Beberapa memiliki **bootloader terkunci** atau **WiFi/BT chip yang tidak ada driver Linux**. Ethernet biasanya selalu berfungsi.

---

## Alat dan Bahan

{% table %}
- Item
- Keterangan
---
- **TV Box Amlogic**
- Pastikan chipset didukung
---
- **MicroSD Card**
- 16GB atau lebih (Class 10/UHS-I)
---
- **Card Reader**
- USB Card Reader
---
- **Kabel Ethernet**
- Untuk koneksi network
---
- **Mouse & Keyboard**
- Untuk konfigurasi awal (opsional)
---
- **HDMI Monitor**
- Untuk melihat proses boot pertama
---
- **Toothpick / Jarum**
- Untuk masuk ke recovery/boot mode
{% /table %}

---

## Langkah 1: Download Armbian untuk Amlogic

### Pilih Image yang Tepat

1. Kunjungi halaman Armbian untuk Amlogic:\
   [github.com/ophub/amlogic-s9xxx-armbian](https://github.com/ophub/amlogic-s9xxx-armbian)
1. Pilih image sesuai chipset Anda:
   - `Armbian_xxx_Aml_s905x_bullseye_xxx.img.gz` → untuk S905X
   - `Armbian_xxx_Aml_s912_bullseye_xxx.img.gz` → untuk S912
   - `Armbian_xxx_Aml_s905x2_bullseye_xxx.img.gz` → untuk S905X2
1. **Pilih versi Server (CLI)** jika tujuannya pure server.\
   Pilih versi Desktop jika butuh GUI sesekali.

\> **Tips**: Versi **Bullseye** (Debian 11) atau **Bookworm** (Debian 12) lebih stabil untuk server. Versi **Jammy** (Ubuntu 22.04) jika butuh package lebih baru.

---

## Langkah 2: Flash Image ke MicroSD

### Menggunakan BalenaEtcher

1. Buka BalenaEtcher
1. Pilih file `.img` atau `.img.gz` Armbian
1. Pilih MicroSD Card
1. Klik **Flash**

### Menggunakan `dd` (Linux)

```bash
# Cek device SD card
lsblk

# Flash (GANTI sdX dengan device Anda!)
sudo dd if=Armbian_xxx.img of=/dev/sdX bs=4M status=progress conv=fsyn
```

## **Langkah 3: Persiapan Boot dari MicroSD**

TV Box Amlogic tidak akan otomatis boot dari MicroSD. Anda perlu memaksa masuk ke **recovery mode** atau **boot mode**.

### **Metode Toothpick (Paling Umum)**

1. **Cabut power** TV Box
1. **Masukkan MicroSD** ke slot
1. **Tekan dan tahan tombol reset** di dalam lubang AV port dengan toothpick/jarum
1. **Sambil menekan**, colokkan power adapter
1. **Tahan 5-10 detik** sampai muncul logo Armbian di layar
1. **Lepaskan tombol**

### **Metode Alternatif**

Jika tidak ada tombol reset:

- Beberapa TV Box bisa boot dari SD dengan **menggunakan app Android** seperti **LibreELEC USB-SD Creator** untuk rewrite bootloader
- Atau gunakan **Amlogic Burn Card Maker** untuk membuat SD card bootable

> **Catatan**: Jika TV Box tetap boot ke Android, kemungkinan bootloader tidak mendukung SD boot. Solusi: **flash Armbian ke internal eMMC** (dibahas di bagian lanjutan).

---

## **Langkah 4: Boot Pertama & Konfigurasi**

### **Proses Boot**

1. TV Box akan boot dari MicroSD
1. Tampilan awal akan resize filesystem (tunggu 1-2 menit)
1. Login prompt muncul

### **Login Default**

**Table**

{% table %}
- **Username**
- **Password**
---
- `root`
- `1234`
{% /table %}

### **First Boot Wizard**

Setelah login, wizard akan meminta:

1. **Ubah password root**
1. **Buat user baru** (wajib, root login via SSH akan disabled)
1. **Pilih timezone** → `Asia/Jakarta`
1. **Pilih locale** → `en_US.UTF-8`
1. **Pilih shell** → `bash`

---

## **Langkah 5: Install Armbian ke Internal eMMC (Opsional tapi Direkomendasikan)**

Boot dari MicroSD lebih lambat dan tidak stabil untuk long-term. Sebaiknya install Armbian ke **internal eMMC** TV Box.

### **Cek eMMC Terdeteksi**

**bash**

```bash
lsblk
# Cari device seperti /dev/mmcblk1 atau /dev/mmcblk2
```

### **Install ke eMMC dengan `armbian-install`**

**bash**

```bash
# Jalankan script install
sudo armbian-install

# Atau jika tidak tersedia:
sudo nand-sata-install
```

Pilihan yang muncul:

1. **Boot from eMMC** → install sistem ke eMMC
1. **Boot from SD / system on eMMC** → boot dari SD tapi rootfs di eMMC
1. **Boot from eMMC / system on SATA/NVMe** → untuk USB/SATA

Pilih **opsi 1** untuk install penuh ke eMMC.

> **Peringatan**: Proses ini akan **menghapus Android** di TV Box Anda. Pastikan sudah yakin!

Setelah selesai:

**bash**

```bash
sudo poweroff
```

Cabut MicroSD, lalu nyalakan TV Box. Sekarang Armbian akan boot langsung dari eMMC yang jauh lebih cepat.

---

## **Langkah 6: Optimasi untuk Server**

### **1. Disable GUI (Jika Install Versi Desktop)**

Jika Anda install versi desktop tapi ingin pure server:

**bash**

```bash
# Disable display manager
sudo systemctl disable lightdm gdm3 sddm

# Set multi-user target (CLI only)
sudo systemctl set-default multi-user.target

# Reboot
sudo reboot
```

### **2. Optimasi CPU Governor**

**bash**

```bash
sudo nano /etc/default/cpufrequtils
```

Isi:

**bash**

```bash
ENABLE=true
GOVERNOR=ondemand
MIN_SPEED=100000
MAX_SPEED=1900000
```

Restart:

**bash**

```bash
sudo systemctl restart cpufrequtils
```

### **3. Disable Bluetooth & WiFi (Jika Tidak Digunakan)**

Hemat RAM dan CPU:

**bash**

```bash
sudo systemctl disable bluetooth
sudo systemctl disable bluetooth.service
sudo systemctl disable NetworkManager-wait-online.service

# Blacklist driver WiFi (jika WiFi tidak stabil di Linux)
echo "blacklist dhd" | sudo tee -a /etc/modprobe.d/blacklist.conf
echo "blacklist cfg80211" | sudo tee -a /etc/modprobe.d/blacklist.conf
```

### **4. Setup Static IP**

**bash**

```bash
sudo nano /etc/network/interfaces
```

Isi:

**bash**

```bash
auto eth0
iface eth0 inet static
    address 192.168.1.100
    netmask 255.255.255.0
    gateway 192.168.1.1
    dns-nameservers 1.1.1.1 8.8.8.8
```

Restart network:

**bash**

```bash
sudo systemctl restart networking
```

### **5. Thermal Management**

TV Box Amlogic cenderung panas. Pastikan cooling cukup:

**bash**

```bash
# Cek suhu CPU
cat /sys/class/thermal/thermal_zone0/temp
# Output: 45000 = 45°C

# Install fan control (jika ada fan)
sudo apt install lm-sensors -y
```

---

## **Langkah 7: Install Docker & Deploy Aplikasi Server**

Setelah Armbian berjalan optimal, install Docker untuk menjalankan berbagai service:

### **Install Docker**

**bash**

```bash
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
newgrp docker
```

### **Install Docker Compose**

**bash**

```bash
sudo apt install docker-compose-plugin -y
```

### **Project 1: File Browser (NAS Ringan)**

**yaml**

```yaml
# docker-compose.yml
version: "3"
services:
  filebrowser:
    image: filebrowser/filebrowser:s6
    container_name: filebrowser
    ports:
      - "8080:80"
    volumes:
      - /:/srv
      - ./filebrowser.db:/database.db
    restart: unless-stopped
```

### **Project 2: Pi-hole**

**yaml**

```yaml
version: "3"
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
      WEBPASSWORD: 'admin123'
    volumes:
      - './etc-pihole:/etc/pihole'
      - './etc-dnsmasq.d:/etc/dnsmasq.d'
    restart: unless-stopped
```

### **Project 3: Uptime Kuma (Monitoring)**

**yaml**

```yaml
version: "3"
services:
  uptime-kuma:
    image: louislam/uptime-kuma:latest
    container_name: uptime-kuma
    ports:
      - "3001:3001"
    volumes:
      - ./uptime-kuma-data:/app/data
    restart: unless-stopped
```

---

## **Troubleshooting Khusus Amlogic**

### **TV Box Boot Loop ke Android**

**Solusi**: Bootloader Android masih aktif. Gunakan **Amlogic USB Burning Tool** untuk flash custom bootloader, atau gunakan **Multitool** dari balena150.

### **Ethernet Tidak Terdeteksi**

**Solusi**: Beberapa TV Box menggunakan chip Ethernet yang tidak umum. Cek dengan:

**bash**

```bash
dmesg | grep -i eth
lspci
lsusb
```

Jika tidak terdeteksi, cari driver spesifik di forum Armbian atau gunakan **USB Ethernet Adapter**.

### **HDMI Tidak Keluar Output**

**Solusi**: Beberapa TV Box membutuhkan **DTB (Device Tree Blob) yang tepat**. Ganti file `.dtb` di `/boot/dtb/amlogic/` dengan yang sesuai model TV Box Anda.

### **Panas Berlebihan & Thermal Throttling**

**Solusi**:

- Tambahkan **heatsink** pada chip Amlogic
- Pasang **fan kecil 5V** di dalam casing
- Turunkan `MAX_SPEED` di cpufrequtils
- Pastikan casing TV Box tidak tertutup rapat

### **WiFi Tidak Berfungsi**

**Solusi**: WiFi di TV Box Amlogic sangat sering **tidak ada driver Linux**. Solusi terbaik adalah menggunakan **kabel Ethernet** atau **USB WiFi Adapter** yang support Linux (Realtek RTL8812AU, MT7601U).

---

## **Perbandingan TV Box Amlogic vs Raspberry Pi**

**Table**

{% table %}
- **Spesifikasi**
- **TV Box Amlogic S905X2**
- **Raspberry Pi 4 2GB**
---
- **Harga**
- Rp 250-350 ribu
- Rp 600-800 ribu
---
- **CPU**
- Quad-core Cortex-A53
- Quad-core Cortex-A72
---
- **RAM**
- 2-4GB DDR3/4
- 2-8GB LPDDR4
---
- **Storage**
- eMMC 8-16GB + MicroSD
- MicroSD only
---
- **Ethernet**
- 100M / Gigabit
- Gigabit
---
- **USB**
- USB 2.0 / 3.0
- USB 2.0 + 3.0
---
- **GPIO**
- Tidak ada
- 40-pin GPIO
---
- **Komunitas**
- Aktif (China/Internasional)
- Sangat besar
{% /table %}

> **Kesimpulan**: Jika Anda butuh **server headless tanpa GPIO**, TV Box Amlogic adalah pilihan **jauh lebih hemat biaya**. Raspberry Pi unggul jika butuh GPIO, kamera, atau HAT.

---

## **Kesimpulan**

TV Box Amlogic yang terbengkalai bisa dihidupkan kembali sebagai **Linux server** yang powerful dengan Armbian. Dengan biaya di bawah Rp 400 ribu, Anda mendapatkan:

- Server Linux 24/7 dengan konsumsi daya **5-10 watt**
- Storage internal eMMC yang lebih cepat dari SD Card
- Kemampuan menjalankan **Docker, NAS, Pi-hole, Home Assistant**
- Upgrade path ke **eMMC** untuk performa lebih baik

Meskipun ada keterbatasan seperti **tidak adanya GPIO** dan **WiFi yang sering tidak support**, untuk use case **server jaringan dan home automation**, TV Box Amlogic + Armbian adalah kombinasi yang sangat **worth it**.

**Siap mengubah TV Box tua Anda menjadi server produktif?** 🚀
