#!/bin/bash

set -euxo pipefail

TARGET=wasm32-unknown-unknown
NAME=elastic
BINARY=target/$TARGET/release/$NAME.wasm
DIST=www/$NAME.wasm

cargo build --target $TARGET --release
cp $BINARY $DIST
wasm-strip $DIST
wasm-opt -o $DIST -Oz $DIST
ls -l $DIST
