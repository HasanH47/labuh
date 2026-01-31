# Teams & Access Control

Labuh dirancang untuk lingkungan kolaboratif. Dengan fitur **Teams**, Anda dapat mengelola resource bersama anggota tim lainnya dengan kontrol akses yang presisi.

## Membuat Team

1. Klik nama team aktif di bagian atas sidebar.
2. Pilih **Manage Teams**.
3. Klik **Create Team** dan masukkan nama team baru Anda.
4. Anda akan otomatis menjadi **Owner** dari team tersebut.

## Manajemen Anggota

Anda dapat mengundang pengguna lain ke dalam team Anda:

1. Di halaman **Manage Teams**, pilih team yang ingin Anda kelola.
2. Klik tab **Members**.
3. Klik **Invite Member** dan masukkan username atau email pengguna.
4. Pilih role yang sesuai.

### Role & Izin Akses

Labuh memiliki 4 level akses:

| Role          | Deskripsi      | Izin Utama                                   |
| :------------ | :------------- | :------------------------------------------- |
| **Owner**     | Pemilik team   | Akses penuh, termasuk menghapus team.        |
| **Admin**     | Pengelola team | Manajemen anggota, stack, dan registry.      |
| **Developer** | Pengembang     | Membuat/mengubah stack dan container.        |
| **Viewer**    | Pengamat       | Hanya baca (read-only) untuk semua resource. |

## Kepemilikan Resource

Setiap Stack dan Registry di Labuh dimiliki oleh team tertentu. Pastikan Anda berada di context team yang benar (lihat selector di sidebar) saat membuat atau mencari resource.
