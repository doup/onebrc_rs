#!/bin/bash
set -e

if [[ -z "$1" ]]; then
    echo "Usage: $0 <number>"
    exit 1
fi

cargo build --release
time ./target/release/onebrc_rs "$1"
diff "data/measurements-$1.out" "data/measurements-$1.out.result"
