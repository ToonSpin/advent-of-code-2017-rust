#!/bin/bash

for i in $(seq -w 26); do
    if [ -x "target/release/day${i}" ]; then
        echo "------------------------------------------------------------------------- DAY ${i}"
        cat "data/day${i}.txt" | "target/release/day${i}"
    fi
done
