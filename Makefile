.PHONY: help build run test clean docker-up docker-down migrate migrate-reset fmt clippy

help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build the project
	cargo build --release

run: ## Run the development server
	cargo run

test: ## Run tests
	cargo test

clean: ## Clean build artifacts
	cargo clean

docker-up: ## Start Docker services (PostgreSQL + Redis)
	docker-compose up -d

docker-down: ## Stop Docker services
	docker-compose down

docker-logs: ## View Docker service logs
	docker-compose logs -f

migrate: ## Run database migrations
	psql -h localhost -U airbnb_user -d airbnb_db -f migrations/001_initial_schema.sql

migrate-reset: ## Reset database and run migrations
	docker-compose down -v
	docker-compose up -d postgres
	sleep 5
	psql -h localhost -U airbnb_user -d airbnb_db -f migrations/001_initial_schema.sql

fmt: ## Format code
	cargo fmt

clippy: ## Run clippy linter
	cargo clippy

check: fmt clippy test ## Run all checks (format, lint, test)

dev-setup: docker-up migrate ## Setup development environment

dev: dev-setup run ## Setup and run development environment
