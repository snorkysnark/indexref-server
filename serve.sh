#!/bin/bash

kitty -d "$PWD/indexref-client" npm run dev &disown
cargo watch -x 'run --bin indexref-serve'
