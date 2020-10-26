#!/bin/bash

function bench() {
    hyperfine --warmup 3 "cwa $1" "pahi $1"
}

bench blake2stress.wasm
bench bigjson.wasm
bench cpustrain.wasm
bench fibber.wasm
bench k8sparse.wasm
