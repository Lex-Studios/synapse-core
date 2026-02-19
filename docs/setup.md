# Synapse Core — Setup Guide

This guide walks through setting up a local development environment, running the service, and deploying with Docker.

> For an overview of the system design and module responsibilities, see [architecture.md](architecture.md).

---

## Prerequisites

| Tool        | Version        | Installation                               |
|-------------|----------------|---------------------------------------------|
| Rust        | 1.84+ (stable) | [rustup.rs](https://rustup.rs/)            |
| PostgreSQL  | 14+            | Via Docker (recommended) or native install |
| Docker      | 20+            | [docker.com](https://docs.docker.com/get-docker/) |
| sqlx-cli    | 0.7+           | `cargo install sqlx-cli`                   |

---

## 1. Clone the Repository

```bash
git clone https://github.com/synapse-bridgez/synapse-core.git
cd synapse-core
```

---

## 2. Environment Variables

Copy the example env file and customize:

```bash
cp .env.example .env
```

| Variable              | Required | Default | Description                          |
|-----------------------|----------|---------|--------------------------------------|
| `DATABASE_URL`        | ✅       | —       | PostgreSQL connection string         |
| `SERVER_PORT`         | ❌       | `3000`  | Port for the HTTP server             |
| `STELLAR_HORIZON_URL` | ✅       | —       | Stellar Horizon API endpoint         |

**Example `.env`:**

```env
SERVER_PORT=3000
DATABASE_URL=postgres://synapse:synapse@localhost:5432/synapse
STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org
```

---

## 3. Database Setup

### Option A: Docker (Recommended)

Start a PostgreSQL container:

```bash
docker run --name synapse-postgres \
  -e POSTGRES_USER=synapse \
  -e POSTGRES_PASSWORD=synapse \
  -e POSTGRES_DB=synapse \
  -p 5432:5432 \
  -d postgres:14-alpine
```

Verify it's running:

```bash
docker exec -it synapse-postgres pg_isready -U synapse
```

### Option B: Native PostgreSQL

If you have PostgreSQL installed locally:

```bash
createuser synapse --password  # enter "synapse" when prompted
createdb synapse --owner=synapse
```

Update `DATABASE_URL` in `.env` to match your local credentials.

---

## 4. Run Database Migrations

Migrations run **automatically** when the app starts. To run them manually:

```bash
cargo install sqlx-cli  # if not already installed
sqlx migrate run
```

The migration file `migrations/20250216000000_init.sql` creates the `transactions` table and indexes. See [architecture.md](architecture.md) for the full schema.

---

## 5. Build & Run

### Development

```bash
cargo run
```

You should see output like:

```
INFO  synapse_core > Database migrations completed
INFO  synapse_core > listening on 0.0.0.0:3000
```

### Release Build

```bash
cargo build --release
./target/release/synapse-core
```

### Verify

```bash
curl http://localhost:3000/health
# => OK
```

---

## 6. Running Tests

### Create a Test Database

```bash
docker exec -it synapse-postgres psql -U synapse -c "CREATE DATABASE synapse_test;"
```

### Run Tests

```bash
DATABASE_URL=postgres://synapse:synapse@localhost:5432/synapse_test cargo test
```

> **Note:** Some warnings about unused imports or dead code are expected — they correspond to features planned for future issues.

### Lint & Format

```bash
cargo fmt -- --check   # check formatting
cargo fmt              # auto-format
cargo clippy           # lint
```

---

## 7. Docker Compose (Full Stack)

Spin up both PostgreSQL and the application:

```bash
docker compose up --build
```

This starts:

| Service  | Container           | Port  | Description             |
|----------|---------------------|-------|-------------------------|
| postgres | `synapse-postgres`  | 5432  | PostgreSQL 14 Alpine    |
| app      | `synapse-app`       | 3000  | synapse-core server     |

The app waits for PostgreSQL's health check to pass before starting. Migrations run automatically on boot.

To stop:

```bash
docker compose down
```

To also remove the database volume:

```bash
docker compose down -v
```

---

## 8. Docker (Standalone)

Build the image manually:

```bash
docker build -t synapse-core .
```

Run it (assumes PostgreSQL is reachable at the given URL):

```bash
docker run -p 3000:3000 \
  -e DATABASE_URL=postgres://synapse:synapse@host.docker.internal:5432/synapse \
  -e SERVER_PORT=3000 \
  -e STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org \
  synapse-core
```

> **Note:** Use `host.docker.internal` (macOS/Windows) or `172.17.0.1` (Linux) to reach the host machine's PostgreSQL from inside a container.

---

## 9. Project Structure

```
synapse-core/
├── Cargo.toml               # Dependencies and workspace config
├── .env.example              # Example environment variables
├── docker-compose.yml        # Full-stack Docker setup
├── dockerfile                # Multi-stage Rust build
├── migrations/
│   └── 20250216000000_init.sql  # Initial schema migration
├── docs/
│   ├── architecture.md       # System design & module docs
│   └── setup.md              # This file
├── src/
│   ├── main.rs               # Entry point, server setup, migrations
│   ├── config.rs             # Environment variable configuration
│   ├── error.rs              # Custom error types (planned)
│   ├── db/
│   │   ├── mod.rs            # Connection pool creation
│   │   └── models.rs         # Transaction struct & tests
│   ├── handlers/
│   │   ├── mod.rs            # Health check handler
│   │   └── webhook.rs        # Callback handler (planned)
│   ├── services/
│   │   ├── mod.rs            # Service exports (planned)
│   │   └── transaction_processor.rs  # Business logic (planned)
│   └── stellar/
│       ├── mod.rs            # Stellar module exports (planned)
│       └── client.rs         # Horizon API client (planned)
├── tests/
│   └── integration_test.rs   # Integration tests (planned)
└── .github/
    └── workflows/
        └── rust.yml          # CI pipeline
```

---

## API Endpoints

| Method | Path                     | Status       | Description                              |
|--------|--------------------------|--------------|------------------------------------------|
| GET    | `/health`                | ✅ Active    | Health check — returns `"OK"`            |
| POST   | `/callback/transaction`  | 🚧 Planned  | Receive Stellar Anchor Platform webhooks |
| GET    | `/transactions`          | 🚧 Planned  | List transactions with pagination        |
| GET    | `/transactions/:id`      | 🚧 Planned  | Get a single transaction by UUID         |

---

## Troubleshooting

| Problem                                | Solution                                                      |
|----------------------------------------|---------------------------------------------------------------|
| `DATABASE_URL must be set`             | Ensure `.env` exists and `DATABASE_URL` is set                |
| `Failed to connect to test DB`         | Create the test DB: `CREATE DATABASE synapse_test;`           |
| `SQLX_OFFLINE` errors in CI           | Run `cargo sqlx prepare` locally to generate offline query data |
| Port already in use                    | Change `SERVER_PORT` in `.env` or stop the conflicting process |
| Docker connection refused to Postgres  | Use `host.docker.internal` or `172.17.0.1` instead of `localhost` |
