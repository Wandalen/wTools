# src

Source code for the `test_tools` crate.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `readme.md` | Document src/ organization and source file responsibilities |
| `lib.rs` | Crate root: feature-gated re-exports, namespace modules (own/orphan/exposed/prelude) |
| `behavioral_equivalence.rs` | Behavioral equivalence guards between direct and re-exported items |
| `test/` | Runtime test infrastructure: SmokeModuleTest, compiletime helpers, process helpers |
| `standalone/` | Standalone build shims: circular-dependency-free replacements for runtime deps |
