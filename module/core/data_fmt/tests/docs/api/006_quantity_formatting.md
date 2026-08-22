# API: Quantity Formatting

### Scope

- **Purpose**: Drive test coverage for the quantity formatting API contracts in `docs/api/006_quantity_formatting.md`.
- **Responsibility**: Documents API contract test cases for `QuantityStyle`, `DurationError`, and the signatures/contracts of all ten public formatting functions.
- **In Scope**: `QuantityStyle::resolve` environment resolution, duration/number/byte formatter signatures and purity, `parse_duration`/`DurationError` round trip, feature gating (`quantity`, `quantity_parse`).
- **Out of Scope**: Band-selection and magnitude-scaling algorithm internals (see `../algorithm/008_quantity_formatting.md`), usage/selection guidance (see `tests/docs/feature/008_quantity_formatting.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AP-1 | QuantityStyle::resolve returns Colored only when is_tty is true and NO_COLOR is unset | ⏳ |
| AP-2 | all four duration functions honor their documented tier contracts | ⏳ |
| AP-3 | all five number/byte functions are total — every input produces a defined String | ⏳ |
| AP-4 | bytes_human and bytes_si diverge on the identical raw byte count | ⏳ |
| AP-5 | parse_duration and DurationError are only in scope under the opt-in quantity_parse feature | ⏳ |
| AP-6 | parse_duration returns the correct DurationError variant for each class of malformed input | ⏳ |
| AP-7 | disabling the quantity feature removes QuantityStyle and all ten formatters from scope | ⏳ |

---

### AP-1: QuantityStyle::resolve returns Colored only when is_tty is true and NO_COLOR is unset

- **Given:** The four combinations of `is_tty` (`true`/`false`) and `NO_COLOR` (set/unset).
- **When:** `QuantityStyle::resolve(is_tty)` is called under each combination.
- **Then:** Returns `QuantityStyle::Colored` only for `is_tty = true` with `NO_COLOR` unset; all three other combinations return `QuantityStyle::Plain` — `resolve` is the only function in the module that reads the environment.

---

### AP-2: all four duration functions honor their documented tier contracts

- **Given:** `duration_6ch`, `duration_human`, `duration_human_hours`, `duration_ms`, each called with `(u64, QuantityStyle) -> String`.
- **When:** Each is called with a representative input from its documented range table.
- **Then:** `duration_6ch` returns a fixed 6-visible-column result; `duration_human` returns at most two tiers among days→hours→minutes→seconds; `duration_human_hours` returns at most two tiers but never introduces a day tier; `duration_ms` returns sub-second precision below one second and truncated `N.NNs` below one minute — every signature matches `(u64, QuantityStyle) -> String` exactly, with no `Result` wrapper.

---

### AP-3: all five number/byte functions are total — every input produces a defined String

- **Given:** `number_compact`, `bytes_iec`, `bytes_human`, `bytes_si`, `bytes_compact_si`, each called with `(u64, QuantityStyle) -> String`.
- **When:** Each is called across its full documented range (below its smallest unit threshold, at a mid-range magnitude, and at a large magnitude).
- **Then:** Every call returns a non-empty, defined `String`; no function returns `Option`/`Result`; every `(value, style)` pair produces exactly one deterministic output — the ten-function API surface has zero fallible members apart from the separately-gated `parse_duration`.

---

### AP-4: bytes_human and bytes_si diverge on the identical raw byte count

- **Given:** The single raw byte count `1_500_000`.
- **When:** `bytes_human(1_500_000, QuantityStyle::Plain)` and `bytes_si(1_500_000, QuantityStyle::Plain)` are both called.
- **Then:** `bytes_human` (base 1024) returns `"1.43 MB"`; `bytes_si` (base 1000) returns `"1.50 MB"` — the same input produces two different, both-correct outputs depending on which base the caller intends, confirming the API doc's Compatibility Guarantee that the two are stable, intentional divergences rather than a gap to reconcile.

---

### AP-5: parse_duration and DurationError are only in scope under the opt-in quantity_parse feature

- **Given:** Two builds: one with only the default `quantity` feature enabled, one with `quantity` plus `quantity_parse`.
- **When:** Code referencing `parse_duration`/`DurationError` is compiled under each.
- **Then:** The first build fails to compile any reference to `parse_duration`/`DurationError` (not in scope); the second compiles successfully — `parse_duration`'s `humantime` dependency is pulled in only when explicitly opted into.

---

### AP-6: parse_duration returns the correct DurationError variant for each class of malformed input

- **Given:** Three malformed inputs: an empty string `""`, an unrecognized token `"soon"`, and a duration string whose parsed value exceeds `u64::MAX / 2` seconds.
- **When:** `parse_duration(s)` is called for each, with the `quantity_parse` feature enabled.
- **Then:** `""` returns `Err(DurationError::Empty)`; `"soon"` returns `Err(DurationError::InvalidFormat(_))`; the oversized duration returns `Err(DurationError::Overflow(_))` — each of the three documented error variants is independently reachable from a distinct input shape.

---

### AP-7: disabling the quantity feature removes QuantityStyle and all ten formatters from scope

- **Given:** A build with the `quantity` feature disabled.
- **When:** The crate is compiled.
- **Then:** `QuantityStyle` and all ten formatting functions (`duration_6ch`, `duration_human`, `duration_human_hours`, `duration_ms`, `number_compact`, `bytes_iec`, `bytes_human`, `bytes_si`, `bytes_compact_si`, plus `parse_duration`/`DurationError` even if `quantity_parse` were separately enabled) are absent from scope; the `quantity` feature gates the entire module as a single unit — there is no per-function granularity.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/api/006_quantity_formatting.md`](../../../docs/api/006_quantity_formatting.md) | Source API spec — `QuantityStyle`, `DurationError`, ten formatting function signatures, feature gating |
| [`src/quantity/mod.rs`](../../../src/quantity/mod.rs) | `QuantityStyle`, `resolve()`, module re-exports |
| [`src/quantity/duration.rs`](../../../src/quantity/duration.rs) | `duration_6ch`, `duration_human`, `duration_human_hours`, `duration_ms`, `parse_duration`, `DurationError` |
| [`src/quantity/number.rs`](../../../src/quantity/number.rs) | `number_compact`, `bytes_iec`, `bytes_human`, `bytes_si`, `bytes_compact_si` |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/quantity_test.rs`](../../quantity_test.rs) (extend) | Spec tests for AP-1..AP-7 — quantity formatting API contracts |
