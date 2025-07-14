#!/bin/bash

cargo publish && \
wasm-pack build --target web --out-dir pkg-wasm --scope aimoverse && \
cd pkg-wasm && \
npm publish --access public
