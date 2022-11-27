#!/bin/sh

# compile for windows without console
cargo rustc --release --target x86_64-pc-windows-gnu  -- -C link-args=-mwindows

# linux
cargo rustc --release --target x86_64-unknown-linux-gnu
