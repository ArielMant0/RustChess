#!/bin/bash

# This file needs to be run from the git root directory!

# Exit script on the first error
set -o errexit -o nounset

export RUSTFLAGS="--deny warnings"

echo ""
echo "=== Teste Commit"

manifest="/Cargo.toml"

for directory in *; do
    if [[ -d $directory ]]; then
        if [ -e "$directory$manifest" ]; then
            echo "=== Cargo-Manifest gefunden in '$directory' -> Cargo-Modus"
            cargo test --manifest-path "$directory$manifest"
        elif [ $(ls $directory/*.rs | wc -l) -ne 0 ]; then
            echo "=== Sourcedatei(en) gefunden -> rustc-Modus"
            for srcfile in $directory/*.rs; do
                echo "=== Kompiliere und teste '$srcfile'..."
                rustc "$srcfile"
                rustc --test -o rustctest "$srcfile"
                ./rustctest
            done
        else
            echo ""
            echo "Konnte kein Cargo-Projekt oder Sourcedateien finden"
            exit 0
        fi
    fi
done