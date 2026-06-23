# Invariant Doc Entity

### Scope

- **Purpose**: Document behavioral contracts and guarantees maintained across all operations.
- **Responsibility**: Registry and overview of all invariant doc instances.
- **In Scope**: Data model invariants, ANSI/Unicode handling rules, auto-fit behavioral guarantees, heading rendering guarantees.
- **Out of Scope**: API details (see `api/`), formatting features (see `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Data Model](001_data_model.md) | TreeNode invariants, table-shaped validation, RowBuilder contracts | ✅ |
| 002 | [ANSI and Unicode](002_ansi_unicode.md) | ANSI escape handling, Unicode display width rules | ✅ |
| 003 | [Auto-Wrap Backward Compatibility](003_auto_wrap_backward_compat.md) | `auto_wrap(false)` byte-identical to pre-auto-fit behavior | ✅ |
| 004 | [Column Fold Invariants](004_column_fold_invariants.md) | Header non-folding, CSV/TSV bypass, fold determinism | ✅ |
| 005 | [Heading Rendering](005_heading.md) | No-heading passthrough, width ceiling, single-line output | ✅ |
