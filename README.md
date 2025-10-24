# Rust Starter - RESTful API

![CI](https://img.shields.io/badge/ci-github--actions-blue)

A production-ready RESTful HTTP API template built with **Rust** (axum + sqlx), featuring JWT authentication, rate limiting, structured logging, and PostgreSQL.

## Features

- **RESTful HTTP API** with clean architecture
- **JWT Authentication** (HS256) with secure secret storage
- **Rate Limiting** (token-bucket algorithm via governor)
- **Structured logging and tracing** with `tracing`
- **PostgreSQL** with `sqlx` (async, no ORM)
- **Database migrations** via `sqlx-cli`
- **Docker & Docker Compose** for local and production deployment
- **Live reload** in development via `cargo-watch`
- **CI/CD** with GitHub Actions
- **OpenAPI / Swagger** documentation via `utoipa`
- **Graceful Shutdown** and health-check endpoints

## Project Structure

```
.
├── src/
│   ├── main.rs          # Entry point (bootstrap)
│   ├── config.rs        # Configuration from .env
│   ├── routes.rs        # Routing and handler composition
│   ├── handlers/        # HTTP handlers
│   ├── services/        # Business logic
│   ├── repositories/    # Database access (sqlx)
│   ├── models/          # DTOs / models
│   └── middleware/      # Auth, rate limit, logging
├── migrations/          # SQL migrations
├── Dockerfile
├── docker-compose.yml
├── Makefile
└── .env.example
```

## Prerequisites

- Rust 1.70+ (stable toolchain)
- PostgreSQL 14+
- Docker & Docker Compose (for containerization)
- sqlx-cli (optional, for running migrations manually)

## Quick Start

### 1. Setup

```bash
# Copy environment variables
cp .env.example .env

# Edit .env: set DATABASE_URL, JWT_SECRET, etc.
```

### 2. Local Development

#### Option A — With auto-reload (recommended)

```bash
# Install cargo-watch if needed
cargo install cargo-watch

# Start database
docker compose up -d postgres

# Wait for database to be ready
sleep 3

# Run migrations
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run

# Run with live reload
cargo watch -x run
```

Or use the Makefile:

```bash
make dev
```

#### Option B — Standard

```bash
# Start database
docker compose up -d postgres

# Run migrations
sqlx migrate run

# Run the application
cargo run
```

The API will be available at `http://localhost:8080` (or the port specified in .env).

### 3. Docker (Full Stack)

```bash
# Start all services (database + API)
docker compose up -d

# View logs
docker compose logs -f

# Stop services
docker compose down
```

Or use the Makefile:

```bash
make docker-up
make docker-logs
make docker-down
```

## API Endpoints

### Health Checks

- `GET /healthz` — Health check (verifies database connection)
- `GET /ready` — Readiness check

### Authentication

- `POST /auth/register` — Register a new user
- `POST /auth/login` — Login and receive JWT token

### Documentation

- `GET /api-docs` — OpenAPI/Swagger UI (development only)

### Example: Register

```bash
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password123"}'
```

Response:
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

### Example: Login

```bash
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "password123"}'
```

### Example: Protected Endpoint

```bash
curl -X GET http://localhost:8080/protected-endpoint \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Makefile Commands

```bash
make help          # Show available commands
make build         # Build in release mode
make run           # Run in development mode
make test          # Run tests
make clean         # Clean build artifacts
make docker-build  # Build Docker image
make docker-up     # Start all services with Docker Compose
make docker-down   # Stop all services
make migrate-up    # Run SQL migrations
make fmt           # Format code
make lint          # Run clippy linter
make watch         # Run with auto-reload
make dev           # Start development environment (DB + watch)
make setup         # Initial project setup
```

## Environment Variables

Create a `.env` file based on `.env.example`:

| Variable | Description | Example |
|----------|-------------|---------|
| `SERVER_PORT` | HTTP server port | `8080` |
| `SERVER_HOST` | Server bind address | `0.0.0.0` |
| `DATABASE_URL` | PostgreSQL connection string | `postgres://user:pass@localhost/db` |
| `JWT_SECRET` | Secret for JWT signing | *required* |
| `JWT_EXPIRATION_HOURS` | JWT token expiration time | `24` |
| `RATE_LIMIT_RPS` | Rate limit (requests per second) | `10` |
| `RATE_LIMIT_BURST` | Rate limit burst size | `20` |
| `ENV` | Environment (development/production) | `development` |
| `RUST_LOG` | Log level configuration | `info,tust_starter=debug` |

## Database Migrations

Migrations are stored in `migrations/`. Use `sqlx-cli` to run them:

```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Create new migration
sqlx migrate add <migration_name>
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Or use Makefile
make test
```

## CI/CD

GitHub Actions workflow (`.github/workflows/ci.yml`) includes:

1. **Format Check** - `cargo fmt --check`
2. **Linting** - `cargo clippy`
3. **Tests** - `cargo test`
4. **Build** - `cargo build --release`

## Security Best Practices

1. **Use a strong `JWT_SECRET`** (never commit to repository)
2. **Hash passwords** with Argon2 (already implemented)
3. **Parameterized SQL queries** via `sqlx` (prevents SQL injection)
4. **Disable Swagger in production** (already implemented)
5. **Use HTTPS** in production
6. **Keep dependencies updated** - `cargo update`

## Architecture

### Layers

1. **Handlers** - HTTP request/response handling
2. **Services** - Business logic
3. **Repositories** - Database access
4. **Models** - Data structures and DTOs

### Middleware

- **Authentication** - JWT token verification
- **Rate Limiting** - Token bucket algorithm
- **Tracing** - Request/response logging

## Development Tips

### Hot Reload

```bash
cargo watch -x run
```

### Database Reset

```bash
make db-reset
```

### View Logs

```bash
# Docker logs
docker compose logs -f api

# Or filter by service
docker compose logs -f postgres
```

## Production Deployment

1. Set `ENV=production` in your environment
2. Use a strong `JWT_SECRET`
3. Configure proper `DATABASE_URL`
4. Build with `cargo build --release`
5. Run migrations: `sqlx migrate run`
6. Start the binary: `./target/release/tust-starter`

Or use Docker:

```bash
docker build -t tust-starter:latest .
docker run -p 8080:8080 --env-file .env tust-starter:latest
```

## Troubleshooting

### Database Connection Issues

```bash
# Check if PostgreSQL is running
docker compose ps

# View database logs
docker compose logs postgres

# Restart database
docker compose restart postgres
```

### Migration Errors

```bash
# Reset database
make db-reset

# Or manually
docker compose down -v
docker compose up -d postgres
sqlx migrate run
```

## License

This is a template - use as you wish.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## Resources

- [Axum Documentation](https://docs.rs/axum)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Tokio Documentation](https://docs.rs/tokio)
- [Utoipa Documentation](https://docs.rs/utoipa)
