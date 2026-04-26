# Manual Testing Plan: include_md

## Scope

This crate is a placeholder — no macros or functional code exist yet.
Manual testing covers structural and build correctness only.

## Test Checklist

### 1. Build verification

```shell
cargo build --all-features
```

Expected: exits 0, zero warnings.

### 2. Test suite

```shell
cargo nextest run --all-features
```

Expected: `local_smoke_test` and `published_smoke_test` both pass.

### 3. Clippy

```shell
cargo clippy --all-targets --all-features -- -D warnings
```

Expected: exits 0, zero diagnostics.

### 4. Doc build

```shell
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
```

Expected: exits 0; module page shows crate description, not `___.`

## Status

Last verified: 2026-01-21 — all four checks pass on placeholder skeleton.
