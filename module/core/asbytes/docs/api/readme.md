# API Doc Entity

### Scope

- **Purpose**: Document the public API surface — type signatures, method contracts, behavior.
- **Responsibility**: Registry and overview of all API doc instances.
- **In Scope**: `AsBytes` trait and `IntoBytes` trait with all implementations.
- **Out of Scope**: Behavioral invariants (see `invariant/`), feature guides (see `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [AsBytes Trait](001_as_bytes_trait.md) | Borrow data as `&[u8]` without consuming | ✅ |
| 002 | [IntoBytes Trait](002_into_bytes_trait.md) | Consume data into owned `Vec<u8>` | ✅ |
