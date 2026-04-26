# Algorithm Doc Entity

### Scope

- **Purpose**: Document the internal algorithms used by `component_model_meta` to generate code at derive time.
- **Responsibility**: Collect one doc instance per significant algorithmic decision; each instance explains the approach and the problem it solves.
- **In Scope**: Code generation logic that is not obvious from the source — particularly workarounds for language constraints.
- **Out of Scope**: Derive macro public interface (→ `api/`); trait definitions (→ `component_model_types/docs/api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Popular Type Generation](001_popular_type_generation.md) | String-matching approach to generate Assign impls for standard library types | ✅ |
| 002 | [Type Deduplication](002_type_deduplication.md) | HashSet-based dedup preventing conflicting impl errors for same-type fields | ✅ |
