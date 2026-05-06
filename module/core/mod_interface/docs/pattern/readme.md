# Pattern Doc Entity

### Scope

- **Purpose**: Document reusable architectural patterns that govern the design of mod_interface.
- **Responsibility**: Catalog of pattern doc instances covering the five-layer cascade and the absorption crate structure.
- **In Scope**: Problem statement, solution structure, applicability criteria, and known consequences for each pattern.
- **Out of Scope**: Implementation code; patterns describe structure and rationale, not code generation mechanics.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Exposure Level Cascade](001_exposure_level_cascade.md) | Five-layer visibility hierarchy with downward cascade | ✅ |
| 002 | [Absorption Pattern](002_absorption_pattern.md) | Runtime facade crate absorbing the proc-macro companion crate | ✅ |
