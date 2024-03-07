#!/usr/bin/env bash

set -euxo pipefail

VERSION=${REF#"refs/tags/"}
DIST=`pwd`/dist

echo "Packaging $VERSION for $TARGET..."

echo "Installing rust toolchain for $TARGET..."
rustup target add $TARGET

echo "Generate udf registry ..."
gen-udf-reg

echo "Building..."
RUSTFLAGS=" $TARGET_RUSTFLAGS" \
  cargo build --workspace --lib --target $TARGET --release

echo "Creating release archive..."
LIBRARY=libgandiva_rust_udf
LIB_SOURCE=target/$TARGET/release/$LIBRARY
EXT=so

if [[ $OS == macos-latest ]]; then
  EXT=dylib
fi

LIB_SOURCE=$LIB_SOURCE.$EXT
ARCHIVE=$DIST/${LIBRARY}_${VERSION}_${TARGET}.$EXT

mkdir dist
cp $LIB_SOURCE $ARCHIVE
echo "archive=$ARCHIVE" >> $GITHUB_OUTPUT
