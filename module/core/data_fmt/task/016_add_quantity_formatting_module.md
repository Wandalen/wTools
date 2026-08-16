# 016 — Add feature-gated `quantity` formatting module (duration/age, number, byte size)

## MOST Goal

Give data_fmt a feature-gated `quantity` module that renders durations/ages, large counts, and byte sizes as fixed-width, terminal-aware, `NO_COLOR`-respecting strings — the ecosystem's single home for quantity formatting, composing with (never duplicating) the existing table/ANSI/width layers.

- **Motivated:** data_fmt owns every user-facing string-rendering primitive in the ecosystem (10 formatters, ANSI-visible-width measurement, color, terminal sizing) but has **zero** quantity formatters — no way to render a duration, an age, a humanized count, or a byte size in a fixed-width form that respects visible-glyph width and `NO_COLOR`. Consumers therefore hand-roll these. The concrete, committed consumer is the wplan compact job-listing table (**wplan TSK-1509**, filed alongside this task): its one-line-per-job layout is blocked on a fixed **6-visible-char** `NNuNNu` duration/age formatter with dimmed unit letters, which no crate should reimplement. data_fmt already carries the exact infrastructure these formatters need — `unicode_visual_len` / `unicode-width` for visible-width measurement, `ansi_str.rs` / `color_tools` for the dim SGR, `terminal_size` + `NO_COLOR` handling for plain-mode fallback — so the formatter belongs here, producing the cell strings that the existing `TableFormatter` then arranges.
- **Observable:** with `--features quantity`, `data_fmt::quantity::duration_6ch(146, Plain)` returns `"02m26s"` and its colored form has an **ANSI-stripped visible width of exactly 6**; `duration_6ch(36480, …)` → `"10h08m"`; the four magnitude bands (`MMmSSs` / `HHhMMm` / `DDdHHh` / `WWwDDd`) and the ≥99w clamp (`"99w06d"`) each render per the Test Matrix; in colored mode the **unit letters carry a dim SGR and the digits do not**; when `NO_COLOR` is set (or the caller reports a non-TTY) the output is plain ASCII with identical visible glyphs; `number_compact(14464, …)` → `"14k"` and `number_compact(26301958, …)` → `"26.3M"`. Every existing data_fmt test passes unchanged, and the module is **entirely absent from the build unless the `quantity` feature is enabled**.
- **Scoped:** a new `src/quantity/` module behind a new `quantity` Cargo feature; a `[features] quantity = [ … ]` entry reusing the existing color + `unicode-width` gates (no new external dependency); new golden tests under `tests/`; one `docs/` instance describing the formatters; and a **minor version bump `0.8.1` → `0.9.0`** so downstream can pin the quantity-bearing release. No change to any existing formatter, feature, public type, or ANSI/width helper.
- **Testable:** `cargo nextest run --all-features -E 'test(quantity)'` — new golden tests assert exact strings for every band boundary, the clamp, and both plain and colored modes (colored asserts visible width == 6 and that only unit letters are dimmed); `cargo build` with no `quantity` feature confirms the module is fully gated out; `cargo nextest run --all-features` shows no regression; `cargo clippy --all-targets --all-features -- -D warnings` is clean.

## In Scope

