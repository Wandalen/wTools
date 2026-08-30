# 020 — Extend `bytes_human` with a `TB` tier

## MOST Goal

Give `bytes_human` a `TB` magnitude so the base-1024 verbose formatter covers terabyte-scale sizes, restoring the sibling symmetry 017 designed for and unblocking two confirmed downstream defects that no existing `data_fmt` function can serve.

- **Motivated:** 017 built `bytes_si` as the base-1000 sibling of `bytes_human` with an explicitly *identical formatting policy* — "The **only** difference from `bytes_human` is the 1000-based (not 1024-based) divisor — same code shape, same rounding, same singular/plural, same `Plain`/`Colored` contract" (017 In Scope). That symmetry is broken in exactly one place: `bytes_si` carries a `TB` tier and `bytes_human` stops at `GB`, so a 2 TiB value renders `"2048.00 GB"` instead of promoting. No stated rationale accompanies the cap — `number.rs:141` records the behavior ("Magnitudes top out at `GB`") without a reason, while three doc sites describe it as a bare asymmetry against `bytes_si`. Two committed consumers are blocked by it, and **neither can be served by any other `data_fmt` function**: `bytes_si` has `TB` but base-1000 (reinterprets every value), `bytes_iec` has `T` but a compact single-letter layout (`1.5T`), `bytes_compact_si` has both problems.
  - **`wrun_core::format_file_size`** (`module/wrun_core/src/format.rs:51`, yrd_core `family_dev/default`) adopted `bytes_human` and **silently lost its TB tier** — filed there as BUG-1584. Its `wrun` re-export (`module/wrun/src/utils.rs:125`) inherits the regression.
  - **`will_clean::report::format_bytes`** (`module/will_clean/src/report.rs:14`) still hand-rolls a local `B`/`KB`/`MB`/`GB`/`TB` ladder and **cannot adopt `bytes_human`** without breaking `bug_reproducer(issue-294a)` (`module/will_clean/tests/test_report.rs:143`), which pins `1 TB` → `"1.00 TB"`. Tracked there as TSK-1540 AC-1, blocked on precisely this gap.
- **Observable:** with `--features quantity`, `bytes_human( 1_099_511_627_776, Plain )` → `"1.00 TB"` (was `"1024.00 GB"`); `bytes_human( 2_199_023_255_552, Plain )` → `"2.00 TB"`; every band below 1 TiB is byte-identical to today (`0 bytes`, `1 byte`, `512 bytes`, `1.00 KB`, `1.00 MB`, `1.00 GB`, `1.50 GB`); `Colored` continues to dim only the unit; the ANSI-stripped visible width equals the plain width; `bytes_human` remains absent from the build without the `quantity` feature.
- **Scoped:** one `else if` branch and one `const TB` in the existing `bytes_human` in `src/quantity/number.rs`, its rustdoc band table and "top out" sentence, the three `docs/algorithm/008_quantity_formatting.md` sites asserting the `GB` cap, new golden tests in `tests/quantity_test.rs`, and a **minor version bump `0.13.3` → `0.14.0`**. No change to any other formatter, feature, public type, threshold, or ANSI/width helper; no signature change; no new dependency.
- **Testable:** `cargo nextest run -p data_fmt --all-features -E 'binary(quantity_test)'` — new golden tests assert the `TB` band boundary, a mid-`TB` value, the highest `GB` value that must **not** promote (`1_099_511_627_775` → `"1023.99 GB"`), and `Colored` stripped-width parity; `cargo build -p data_fmt` with no `quantity` feature confirms gating; `cargo clippy -p data_fmt --all-targets --all-features -- -D warnings` clean.

## In Scope

