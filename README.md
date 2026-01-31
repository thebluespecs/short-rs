# short-it

URL shortener in Rust using Axum + Diesel + PostgreSQL.

## Tech Stack

- **Axum** - Web framework
- **Diesel** - ORM with compile-time query checking
- **PostgreSQL** - Database
- **Tokio** - Async runtime

## Getting Started

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))
- PostgreSQL
- Diesel CLI

```bash
# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres
```

### Setup

1. Clone the repo
```bash
git clone git@github.com:thebluespecs/short-it.git
cd short-it
```

2. Create `.env` file
```bash
cp .env.example .env
# Edit .env with your database credentials
```

Example `.env`:
```
PORT=8000
HOST=127.0.0.1
DATABASE_URL=postgres://username@localhost:5432/short_it
```

3. Create database and run migrations
```bash
createdb short_it
diesel migration run
```

4. Run the server
```bash
cargo run
```

## API Endpoints

### Health Check
```bash
curl http://localhost:8000/
```

### Shorten a URL
```bash
curl -X POST http://localhost:8000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://github.com/"}'
```

With expiration (in seconds):
```bash
curl -X POST http://localhost:8000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://github.com/", "expires_in_seconds": 3600}'
```

### Get URL Info
```bash
curl http://localhost:8000/:code/info
```

### Redirect
```bash
curl -L http://localhost:8000/:code
```

## Project Structure

```
src/
├── main.rs           # Entry point
├── config.rs         # Environment configuration
├── error.rs          # Error types
├── models.rs         # Database models
├── routes.rs         # Route definitions
├── state.rs          # Application state
├── db.rs             # Database module
│   ├── schema.rs     # Diesel schema (auto-generated)
│   └── repositories/ # Database operations
├── services.rs       # Business logic module
│   ├── base62.rs     # ID encoding
│   └── url.rs        # URL service
└── handlers.rs       # HTTP handlers
    ├── health.rs
    └── url.rs
```

## Migrations

```bash
# Create a new migration
diesel migration generate <name>

# Run migrations
diesel migration run

# Revert last migration
diesel migration revert
```
