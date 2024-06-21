main:
	cargo build

build:
	cargo build --release

install:
	cargo install --path delog

test-command:
	cargo run --bin delog -- cargo run --bin log_simulator
