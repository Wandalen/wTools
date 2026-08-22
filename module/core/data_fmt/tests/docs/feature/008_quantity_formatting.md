# Feature: Quantity Formatting

### Scope

- **Purpose**: Drive test coverage for the quantity formatting feature — formatter selection guidance, style resolution, and the opt-in duration-parsing round trip.
- **Responsibility**: Documents test cases for `docs/feature/008_quantity_formatting.md`.
- **In Scope**: Choosing among the ten formatting functions, `QuantityStyle::resolve` as the single style entry point, feature flag integration (`quantity`/`quantity_parse`), duration parsing round trip.
- **Out of Scope**: Band-selection and magnitude-scaling algorithm internals (see `../algorithm/008_quantity_formatting.md`), function signatures (see `../api/006_quantity_formatting.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| FT-1 | duration_6ch keeps a mixed-magnitude column aligned via its fixed 6-column width | ⏳ |
| FT-2 | duration_human and duration_human_hours diverge at the one-day boundary | ⏳ |
| FT-3 | duration_ms falls back to duration_human tiers at or above one minute | ⏳ |
| FT-4 | IEC and SI byte families produce different results for the same raw byte count | ⏳ |
| FT-5 | QuantityStyle::resolve gives every formatter call site identical color behavior from one shared policy | ⏳ |
| FT-6 | the quantity feature gates the entire module as a single unit | ⏳ |
| FT-7 | quantity_parse is additive and does not alter existing formatter behavior | ⏳ |
| FT-8 | parse_duration round-trips compact and long-form duration strings back into a Duration | ⏳ |

---

### FT-1: duration_6ch keeps a mixed-magnitude column aligned via its fixed 6-column width

- **Given:** A set of durations from different magnitude bands intended for the same table column: `5`, `3665`, `93_600`, `604_800` seconds.
- **When:** Each is formatted with `duration_6ch(secs, QuantityStyle::Plain)`.
- **Then:** Every result is exactly 6 visible characters — the column stays aligned regardless of the input's magnitude, fulfilling the "use for table/column cells that must stay aligned" selection guidance.

---

### FT-2: duration_human and duration_human_hours diverge at the one-day boundary

- **Given:** `secs = 90_061` (one day, one hour, one minute past midnight).
- **When:** `duration_human(90_061, QuantityStyle::Plain)` and `duration_human_hours(90_061, QuantityStyle::Plain)` are both called.
- **Then:** `duration_human` returns `"1d 1h"` (introduces a day tier); `duration_human_hours` returns `"25h 1m"` (never rolls into a day tier) — confirming the documented selection distinction between uptime/age reporting (`duration_human`) and countdown/time-remaining reporting (`duration_human_hours`).

---

### FT-3: duration_ms falls back to duration_human tiers at or above one minute

- **Given:** Three `ms` values: `500` (below one second), `1500` (below one minute), `65_000` (at or above one minute).
- **When:** `duration_ms(ms, QuantityStyle::Plain)` is called for each.
- **Then:** `500` renders `"500ms"`; `1500` renders `"1.50s"` (truncated to hundredths, never rounded); `65_000` renders `"1m 5s"`, matching `duration_human`'s tiered format rather than a raw second count.
- **Note:** The truncate-not-round policy below one minute exists so a value like `59_990` renders `"59.99s"` and can never round up across the minute boundary into `"1m 0s"` (`src/quantity/duration.rs`, `Fix(BUG-1071)`).

---

### FT-4: IEC and SI byte families produce different results for the same raw byte count

- **Given:** A byte count already computed in a decimal convention, `1_500_000`.
- **When:** The value is formatted via both `bytes_si(1_500_000, QuantityStyle::Plain)` and `bytes_human(1_500_000, QuantityStyle::Plain)`.
- **Then:** `bytes_si` returns `"1.50 MB"`; `bytes_human` returns `"1.43 MB"` — the two are not interchangeable on the same raw count; a caller must pick the base matching the value's own origin, per the feature doc's explicit selection warning.

---

### FT-5: QuantityStyle::resolve gives every formatter call site identical color behavior from one shared policy

- **Given:** A simulated TTY (`is_tty = true`) with `NO_COLOR` unset, and a program that calls `QuantityStyle::resolve(is_tty)` once and passes the resulting style to multiple formatter calls (`number_compact`, `duration_6ch`, `bytes_iec`).
- **When:** All formatter calls use the single resolved style.
- **Then:** All calls render with unit letters dimmed (`Colored` behavior); setting `NO_COLOR` to any value before the `resolve` call flips every subsequent formatter call to `Plain` simultaneously — the single-entry-point policy propagates uniformly rather than each call site re-deriving its own.

---

### FT-6: the quantity feature gates the entire module as a single unit

- **Given:** A build with the `quantity` feature disabled (but `enabled` present).
- **When:** The crate is compiled.
- **Then:** `QuantityStyle` and all ten formatting functions are absent from scope; there is no per-function feature flag that could re-enable an individual formatter independently of the others.

---

### FT-7: quantity_parse is additive and does not alter existing formatter behavior

- **Given:** Two builds of the same program logic — one with only `quantity` enabled, one with both `quantity` and `quantity_parse` enabled.
- **When:** `number_compact`, `duration_6ch`, and the other eight formatters are called identically in both builds.
- **Then:** Every formatter call produces byte-identical output in both builds; the only difference between the builds is that `parse_duration`/`DurationError` are additionally in scope under the second one.

---

### FT-8: parse_duration round-trips compact and long-form duration strings back into a Duration

- **Given:** The strings `"1h30m"`, `"7d"`, and the long form `"90 seconds"` (all valid `humantime` grammar), with the `quantity_parse` feature enabled.
- **When:** Each is passed to `parse_duration(s)`.
- **Then:** Each call returns `Ok(Duration)` with the expected number of seconds (`5400`, `604_800`, `90` respectively) — the parser accepts both compact and long-form input per the documented `humantime` grammar (`s`, `m`, `h`, `d`, `w`), and is the inverse of `duration_human`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/feature/008_quantity_formatting.md`](../../../docs/feature/008_quantity_formatting.md) | Source feature spec — formatter selection guidance, style resolution, feature flag integration, duration parsing |
| [`src/quantity/mod.rs`](../../../src/quantity/mod.rs) | Module organization, `QuantityStyle` |
| [`src/quantity/duration.rs`](../../../src/quantity/duration.rs) | Duration formatters, `parse_duration` |
| [`src/quantity/number.rs`](../../../src/quantity/number.rs) | Number/byte formatters |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/quantity_test.rs`](../../quantity_test.rs) (extend) | Spec tests for FT-1..FT-8 — quantity formatting feature |
