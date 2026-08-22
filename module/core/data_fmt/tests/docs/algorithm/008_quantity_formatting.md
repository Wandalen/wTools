# Algorithm: Quantity Formatting

### Scope

- **Purpose**: Drive test coverage for the duration/number/byte formatting algorithms shared by all `QuantityStyle`-aware formatters.
- **Responsibility**: Documents test cases for the algorithm in `docs/algorithm/008_quantity_formatting.md`.
- **In Scope**: `duration_6ch` band selection and week clamp, `number_compact`/`bytes_iec` magnitude scaling and roll-over promotion, `bytes_si` tier-boundary mantissa behavior, `QuantityStyle` unit dimming, extreme-value robustness.
- **Out of Scope**: Terminal width resolution (see `feature/005_auto_fit.md`), table/column layout, function signatures and feature gating (see `../api/006_quantity_formatting.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | duration_6ch band selection renders exactly 6 visible columns | ⏳ |
| AC-2 | number_compact scales to the correct SI magnitude with tier-appropriate precision | ⏳ |
| AC-3 | duration_6ch clamps the weeks band at the maximum representable value | ⏳ |
| AC-4 | rounding roll-over promotes to the next magnitude rather than a four-digit mantissa | ⏳ |
| AC-5 | bytes_si has no roll-over promotion and can render a four-digit mantissa at a tier boundary | ⏳ |
| AC-6 | formatters never panic across the full u64 domain | ⏳ |
| AC-7 | Colored style dims only unit letters; visible glyph count matches Plain | ⏳ |

---

### AC-1: duration_6ch band selection renders exactly 6 visible columns

- **Given:** Four `secs` values, one from each magnitude band: `146` (< 1h), `36_480` (< 1d), `93_600` (< 1w), `604_800` (>= 1w).
- **When:** `duration_6ch(secs, QuantityStyle::Plain)` is called for each value.
- **Then:** Returns `"02m26s"`, `"10h08m"`, `"01d02h"`, `"01w00d"` respectively; each result has exactly 6 visible characters (`.chars().count() == 6`), independent of which band was selected.

---

### AC-2: number_compact scales to the correct SI magnitude with tier-appropriate precision

- **Given:** Three values spanning sub-1000 (`42`), the `k` tier (`14_464`), and the `M` tier (`26_301_958`).
- **When:** `number_compact(n, QuantityStyle::Plain)` is called for each.
- **Then:** Returns `"42"` (below 1000, verbatim, no unit), `"14k"` (0 fractional digits at the `k` tier), `"26.3M"` (1 fractional digit at the `M` tier).

---

### AC-3: duration_6ch clamps the weeks band at the maximum representable value

- **Given:** A duration of 100 weeks or more expressed in seconds (e.g. `secs = 100 * 604_800 = 60_480_000`).
- **When:** `duration_6ch(secs, QuantityStyle::Plain)` is called.
- **Then:** Returns the fixed clamp value `"99w06d"` regardless of how far `secs` exceeds the 99-week-6-day threshold; the result remains exactly 6 visible columns.

---

### AC-4: rounding roll-over promotes to the next magnitude rather than a four-digit mantissa

- **Given:** `n = 999_999` for `number_compact` (one below the `M`-tier boundary) and `n = 1_048_575` for `bytes_iec` (one byte below 1 MiB, whose K-tier mantissa rounds to `"1024.0"`).
- **When:** `number_compact(999_999, QuantityStyle::Plain)` and `bytes_iec(1_048_575, QuantityStyle::Plain)` are called.
- **Then:** `number_compact` returns `"1M"`, not `"1000k"`; `bytes_iec` returns `"1M"`, not `"1024K"` — in both cases the 0/1-decimal rounding that would otherwise reach `"1000"`/`"1024"` instead promotes to the next unit and re-renders.

---

### AC-5: bytes_si has no roll-over promotion and can render a four-digit mantissa at a tier boundary

- **Given:** `bytes = 999_999_999_999` (one below the 1 TB threshold of `1_000_000_000_000`).
- **When:** `bytes_si(999_999_999_999, QuantityStyle::Plain)` is called.
- **Then:** Returns `"1000.00 GB"`, not `"1.00 TB"` — unlike `number_compact`/`bytes_iec`, `bytes_si` selects its magnitude by comparing the raw value against fixed thresholds rather than a running mantissa, so it has no `starts_with("1000")` rollover guard and can legitimately produce a 4-digit mantissa.
- **Note:** This is the documented exception to the "no spurious width from rounding" property — see `docs/algorithm/008_quantity_formatting.md § SI-vs-IEC Contrast`.

---

### AC-6: formatters never panic across the full u64 domain

- **Given:** The extreme inputs `n = 0` and `n = u64::MAX` applied to `number_compact`, `bytes_iec`, and `bytes_si`.
- **When:** Each formatter is called with both extreme values under both `QuantityStyle::Plain` and `QuantityStyle::Colored`.
- **Then:** Every call returns a non-empty `String` with no panic and no overflow; there is no error variant to construct because all quantity/byte formatters are total functions of `(value, style)` — the input type is `u64`, not a fallible string, so no "invalid input" shape exists other than the numeric extremes of the domain itself.

---

### AC-7: Colored style dims only unit letters; visible glyph count matches Plain

- **Given:** `number_compact(14_464, style)` evaluated once under `QuantityStyle::Plain` and once under `QuantityStyle::Colored`.
- **When:** Both results are compared.
- **Then:** The `Colored` result wraps only the unit letter (`"k"`) in a gray SGR escape; the digits (`"14"`) remain unstyled; stripping ANSI escapes from the `Colored` result yields a string identical to the `Plain` result; both have the same visible glyph count (verified via the crate's `visual_len`).

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/algorithm/008_quantity_formatting.md`](../../../docs/algorithm/008_quantity_formatting.md) | Source algorithm spec — duration band selection, SI/IEC magnitude scaling, mantissa rendering, style policy |
| [`src/quantity/mod.rs`](../../../src/quantity/mod.rs) | `QuantityStyle`, `resolve()`, `styled_unit()` — style policy and unit dimming |
| [`src/quantity/duration.rs`](../../../src/quantity/duration.rs) | `duration_6ch()` — band selection, clamp, 6-column assembly |
| [`src/quantity/number.rs`](../../../src/quantity/number.rs) | `number_compact()`, `bytes_iec()`, `bytes_si()`, `format_mantissa()` — magnitude scaling |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/quantity_test.rs`](../../quantity_test.rs) (extend) | Spec tests for AC-1..AC-7 — quantity formatting algorithm |
