#!/usr/bin/env bash
hyperfine -N --output null -n release -w 1000 -r 10000 -s "cargo build --release" "target/release/moruste-code"
