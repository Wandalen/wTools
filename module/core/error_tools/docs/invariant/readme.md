# Invariant

### Scope

Behavioral contracts that must hold for `error_tools` to function correctly and safely.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | Exclusive Error Dependency | Sole error framework mandate for consumers | active |
| 002 | Zero-Cost Facade | Pure re-export with no wrapper overhead | active |
| 003 | Alloc Feature Requires No-Std | use_alloc depends on no_std invariant | active |
