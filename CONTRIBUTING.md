# Contributing to Labuh

Thank you for your interest in contributing to Labuh! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Code Style](#code-style)
- [Making Changes](#making-changes)
- [Adding Templates](#adding-templates)
- [Testing](#testing)
- [Pull Request Guidelines](#pull-request-guidelines)

## Development Setup

### Prerequisites

- **Rust** (latest stable) - [rustup.rs](https://rustup.rs)
- **Node.js** (v20+) - [nodejs.org](https://nodejs.org)
- **Docker** - [docker.com](https://docker.com)

### Backend Setup

```bash
cd backend

# Copy environment file
cp .env.example .env

# Run development server
cargo run

# Or with auto-reload (install cargo-watch first)
cargo install cargo-watch
cargo watch -x run
```

### Frontend Setup

```bash
cd frontend

# Install dependencies
npm install

# Run development server
npm run dev
```

### Full Stack Development

Run both backend and frontend simultaneously:

```bash
# Terminal 1 - Backend
cd backend && cargo watch -x run

# Terminal 2 - Frontend
cd frontend && npm run dev
```

The backend serves at `http://localhost:3000` and frontend dev server at `http://localhost:5173`.

## Project Structure

```
labuh/
├── backend/               # Rust backend (Axum)
│   ├── src/
│   │   ├── api/          # HTTP handlers & WebSocket
│   │   ├── domain/       # Domain models & traits
│   │   ├── infrastructure/  # Docker, database adapters
│   │   └── usecase/      # Business logic
│   └── migrations/       # SQLite migrations
├── frontend/             # SvelteKit frontend
│   └── src/
│       ├── lib/          # Components, API, stores
│       └── routes/       # Pages
├── website/              # Documentation site (VitePress)
│   ├── guide/            # User guides
│   └── api/              # API reference
├── template/             # Default compose templates (JSON)
└── deploy/               # Installation scripts
```

## Documentation

The documentation site is built with [VitePress](https://vitepress.dev) and located in `/website`.

### Setup

```bash
cd website
npm install
npm run dev
```

The docs will be available at `http://localhost:5173`.

### Writing Docs

Documentation files are in Markdown format:

- **`/guide/`** - User guides and tutorials
- **`/api/`** - API reference documentation
- **`index.md`** - Homepage

### Adding a New Page

1. Create a new `.md` file in the appropriate folder
2. Add frontmatter at the top:

   ```markdown
   ---
   title: Page Title
   description: Brief description
   ---

   # Page Title

   Content here...
   ```

3. Update sidebar in `.vitepress/config.ts` if needed

### Building

```bash
npm run build    # Build static site
npm run preview  # Preview production build
```

## Code Style

### Rust (Backend)

We use standard Rust formatting and linting:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Both must pass before committing
cargo fmt && cargo clippy
```

**Guidelines:**

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `Result<T>` for fallible operations
- Prefer `tracing` macros for logging (`tracing::info!`, `tracing::error!`)
- Keep functions focused and small

### TypeScript/Svelte (Frontend)

```bash
# Format and lint
npm run lint
npm run format

# Type check
npm run check
```

**Guidelines:**

- Use TypeScript strictly (no `any`)
- Follow existing component patterns
- Use Svelte stores for shared state

## Making Changes

### Branching Strategy

```bash
# Create feature branch from main
git checkout main
git pull origin main
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/bug-description
```

### Commit Messages

Use conventional commits:

```
feat: add stack rollback feature
fix: resolve webhook token generation
docs: update installation guide
refactor: extract compose parser to separate module
chore: update dependencies
```

## Adding Templates

Templates are JSON files in the `/template` folder. To add a new template:

### 1. Create JSON File

```bash
touch template/your-template.json
```

### 2. Template Format

```json
{
  "id": "unique-id",
  "name": "Display Name",
  "description": "Brief description of the template.",
  "icon": "lucide-icon-name",
  "compose_content": "version: '3.8'\nservices:\n  ...",
  "default_env": [
    {
      "key": "ENV_VAR_NAME",
      "value": "default_value",
      "description": "What this variable does"
    }
  ]
}
```

### 3. Icon Names

Use [Lucide](https://lucide.dev/icons) icon names (lowercase, kebab-case):

- `database` - for databases
- `globe` - for web apps
- `server` - for backend services
- `file-text` - for CMS/blogs

### 4. Compose Content

- Use `${VAR_NAME}` for environment variable substitution
- Include `restart: always` for production-ready templates
- Define volumes for persistent data

## Testing

### Backend

```bash
cd backend

# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Frontend

```bash
cd frontend

# Run tests
npm run test

# Type checking
npm run check
```

## Pull Request Guidelines

### Before Submitting

1. **Format & Lint**: Ensure code passes formatting and linting

   ```bash
   # Backend
   cargo fmt && cargo clippy

   # Frontend
   npm run lint && npm run format
   ```

2. **Test**: Run relevant tests

   ```bash
   cargo test
   npm run test
   ```

3. **Build**: Ensure production build works
   ```bash
   cargo build --release
   npm run build
   ```

### PR Description

Include:

- **What**: Brief description of changes
- **Why**: Motivation or issue being fixed
- **How**: High-level approach (if not obvious)
- **Testing**: How you tested the changes

### Review Process

1. Open PR against `main` branch
2. Ensure CI checks pass
3. Request review from maintainers
4. Address feedback
5. Squash and merge when approved

## Questions?

- Open an issue for bugs or feature requests
- Discussions for questions and ideas

Thank you for contributing! 🚀
