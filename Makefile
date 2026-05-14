BINARY=escrowflow_backend

.PHONY: all build run test docker-up clean

all: build

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

test:
	cargo test

docker-up:
	docker compose up --build

clean:
	cargo clean
