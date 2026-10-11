# Invariant Doc Definition

### Scope

- **Purpose**: Record the properties every staleness verdict and every emitted setting depend on.
- **Responsibility**: Index the invariant doc instances: statement, enforcement, consequences.
- **In Scope**: Invariant statements, enforcement mechanisms, violation consequences, known gaps, and historical violations.
- **Out of Scope**: Feature behavior (→ `feature/`) and interface details (→ `api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Hash Algorithm Parity](001_query_dispatch_invariant.md) | Cache-file names derived exactly as sqlx derives them | ✅ |
| 002 | [Isolated Target-Dir Regeneration](002_consumer_integration_contract.md) | Live repairs never deadlock, never recurse, always write | ✅ |
| 003 | [Resolution Parity with sqlx](003_sqlx_resolution_parity.md) | Sites, SQL text, lookup, variables and precedence as sqlx resolves them | ✅ |
| 004 | [Declared Rerun Inputs](004_declared_rerun_inputs.md) | Every decision input declared to Cargo, nothing that changes during builds | ✅ |
