# Makefile - Exercising W3C's MNX (potential successor to MusicXML)

.PHONY: all build test clippy format
.PHONY: install regenerate fetch generate canary

# Suitable as local mimic of CI pipeline and/or as pre-commit hook:
all: test format build doc clippy

# Run canary tests before and after for context to any failures.
regenerate: canary fetch generate canary

W3C_MNX_DIR ?= w3c-mnx
SCHEMA_DIR ?= ${W3C_MNX_DIR}/docs
EXAMPLES_DIR ?= ${W3C_MNX_DIR}/doctools/media/examples/json
SCHEMA_URL ?= https://w3c.github.io/mnx/docs/mnx-schema.json
SCHEMA_LOG ?= ${W3C_MNX_DIR}/mnx-schema.log

# Accommodate running via Emacs M-x compile, which never sees ~/.bashrc
CARGO_BIN ?= ${HOME}/.cargo/bin
CRATE_DIR ?= $(shell ${CARGO_BIN}/cargo locate-project | jq .root | sed 's%/Cargo.toml%%')
PATH := "${CARGO_BIN}:${PATH}"

version:
	${CARGO_BIN}/cargo --version
	${CARGO_BIN}/rustc --version
	${CARGO_BIN}/cargo clippy --version
	echo ${CRATE_DIR}

# Generate .rs from JSON schema via typify.
# typify requires rustfmt.
install:
	PATH=${PATH} \
	  rustup component add rustfmt
	PATH=${PATH} \
	  cargo install \
	    --git=https://github.com/dpezely/typify.git \
	    --branch=dpezely/subtype-naming \
	    cargo-typify

# `wget -N ...` sets file timestamp to match server, albeit with redundant GET.
fetch:
	[ -d "${W3C_MNX_DIR}" ] \
	  || git clone https://github.com/w3c/mnx.git "${W3C_MNX_DIR}" \
	  || (cd "${W3C_MNX_DIR}/" && git pull)
	wget -N ${SCHEMA_URL} -P "${SCHEMA_DIR}/"

# The results are committed into code repo.
generate:
	RUST_LOG=debug,typify=debug \
	PATH=${PATH} \
	  cargo typify "${SCHEMA_DIR}/mnx-schema.json" > ${SCHEMA_LOG} 2>&1
	@echo '//! "MNX is a new, open standard for representing music notation for' \
	  > ${CRATE_DIR}/src/mnx.rs
	@echo '//! interchange and internal use in software applications. It builds' \
	  >> ${CRATE_DIR}/src/mnx.rs
	@echo '//! on the ideas and success of MusicXML \[...]"' >> ${CRATE_DIR}/src/mnx.rs
	@echo '//!' >> ${CRATE_DIR}/src/mnx.rs
	@echo '//! This **code has been generated**, thus modifications will be ignored.' \
	  >> ${CRATE_DIR}/src/mnx.rs
	@echo '//!' >> ${CRATE_DIR}/src/mnx.rs
	@echo '//! Official documentation: <https://w3c.github.io/mnx/docs/>' \
	  >> ${CRATE_DIR}/src/mnx.rs
	@echo '//!' >> ${CRATE_DIR}/src/mnx.rs
	@echo '//! Official schema: <https://w3c.github.io/mnx/docs/mnx-schema.json>' \
	  >> ${CRATE_DIR}/src/mnx.rs
	@echo '//!' >> ${CRATE_DIR}/src/mnx.rs
	@echo '//! Run: cargo install cargo-typify && cargo typify mnx-schema.json' \
	  >> ${CRATE_DIR}/src/mnx.rs
	echo '#![allow(unused)]' >> ${CRATE_DIR}/src/mnx.rs
	echo '#![allow(clippy::derivable_impls)]' >> ${CRATE_DIR}/src/mnx.rs
	cat "${SCHEMA_DIR}/mnx-schema.rs" >> ${CRATE_DIR}/src/mnx.rs
	@echo "Output was written to:"
	@echo " ${SCHEMA_LOG}"

build:
	PATH=${PATH} \
	  cargo build

# Canary tests merely confirm known behavior regardless of correctness.
# Sample JSON from which official docs were generated are in
# ${W3C_MNX_DIR}/mnx/doctools/media/examples/json/
canary:
	[ -d "${W3C_MNX_DIR}" ] \
	  || git clone https://github.com/w3c/mnx.git "${W3C_MNX_DIR}" \
	  || (cd "${W3C_MNX_DIR}/" && git pull)
	[ $(shell ls -1 "${EXAMPLES_DIR}" | wc -l) = 47 ]
	RUST_LOG=debug \
	PATH=${PATH} \
	  cargo test -- --nocapture \
	     --include-ignored canary

test:
	RUST_LOG=debug,typify=info \
	PATH=${PATH} \
	  cargo test -- --nocapture \
	     --include-ignored

clippy:
	PATH=${PATH} \
	  cargo +nightly clippy -Zunstable-options \
	    --no-deps --all-targets --all-features
	PATH=${PATH} \
	  cargo clippy --no-deps --all-targets --all-features -- -D warnings

format:
	PATH=${PATH} \
	  cargo fmt --check

doc:
	RUSTDOCFLAGS="-D warnings" \
	PATH=${PATH} \
	  cargo doc --no-deps
