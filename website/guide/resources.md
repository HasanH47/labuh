# Resource Management & Monitoring

Labuh memberikan kontrol penuh atas penggunaan resource server (CPU & RAM) untuk setiap container, lengkap dengan monitoring real-time.

## Membatasi Resource (Limits)

Anda dapat mengatur batas penggunaan resource untuk setiap service dalam stack:

1. Buka halaman detail **Stack**.
2. Pilih tab **Resources**.
3. Di panel **Resource Limits**, Anda dapat mengatur:
   - **CPU Limit**: Batas penggunaan CPU (misal: 0.5 untuk setengah core).
   - **Memory Limit**: Batas RAM dalam Megabyte (MB).
4. Klik **Save** dan lakukan **Restart/Redeploy** pada stack agar perubahan diterapkan oleh Docker Engine.

## Monitoring Real-time

Labuh mencatat metrik penggunaan resource secara otomatis:

- **Dashboard Utama**: Menampilkan ringkasan penggunaan CPU, RAM, dan Disk server secara keseluruhan.
- **Stack Detail**: Di tab **Resources**, Anda dapat melihat grafik historis penggunaan CPU dan RAM untuk setiap container dalam stack.
- **Rentang Waktu**: Anda dapat mengubah filter waktu (1 jam, 6 jam, dsb) untuk melihat tren penggunaan.

## Optimalisasi untuk STB/IoT

Karena Labuh didesain ringan, metrik historis disimpan dalam SQLite internal. Untuk menjaga database tetap efisien, Labuh secara otomatis melakukan **pruning** (pembersihan) data metrik yang sudah lama (di atas 7 hari) secara berkala.
