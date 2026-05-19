# Invariant: Rendering Performance

### Scope

- **Purpose**: Bounds template rendering latency to ensure acceptable responsiveness.
- **Responsibility**: Documents the rendering performance constraint and its measurement method.
- **In Scope**: Single-template rendering latency; 10KB template with 50 parameters.
- **Out of Scope**: Pack/materialize pipeline latency (→ genfile CLI NFR1), filesystem I/O time.

### Invariant Statement

Template rendering must complete within 100ms median latency for templates up to 10KB in size with up to 50 parameter substitutions on hardware from 2020 or newer.

### Enforcement Mechanism

Measured via benchmark tests using `criterion` or equivalent. Handlebars renders synchronously with no external I/O during rendering. The constraint is checked by running the benchmark suite and verifying median latency stays below the threshold.

### Violation Consequences

Exceeding 100ms renders genfile_core unsuitable as a dependency for interactive tools like `willbe`, which must respond quickly to user commands. Performance regressions are caught by benchmark comparisons in CI.

### Features

| File | Relationship |
|------|--------------|
| [`feature/007_handlebars_renderer.md`](../feature/007_handlebars_renderer.md) | The renderer whose performance this invariant bounds |
