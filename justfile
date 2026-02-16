build-docs:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps -p merge-it --all-features --open
