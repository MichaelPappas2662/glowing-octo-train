build:
	turbo type-check build --parallel
	cd packages/server && cargo build

start: build
	cd packages/web-app && pnpm start &
	cd packages/server && cargo run
