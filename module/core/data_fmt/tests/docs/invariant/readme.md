# Invariant Test Spec Doc Entity

### Scope

- **Purpose**: Provide test coverage specifications for all invariant doc instances.
- **Responsibility**: Registry and overview of all invariant test spec instances.
- **In Scope**: IN-N invariant enforcement cases in Given/When/Then format for all 4 invariant elements; minimum 2 cases per spec; behavioral guarantees that must hold across all rendering scenarios.
- **Out of Scope**: Algorithm correctness cases (see `../algorithm/`), manual test procedures (see `tests/manual/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Data Model](001_data_model.md) | Invariant spec for data model enforcement | ⏳ |
| 002 | [ANSI and Unicode](002_ansi_unicode.md) | Invariant spec for ANSI and Unicode measurement | ⏳ |
| 003 | [Auto-Wrap Backward Compatibility](003_auto_wrap_backward_compat.md) | Invariant spec for auto-wrap backward compatibility | ⏳ |
| 004 | [Column Fold Invariants](004_column_fold_invariants.md) | Invariant spec for column fold behavioral guarantees | ✅ |
