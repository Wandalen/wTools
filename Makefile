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
# === General commands
#

# Generate crates documentation from Rust sources.
#
# Usage :
#	make doc [private=(yes|no)] [open=(yes|no)] [clean=(no|yes)]

doc :
ifeq ($(clean),yes)
	@rm -rf target/doc/
endif
	cargo doc --all-features --package editor_tui \
		$(if $(call eq,$(private),no),,--document-private-items) \
		$(if $(call eq,$(open),no),,--open)

# Format Rust sources with rustfmt.
#
# Usage :
#	make fmt [check=(no|yes)]

fmt :
	{ find -L module -name *.rs -print0 ; } | xargs -0 rustfmt +nightly $(if $(call eq,$(check),yes),-- --check,)

# cargo +nightly fmt --all $(if $(call eq,$(check),yes),-- --check,)

# Lint Rust sources with Clippy.
#
# Usage :
#	make lint

lint :
	cargo clippy --all-features -- -D warnings

# Format nad lint Rust sources.
#
# Usage :
#	make lint

normalize : fmt lint

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
	docs \
	fmt \
	lint \
	test \
	up \
	doc
