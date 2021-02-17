#!/bin/bash

set -euxo pipefail

TARGET=wasm32-unknown-unknown
NAME=elastic
BINARY=target/$TARGET/release/$NAME.wasm
DIST=www/$NAME.wasm
UNOPT=www/$NAME.unopt.wasm

cargo build --target $TARGET --release
cp $BINARY $DIST
cp $BINARY $UNOPT
wasm-strip $DIST
wasm-opt -o $DIST -Oz $DIST
wasm2wat $DIST | sed -E 's/\(export "(__data_end|__heap_base)" \([a-z 0-9]*\)\)//' | wat2wasm - -o $DIST

#python ~/Projects/emscripten/tools/wasm-sourcemap.py --dwarfdump /bin/llvm-dwarfdump -o $UNOPT.map $UNOPT
ls -l $DIST
