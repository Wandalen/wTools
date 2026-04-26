# Invariant Doc Entity

### Scope

- **Purpose**: Document correctness properties that must always hold in `sqlx_query`.
- **Responsibility**: Index all `invariant/` doc instances describing behavioral properties and their enforcement mechanisms.
- **In Scope**: Invariant statements, enforcement mechanisms, violation consequences.
- **Out of Scope**: Feature design (-> `feature/`), API specification (-> `api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Query Dispatch Mode Consistency](001_query_dispatch_invariant.md) | Document dispatch correctness contract | ✅ |
| 002 | [Consumer Integration Contract](002_consumer_integration_contract.md) | Document consumer dependency requirements | ✅ |
