.PHONY: run format check allow-input revoke-input dev

DEV_USER := $(shell id -un)

run: format
	cargo run

format:
	cargo fmt
	cargo clippy

check:
	cargo check

allow-input:
	sudo setfacl -m u:$(DEV_USER):r-- /dev/input/event*

revoke-input:
	sudo setfacl -x u:$(DEV_USER) /dev/input/event*

dev: allow-input
	$(MAKE) run

