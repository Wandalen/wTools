# standalone

Standalone build mode: circular-dependency-free shims for test_tools.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `readme.md` | Document standalone/ organization and purpose |
| `mod.rs` | Top-level macros, error_tools, mem_tools, typing_tools, re-exports |
| `collection_tools.rs` | Collection type aliases, hashmap_compat wrappers, constructor macros |
| `diagnostics_tools.rs` | Assertion macros (a_id, a_true, cta_true, etc.) |
| `impls_index.rs` | Placeholder impls_index compatibility module |
