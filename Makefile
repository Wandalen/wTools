# abc def
# === common
#

# Comma
comma := ,

# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)

#
# === Parameters
#

VERSION ?= $(strip $(shell grep -m1 'version = "' Cargo.toml | cut -d '"' -f2))

#
# === Git
#

# Sync local repostiry.
#
# Usage :
#	make git.sync [message='description of changes']

git.sync :
	git add --all && git commit -am $(message) && git pull

sync : git.sync

#
# === External cargo crates commands
#

# Check vulnerabilities with cargo-audit.
#
# Usage :
#	make audit

audit :
	cargo audit

#
# === General commands
#

# Generate crates documentation from Rust sources.
#
# Usage :
#	make doc [private=(yes|no)] [open=(yes|no)] [clean=(no|yes)] [manifest_path=(|[path])]

doc :
ifeq ($(clean),yes)
	@rm -rf target/doc/
endif
	cargo doc --all-features \
		$(if $(call eq,$(private),no),,--document-private-items) \
		$(if $(call eq,$(manifest_path),),--manifest-path ./Cargo.toml,--manifest-path $(manifest_path)) \
		$(if $(call eq,$(open),no),,--open)

# Lint Rust sources with Clippy.
#
# Usage :
#	make lint [warnings=(no|yes)] [manifest_path=(|[path])]

lint :
	cargo clippy --all-features \
		$(if $(call eq,$(manifest_path),),--manifest-path ./Cargo.toml,--manifest-path $(manifest_path)) \
		$(if $(call eq,$(warnings),no),-- -D warnings,)

# Check Rust sources `check`.
#
# Usage :
#	make check [manifest_path=(|[path])]

check :
	cargo check \
		$(if $(call eq,$(manifest_path),),--manifest-path ./Cargo.toml,--manifest-path $(manifest_path))

# Format and lint Rust sources.
#
# Usage :
#	make normalize

normalize : fmt lint

# Perform common checks on the module.
#
# Usage :
#	make checkmate

checkmate : doc lint check

# Format Rust sources with rustfmt.
#
# Usage :
#	make fmt [check=(no|yes)]

fmt :
	{ find -L module -name *.rs -print0 ; } | xargs -0 rustfmt +nightly $(if $(call eq,$(check),yes),-- --check,)

# cargo +nightly fmt --all $(if $(call eq,$(check),yes),-- --check,)

# Run project Rust sources with Cargo.
#
# Usage :
#	make up

up :
	cargo up

# Run project Rust sources with Cargo.
#
# Usage :
#	make clean

clean :
	cargo clean && rm -rf Cargo.lock && cargo cache -a && cargo update

# Run Rust tests of project.
#
# Usage :
#	make test

test :
	cargo test --all-features

# Run format link test and tests.
#
# Usage :
#	make all

all : fmt lint test

#
# === .PHONY section
#

.PHONY : \
	all \
	audit \
	docs \
	lint \
	check \
	fmt \
	normalize \
	checkmate \
	test \
	up \
	doc
