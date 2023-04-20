.PHONY: build

init:
	python3 scripts/setup.py

patch:
	python3 scripts/patch.py

build:
	cargo build --release