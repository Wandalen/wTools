# Invariant Doc Entity

### Scope

- **Purpose**: Document the behavioral contracts and constraints that all `asbytes` conversions must uphold.
- **Responsibility**: Registry and overview of all invariant doc instances.
- **In Scope**: POD safety constraint; native byte order constraint.
- **Out of Scope**: API signatures (see `api/`), feature guides (see `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [POD Safety](001_pod_safety.md) | All generic type parameters must be POD-safe | ✅ |
| 002 | [Native Byte Order](002_native_endian.md) | Byte output uses host-native endianness; no conversion performed | ✅ |
