# Pattern Doc Entity

### Scope

- **Purpose**: Document the architectural design patterns applied in `meta_tools` that explain structural decisions consumers and contributors need to understand.
- **Responsibility**: Collect pattern doc instances — each with a problem statement, solution, applicability rules, and consequences — for patterns that cannot be inferred from reading the source alone.
- **In Scope**: The facade aggregation pattern and any other non-obvious structural decisions that warrant explanation.
- **Out of Scope**: Feature scope rationale (see `feature/`), macro API details (see `api/`), implementation internals.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Facade Aggregation](001_facade_aggregation.md) | Aggregate multiple independently-releasable crates into a single import point | ✅ |
