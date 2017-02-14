#!/bin/bash

# This file needs to be run from the git root directory!

# Exit script on the first error
set -o errexit -o nounset

export RUSTFLAGS="--deny warnings"

taskdir=/chess

echo ""
echo "=== Teste das Projekt in $taskdir"

manifest="$taskdir/Cargo.toml"
if [ -e "$manifest" ]; then
    echo "=== Cargo-Manifest gefunden in '$manifest' -> Cargo-Modus"
    cargo test --manifest-path "$manifest"
elif [ $(ls $taskdir/*.rs | wc -l) -ne 0 ]; then
    echo "=== Sourcedatei(en) gefunden -> rustc-Modus"
    for srcfile in $taskdir/*.rs; do
        echo "=== Kompiliere und teste '$srcfile'..."
        rustc "$srcfile"
        rustc --test -o rustctest "$srcfile"
        ./rustctest
    done
else
    echo ""
    echo "!!! Falsch konfigurierter Aufgabenordner oder ungelöste Aufgabe"
    echo "!!! Bitte .rs-Dateien in '$taskdir' ablegen"
    echo "!!! Oder ein Cargo-Projekt mit 'cargo init' darin erzeugen"
    echo "!!! Alternativ den Ordner löschen"
    exit 1
fi
done