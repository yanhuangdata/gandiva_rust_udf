#!/usr/bin/env bash

set -euxo pipefail

VERSION=${REF#"refs/tags/"}
DIST=`pwd`/dist

echo "Packaging $VERSION for $TARGET..."

echo "Installing rust toolchain for $TARGET..."
rustup target add $TARGET

echo "Building..."
cargo build --workspace --lib --target $TARGET --release

echo "Creating release archive..."
LIBRARY=libudf_core
LIB_SOURCE=target/$TARGET/release/$LIBRARY
EXT=so

if [[ $OS == macos-latest ]]; then
  EXT=dylib
fi

LIB_SOURCE=$LIB_SOURCE.$EXT
ARCHIVE=$DIST/$LIBRARY_$VERSION_$TARGET.$EXT

mkdir dist
cp $LIB_SOURCE $ARCHIVE
echo "archive=$ARCHIVE" >> $GITHUB_OUTPUT