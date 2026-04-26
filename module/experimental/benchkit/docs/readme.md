# Docs

### Scope

Design and API documentation for `benchkit`.

### Responsibility Table

| File | Responsibility |
|--------|----------------|
| `entities.md` | Master index of all doc entity types and instances |
| `doc_graph.yml` | Cross-reference dependency graph for all doc instances |
| `feature/` | User-facing capability scope, design, and cross-references |
| `api/` | Public API surface documentation |
| `invariant/` | Correctness properties that must hold unconditionally |
| `pattern/` | Architectural design patterns |

### System Actors

| Actor | Role |
|-------|------|
| Benchmark author | Developer who writes benchmark functions using benchkit; consumes the measurement, data generation, and reporting APIs |
| CI/CD pipeline | Automated system that runs benchmarks on each commit and writes results into version-controlled markdown files |
| Documentation reader | Developer or reviewer who reads benchmark result tables in project markdown files; never interacts with benchkit directly |
| Project maintainer | Developer who manages benchkit's feature flags, dependency versions, and public API stability guarantees |

### Vocabulary

| Term | Definition |
|------|------------|
| benchmark result | The aggregate timing statistics collected from a single named benchmark run: mean, min, max, standard deviation, and operations per second |
| benchmark suite | A named group of related benchmark functions executed together in a single pass |
| section updater | The component that performs in-place replacement of a named section within an existing markdown file |
| comparative analysis | A ranked comparison of two or more algorithm variants run under identical conditions, reporting relative speedup ratios |
| data generator | A utility that produces deterministic, optionally seeded collections of test data for use inside timed benchmark regions |
| section marker | The identifier string that delimits the boundaries of an auto-generated section within a markdown documentation file |
| update chain | A sequence of section update operations applied to one or more markdown files within a single benchmark run |
| timing loop | The measured execution region surrounding repeated invocations of the subject function; report generation and data generation occur outside this region |
| report template | A pre-built markdown structure for common benchmark result layouts, rendered into a string for insertion via the section updater |
| feature flag | A Cargo compilation switch that opt-in enables a benchkit capability; all flags compose additively on top of the `enabled` base |
