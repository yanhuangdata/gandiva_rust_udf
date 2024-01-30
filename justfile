set shell := ["zsh", "-uc"]

build build_type="":
  cargo build --workspace --lib

test:
    cargo test --workspace

clean:
    rm -fr target
