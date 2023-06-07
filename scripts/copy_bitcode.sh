#!/bin/bash

# get first arg as BUILT_TYPE
if [ $# -eq 0 ]
  then
    echo "No arguments supplied"
    exit 1
fi
BUILD_TYPE=$1
TARGET_PLATFORM=x86_64-apple-darwin
GANDIVA_BITCODE_DIR="$ARROW_ROOT/cpp/cmake-build-ninja-${BUILD_TYPE}-gandiva/src/gandiva"
DEPS_DIR=./target/${TARGET_PLATFORM}/${BUILD_TYPE}/deps

test -f ${GANDIVA_BITCODE_DIR}/irhelpers.bc.bak || cp ${GANDIVA_BITCODE_DIR}/irhelpers.bc ${GANDIVA_BITCODE_DIR}/irhelpers.bc.bak
rm -f ${GANDIVA_BITCODE_DIR}/*.bc
cp ${GANDIVA_BITCODE_DIR}/irhelpers.bc.bak ${GANDIVA_BITCODE_DIR}/irhelpers.bc
cp ${DEPS_DIR}/*.bc ${GANDIVA_BITCODE_DIR}
# panic_abort is conflict with panic_unwind
rm -f ${GANDIVA_BITCODE_DIR}/panic_abort-*.bc
pushd ${GANDIVA_BITCODE_DIR} && llvm-link $(ls *.bc) -o irhelpers.bc && popd
