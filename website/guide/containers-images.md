# Images & Containers

Meskipun Stack (Docker Compose) adalah cara utama mengelola aplikasi di Labuh, Anda tetap memiliki kontrol penuh atas individual image dan container.

## Manajemen Image

Di menu **Images**, Anda dapat melihat daftar semua Docker image yang tersimpan di host:

- **Pull Image**: Menarik image baru dari registry publik atau private.
- **Cleanup**: Menghapus image yang sudah tidak terpakai untuk menghemat ruang disk.
- **Filter**: Mencari image berdasarkan nama atau tag.

## Manajemen Container

Menu **Containers** menampilkan semua container yang berjalan di server, baik yang bagian dari Stack maupun container standalone:

- **Status Overview**: Melihat container mana yang sedang `running`, `exited`, atau `paused`.
- **Quick Links**: Akses cepat ke log, statistik, dan shell terminal.
- **Details**: Melihat konfigurasi detail seperti Environment Variables, Network, Port mapping, dan Mounts.

### Log Viewer

Labuh menyediakan Log Viewer dengan fitur **Streaming (SSE)**. Log akan muncul seketika saat aplikasi Anda menghasilkan output, tanpa perlu melakukan refresh halaman.

### Terminal Interaktif

Anda dapat masuk ke dalam shell container (`/bin/sh` atau `/bin/bash`) langsung dari dashboard. Ini sangat berguna untuk debugging cepat atau menjalankan perintah administratif di dalam container.
