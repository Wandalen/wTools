# Feature: Data Generation

### Scope

- **Purpose**: Supply deterministic, reproducible test data so benchmarks produce consistent results across runs.
- **Responsibility**: Documents data generator types, size parameters, and integration with measurement.
- **In Scope**: Random data generation with seeded reproducibility, string data, numeric sequences, parser-oriented inputs.
- **Out of Scope**: Statistical validation of generated data (→ feature/004); benchmark execution (→ feature/001).

### Design

Data generators are stateless factories that produce fixed-size data collections on demand. Each generator accepts a size parameter and an optional seed, guaranteeing identical output for the same inputs across machines.

Two generator families exist: general-purpose generators (numeric sequences, random strings, byte arrays) and parser-oriented generators (structured text simulating real parse inputs such as comma-separated values and nested data).

Generators integrate naturally with the measurement feature: a generator produces the dataset, the dataset is captured in a closure, and the closure is passed to the timing loop. Separating generation from measurement ensures generation overhead does not pollute timing results.

### Cross-References

| Type   | File                             | Responsibility                                      |
|--------|----------------------------------|-----------------------------------------------------|
| source | `src/generators.rs`              | General-purpose data generator implementations      |
| source | `src/data_generation.rs`         | High-level data generation facade                   |
| source | `src/parser_data_generation.rs`  | Parser-oriented structured input generators         |
| test   | `tests/`                         | Reproducibility and correctness tests               |
| doc    | `docs/api/001_benchkit_api.md`   | Public API surface including generator operations   |
| doc    | `docs/pattern/001_toolkit_not_framework.md` | Design principle this feature exemplifies         |