- `src/quantity/number.rs` — in the existing `bytes_human`, add `const TB : u64 = GB * 1024;` and a leading `if bytes >= TB` branch formatting `{:.2} TB` via the existing `styled_unit`, mirroring `bytes_si`'s TB branch exactly. Update the rustdoc: replace the "Magnitudes top out at `GB`" sentence and add the `>= 1 TiB` row to the band table.
- `tests/quantity_test.rs` — golden tests for the `TB` boundary, a mid-`TB` value, the non-promoting `GB` ceiling, and `Colored` stripped-visible-width parity at `TB` scale.
- **Docs — five sites across three files** (a grep sweep for `tops out` found two files beyond the algorithm doc originally scoped):
  - `docs/algorithm/008_quantity_formatting.md` — the `bytes_si` sibling note (Abstract), the `bytes_human` counterpart note (promoted to a full step-list entry, since the algorithm doc previously described `bytes_human` only by contrast), the SI-vs-IEC contrast table's Units row, and the Key Properties four-digit-mantissa bullet (now true of both verbose formatters, not just `bytes_si`).
  - `docs/feature/008_quantity_formatting.md` — the byte-formatter selection guide's `bytes_human` band note.
  - `docs/api/006_quantity_formatting.md` — the formatter inventory's `bytes_human` band list, and the § Stability claim, which through `0.13.3` declared the `GB` cap "a stable, intentional divergence — not a gap to be reconciled". **This task deliberately reverses a documented API decision**; the rewritten stability note records the reversal, its rationale, and the version it landed in rather than silently dropping the old claim.
- `Cargo.toml` — bump `version` `0.13.3` → `0.14.0`. No new feature, no new dependency.

## Out of Scope

- Any change to `bytes_si`, `bytes_iec`, `bytes_compact_si`, `number_compact`, the duration formatters, `strip_ansi`, `visual_len`, or any table/tree/expanded formatter.
- A `PB`/`EB` tier — no consumer exists; adding one now is the speculative-consumer YAGNI failure 016's D3 and 017's Out of Scope both caught. `TB` is added because two consumers concretely require it today.
- The `bytes_si` four-digit-mantissa property at its own tier boundary (doc 008 § Key Properties) — pre-existing, unrelated, and untouched here.
- **Consumer-side adoption** — fixing BUG-1584 (`wrun_core`) and TSK-1540 AC-1 (`will_clean`) are `family_dev/default` concerns, gated on this task publishing.
- **Publishing to crates.io** — this task delivers the in-tree `0.14.0` bump; the actual `cargo publish` is the maintainer's user-gated release step, as 017 established.

## Work Procedure

1. **Tests first (TDD).** Add the four golden assertions to `tests/quantity_test.rs`; confirm the TB cases fail against the current `GB`-capped implementation.
2. **Implement.** Add `const TB` and the `if bytes >= TB` branch at the head of `bytes_human`'s chain, mirroring `bytes_si`. Reuse `styled_unit` — do not introduce a second color path.
3. **Docs.** Update the `bytes_human` rustdoc band table and "top out" sentence; correct the three `008_quantity_formatting.md` sites.
4. **Version.** Bump `Cargo.toml` `0.13.3` → `0.14.0`.
5. **Verify.** Scoped, per this repo's `rulebook.md` (full-workspace runs are forbidden for agents): `cargo nextest run -p data_fmt --all-features -E 'binary(quantity_test)'`; `cargo nextest run -p data_fmt --all-features` for crate-level regression; `cargo build -p data_fmt` without `quantity`; `cargo clippy -p data_fmt --all-targets --all-features -- -D warnings`.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| `bytes_human( 1_099_511_627_776, Plain )` | band boundary → TB | `"1.00 TB"` |
| `bytes_human( 2_199_023_255_552, Plain )` | mid-TB band | `"2.00 TB"` |
| `bytes_human( 1_649_267_441_664, Plain )` | TB band, 2-decimal | `"1.50 TB"` |
| `bytes_human( 1_098_437_885_952, Plain )` | highest clean non-promoting GB | `"1023.00 GB"` |
| `bytes_human( 1_099_511_627_775, Plain )` | 1 byte below the boundary | `"1024.00 GB"` — mantissa rounds up but does **not** promote; the pre-existing four-digit artifact doc 008 already records for `bytes_si`, unchanged here |
| `bytes_human( 1_073_741_824, Plain )` | GB band unchanged | `"1.00 GB"` |
| `bytes_human( 1024, Plain )` | KB band unchanged | `"1.00 KB"` |
| `bytes_human( 1, Plain )` | singular special-case unchanged | `"1 byte"` |
| `bytes_human( 1_099_511_627_776, Colored )` | colored at TB scale | stripped width == Plain width; `TB` dimmed, digits unstyled |
| `bytes_si( 1_099_511_627_776, Plain )` (contrast) | base-1000 sibling, same input | `"1.10 TB"` (bases still diverge) |
| build with no `quantity` feature | gating | `bytes_human` absent; crate builds |

