# 017 — Extend `quantity` with SI (decimal, 1000-based) byte formatting

## MOST Goal

Give `data_fmt::quantity` an SI/decimal (1000-based) byte-size formatter — the base-1000 sibling of the existing base-1024 `bytes_human` — so ecosystem consumers that report storage/memory in decimal units (GB = 1 000 000 000 bytes) have a shared primitive instead of hand-rolling one. Revives 016's explicitly-deferred "decimal (SI) byte sizes — deferred (no consumer)" now that a concrete consumer exists.

- **Motivated:** data_fmt is the ecosystem's single home for user-facing quantity strings; 016 shipped `bytes_iec` (IEC binary, single-letter `1.5K`) and `bytes_human` (binary/1024, 2-decimal `1.50 KB`), but **explicitly deferred decimal/SI byte sizes for lack of a consumer** (016 Out of Scope, line 34). That consumer now exists and is committed: **`glassbox::memory_tracking::report::format_bytes`** (`module/glassbox/src/memory_tracking/report.rs:186`) hand-rolls SI byte formatting with `const KB = 1_000; MB = 1_000_000; GB = 1_000_000_000` — a **deliberate decimal base** (RAM/tooling reports in GB=10⁹, not GiB), so it cannot adopt the existing 1024-based `bytes_human`/`bytes_iec` without changing its numbers. data_fmt already owns the visible-width/color/`NO_COLOR` infrastructure this formatter needs; the SI variant belongs beside its IEC sibling, not re-implemented in a leaf crate.
- **Observable:** with `--features quantity`, a new `data_fmt::quantity::bytes_si( n, style )` renders decimal/1000-based sizes as the SI sibling of `bytes_human`: `bytes_si( 0, Plain )` → `"0 bytes"`; `bytes_si( 1, Plain )` → `"1 byte"`; `bytes_si( 512, Plain )` → `"512 bytes"`; `bytes_si( 1_000, Plain )` → `"1.00 KB"`; `bytes_si( 1_500_000, Plain )` → `"1.50 MB"` (contrast `bytes_human( 1_500_000 )` = `"1.43 MB"`, base-1024); `bytes_si( 2_304_000_000, Plain )` → `"2.30 GB"`; in `Colored` mode the unit suffix carries the dim SGR and the digits do not, and the ANSI-stripped visible width equals the plain width; under `NO_COLOR`/non-TTY the output is plain ASCII. Every existing data_fmt test passes unchanged; `bytes_si` is entirely absent from the build unless the `quantity` feature is enabled.
- **Scoped:** one new public function `bytes_si` in the existing `src/quantity/number.rs` (beside `bytes_iec`/`bytes_human`), re-exported exactly as its siblings are; new golden tests in the existing `tests/quantity_test.rs`; the `docs/algorithm/008_quantity_formatting.md` instance extended with the SI band table and the SI-vs-IEC contrast; and a **minor version bump `0.11.0` → `0.12.0`** (plus the yrd_core central `[workspace.dependencies.data_fmt]` requirement bumped in lockstep) so downstream can pin the SI-bearing release. No change to any existing formatter, feature, public type, threshold, or ANSI/width helper.
- **Testable:** `cargo nextest run -p data_fmt --all-features -E 'binary(quantity_test)'` — new golden tests assert exact strings for every SI band boundary (`bytes`/`KB`/`MB`/`GB`/`TB`), the `1 byte` singular special-case, the 1000-vs-1024 divergence against a matched `bytes_human` input, and both Plain and Colored modes (Colored asserts stripped-visible-width parity with Plain and that only the unit suffix is dimmed); `cargo build -p data_fmt` with no `quantity` feature confirms `bytes_si` is gated out; `cargo nextest run -p data_fmt --all-features` shows no regression; `cargo clippy -p data_fmt --all-targets --all-features -- -D warnings` is clean.

## In Scope

