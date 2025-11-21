#!/bin/bash

ENV_FILE=.env

export NODE_HOSTNAME=$(hostname)
export $(grep -v '^#' $ENV_FILE | xargs)
clear && cargo fmt && RUST_LOG=debug cargo run
unset NODE_HOSTNAME
unset $(grep -v '^#' $ENV_FILE | cut -d= -f1)
