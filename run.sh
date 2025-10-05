#!/bin/bash

export $(grep -v '^#' .env | xargs)
clear && cargo fmt && RUST_LOG=debug cargo run
unset $(grep -v '^#' .env | cut -d= -f1)
