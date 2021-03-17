#! /bin/bash

set -e
cd coprocessor
cargo build --release
./board/runner.sh ./target/avr-atmega328p/release/coprocessor.elf