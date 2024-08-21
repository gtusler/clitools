@ECHO OFF

cargo build --release --package clitinfo
cargo build --release --package countlines
cargo build --release --package mimers
cargo build --release --package progressbar
cargo build --release --package sysinfo
cargo build --release --package rot

PAUSE
