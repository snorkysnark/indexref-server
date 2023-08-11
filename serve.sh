#!/bin/bash

kitty -d "$PWD/indexref-client" npm run dev &disown
cargo run --bin indexref-serve
