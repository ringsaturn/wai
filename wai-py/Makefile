install:
	maturin develop

install-release:
	maturin develop --release

bench:
	pytest tests/test_benchmark.py

fmt:
	cargo fmt
	ruff check --select I --fix .
	rye fmt

lint:
	cargo clippy
	rye lint
