# Pattern: Toolkit Not Framework

### Scope

- **Purpose**: Allow benchmark authors to integrate benchkit into any project structure without restructuring their project.
- **Responsibility**: Documents the design principle that governs all benchkit API and feature decisions.
- **In Scope**: The composability decision and its effect on API shape; contrast with framework-style alternatives.
- **Out of Scope**: Specific API operations (→ api/001); feature-specific design decisions (→ feature/).

### Problem

Existing benchmarking solutions impose a fixed workflow: dedicated directory structure, specific runner architecture, and framework-controlled report formats. Projects must restructure to fit the tool. When the tool's requirements conflict with the project's own organization (co-located test benchmarks, documentation-driven development, custom output formats), developers either abandon the tool or fight it continuously.

### Solution

benchkit provides independent, composable building blocks rather than a complete workflow. Each capability (timing, data generation, report updating, analysis) is a standalone module that can be used in isolation or composed with others. Nothing is mandatory except the feature flag that enables it. Benchmark code may live in `benches/`, `tests/`, `examples/`, or anywhere else the author chooses. The toolkit adapts to the project; the project does not adapt to the toolkit.

### Applicability

Apply this pattern when:
- The primary users are library and application developers who already have an established project structure
- The domain (benchmarking) benefits from incremental adoption — start with timing, add reports later
- Interoperability with existing tooling (criterion, cargo bench) is a requirement, not an afterthought
- Documentation integration is a first-class goal rather than an afterthought

### Consequences

**Positive**: Low adoption barrier — a single timing call is sufficient to start; composability enables progressive complexity; no imposed workflow means no workflow conflicts with existing projects.

**Negative**: No prescribed structure means benchkit cannot make strong assumptions about where files are located, requiring callers to provide paths explicitly; the toolkit cannot auto-discover benchmarks the way a framework runner can; users who want a fully configured out-of-box experience need to make more choices.

### Cross-References

| Type   | File                                       | Responsibility                                        |
|--------|--------------------------------------------|-------------------------------------------------------|
| doc    | `docs/feature/001_measurement_timing.md`   | Toolkit composability: measurement used standalone    |
| doc    | `docs/feature/002_data_generation.md`      | Toolkit composability: data generation independent    |
| doc    | `docs/feature/003_markdown_reports.md`     | Toolkit composability: reports optional module        |
| doc    | `docs/feature/004_performance_analysis.md` | Toolkit composability: analysis layered on top        |
| config | `Cargo.toml`                               | Feature flags enforcing opt-in composability          |
| doc    | `docs/invariant/004_usability_nfr.md`      | Measurable integration ease constraint derived from this pattern |
