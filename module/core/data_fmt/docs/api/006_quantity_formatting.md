# API: Quantity Formatting

### Scope

- **Purpose**: Document the public API surface for terminal-aware quantity formatting.
- **Responsibility**: Define `QuantityStyle`, `DurationError`, and the signatures/contracts of all ten public formatting functions.
- **In Scope**: Function signatures, `QuantityStyle` construction and resolution, feature gating (`quantity`, `quantity_parse`), error variants.
- **Out of Scope**: Band-selection and magnitude-scaling algorithm internals (see `../algorithm/008_quantity_formatting.md`), usage patterns (see `../feature/008_quantity_formatting.md`).

### Sources

| File | Relationship |
|------|-------------|
| [`src/quantity/mod.rs`](../../src/quantity/mod.rs) | `QuantityStyle`, `resolve()`, module re-exports |
| [`src/quantity/duration.rs`](../../src/quantity/duration.rs) | `duration_6ch`, `duration_human`, `duration_human_hours`, `duration_ms`, `parse_duration`, `DurationError` |
| [`src/quantity/number.rs`](../../src/quantity/number.rs) | `number_compact`, `bytes_iec`, `bytes_human`, `bytes_si`, `bytes_compact_si` |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/quantity_test.rs`](../../tests/quantity_test.rs) | Golden values per function, band, and magnitude |

### Abstract

Ten public functions plus one style enum form the quantity formatting API: four duration formatters, five number/byte formatters, and one string-to-`Duration` parser gated behind the opt-in `quantity_parse` feature. Every formatter is a pure function of `(value, style)` — the environment is consulted only by `QuantityStyle::resolve`. All ten functions and `QuantityStyle` require the `quantity` feature (default-enabled).

### Operations

#### QuantityStyle

```rust
pub enum QuantityStyle { Plain, Colored }
impl QuantityStyle { pub fn resolve( is_tty : bool ) -> Self }
```

`resolve( is_tty )` returns `Colored` only when `is_tty` is `true` **and** the `NO_COLOR` environment variable is unset; any other combination yields `Plain`. This is the one function in the module that reads the environment — every formatter itself stays pure and deterministic.

#### Duration Functions

```rust
pub fn duration_6ch( secs : u64, style : QuantityStyle ) -> String
pub fn duration_human( secs : u64, style : QuantityStyle ) -> String
pub fn duration_human_hours( secs : u64, style : QuantityStyle ) -> String
pub fn duration_ms( ms : u64, style : QuantityStyle ) -> String
```

`duration_6ch` — fixed 6-visible-column form (`NNuNNu`) for aligned table cells; clamps at `99w06d`. `duration_human` — variable-width, at most two tiers (days→hours→minutes→seconds), for inline prose. `duration_human_hours` — identical to `duration_human` below one day, but never introduces a day tier (keeps counting hours: `90_061` → `"25h 1m"` vs `duration_human`'s `"1d 1h"`). `duration_ms` — sub-second precision (`Nms`) below one second, `N.NNs` (truncated, never rounded) below one minute, falls back to `duration_human` tiers at or above one minute.

#### Number/Byte Functions

```rust
pub fn number_compact( n : u64, style : QuantityStyle ) -> String
pub fn bytes_iec( n : u64, style : QuantityStyle ) -> String
pub fn bytes_human( bytes : u64, style : QuantityStyle ) -> String
pub fn bytes_si( bytes : u64, style : QuantityStyle ) -> String
pub fn bytes_compact_si( n : u64, style : QuantityStyle ) -> String
```

`number_compact` — SI magnitude scaling (`k`/`M`/`G`/`T`, base 1000), compact single-letter form. `bytes_iec` — IEC binary magnitude scaling (`K`/`M`/`G`/`T`, base 1024), compact single-letter form. `bytes_human` — verbose `N.NN UNIT` form, base 1024, `KB`/`MB`/`GB`/`TB`, singular/plural exact count below `1 KB`. `bytes_si` — decimal/SI sibling of `bytes_human`: same verbose layout, same sub-`KB` exact-count rule, same band set, but base 1000 — the same raw byte count renders differently under the two (`1_500_000` → `"1.50 MB"` via `bytes_si` vs `"1.43 MB"` via `bytes_human`). `bytes_compact_si` — decimal/compact sibling of `bytes_iec`: base 1000, single-letter units, but magnitude-adaptive precision (one decimal below 10× a unit, none at 10× and above) rather than `bytes_iec`'s always-up-to-one-decimal-with-trailing-zero-dropped policy.

#### DurationError and parse_duration

Gated behind the opt-in `quantity_parse` feature (pulls in the `humantime` dependency):

```rust
#[ cfg( feature = "quantity_parse" ) ]
pub enum DurationError { Empty, InvalidFormat( String ), Overflow( String ) }

#[ cfg( feature = "quantity_parse" ) ]
pub fn parse_duration( s : &str ) -> Result< core::time::Duration, DurationError >
```

The inverse of `duration_human`: reads compact human duration strings (`"1h30m"`, `"7d"`, long forms like `"90 seconds"`) back into a `core::time::Duration`, following the `humantime` grammar (`s`, `m`, `h`, `d`, `w`).

### Error Handling

The ten formatting functions never return errors — every `(value, style)` input produces a defined string. `parse_duration` (feature `quantity_parse` only) returns `Result< Duration, DurationError >` with three variants: `Empty` (empty input string), `InvalidFormat( String )` (unrecognized duration syntax), `Overflow( String )` (parsed value exceeds `u64::MAX / 2` seconds).

### Feature Flags

| Feature | Enables | External Deps |
|---------|---------|----------------|
| `quantity` | `QuantityStyle` and all ten formatting functions (default-enabled; requires `enabled`) | None |
| `quantity_parse` | `parse_duration`, `DurationError` | `humantime` |

### Compatibility Guarantees

All ten formatter signatures are stable — `(value : u64, style : QuantityStyle) -> String` (or `&str` for `parse_duration`'s input). `duration_6ch`'s 6-visible-column output width is a stable invariant callers may depend on for alignment. Digits are never ANSI-styled under `Colored`; only unit letters are — a `Colored` result and its `Plain` counterpart always have identical visible glyph counts. `bytes_human` and `bytes_si` carry the same `KB`/`MB`/`GB`/`TB` band set; the 1024-vs-1000 divisor is their only difference, and it is a stable, intentional divergence — not a gap to be reconciled. (Through `0.13.3`, `bytes_human` topped out at `GB` and that cap was itself documented here as intentional; task `020` reversed it in `0.14.0` after two consumers required base-1024 verbose `TB` output that no other formatter in this crate provides. A `>= 1 TiB` value that rendered `"1024.00 GB"` or `"2048.00 GB"` now renders `"1.00 TB"` / `"2.00 TB"` — the behavior change the minor bump signals.)
