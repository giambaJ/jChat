.PHONY: setup build

setup: update apply

update:
	git submodule update --init --recursive

apply:
	cd chat && git apply ../patches/jChat.patch

patch:
	cd chat && git diff > ../patches/jChat.patch

unpatch:
	cd chat && git reset --hard

build:
	cargo build --release