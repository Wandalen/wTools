# Invariant Doc Entity

### Scope

- **Purpose**: Document behavioral contracts and guarantees maintained across all operations.
- **Responsibility**: Registry and overview of all invariant doc instances.
- **In Scope**: In-memory storage contract, blocking network I/O constraint.
- **Out of Scope**: API details (see `api/`), feature guides (see `feature/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [In-Memory Storage](001_in_memory_storage.md) | All archive content held in HashMap; no disk I/O after load | ✅ |
| 002 | [Blocking Network](002_blocking_network.md) | Network operations are synchronous; no async support | ✅ |
