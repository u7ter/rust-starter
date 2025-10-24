.PHONY: help build run test clean docker-build docker-up docker-down migrate-up migrate-down fmt lint watch

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-20s %s\n", $$1, $$2}'

build: ## Build the project in release mode
	cargo build --release

run: ## Run the project in development mode
	cargo run

test: ## Run all tests
	cargo test

test-verbose: ## Run tests with verbose output
	cargo test -- --nocapture

clean: ## Clean build artifacts
	cargo clean

docker-build: ## Build Docker image
	docker build -t tust-starter:latest .

docker-up: ## Start all services with Docker Compose
	docker compose up -d

docker-down: ## Stop all services
	docker compose down

docker-logs: ## View logs from all services
	docker compose logs -f

migrate-up: ## Run database migrations
	sqlx migrate run

migrate-down: ## Revert last database migration
	sqlx migrate revert

fmt: ## Format code with rustfmt
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt -- --check

lint: ## Run clippy linter
	cargo clippy -- -D warnings

lint-fix: ## Run clippy with automatic fixes
	cargo clippy --fix

watch: ## Run with auto-reload on file changes
	cargo watch -x run

check: ## Run all checks (fmt, clippy, test)
	make fmt-check
	make lint
	make test

dev: ## Start development environment (DB + watch)
	docker compose up -d postgres
	@echo "Waiting for database to be ready..."
	@sleep 3
	make migrate-up
	make watch

setup: ## Initial project setup
	cp .env.example .env
	@echo "Please edit .env file with your configuration"
	cargo install cargo-watch sqlx-cli --no-default-features --features postgres

db-reset: ## Reset database (drop and recreate)
	docker compose down -v
	docker compose up -d postgres
	@sleep 3
	make migrate-up
