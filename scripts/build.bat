@ECHO OFF

cargo build --release --package clitinfo
cargo build --release --package countlines
cargo build --release --package installer
cargo build --release --package listdirs
cargo build --release --package mimers
cargo build --release --package progressbar
cargo build --release --package rot
cargo build --release --package sysinfo
cargo build --release --package timers

REM mkdir bin
cp target/release/clitinfo bin
cp target/release/clit-install bin
cp target/release/countlines bin
cp target/release/listdirs bin
cp target/release/mimers bin
cp target/release/progressbar bin
cp target/release/rot bin
cp target/release/sysinfo bin
cp target/release/timers bin

PAUSE
