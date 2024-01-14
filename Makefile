# Load environment variables
include .env
export

# Start PostgreSQL and Adminer containers
start-db:
	docker start health-passport || docker run --name health-passport -e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} -p 5432:5432 -d postgres
	docker start adminer || docker run --name adminer -p 8080:8080 --link health-passport:db -d adminer

# Stop PostgreSQL and Adminer containers
stop-db:
	docker stop health-passport
	docker stop adminer

# Install dependencies
install-deps:
	cd packages/web-app && pnpm install
	cd packages/server && cargo check

# Build
build: install-deps
	turbo type-check build --parallel
	cd packages/server && cargo build

# Start the entire application including DB and Adminer
start: start-db build
	cd packages/web-app && pnpm start &
	cd packages/server && cargo run