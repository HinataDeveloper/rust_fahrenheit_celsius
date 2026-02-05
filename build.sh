#! /bin/bash

clear
cargo check && cargo build && cargo run -- $1 $2 $3
