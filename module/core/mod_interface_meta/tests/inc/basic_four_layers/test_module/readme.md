# Test Module Directory

Test fixture modules for validating four-layer namespace generation.

### Responsibility Table

| File | Responsibility |
|------|---------------|
| mod_own.rs | Test fixture for own namespace layer |
| mod_orphan.rs | Test fixture for orphan namespace layer |
| mod_exposed.rs | Test fixture for exposed namespace layer |
| mod_prelude.rs | Test fixture for prelude namespace layer |

## Purpose

These modules serve as minimal test fixtures to verify that the `mod_interface!` macro correctly generates and propagates content through all four namespace layers (own → orphan → exposed → prelude).

Each module exports a single function returning `true`, allowing tests to verify namespace accessibility and hierarchical propagation without additional complexity.
