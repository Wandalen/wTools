# Feature Doc Definition

### Scope

- **Purpose**: Describe what `sqlx_query` does for a consumer's build, end to end.
- **Responsibility**: Index the feature doc instances, one navigational hub per capability.
- **In Scope**: Behavior, decisions, workflow and limitations, with pointers to the APIs, invariants, sources and tests behind each capability.
- **Out of Scope**: Interface details (→ `api/`) and correctness properties (→ `invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Offline Mode Enforcement](001_offline_mode_enforcement.md) | Fail fast on stale cache entries, with a soft-warn opt-out | ✅ |
| 002 | [Auto Regeneration](002_auto_regeneration.md) | Opt-in repair of stale entries with a nested live check | ✅ |
