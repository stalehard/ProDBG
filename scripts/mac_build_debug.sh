#!/bin/bash 
bin/macosx/tundra/tundra2 -v macosx-clang-debug
cargo build
cd src/plugins/bitmap_memory
cargo build
cd ../../prodbg/tests/rust_api_test
cargo build
cd ../../../../

