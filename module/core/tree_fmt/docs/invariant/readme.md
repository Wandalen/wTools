# Invariant Doc Entity

### Scope

- **Purpose:** Document behavioral contracts and guarantees maintained across all operations.
- **Responsibility:** Registry and overview of all invariant doc instances.
- **In Scope:** Data model invariants, ANSI/Unicode handling rules.
- **Out of Scope:** API details (see `api/`), formatting features (see `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Data Model](001_data_model.md) | TreeNode invariants, table-shaped validation, RowBuilder contracts | ✅ |
| 002 | [ANSI and Unicode](002_ansi_unicode.md) | ANSI escape handling, Unicode display width rules | ✅ |
