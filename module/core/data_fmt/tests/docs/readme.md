# Test Surface Specs

## Scope

Formal test specification documents for all algorithm and invariant test surface
elements defined in `docs/algorithm/` and `docs/invariant/`.

**In Scope:**
- `algorithm/001` Multiline Cell Rendering
- `algorithm/002` Word Wrapping
- `algorithm/003` Tree Column Alignment
- `algorithm/004` Budget Allocation
- `algorithm/005` Column Fold Detection
- `algorithm/006` CLI Help Alignment
- `invariant/001` Data Model
- `invariant/002` ANSI and Unicode
- `invariant/003` Auto-Wrap Backward Compatibility
- `invariant/004` Column Fold Invariants

**Out of Scope:** Test code organization (see `tests/readme.md`), manual testing
procedures (see `tests/manual/readme.md`).

## Overview Table

| Directory | Responsibility |
|-----------|----------------|
| `algorithm/` | Algorithm correctness spec files; AC-N cases, min 4 per spec |
| `invariant/` | Invariant enforcement spec files; IC-N cases, min 2 per spec |
