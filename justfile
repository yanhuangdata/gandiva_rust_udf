set shell := ["zsh", "-uc"]

all: build test bitcode copy

build_deploy: build deploy

deploy: bitcode copy

build build_type="":
    RUSTFLAGS="-C target-cpu=westmere --emit=llvm-ir" cargo build --workspace --target x86_64-apple-darwin {{build_type}}

test:
    cargo test --workspace

bitcode:
    find . -name "*.ll" | xargs -I {} llvm-as {}

clean:
    rm -fr target
    find . -iname "*.bc" | xargs rm
    find . -iname "*.ll" | xargs rm

copy:
    just _copy_bitcode debug
    # just _copy_bitcode release

_copy_bitcode build_type="debug":
    ./scripts/copy_bitcode.sh {{build_type}}

nightly_rust:
    rustup default nightly-2022-08-06-x86_64-apple-darwin
