# Feature: Quantity Formatting

### Scope

- **Purpose**: Provide compact, terminal-aware formatting for durations, counts, and byte sizes in CLI output, with consistent optional ANSI dimming and no manual escape-code management.
- **Responsibility**: Document formatter selection guidance, style resolution, and the opt-in duration-parsing round trip.
- **In Scope**: Choosing among the ten formatting functions, `QuantityStyle` resolution, feature flag integration, duration parsing round trip.
- **Out of Scope**: Band-selection and magnitude-scaling algorithm internals (see `../algorithm/008_quantity_formatting.md`), function signatures (see `../api/006_quantity_formatting.md`).

### APIs

| File | Relationship |
|------|-------------|
| [006_quantity_formatting.md](../api/006_quantity_formatting.md) | `QuantityStyle`, `DurationError`, and all ten formatting function signatures |

### Sources

| File | Relationship |
|------|-------------|
| [`src/quantity/mod.rs`](../../src/quantity/mod.rs) | Module organization, `QuantityStyle` |
| [`src/quantity/duration.rs`](../../src/quantity/duration.rs) | Duration formatters, `parse_duration` |
| [`src/quantity/number.rs`](../../src/quantity/number.rs) | Number/byte formatters |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/quantity_test.rs`](../../tests/quantity_test.rs) | Golden values per function, band, and magnitude |

### Design

**Feature flags**: `quantity` (default-enabled, requires `enabled`, zero external deps); `quantity_parse` (opt-in, adds `humantime`)

#### Choosing a Duration Formatter

- **`duration_6ch`** — fixed 6-visible-column width; use for table/column cells that must stay aligned.
- **`duration_human`** — variable-width, day-capable; use for uptimes and ages that may span multiple days.
- **`duration_human_hours`** — variable-width, hour-capped (never rolls into a day tier); use for countdowns or "time remaining" estimates where an unbroken hour count reads better than a day/hour split.
- **`duration_ms`** — sub-second precision below one minute, falling back to `duration_human` tiers above it; use for elapsed timers and latency reporting.

#### Choosing a Number/Byte Formatter

- **`number_compact`** — generic counts (item counts, request counts); SI magnitudes, compact single-letter form.
- **`bytes_iec`** / **`bytes_human`** — binary byte sizes (RAM, buffer capacities) where 1 KiB genuinely is 1024 bytes; compact single-letter (`bytes_iec`) vs verbose two-decimal (`bytes_human`, tops out at `GB`).
- **`bytes_si`** / **`bytes_compact_si`** — decimal byte sizes (disk-vendor figures, network throughput, or any source already using a decimal convention); verbose two-decimal extending to `TB` (`bytes_si`) vs compact magnitude-adaptive precision (`bytes_compact_si`).

The IEC and SI byte families are not interchangeable on the same raw count — pick the base matching the value's own origin, not by habit (see `../algorithm/008_quantity_formatting.md § SI-vs-IEC Contrast` for the concrete divergence).

#### Style Resolution

`QuantityStyle::resolve( is_tty )` is the recommended single entry point: it folds a caller-supplied TTY check with the `NO_COLOR` environment variable into one policy, so every formatter call site in a program shares identical color behavior. Callers with their own color policy may construct `QuantityStyle::Plain` or `QuantityStyle::Colored` directly instead — the formatters themselves never read the environment.

#### Duration Parsing (opt-in)

`parse_duration` is the inverse of `duration_human`, reading compact strings a CLI user would type (`"1h30m"`, `"7d"`, `"90 seconds"`) back into a `core::time::Duration`. Gated behind the opt-in `quantity_parse` feature to keep the `humantime` dependency out of the default build.

#### Feature Flag Integration

The `quantity` feature gates the entire module — all ten formatters and `QuantityStyle` — as a single unit; there is no per-function feature granularity. `quantity_parse` is additive: enabling it adds `parse_duration`/`DurationError` without changing any existing formatter's behavior or signature.
