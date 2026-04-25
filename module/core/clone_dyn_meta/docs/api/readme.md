# API Doc Entity

Public API contracts for `clone_dyn_meta`.

### Scope

- **Purpose:** Specify the public interface contracts: invocation forms, parameters, constraints, and generated output guarantees.
- **Responsibility:** Serve as the authoritative reference for how the attribute macro is called and what it produces.
- **In Scope:** Attribute macro signatures, parameter tables, input constraints, generated impl descriptions.
- **Out of Scope:** Feature rationale (`feature/`), internal implementation (`algorithm/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | clone_dyn_attr | `#[clone_dyn]` invocation contract, parameters, and generated output | ✅ |
