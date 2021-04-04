SHELL                      := /bin/bash

MODULE_PATH                := common

CARGO_BIN                  := $(shell which cargo)

WORKSPACE_CARGO_FILE       := Cargo.toml

README.md: README.tpl $(WORKSPACE_CARGO_FILE) $(MODULE_PATH)/Cargo.toml $(MODULE_PATH)/src/lib.rs
	$(CARGO_BIN) readme -r $(MODULE_PATH) -t ../README.tpl -o ../$@

