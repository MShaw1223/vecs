#!/bin/bash
cd "$(dirname "$0")"

# 9 lots of dashes
dashes="--- --- --- --- --- --- --- --- ---"

PrintDashes(){
    printf "%s\n" "$dashes"
}

for binFilePath in src/bin/*.rs; do
    binFile=$(basename "$binFilePath" .rs)
    PrintDashes
    echo "Running $binFile..."
    cargo run --bin $binFile
    echo "Finished $binFile..."
    PrintDashes
done
