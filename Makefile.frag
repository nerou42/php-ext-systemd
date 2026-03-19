.DEFAULT_GOAL = target

target: src/lib.rs Cargo.lock .cargo/config.toml
	cargo build -r && \
  cp ./target/release/libsystemd.$(SHLIB_SUFFIX_NAME) ./modules/systemd.$(SHLIB_DL_SUFFIX_NAME)

test: cargo_test

cargo_test:
	cargo test

clean: cargo_clean

cargo_clean:
	cargo clean
