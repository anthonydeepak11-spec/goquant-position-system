# GoQuant — Position Management Backend

A small position management backend for a simplified leveraged trading platform.  
Built with **Rust**, **Axum**, **SQLx**, and **PostgreSQL**.  
Provides APIs to **open**, **list**, **fetch**, and **close** positions.

---

## Quick status
This repository contains the full project source and is already pushed to GitHub.  
If you see `.git-backup` folders, they are old nested-repository backups — harmless and can be cleaned later.

---

## Table of contents
- [Project overview](#project-overview)
- [Folder structure](#folder-structure)
- [Prerequisites](#prerequisites)
- [Setup (local)](#setup-local)
- [Environment variables (.env example)](#environment-variables-env-example)
- [Build & run](#build--run)
- [API documentation](#api-documentation)
- [Tests & migrations](#tests--migrations)
- [Commit message guide](#commit-message-guide)
- [Submission email template](#submission-email-template)

---

## Project overview
This backend implements a minimal position management API for leveraged trading flows:
- Open/create a position  
- List positions  
- Get position by id  
- Close a position  

Tech stack used:
- **Rust**
- **Axum**
- **SQLx**
- **PostgreSQL**

---

## Folder structure
```
goquant-position-system/
├─ Cargo.toml.txt
├─ README.md
├─ backend/
│  ├─ Cargo.toml
│  ├─ src/
│  │  ├─ main.rs
│  │  ├─ api.rs
│  │  └─ margin.rs
│  └─ .env
├─ core/
│  ├─ Cargo.toml
│  └─ src/lib.rs
└─ .gitignore
```


## Prerequisites
To run or build this project locally, install the following:

- **Rust & Cargo** — https://rustup.rs  
- **PostgreSQL** (local or Docker)  
- **Git**  
- Optional: `sqlx-cli` for migrations

---

## Setup (local)

### 1. Clone the repository
```bash
git clone https://github.com/anthonydeepak11-spec/goquant-position-system.git
cd goquant-position-system/backend
```

### 2. Create the PostgreSQL database and user
```sql
CREATE DATABASE goquant_db;
CREATE USER goquant_user WITH PASSWORD 'change_this';
GRANT ALL PRIVILEGES ON DATABASE goquant_db TO goquant_user;
```
### 3. Create a `.env` file (example below)

---

## Environment variables (.env example)

Create a file named `.env` inside the `backend/` folder with the following:

```
DATABASE_URL=postgres://goquant_user:change_this@localhost:5432/goquant_db
RUST_LOG=info
PORT=8000
```

---
## Build & run

### Build:
```bash
cargo build --release
```

### Run:
```bash
cargo run
```

Windows PowerShell:
```powershell
$Env:DATABASE_URL="postgres://goquant_user:change_this@localhost:5432/goquant_db"
cargo run
```

If using SQLx migrations:
```bash
sqlx migrate run --database-url $DATABASE_URL
```

---
## API documentation

### POST /positions  
Open a new position.
```json
{
  "user_id": "user-123",
  "instrument": "BTC-USD",
  "side": "buy",
  "quantity": 0.5,
  "leverage": 10,
  "price": 54000.0
}
```

---

### GET /positions  
List all positions.

---

### GET /positions/{id}  
Fetch a position by ID.

---

### POST /positions/{id}/close  
Close an existing position.
```json
{ "close_price": 54500.0 }
```

---
## Commit message guide

Use clear, short commit messages such as:

- `Initial project import`
- `Add API routes`
- `Add database code`
- `Fix validation`
- `Docs: update README`

---


