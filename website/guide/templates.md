# App Templates

**App Templates** memungkinkan Anda mendeploy aplikasi populer hanya dengan beberapa klik. Labuh menyediakan galeri template yang siap pakai, mulai dari database hingga content management systems.

## Menggunakan Template

1. Buka menu **Templates** di sidebar.
2. Jelajahi galeri untuk menemukan aplikasi yang Anda butuhkan (misal: WordPress, Ghost, Redis).
3. Klik tombol **Deploy** pada template pilihan Anda.
4. Isi konfigurasi yang diperlukan:
   - **Stack Name**: Nama unik untuk stack baru Anda.
   - **Team**: Pilih team mana yang akan memiliki stack ini (jika Anda memiliki akses ke beberapa team).
5. Klik **Create Stack** dan Labuh akan mengurus sisanya!

## Mengimpor Template Custom

Selain template bawaan, Anda juga bisa menambahkan template Anda sendiri:

1. Klik tombol **Add Template** di galeri.
2. Anda memiliki dua opsi:
   - **Import via URL**: Masukkan link ke file JSON template publik.
   - **Manual JSON**: Paste konten JSON template secara langsung.
3. Setelah di-import, template akan muncul di galeri pribadi team Anda.

## Format Template

Template Labuh menggunakan format JSON yang mendefinisikan nama, deskripsi, ikon, dan konten `docker-compose.yml` yang akan digunakan saat deployment.
