
default: chk

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

chk:
    cargo check --features all

lint:
    cargo clippy
