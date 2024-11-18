#!/usr/bin/env bash

cargo build --release --package clitinfo
cargo build --release --package countlines
cargo build --release --package installer
cargo build --release --package uninstaller
cargo build --release --package listdirs
cargo build --release --package mimers
cargo build --release --package progressbar
cargo build --release --package rot
cargo build --release --package sysinfo
cargo build --release --package timers

# mkdir bin
cp target/release/clitinfo bin
cp target/release/clit-install bin
cp target/release/clit-uninstall bin
cp target/release/countlines bin
cp target/release/listdirs bin
cp target/release/mimers bin
cp target/release/progressbar bin
cp target/release/rot bin
cp target/release/sysinfo bin
cp target/release/timers bin

echo "Generating completion file for clitinfo";
./bin/clitinfo --generate-completion zsh > completion/clitinfo

echo "Generating completion file for countlines";
./bin/countlines --generate-completion zsh > completion/countlines

echo "Generating completion file for clit-install";
./bin/clit-install --generate-completion zsh > completion/clit-install

echo "Generating completion file for clit-uninstall";
./bin/clit-uninstall --generate-completion zsh > completion/clit-uninstall

echo "Generating completion file for listdirs";
./bin/listdirs --generate-completion zsh > completion/listdirs

echo "Generating completion file for mimers";
./bin/mimers --generate-completion zsh > completion/mimers

echo "Generating completion file for progressbar";
./bin/progressbar generate-completion zsh > completion/progressbar

echo "Generating completion file for rot";
./bin/rot --generate-completion zsh > completion/rot

echo "Generating completion file for sysinfo";
./bin/sysinfo --generate-completion zsh > completion/sysinfo

echo "Generating completion file for timers";
./bin/timers generate-completion zsh > completion/timers