- `src/quantity/` (new module, entirely under `#[cfg(feature = "quantity")]`):
  - **`duration_6ch(secs: u64, style) -> String`** — fixed **6-visible-char** `NNuNNu` (two units, each 2 zero-padded digits, largest non-zero unit leads). Bands by total seconds:
    - `secs < 3600` → `MMmSSs` (minutes `00`–`59`, seconds `00`–`59`; `0s` → `00m00s`)
    - `3600 ≤ secs < 86_400` → `HHhMMm` (hours `01`–`23`)
    - `86_400 ≤ secs < 604_800` → `DDdHHh` (days `01`–`06`)
    - `secs ≥ 604_800` → `WWwDDd` (weeks `01`–`99`, days `00`–`06`)
    - **Clamp:** any value ≥ `99w06d` renders `99w06d` (never exceeds 6 visible chars).
    - Colored mode: unit letters wrapped in data_fmt's existing dim/gray SGR helper; digits unstyled. Visible width is always exactly 6 (verified via `unicode_visual_len`).
  - **`number_compact(n: u64, style) -> String`** — SI-compact (`k`/`M`/`G`/`T`); **confirmed anchors** `14464` → `"14k"`, `26301958` → `"26.3M"`; unit letter dimmed in colored mode. (Exact rounding/precision policy finalized during implementation via golden tests, constrained to reproduce those two anchors — see Out of Scope for what is deliberately left as latitude.)
  - **`bytes_iec(n: u64, style) -> String`** — IEC binary units (`K`/`M`/`G`/`Ti`…), ≤1 decimal, dimmed unit suffix.
  - **`QuantityStyle`** (`Plain` vs `Colored`) plus a resolver that honors `NO_COLOR` and a caller-supplied `is_tty`, delegating to data_fmt's existing color-enable path — **must not** introduce a second `NO_COLOR`/TTY code path.
