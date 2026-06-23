# API: Output Module

### Scope

- **Purpose**: Document the public interface for CLI output processing in `cli_fmt`.
- **Responsibility**: Reference for the processing function, configuration type, stream filter, and result type.
- **In Scope**: Processing entry point, stream merging helper, configuration options, filter variants, and result structure.
- **Out of Scope**: Processing logic and behavioral decisions — see `feature/001_output_processing.md`.

### Abstract

The output processing API provides a single entry-point function that accepts raw
stdout and stderr strings plus a configuration value and returns processed output with
metadata. A stream merging helper is also available for cases where only the merging
stage is needed.

Both functions are infallible — they perform no I/O and cannot produce errors.

### Operations

**Process output** — applies stream selection, line filtering, and width truncation
in sequence. Accepts separate stdout and stderr strings and an output configuration.
Returns a processed output value containing the result text and accuracy metadata.

**Merge streams** — combines stdout and stderr into a single string according to the
stream filter. Does not apply line filtering or width truncation.

**Output configuration** — a builder-pattern value for specifying processing options:
- Head limit: retain only the first N lines of combined output.
- Tail limit: retain only the last N lines of combined output.
- Width limit: maximum visible character width per line; zero disables truncation at
  runtime (stored as `Some(0)`, not `None`; `has_processing()` returns true for
  `with_width(0)` even though no truncation occurs at runtime).
- Truncation suffix: marker appended to lines that were width-truncated.
- Stream filter: which streams to include and in what order.
- Unicode-aware mode: opt-in grapheme-based width measurement.

**Configuration state queries** — two predicate methods:
- `is_default()`: returns true only when all fields hold their default values —
  head/tail/width not set (`None`), stream filter `Both`, suffix `"→"`,
  unicode-aware false.
- `has_processing()`: returns true when head, tail, or width is explicitly set
  (including `with_width(0)`); indicates at least one processing option was configured.

**Stream filter** — a three-variant value selecting which streams to include:
- Both streams: stderr placed before stdout (errors visible before normal output).
- Stdout only: stderr discarded.
- Stderr only: stdout discarded.

**Processed output** — the result value containing:
- Content string: the processed output, ready for display.
- Lines omitted count: number of lines removed by head/tail filtering; zero when no filtering applied or when head and tail windows overlap.
- Width truncated flag: true if any line was shortened by the width limit.

### Error Handling

Both public functions are infallible. They perform no I/O and accept any string input. No error type is returned and no panics occur.

### Compatibility Guarantees

The function signatures, configuration fields, and result structure are stable across patch and minor versions. New configuration options may be added in minor versions with backward-compatible defaults. Changes to existing option semantics require a major version bump.

### API Tests

| File | Relationship |
|------|-------------|
| [`../../tests/docs/api/001_output_api.md`](../../tests/docs/api/001_output_api.md) | Test specification verifying the API contracts defined here |

### Features

| File | Relationship |
|------|-------------|
| [`../feature/001_output_processing.md`](../feature/001_output_processing.md) | Behavioral description of output processing |

### Invariants

| File | Relationship |
|------|-------------|
| [`../invariant/001_architectural_boundary.md`](../invariant/001_architectural_boundary.md) | Boundary principle this API implements |

### Sources

| File | Relationship |
|------|-------------|
| `src/output.rs` | Implementation of process_output, merge_streams, and all API types |

### Tests

| File | Relationship |
|------|-------------|
| `tests/output.rs` | API contract verification across processing configurations |
