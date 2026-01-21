# Labuh - User Guide

## Getting Started

### 1. Access the Dashboard

Open your browser and navigate to:

- Local development: `http://localhost:5173`
- Production: `https://your-domain.com`

### 2. Create an Account

1. Click **Register** on the landing page
2. Enter your email and password
3. Click **Create Account**

### 3. Login

Use your registered email and password to login.

---

## Dashboard Overview

The dashboard shows:

- **Running Containers** - Number of active containers
- **Images** - Local Docker images count
- **Projects** - Your deployed applications
- **System Status** - Backend health

### System Overview Panel

- **CPU Cores** - Server CPU count
- **Memory** - RAM usage percentage and free space
- **Disk** - Storage usage and availability
- **Uptime** - Server uptime and load average

---

## Managing Containers

### View Containers

Go to **Dashboard → Containers** to see all containers.

Each container shows:

- Name and image
- Status (running/stopped/exited)
- Action buttons

### Create a Container

1. Click **Create Container**
2. Enter a name (e.g., `my-nginx`)
3. Enter image (e.g., `nginx:latest`)
4. Click **Create**

### Container Actions

| Button     | Action                  |
| ---------- | ----------------------- |
| ▶️ Play    | Start stopped container |
| ⏹️ Stop    | Stop running container  |
| 🔄 Restart | Restart container       |
| 🗑️ Delete  | Remove container        |

---

## Managing Images

### Pull an Image

1. Go to **Dashboard → Images**
2. Enter image name (e.g., `nginx:latest`)
3. Click **Pull**

Examples:

- `nginx:latest` - Official nginx
- `postgres:16-alpine` - PostgreSQL
- `ghcr.io/user/repo:tag` - GitHub Container Registry

### Delete an Image

Click the 🗑️ button next to the image.

> ⚠️ You cannot delete images that are in use by containers.

---

## Managing Projects

Projects are a higher-level abstraction over containers, making deployment easier.

### Create a Project

1. Go to **Dashboard → Projects**
2. Click **Create Project**
3. Fill in:
   - **Name** - Project name (e.g., `my-app`)
   - **Description** - Optional description
   - **Docker Image** - Image to deploy
   - **Container Port** - Port the app listens on
4. Click **Create Project**

### Deploy a Project

1. Click **🚀 Deploy** on the project card
2. Labuh will:
   - Pull the latest image
   - Create a container
   - Start the container
   - Configure routing

### Project Lifecycle

| Status  | Description                      |
| ------- | -------------------------------- |
| stopped | Project created but not deployed |
| running | Container is running             |
| error   | Deployment failed                |

### Project Actions

- **Deploy** - Start/restart the project
- **Stop** - Stop the running container
- **Restart** - Restart the container
- **Delete** - Remove project and container

---

## Tips & Best Practices

### Use Project Names Carefully

Project names become:

- Container names (prefixed with `labuh-`)
- URL slugs (e.g., `my-app.your-domain.com`)

### Port Mapping

Ensure the port you specify matches what your application listens on:

- nginx: 80
- Node.js apps: 3000 (typically)
- Python Flask: 5000

### Environment Variables

When creating a project, you can pass environment variables as JSON:

```json
{
  "DATABASE_URL": "postgres://...",
  "API_KEY": "secret123"
}
```

---

## Keyboard Shortcuts

| Shortcut     | Action           |
| ------------ | ---------------- |
| `D` then `D` | Toggle dark mode |
| Click logo   | Go to dashboard  |

---

## Troubleshooting

### "Network error" when pulling images

1. Check Docker is running
2. Check internet connectivity
3. Check image name is correct

### Container won't start

1. Check logs: Click on container → View Logs
2. Common issues:
   - Port already in use
   - Image not found
   - Invalid environment variables

### Dashboard shows 0 containers

1. Make sure Docker is accessible to Labuh
2. Check backend logs: `journalctl -u labuh -f`
