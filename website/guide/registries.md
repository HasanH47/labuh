# Private Registries

Untuk mendeploy image dari repository private (seperti Docker Hub Private, GHCR, atau GitLab), Labuh memungkinkan Anda menyimpan kredensial registry secara aman.

## Menambahkan Registry

1. Pergi ke menu **Settings** > **Registries**.
2. Klik **Add Registry**.
3. Isi detail berikut:
   - **Name**: Nama alias untuk registry Anda.
   - **Server Address**: URL registry (misal: `index.docker.io` atau `ghcr.io`).
   - **Username**: Username Anda di registry tersebut.
   - **Password/Token**: Password atau Personal Access Token (PAT).
4. Klik **Save**.

## Menggunakan Registry saat Deployment

Setelah registry ditambahkan, Labuh akan otomatis menggunakan kredensial tersebut saat Anda melakukan operasi **Pull** atau **Deploy** pada image yang beralamat di registry yang cocok.

> [!TIP]
> Untuk GitHub Container Registry (GHCR.io), sangat disarankan menggunakan **Personal Access Token (PAT)** dengan scope `read:packages`.
