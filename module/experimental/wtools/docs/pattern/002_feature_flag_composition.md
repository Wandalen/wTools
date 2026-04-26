# Pattern: Feature Flag Composition

### Scope

- **Purpose**: Provide granular compile-time control over which ecosystem capabilities are included.
- **Responsibility**: Document the per-category namespaced feature flag hierarchy and the standard tier structure applied uniformly across all 10 categories.
- **In Scope**: Category namespace convention, five standard tiers per category, granular sub-features, meta-features, cross-category composition.
- **Out of Scope**: Individual category flag lists (see feature/001-010), namespace hierarchy (see api/).

### Problem

An aggregation crate with 10 constituent crates and dozens of individual capabilities needs a feature flag system that satisfies three competing requirements:

1. Users who want everything need a single flag.
2. Users who want one specific capability need surgical precision.
3. The flag namespace must be navigable — not a flat list of 60+ unrelated names.

### Solution

A two-level naming convention with consistent tiers:

**Level 1 — Category prefix.** Every flag begins with its category name: `iter_`, `meta_`, `string_`, etc. This groups related flags visually and prevents name collisions.

**Level 2 — Standard tiers.** Each category offers up to five standard flags:

| Tier | Pattern | Meaning |
|------|---------|---------|
| Base | `{cat}` | Include the sub-crate; enable nothing else |
| Default | `{cat}_default` | The recommended default features for this category |
| Full | `{cat}_full` | Every capability in this category |
| no_std | `{cat}_no_std` | no_std-compatible subset |
| use_alloc | `{cat}_use_alloc` | Allocator support within no_std mode |

Beyond the five tiers, categories may offer granular sub-features following the pattern `{cat}_{capability}` (e.g., `typing_inspect_type`, `string_indentation`).

**Meta-features** span all categories:

| Flag | Effect |
|------|--------|
| `default` | Activates `{cat}_default` for every category |
| `full` | Activates `{cat}_full` for every category |
| `enabled` | Marker flag (currently empty; reserved for crate activation) |
| `no_std` | Global no_std mode |
| `use_alloc` | Global allocator support in no_std |
| `nightly` | Nightly-only features (currently reserved, empty) |

### Applicability

**Apply when:**
- An aggregation crate offers multiple independently useful capability domains.
- Users need both coarse-grained (enable everything) and fine-grained (enable one macro) control.
- The total flag count exceeds 20, making flat naming unnavigable.

**Do not apply when:**
- A crate has fewer than 3 feature flags — simple flat naming suffices.
- All features are tightly coupled — separate flags add complexity without value.

### Consequences

**Benefits:**
- Predictable naming — knowing any one category's flags teaches the pattern for all 10.
- Compile-time savings — disabling unused categories eliminates their compilation cost entirely.
- no_std flexibility — granular per-category no_std flags enable mixed-environment use.

**Liabilities:**
- Flag count is high (~60+ flags) even though the naming is systematic.
- Some categories have identical default and full sets, making the distinction redundant for those categories.
- Inter-feature dependencies (e.g., string_split activates parse_request internally) are not immediately visible from flag names alone.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `../../Cargo.toml` | Complete feature flag definitions (lines 35-414) |
| doc | `001_ecosystem_aggregation.md` | Companion pattern for the aggregation approach |
| doc | `../api/001_namespace_hierarchy.md` | How enabled flags map to available namespace modules |
