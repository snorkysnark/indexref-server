#!/bin/bash
set -e

APPNAME='indexref-server'
BINARY=$APPNAME

cargo build --release --features static_server
(cd ./client; npm run build)

# Delete previous
rm -r ./target/indexref-server || true
rm ./target/indexref-server.zip || true

# Archive
mkdir ./target/$APPNAME
cp ./target/release/$BINARY ./target/$APPNAME/$BINARY
cp -r ./client/dist/ ./target/$APPNAME/static
(cd ./target; zip -r -D $APPNAME.zip ./$APPNAME || 7z a -tzip $APPNAME.zip ./$APPNAME)