- `src/quantity/number.rs` — add **`bytes_si( n: u64, style: QuantityStyle ) -> String`** under the existing `#[ cfg( feature = "quantity" ) ]` gate, beside `bytes_iec` and `bytes_human`:
  - Decimal/SI thresholds: `1 KB = 1_000`, `1 MB = 1_000_000`, `1 GB = 1_000_000_000`, `1 TB = 1_000_000_000_000` bytes.
  - Formatting policy identical to `bytes_human`'s (its base-1024 sibling): raw byte counts below 1 KB render as an integer with the `byte`/`bytes` word (singular `1 byte`, `0 bytes`, `512 bytes`); `≥ 1 KB` renders 2-decimal with the two-letter SI unit (`1.00 KB`, `1.50 MB`, `2.30 GB`). The **only** difference from `bytes_human` is the 1000-based (not 1024-based) divisor — same code shape, same rounding, same singular/plural, same `Plain`/`Colored` contract.
  - `Colored` mode wraps the unit suffix (not the digits) in data_fmt's existing dim/gray SGR helper, exactly as `bytes_human`/`bytes_iec` do; reuse the existing `QuantityStyle` resolver — **must not** introduce a second color/`NO_COLOR`/TTY path.
- `src/lib.rs` (or the `quantity` module re-export site) — re-export `bytes_si` in the same place and manner as `bytes_iec`/`bytes_human`.
- `Cargo.toml` — bump `version` `0.11.0` → `0.12.0`. **No** new feature, **no** new dependency (reuses the existing `quantity` gate).
- Workspace-root `Cargo.toml` (yrd_core `wtools/dev`) — bump the central `[workspace.dependencies.data_fmt]` requirement in lockstep with the crate version (016's Deviation 2 established this is required or the workspace won't resolve).
- `tests/quantity_test.rs` — add golden tests covering every SI band boundary, the `1 byte`/`0 bytes` special-cases, a 1000-vs-1024 divergence assertion (same input to `bytes_si` and `bytes_human`, different output), and Colored-stripped-equals-Plain width parity.
- `docs/algorithm/008_quantity_formatting.md` — extend with the `bytes_si` SI band table and an explicit SI-vs-IEC/`bytes_human` contrast note (why both bases exist, when to use which).

## Out of Scope

