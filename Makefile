# Rust project name
PROJECT_NAME := friendly-usb-tree

# Directories
SRC_DIR := src
OUT_DIR := target

# Compiler
RUSTC := rustc

# Cargo
CARGO := cargo

# Default target
.DEFAULT_GOAL := build

# Build target
.PHONY: build
build:
    $(CARGO) build

# Run target
.PHONY: run
run:
    $(CARGO) run

# Clean target
.PHONY: clean
clean:
    $(CARGO) clean
