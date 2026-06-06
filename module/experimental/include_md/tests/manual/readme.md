# Manual Testing Plan: include_md

## Scope

This plan covers manual verification of both `include_md!` and `include_md_section!` proc-macros —
build correctness, automated test suite, clippy cleanliness, and integration scenarios beyond the
automated test matrix.

## Prerequisites

- Rust toolchain installed (stable; `rustup show` to confirm)
- Working directory: crate root (`module/experimental/include_md/`)
- `cargo nextest` installed (`cargo install cargo-nextest`)

## Scenarios

### 1. Full automated test suite

```shell
RUSTFLAGS="-D warnings" cargo nextest run --all-features
```

Expected: all 27 tests pass (6 in `file_inclusion`, 18 in `section_extraction`, 2 smoke tests),
0 skipped.

### 2. Doc tests

```shell
RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features
```

Expected: exits 0; no doc-test failures.

### 3. Clippy

```shell
cargo clippy --all-targets --all-features -- -D warnings
```

Expected: exits 0, zero diagnostics.

### 4. Example runs

```shell
cargo run --example include_md_trivial --features enabled
```

Expected: exits 0; prints both macro outputs without error.

### 5. Integration — include_md! path resolution

From `tests/file_inclusion.rs`, `include_md!("fixture/sample.md")` resolves relative to
`tests/` (the calling source file's directory). Manually verify by checking the test passes:

```shell
cargo nextest run --all-features valid_file_returns_full_contents
```

Expected: PASS.

### 6. Integration — empty file

```shell
cargo nextest run --all-features empty_file_returns_empty_string
```

Expected: PASS; confirms `include_md!` accepts zero-byte files.

### 7. Integration — code block fence boundary (BUG-005 regression)

```shell
cargo nextest run --all-features code_block_heading_not_a_boundary tilde_fence_heading_not_a_boundary
```

Expected: both PASS; confirms headings inside fenced code blocks do not terminate sections.

### 8. Compile-fail scenarios

The following scenarios require subprocess `cargo check` — they run automatically as part of the
nextest suite. Confirm manually by ensuring the suite completes with 0 failures.

| Scenario | Test name |
|----------|-----------|
| Missing file → compile error | `missing_file_is_compile_error` (both test files) |
| Heading not found → compile error | `heading_not_found_is_compile_error` |
| Wrong case → compile error | `wrong_case_heading_is_compile_error` |
| Oversized file → compile error | `oversized_file_is_compile_error` (both test files) |
| Invalid UTF-8 → compile error | `invalid_utf8_is_compile_error` (both test files) |
| Arity errors → compile error | `no_args_`, `one_arg_`, `two_args_`, `three_args_` variants |

## Success Criteria

- All 27 nextest tests PASS
- 0 doc-test failures
- 0 clippy warnings or errors
- Example runs without error

## Last Verified

2026-06-06 — Level 3 PASS: 27/27 nextest, 0 doc, 0 clippy.