## Execution State

- **State:** ✅ (Completed)
- **ID:** 020
- **Slug:** extend_bytes_human_tb_tier
- **Executor:** any
- **Priority:** 4
- **Value:** 7
- **Easiness:** 9
- **Safety:** 8
- **Dir:** `module/core/data_fmt`
- **Closes:** null
- **Reopen Count:** 0

## History

- **[2026-08-30]** `COMPLETED` — Implemented, verified, and closed. `bytes_human` gained `const TB : u64 = GB * 1024;` and a leading `if bytes >= TB` branch mirroring `bytes_si`'s shape exactly; 3 new golden tests plus a 7th entry in the colored-parity size array. **TDD confirmed red first** — `1 TiB` rendered `"1024.00 GB"` against the pre-change build. Verification (all scoped per this repo's `rulebook.md`; no full-workspace run): `cargo nextest run -p data_fmt --all-features` **1007/1007 pass**; `cargo test --doc -p data_fmt --all-features` with `RUSTDOCFLAGS="-D warnings"` **86/86 pass**; `cargo build -p data_fmt --no-default-features` confirms the `quantity` gate still holds; `cargo clippy -p data_fmt --all-targets --all-features -- -D warnings` **clean**; `cargo check -p config_hierarchy` confirms the sole in-workspace consumer still resolves. Tier 2 Dual-Role Self-Check over 6 gates: **6/6 PASS**, 3 defects caught by the adversarial pass and fixed — (1) the planned `"1023.99 GB"` boundary assertion was arithmetically wrong (`{:.2}` rounds 1023.99999999906 **up**), corrected to `"1024.00 GB"`; (2) the doc surface was 5 sites across 3 files, not the 3 sites in 1 file originally scoped; (3) the workspace pin `~0.13.2` excludes `0.14.0` and would have broken `config_hierarchy` resolution — bumped to `~0.14.0` in lockstep, the same coupling task 017 established. Notably `docs/api/006_quantity_formatting.md` § Stability had declared the `GB` cap "a stable, intentional divergence — not a gap to be reconciled"; that note was **rewritten to record the reversal, its rationale, and the version it landed in** rather than silently dropped, since this task knowingly overturns a documented API decision — which is what the minor bump signals. `readme.md` needed no change (it never described the byte bands). **Not yet published to crates.io** — that remains the maintainer's user-gated release step, per 017.
- **[2026-08-30]** `CREATED` — Filed from the yrd_core `family_dev/default` adoption sweep, where two independent consumers hit the same wall. BUG-1584 root-caused it precisely: no published `data_fmt` function offers base-1024 + verbose + `TB` together (`bytes_human` caps at `GB`; `bytes_si` is base-1000; `bytes_iec`/`bytes_compact_si` are compact), and its Fix Location names "extend `data_fmt`" as resolution 1 — "Correct at the root and fixes every current and future consumer at once, including unblocking TSK-1540." Verified against `data_fmt 0.13.3` source that the cap is real and that no existing test asserts `bytes_human` output at or above 1 TiB, so the added tier breaks no golden. Scope held to `TB` only (no `PB`/`EB`) per 016 D3 / 017 no-speculative-consumer discipline.

## Related Documentation

- `docs/algorithm/008_quantity_formatting.md` — quantity formatting algorithm, band tables, SI-vs-IEC contrast
- `task/completed/017_extend_quantity_si_bytes.md` — added `bytes_si`; established the sibling-symmetry design intent this task restores
- `task/completed/016_add_quantity_formatting_module.md` — introduced the `quantity` module, `bytes_human`, `bytes_iec`
