main:
	cargo build

build:
	cargo build --release

test-command:
	cargo run --bin delog -- cargo run --bin log_simulator