- `Cargo.toml` — add `[features] quantity = [ <existing color + unicode-width gates> ]`; bump `version` `0.8.1` → `0.9.0`.
- `tests/quantity_*.rs` — golden tests covering every Test-Matrix row plus a build-time check that the module is gated out without the feature.
- `docs/` — one instance (feature/algorithm/invariant per data_fmt's own doc conventions) documenting the band table, the `99w06d` clamp, the fixed-6-visible-width invariant, and the plain/colored contract.

## Out of Scope

- Any change to existing formatters, features, public types, or ANSI/width helpers (tables, trees, expanded, json/yaml/html/logfmt/sql/toml, `ansi_str.rs`, `color_tools`).
- Status-glyph vocabulary, the compact/wide/detail job-table layout, column selection, or any wplan-specific rendering — those are the **consumer's** concern (wplan TSK-1509), not this module's.
- Publishing to crates.io and the downstream wplan re-pin — this task delivers the in-tree `0.9.0` bump; the actual `cargo publish` is the data_fmt maintainer's release step that unblocks TSK-1509, tracked there.
- Locale/i18n, negative or sub-second durations, and decimal (SI) byte sizes — deferred (no consumer). The exact `number_compact` rounding/precision policy beyond the two confirmed anchors is implementation latitude (pinned by golden test), not a deferral of the formatter itself (whose consumer is TSK-1509's quantity sweep plus the user-committed shared-quantity-home decision).

## Work Procedure

1. **Feature + version.** Add `quantity = [ … ]` to `Cargo.toml [features]`, reusing the existing color and `unicode-width` gate entries (no new external dep). Bump `version` `0.8.1` → `0.9.0`.
2. **Tests first (TDD).** Create `tests/quantity_duration.rs` with a golden assertion for every Test-Matrix duration row (plain + colored), each colored case also asserting `unicode_visual_len(result) == 6`. Write them failing before any implementation.
3. **Implement `duration_6ch`.** In `src/quantity/duration.rs`: select band by seconds, compute the two units, zero-pad to 2 digits each, largest-non-zero-unit-leads, apply the `99w06d` clamp; emit the plain 6-char form.
4. **Colored mode + style resolution.** Wrap unit letters (not digits) in data_fmt's existing dim/gray SGR helper; add `QuantityStyle` and a resolver that delegates to the existing `NO_COLOR`/`is_tty` color-enable path — reuse, do not reimplement.
5. **`number_compact` + `bytes_iec`.** Implement with their own golden tests; lock the `14464`→`14k` and `26301958`→`26.3M` anchors and the IEC byte forms.
6. **Docs.** Add the `docs/` instance (band table, clamp, fixed-6-width invariant, plain/colored contract) per data_fmt's doc conventions.
7. **Verify.** `cargo build` (no features) → module gated out; `cargo nextest run --all-features -E 'test(quantity)'` green; `cargo nextest run --all-features` full — no regression; `cargo clippy --all-targets --all-features -- -D warnings` clean. (data_fmt wrapper: `w3 .test level::3`.)

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| `duration_6ch(5, Plain)` | band `MMmSSs`, low | `"00m05s"` |
| `duration_6ch(146, Plain)` | band `MMmSSs` | `"02m26s"` |
| `duration_6ch(3600, Plain)` | band boundary → `HHhMMm` | `"01h00m"` |
| `duration_6ch(12060, Plain)` | band `HHhMMm` | `"03h21m"` |
| `duration_6ch(36480, Plain)` | band `HHhMMm` | `"10h08m"` |
| `duration_6ch(86400, Plain)` | band boundary → `DDdHHh` | `"01d00h"` |
| `duration_6ch(604800, Plain)` | band boundary → `WWwDDd` | `"01w00d"` |
| `duration_6ch(60_566_400, Plain)` | ≥ 99w6d | `"99w06d"` (clamp) |
| `duration_6ch(146, Colored)` | colored | visible width == 6; `m`/`s` dimmed, digits unstyled |
| `duration_6ch(146, Plain)` under `NO_COLOR` | plain fallback | identical to plain; no ANSI bytes |
| `number_compact(14464, Plain)` | SI compact | `"14k"` |
| `number_compact(26301958, Plain)` | SI compact | `"26.3M"` |
| `bytes_iec(1536, Plain)` | IEC binary | `"1.5K"` (1 decimal, IEC) |
| build with no `quantity` feature | gating | `data_fmt::quantity` absent; crate builds |

## Execution State

- **State:** 🎯 (Verified)
- **ID:** 016
- **Slug:** add_quantity_formatting_module
- **Executor:** any
- **Priority:** 4
- **Value:** 7
- **Easiness:** 6
- **Safety:** 8
- **Dir:** `module/core/data_fmt`
- **Closes:** null
- **Reopen Count:** 0

## History

- **[2026-08-16]** `CREATED` — Filed from the wplan compact job-listing design; data_fmt is the ecosystem formatting home and has zero quantity formatters. Concrete consumer: wplan TSK-1509 (needs the fixed 6-visible-char `NNuNNu` duration/age formatter). Feature-gated module, reuses existing ANSI/width/color infra, minor bump `0.8.1`→`0.9.0`.

## Related Documentation

- **Consumer (cross-repo):** `/home/user1/pro/lib/yrd_wplan/wplan/task/unverified/1509_adopt_data_fmt_quantity_in_wplan.md` — the committed wplan adoption task that consumes this module (its compact/wide job listing); the concrete need justifying this work. This task must ship (and the `0.9.0` release be published) before TSK-1509 can complete.
- `src/ansi_str.rs`, `src/themes.rs` / `color_tools` — existing visible-width (`unicode_visual_len`) and dim-SGR machinery this module reuses rather than duplicates.
- `src/formatters/table/` — the layer that arranges the cell strings this module produces (composition boundary: quantity makes the cell, table arranges it).

## Verification Record

**Gate Check** (Tier 2 — Dual-Role Self-Check, self-administered by dev; no subagent dispatch)

- **Date:** 2026-08-16
- **Result:** PASS — 8/8 dimensions 🟢 (1 Blocking finding caught and fixed)

| Gate | Name | Prev | Now | Issues | Fixes |
|------|------|------|-----|--------|-------|
| D1 | Scope Coherence | 🟢 | 🟢 | — | — |
| D2 | MOST Goal Quality | 🟢 | 🟢 | — | — |
| D3 | Value / YAGNI | 🟡 | 🟢 | Out-of-Scope phrase "no committed consumer" undercut `number_compact` (task 015's exact failure mode) | Reworded to defer only the precision policy; `duration_6ch`'s consumer (wplan TSK-1509) is ironclad |
| D4 | Implementation Readiness | 🟢 | 🟢 | — | — |
| D5 | Execution Scope | 🟢 | 🟢 | all deliverables inside `module/core/data_fmt`; wplan TSK-1509 is citation-only | — |
| D6 | Crate Scope Unity | 🟢 | 🟢 | — | — |
| D7 | Crate Locality | 🟢 | 🟢 | — | — |
| D8 | Crate Single Responsibility | 🟢 | 🟢 | — | — |
| **Total** | | 🔴 | 🟢 | 1 Blocking (fixed) | 8/8 PASS |
