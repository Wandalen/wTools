# Compile-Fail Test Fixtures

### Scope

Source files that must fail to compile, used by `tests/compile_fail_test.rs` via trybuild.
Each `.rs` file has a companion `.stderr` file recording the expected compiler error.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `into_hmap_no_annotation.rs` | FT-02: into_hmap! compile-fail without type annotation |
