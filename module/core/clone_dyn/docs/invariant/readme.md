# Invariant Doc Entity

### Scope

- **Purpose**: Document behavioral contracts that must hold for `clone_dyn` at all times.
- **Responsibility**: Specify the box-only scope restriction and build quality standards.
- **In Scope**: Smart pointer restrictions, NFR build/quality standards.
- **Out of Scope**: Runtime safety contracts (clone_dyn_types), macro correctness constraints (clone_dyn_meta).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Box-Only Restriction](001_box_only.md) | `Box<dyn Trait>` is the only supported smart pointer | ✅ |
| 002 | [Quality Standards](002_quality_standards.md) | Code quality, test coverage, and documentation NFRs | ✅ |
