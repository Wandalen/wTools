# Algorithm Doc Entity

### Scope

- **Purpose**: Document the internal algorithms of `clone_dyn_types` for contributors.
- **Responsibility**: Specify algorithm steps, inputs, outputs, and correctness requirements.
- **In Scope**: Fat pointer surgery in `clone_into_box`.
- **Out of Scope**: Public API contracts (`api/`), caller-facing behavioral specs (`feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | Fat Pointer Surgery | DST clone via fat pointer data component swap | ✅ |
