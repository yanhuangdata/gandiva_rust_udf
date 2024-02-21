#!/usr/bin/env bash

set -euxo pipefail

VERSION=${REF#"refs/tags/"}
DIST=`pwd`/dist

echo "Packaging $VERSION for $TARGET..."

echo "Installing rust toolchain for $TARGET..."
rustup target add $TARGET

echo "Building..."
RUSTFLAGS="--codegen target-feature=+crt-static $TARGET_RUSTFLAGS" \
  cargo build --workspace --lib --target $TARGET --release
LIBRARY=libudf_core
LIB_SOURCE=target/$TARGET/release/libudf_core
EXT=so

if [[ $OS == macos-latest ]]; then
  EXT=dylib
fi

LIB_SOURCE=$LIB_SOURCE.$EXT
ARCHIVE=$DIST/$LIBRARY_$VERSION_$TARGET.$EXT

echo "Creating release archive..."
mkdir dist
cp $LIB_SOURCE $ARCHIVE
echo "archive=$ARCHIVE" >> $GITHUB_OUTPUT
