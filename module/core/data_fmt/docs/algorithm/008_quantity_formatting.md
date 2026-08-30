# Algorithm: Quantity Formatting

### Scope

- **Purpose**: Render durations/ages, humanized counts, and byte sizes as compact, optionally color-dimmed strings for dense columnar CLI output.
- **Responsibility**: Documents duration band selection and the fixed 6-visible-column contract, SI/IEC magnitude scaling with rounding roll-over, and the plain/colored unit-dimming rule.
- **In Scope**: `duration_6ch` band selection and clamp, `number_compact` SI scaling, `bytes_iec` IEC scaling, `bytes_si` SI/decimal byte scaling, mantissa rendering, `QuantityStyle` and the `NO_COLOR`/TTY resolution policy.
- **Out of Scope**: Terminal width resolution (see `feature/005_auto_fit.md § Terminal Width Detection`), table/column layout — these formatters emit standalone cell strings only.

### Features

| File | Relationship |
|------|-------------|
| [008_quantity_formatting.md](../feature/008_quantity_formatting.md) | Feature that this algorithm implements |

### Sources

| File | Relationship |
|------|-------------|
| [`src/quantity/mod.rs`](../../src/quantity/mod.rs) | `QuantityStyle`, `resolve()`, `styled_unit()` — style policy and unit dimming |
| [`src/quantity/duration.rs`](../../src/quantity/duration.rs) | `duration_6ch()` — band selection, clamp, 6-column assembly |
| [`src/quantity/number.rs`](../../src/quantity/number.rs) | `number_compact()`, `bytes_iec()`, `bytes_si()`, `format_mantissa()` — magnitude scaling |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/quantity_test.rs`](../../tests/quantity_test.rs) | Golden values per band/magnitude, the 6-column width invariant, and roll-over/clamp edges |

### Abstract

Three formatters share one style policy. `duration_6ch` selects a two-unit layout by magnitude band so its **visible** width is always exactly 6 columns (`NNuNNu`), clamping at `99w06d`. `number_compact` and `bytes_iec` scale a value into the largest fitting magnitude (base 1000 / base 1024 respectively) and render a short mantissa, promoting on a rounding roll-over so the mantissa never grows a spurious fourth digit. Under `QuantityStyle::Colored` the unit letters are dimmed via a gray SGR while the digits stay unstyled; the visible glyph count is identical to `Plain` because the ANSI escapes carry zero display width.

`bytes_si` (also in `number.rs`) is the decimal/SI sibling of `bytes_human`: identical verbose `N.NN UNIT` layout, singular/plural sub-`KB` rendering, and `KB`/`MB`/`GB`/`TB` band set — the divisor (1000 rather than 1024) is the only difference between them. See § SI-vs-IEC Contrast below.

### Algorithm

**duration_6ch(secs, style):**

1. **Select band** by `secs`: `< 1h` → (minutes, `m`, seconds, `s`); `< 1d` → (hours, `h`, minutes, `m`); `< 1w` → (days, `d`, hours, `h`); `>= 1w` → (weeks, `w`, days, `d`).
2. **Clamp**: in the weeks band, if `weeks > 99`, emit the fixed pair `(99, w, 6, d)` — the largest representable value.
3. **Assemble**: each unit renders as a zero-padded 2-digit value (`{:02}`) immediately followed by its unit letter; concatenate the two segments. Every band yields exactly `2 digits + 1 unit + 2 digits + 1 unit` = 6 visible columns.

**number_compact(n, style)** (base 1000, units `k`/`M`/`G`/`T`):

1. If `n < 1000`, return `n` verbatim (no unit).
2. Divide by 1000 repeatedly, incrementing the unit index, while the running value is `>= 1000` and a larger unit remains.
3. Render the mantissa with 0 fractional digits at the `k` tier and 1 at larger tiers, dropping a trailing `.0`.
4. **Roll-over promotion**: if the rendered mantissa reached `"1000"` (0-decimal rounding at a tier boundary) and a larger unit exists, advance one unit and re-render the mantissa with 1 digit — so e.g. `999_999` becomes `1M`, never `1000k`.

**bytes_iec(n, style)**: identical to `number_compact` but base 1024, units `K`/`M`/`G`/`T`, `NB` below 1024, mantissa always up to 1 digit, roll-over threshold `"1024"`.

**bytes_si(bytes, style)** (base 1000, verbose layout, units `KB`/`MB`/`GB`/`TB`):

1. Below `1 KB` (1000), render an exact integer count with the spelled-out unit word: singular `1 byte`, plural `N bytes` (`0` and `2+`).
2. `>= 1 KB`, select the magnitude among `KB`/`MB`/`GB`/`TB` (1000-based) by comparing the raw `bytes` value against each threshold in descending order — not by scaling a running mantissa like `number_compact`/`bytes_iec` — and render with exactly 2 fractional digits (`{:.2}`). There is no roll-over promotion step, so a value just below a tier's threshold can still round up to a 4-digit mantissa after formatting: `999_999_999_999` (just under the 1 TB threshold) renders `"1000.00 GB"`, not `"1.00 TB"` — `number_compact`/`bytes_iec` avoid this via their `starts_with("1000"/"1024")` rollover check; `bytes_si` has none.
3. Same `styled_unit` dimming as every other formatter in this module — the unit token is dimmed under `Colored`, the digits and separating space are not.

**bytes_human(bytes, style)** (base 1024, verbose layout, units `KB`/`MB`/`GB`/`TB`): step-for-step identical to `bytes_si` above — same sub-`KB` singular/plural counts, same descending threshold comparison against the raw `bytes` value, same 2-decimal mantissa, same absent roll-over promotion, same `styled_unit` dimming — with `1024` substituted for `1000` at every threshold. Because it too lacks a promotion step, it shows the same tier-edge artifact: `1_099_511_627_775` (one byte under 1 TiB) renders `"1024.00 GB"`, not `"1.00 TB"`.

The two are pure decimal/binary counterparts — see § SI-vs-IEC Contrast.

**Styling**: `styled_unit(unit, style)` returns the unit unchanged under `Plain`; under `Colored` it wraps the unit in a gray SGR via `DecoratedText` (digits are never wrapped). `QuantityStyle::resolve(is_tty)` returns `Colored` only when `is_tty` is true **and** `NO_COLOR` is unset, else `Plain`.

### SI-vs-IEC Contrast

Two byte-size bases coexist in this module, each with a real, distinct consumer:

| | `bytes_iec` / `bytes_human` (IEC, base 1024) | `bytes_si` (SI, base 1000) |
|---|---|---|
| Divisor | 1024 per step | 1000 per step |
| Units | `K`/`M`/`G`/`T` (`bytes_iec`), `KB`/`MB`/`GB`/`TB` (`bytes_human`) | `KB`/`MB`/`GB`/`TB` |
| Layout | compact single-letter (`bytes_iec`) or verbose 2-decimal (`bytes_human`) | verbose 2-decimal |
| Use when | reporting actual binary sizes (RAM pages, buffer capacities) where 1 KiB genuinely is 1024 bytes | reporting a figure already computed in decimal (disk vendors, network throughput, or an existing decimal convention such as `glassbox`'s memory report) |

`bytes_si( 1_500_000, Plain )` → `"1.50 MB"` vs `bytes_human( 1_500_000, Plain )` → `"1.43 MB"` on the **same raw byte count** — the two are not interchangeable; a consumer must pick the base matching its own units rather than assuming either is "the" byte formatter.

### Key Properties

- **Fixed 6-column duration**: `duration_6ch` output is always exactly 6 visible columns (verified via the crate's `visual_len`), independent of magnitude and style — the property columnar callers depend on for alignment.
- **Digits never dimmed**: only unit letters carry color; a colored result and its plain counterpart have identical visible glyph counts because ANSI SGR escapes have zero display width.
- **No spurious width from rounding (except the verbose pair)**: `number_compact`/`bytes_iec`/`bytes_compact_si` promote to the next unit on a rounding roll-over rather than emitting a four-digit mantissa (`1000k`, `1024K`). The two verbose formatters — `bytes_si` and `bytes_human` — have no such promotion step and can each render a 4-digit mantissa at a tier boundary (see Algorithm above: `"1000.00 GB"` and `"1024.00 GB"` respectively).
- **Pure formatting path**: the environment (`NO_COLOR`) is consulted only in `resolve`; the formatters themselves are deterministic pure functions of `(value, style)`.
- **Std-only**: assembly uses `format!` and integer/float arithmetic plus the crate's existing `DecoratedText`; no additional dependencies.

### Complexity

- Time: O(1) — a bounded number of divisions (≤4 magnitude steps) and a single `format!`.
- Space: O(1) beyond the returned string.