- Any change to existing formatters, features, public types, thresholds, or ANSI/width helpers (`duration_6ch`, `duration_human`, `duration_ms`, `parse_duration`, `number_compact`, `bytes_iec`, `bytes_human`, `strip_ansi`, `visual_len`, tables/trees/expanded, `ansi_str.rs`, `color_tools`).
- **A full-precision / 3-tier duration variant** (e.g. `wrun`'s `{h}h {m}m {s}s` all-tiers form plus 3-decimal sub-minute `{}.{:03}s`, `module/wrun/src/utils.rs:119`) — **deferred, consumer-gated** exactly as 016 deferred SI bytes. The existing `duration_human` (2-tier, trailing-zero-dropping) and `duration_ms` (2-decimal hundredths) already cover most duration needs at "equivalent-or-improved" output; whether `wrun` genuinely requires its exact 3-tier/3-decimal form preserved (rather than adopting `duration_human`/`duration_ms`) is determined **during the wplan adoption** (the per-crate `wrun` adoption step of the decomposed TSK-1539 initiative). Build this variant only if that step confirms the exact output must be preserved — then as a separate follow-on data_fmt task (018), not here. Filing it now, before that confirmation, would be the speculative-consumer YAGNI failure 016's D3 caught.
- Locale/i18n, negative or sub-second byte counts, mixed SI/IEC auto-detection, and any consumer-side migration (the glassbox adoption itself is TSK-1539's concern, gated on this task publishing).
- Publishing to crates.io — this task delivers the in-tree `0.12.0` bump; the actual `cargo publish` is the data_fmt maintainer's release step (user-gated), which unblocks the glassbox half of the wplan adoption.

## Work Procedure

1. **Version + central pin.** Bump `Cargo.toml` `version` `0.11.0` → `0.12.0`; bump the yrd_core workspace-root `[workspace.dependencies.data_fmt]` requirement in lockstep (else the workspace won't resolve — 016 Deviation 2). No new feature or dependency.
2. **Tests first (TDD).** In `tests/quantity_test.rs`, add golden assertions for every SI band boundary (`0`/`1`/`512` bytes; `1_000` → `1.00 KB`; `1_500_000` → `1.50 MB`; `2_304_000_000` → `2.30 GB`; a TB-band value), the `1 byte` singular, and a `bytes_si` vs `bytes_human` divergence on the same input (`1_500_000` → `1.50 MB` vs `1.43 MB`). Add a Colored case asserting stripped-visible-width parity with Plain and that only the unit suffix is dimmed. Write them failing first.
3. **Implement `bytes_si`.** In `src/quantity/number.rs`, add `bytes_si` beside `bytes_human`, reusing its exact structure with the 1000-based divisor and thresholds; wire the `Colored` unit-suffix dimming through the existing `QuantityStyle` resolver (reuse, do not reimplement). Re-export in `lib.rs` alongside the siblings.
4. **Docs.** Extend `docs/algorithm/008_quantity_formatting.md` with the SI band table and the SI-vs-IEC/`bytes_human` contrast (why both bases coexist; consumer guidance).
5. **Verify.** `cargo build -p data_fmt` (no `quantity` feature) → `bytes_si` gated out; `cargo nextest run -p data_fmt --all-features -E 'binary(quantity_test)'` green; `cargo nextest run -p data_fmt --all-features` full — no regression; `cargo clippy -p data_fmt --all-targets --all-features -- -D warnings` clean. (data_fmt wrapper: `w3 .test level::3`.) Run host-side, detached, scoped `-p data_fmt` (this crate is a yrd_core wtools `dev`-workspace member, verified host-side, not in the wplan container).

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| `bytes_si( 0, Plain )` | below-1KB integer band | `"0 bytes"` |
| `bytes_si( 1, Plain )` | singular special-case | `"1 byte"` |
| `bytes_si( 512, Plain )` | below-1KB integer band | `"512 bytes"` |
| `bytes_si( 1_000, Plain )` | band boundary → KB | `"1.00 KB"` |
| `bytes_si( 1_500_000, Plain )` | MB band, 2-decimal | `"1.50 MB"` |
| `bytes_human( 1_500_000, Plain )` (contrast) | base-1024 sibling, same input | `"1.43 MB"` (proves SI vs IEC divergence) |
| `bytes_si( 2_304_000_000, Plain )` | GB band | `"2.30 GB"` |
| `bytes_si( 1_000_000_000_000, Plain )` | band boundary → TB | `"1.00 TB"` |
| `bytes_si( 1_500_000, Colored )` | colored | stripped-visible width == Plain width; unit `MB` dimmed, digits unstyled |
| `bytes_si( 1_500_000, Plain )` under `NO_COLOR` | plain fallback | identical to Plain; no ANSI bytes |
| build with no `quantity` feature | gating | `data_fmt::quantity::bytes_si` absent; crate builds |

## Execution State

- **State:** ✅ (Done)
- **ID:** 017
- **Slug:** extend_quantity_si_bytes
- **Executor:** any
- **Priority:** 3
- **Value:** 6
- **Easiness:** 8
- **Safety:** 8
- **Dir:** `module/core/data_fmt`
- **Closes:** null
- **Reopen Count:** 0

## History

- **[2026-08-18]** `CREATED` — Filed from the "extend data_fmt, don't clone" workspace-adoption review. Revives 016's explicitly-deferred decimal/SI byte sizes now that a concrete committed consumer exists: `glassbox::memory_tracking::report::format_bytes` uses a deliberate 1000-based (`KB=1_000`/`MB=10⁶`/`GB=10⁹`) format that cannot adopt the existing 1024-based `bytes_human`/`bytes_iec`. Adds `bytes_si` as the base-1000 sibling of `bytes_human` (identical policy, 1000-divisor), minor bump `0.11.0`→`0.12.0` + central pin lockstep. The full-precision/3-tier duration variant (consumer `wrun`) is deliberately deferred, consumer-gated, per the 016 no-speculative-consumer discipline. Consumer/adoption tracked in wplan TSK-1539.
- **[2026-08-18]** `VERIFIED` — Readiness Verification Gate PASS (Tier 2 Dual-Role Self-Check, self-administered by ai; 8/8 dimensions 🟢, 0 Blocking). State ❓ → 🎯; moved `unverified/` → task root; registered in the data_fmt Tasks Index. Execution-ready; the glassbox consumer's adoption remains gated on publishing `0.12.0` (tracked in wplan TSK-1539).
- **[2026-08-18]** `ACCEPTED` — Acceptance Verification Gate PASS (Tier 5 Full MAAV, 2 rounds + 1 mechanical fix). R1's reported BUG-023 test failures were a false negative from concurrent-cargo contention across the 2 parallel verifier agents, not a real defect — confirmed via direct isolated re-run and a fresh uncontended full-suite run (940/940). R2 fresh challenger independently reconfirmed clean and scope-disjoint from the BUG-023 fix site. Corrected the task's own imprecise `-E 'test(bytes_si)'` filter (4/6 match) to `-E 'binary(quantity_test)'` (41/41). State 🎯 → ✅; moved to `completed/`.

## Related Documentation

- **Consumer (cross-repo, adoption-side):** `/home/user1/pro/lib/yrd_wplan/wplan/task/unverified/1539_adopt_data_fmt_quantity_across_remaining_crates.md` — the wplan workspace-adoption initiative whose glassbox per-crate slice consumes `bytes_si`; the concrete need justifying this work. This task must ship (and `0.12.0` be published) before that glassbox slice can compile.
- **Sibling precedent:** `module/core/data_fmt/task/016_add_quantity_formatting_module.md` — the task that created the `quantity` module and `bytes_iec`/`bytes_human`, and **explicitly deferred SI/decimal bytes for lack of a consumer** (this task revives exactly that deferral). Same feature gate, same `QuantityStyle`/color contract, same version-bump-plus-central-pin discipline (016 Deviation 2).
- `src/quantity/number.rs` — the module where `bytes_iec`/`bytes_human` live and `bytes_si` is added beside them.
- `docs/algorithm/008_quantity_formatting.md` — the doc instance extended with the SI band table and SI-vs-IEC contrast.
- `module/glassbox/src/memory_tracking/report.rs:186` — the committed consumer's hand-rolled SI formatter that adopts `bytes_si` once published (adoption itself is TSK-1539's glassbox slice, not this task).

## Verification Record

**Gate Round 1** (Tier 2 — Dual-Role Self-Check, one-shot, self-administered by ai; no subagent dispatch)

- **Date:** 2026-08-18
- **Result:** PASS — 8/8 dimensions 🟢 (0 Blocking; 1 non-blocking D3 note: single committed consumer, accepted per 016 `duration_6ch` precedent)

| Gate | Name | Prev | Now | Issues | Fixes |
|------|------|------|-----|--------|-------|
| D1 | Scope Coherence | — | 🟢 | — | — |
| D2 | MOST Goal Quality | — | 🟢 | — | — |
| D3 | Value / YAGNI | — | 🟢 | single committed consumer (glassbox, deliberate SI-1000 base — cannot use 1024 siblings); accepted per 016 `duration_6ch` precedent; duration variant deferred to avoid speculative-consumer trap | — |
| D4 | Implementation Readiness | — | 🟢 | — | — |
| D5 | Execution Scope | — | 🟢 | all paths inside yrd_core `data_fmt` + workspace root; glassbox is citation-only | — |
| D6 | Crate Scope Unity | — | 🟢 | single crate (`data_fmt`); central-pin bump is release mechanics per 016 Deviation 2 | — |
| D7 | Crate Locality | — | 🟢 | `data_fmt` is the ecosystem formatting leaf/home | — |
| D8 | Crate Single Responsibility | — | 🟢 | remains "formatting primitives"; `bytes_si` is a base-1000 sibling, no new concern | — |
| **Total** | | — | 🟢 | 0 Blocking | 8/8 PASS |

**Acceptance Gate** (Tier 5 — Full MAAV; independent dispatch per PROC16, no self-verification)

- **Date:** 2026-08-18
- **Result:** PASS — 2 rounds to converge (R1 FAIL on a suspected regression, diagnosed as a false negative; R2 PASS on a confirmed-uncontended re-run)

| Round | Dispatch | Verdict | Issues | Resolution |
|-------|----------|---------|--------|------------|
| 1 | Primary + Adversarial (2 agents) | 🔴 FAIL | Both reported 2 `cargo nextest` failures tied to BUG-023 (`hard_break_str`/`push_overlong_word` visual-width tests) | Re-ran the same tests directly in isolation (clean) and the full suite fresh via `longrun` (940/940 clean) — root cause was concurrent `cargo build`/nextest/clippy contention across the 2 simultaneously-dispatched verifier agents sharing one `target/` dir, not a real defect |
| 2 | Fresh Challenger (1 agent, blind to R1 findings) | 🟢 PASS | — | Independently reconfirmed 940/940 on a verified-uncontended run (checked `ps aux` first) and confirmed task 017's file set never touches/references `ansi_str.rs`/`wrap.rs` (the BUG-023 fix site) — scope disjoint |
| — | Direct mechanical check (Tier 1, no dispatch) | 🟢 PASS | Task's own `Testable`/Work-Procedure command (`-E 'test(bytes_si)'`) matched only 4/6 tests genuinely exercising `bytes_si` (2 more assert it inside shared `prose_formatters_*` loops without "bytes_si" in the fn name) | Corrected to `-E 'binary(quantity_test)'` in both places; verified directly (41/41 pass under the corrected filter; full suite 941/941; clippy clean) |
