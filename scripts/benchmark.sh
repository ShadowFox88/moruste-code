#!/usr/bin/env bash
TO_RUN="moruste-code this is a message"

hyperfine -Nn debug -w 10000 -r 10000 -s "cargo build" "target/debug/$TO_RUN"
hyperfine -Nn release -w 10000 -r 10000 -s "cargo build --release" "target/release/$TO_RUN"
