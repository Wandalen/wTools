# API Doc Entity

### Scope

- **Purpose**: Document the public interface — macro entry points and field attribute contracts.
- **Responsibility**: Collect all API doc instances defining macro entry points and attribute contracts.
- **In Scope**: One instance per logical grouping of public entry points or attribute contracts.
- **Out of Scope**: Behavioral rationale and constraints — see `feature/` and `invariant/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Derive API](001_derive_api.md) | `#[derive(Former)]` entry point, struct/enum applicability, and all field attributes | ✅ |
