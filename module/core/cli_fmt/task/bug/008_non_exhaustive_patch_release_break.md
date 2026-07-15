# BUG-008: CliHelpData's non_exhaustive migration break shipped as a semver-patch release (0.9.1→0.9.2)

- **Severity:** Medium
- **State:** 🟢 Verified
- **Affects:** Any consumer of `cli_fmt::help::CliHelpData` with a non-exact-pinned dependency requirement (e.g. `cli_fmt = "0.9"`, `cli_fmt = "^0.9.1"`, or a bare `"0.9.1"`) who constructs `CliHelpData` via an exhaustive struct literal
- **Component:** `Cargo.toml` / `changelog.md` — the v0.9.2 release's version classification (not `src/help.rs`'s content itself; see Refs)
- **Filed:** 2026-07-15
- **Updated:** 2026-07-15
- **Validated By:** Direct external-crate reproduction (`cargo check` against an isolated scratch consumer crate) + Tier 2 Dual-Role Self-Check
- **Validation Date:** 2026-07-15

## Symptom

```
error[E0639]: cannot create non-exhaustive struct using struct expression
  --> module/playbook/src/help.rs:7:14
```

Any pre-0.9.2 caller that built `CliHelpData` via a struct literal — the only construction
pattern that existed before 0.9.2 — fails to compile against 0.9.2 or later.

## Impact

`CliHelpData` was marked `#[non_exhaustive]` in the 0.9.1 → 0.9.2 release (confirmed via
`task/completed/005_extend_cli_help_template_multi_section.md`'s own Work Procedure step 8 and
Outcomes: "Version bumped 0.9.1 → 0.9.2"). Under Cargo's documented caret-requirement rule, for a
0.x.y dependency with y > 0, `^0.9.1` (or a bare `"0.9.1"`) resolves to `>=0.9.1, <0.10.0` — a
range that includes 0.9.2. Any downstream consumer with that kind of non-exact requirement would
have had `cargo update` silently re-resolve to 0.9.2 with **no `Cargo.toml` edit and no semver
warning**, and their existing exhaustive-struct-literal code would then fail to compile. This is a
release-classification defect, not a defect in the `#[non_exhaustive]` design decision itself —
that decision is legitimate, deliberate API evolution (see Root Cause).

## How Discovered

Bumping `family_kbase/playbook`'s workspace `cli_fmt` pin from `=0.8.0` (predates `option_groups`
entirely) straight to `=0.12.1` (latest) and running `cargo check -p playbook --all-features`
surfaced the E0639 error above. Tracing the break to its origin via `changelog.md` and
`task/completed/005_extend_cli_help_template_multi_section.md` pinpointed the exact version
(0.9.2) and task that introduced it, and cross-referencing that version transition against
Cargo's own semver-compatibility rule surfaced the classification mismatch.

## Minimum Reproducible Example

```rust
// Reproducer: this is exactly the construction pattern every caller used before 0.9.2 —
// cli_fmt's own pre-0.9.2 examples/tests used this same exhaustive-literal form.
// Note: CliHelpData is #[non_exhaustive] since 0.9.2 — this now fails with E0639.
use cli_fmt::help::*;

fn build_help() -> CliHelpData
{
  CliHelpData
  {
    binary   : "myapp".into(),
    tagline  : "A useful tool".into(),
    groups   : vec![],
    options  : vec![],
    examples : vec![],
    usage_lines   : vec![],
    arguments     : vec![],
    option_groups : vec![],
  }
}
// error[E0639]: cannot create non-exhaustive struct using struct expression
```

Concrete real-world instance: `family_kbase/playbook/module/playbook/src/help.rs:7-40` (a separate
repo/workspace) — a 5-field exhaustive `CliHelpData { .. }` literal, unmodified since before
0.9.2 shipped, now fails identically once `cli_fmt` resolves to `>=0.9.2`.

## Hypothesis Table

| ID | Hypothesis | State | Summary | Evidence |
|----|-----------|-------|---------|----------|
| H1 | `#[non_exhaustive]` on `CliHelpData` is itself the defect | ❌ Refuted | Deliberate, well-scoped API evolution: task 005 explicitly added it, ships a doc-comment migration example and a `compile_fail` doctest proving the new pattern and rejecting the old one | E2, E4 |
| H2 | The 0.9.1→0.9.2 version bump misclassified a breaking change as a patch-level release | ✅ Root Cause | `#[non_exhaustive]` removes external struct-literal construction — compile-breaking for any exhaustive-literal caller — yet landed in the version slot Cargo's caret rule treats as backward-compatible | E1, E3 |
| H3 | playbook's own exact-pin (`=0.8.0`) caused the break; unrelated to cli_fmt's versioning | ❌ Refuted (partially) | The exact-pin explains why *this specific* consumer only broke on a manual multi-version jump rather than a routine `cargo update` — it does not change that any consumer using a bare/caret `"0.9.1"` requirement would break silently on a routine update with zero `Cargo.toml` edit | E1, E3 |

## Evidence Table

| # | Location | What it shows | Hypothesis |
|---|----------|---------------|------------|
| E1 | `task/completed/005_extend_cli_help_template_multi_section.md` — Work Procedure step 8, Outcomes | "Bump `version = "0.9.1"` → `"0.9.2"`" / "Version bumped 0.9.1 → 0.9.2" — confirms the exact transition the break shipped in | H2 ✅, H3 |
| E2 | `changelog.md:19-26` — v0.9.2 "Extend CliHelpData..." entry | "**Changed:** `CliHelpData` marked `#[non_exhaustive]` — external struct literals rejected (E0639)" — the change is documented in prose; the entry header ("Extend CliHelpData with grouped options, usage lines, and arguments") and version number do not signal "breaking" | H1 ❌, H2 ✅ |
| E3 | Cargo Book — "Specifying Dependencies : Caret requirements" | For 0.x.y with y > 0, `^0.9.1` (or bare `"0.9.1"`) matches `>=0.9.1, <0.10.0`, which includes `0.9.2` | H2 ✅, H3 |
| E4 | `src/help.rs:123-151` | `CliHelpData`'s own doc comment already carries a `Default` + field-assignment migration example and a `compile_fail` doctest proving the old pattern is rejected — the *design* is sound and already self-documenting | H1 ❌ |
| E5 | `family_kbase/playbook/module/playbook/src/help.rs:7` | Concrete exhaustive `CliHelpData { .. }` literal (5 fields), confirmed to fail with `error[E0639]` once `cli_fmt` resolves to `>=0.9.2` (observed at 0.12.1) | H2 ✅ |

## Root Cause

```
Cargo.toml: version "0.9.1" -> "0.9.2"        (patch-level slot, y unchanged)
  src/help.rs: CliHelpData gains #[non_exhaustive]  <- removes external struct-literal construction
                                                        (compile-breaking, not additive)

Cargo's own compatibility rule: ^0.9.1 ≡ >=0.9.1, <0.10.0
  0.9.2 ⊂ that range  =>  Cargo treats 0.9.1 -> 0.9.2 as "safe to auto-upgrade"
  but the actual change is source-breaking for exhaustive-literal callers
```

The 0.9.2 release bundled two kinds of change under one version bump: purely additive new fields
(`usage_lines`, `arguments`, `option_groups` — genuinely patch-safe) and a structural seal
(`#[non_exhaustive]`) that removes a previously-available external construction capability. Only
the first kind belongs at a patch-level (`0.9.1`→`0.9.2`) slot; the second is breaking under the
`0.x` convention where the second component (`y` in `0.y.z`) plays the role of "major" for
compatibility purposes. Bundling both together let the breaking half ride along inside a version
Cargo classifies as compatible.

## Why Not Caught

Task 005's own verification (Level 3: `cargo nextest run --all-features`, `cargo test --doc
--all-features`, `cargo clippy --all-targets --all-features -- -D warnings`) and its D1–D4
completeness scoring all ran *within* the cli_fmt crate itself. None of those checks — nor the
completeness dimensions scored (Scope Coherence, MOST Goal Quality, Value/YAGNI, Implementation
Readiness) — evaluate whether a version bump's classification matches Cargo's own
backward-compatibility contract; that question is only observable from a downstream, non-exact-pinned
consumer's perspective, which no single-crate test suite or in-crate review dimension covers.

## Suggested Fix

Two independent, non-conflicting options — presented for the maintainer to choose from, not applied here:

**Option A — versioning process (addresses the root cause):** Adopt an explicit rule for this crate
(and any other pre-1.0 crate in this workspace): any change to a public type that removes
previously-available external constructibility — adding `#[non_exhaustive]` to a formerly-exhaustive
struct, removing a public field, narrowing a public field's type, etc. — must bump the *second*
version component (`0.y.z` → `0.(y+1).0`), never only the third. 0.9.2 is already published and
cannot be renumbered retroactively; whether to yank it is a separate, higher-blast-radius decision
outside this bug report's scope to recommend unilaterally.

**Option B — ergonomic mitigation (reduces friction, does not fix the classification defect):** Add
an associated constructor, e.g. `CliHelpData::new(binary: impl Into<String>, tagline: impl
Into<String>) -> Self`, covering the two fields nearly every caller sets unconditionally, so external
callers building from scratch don't need a bare `::default()` followed by N separate `data.field =
...;` statements. This is optional and orthogonal to Option A.

## Prevention

For any future 0.x release, apply the Cargo/Rust-community pre-1.0 semver convention explicitly
before choosing a version number: treat the second component (`y` in `0.y.z`) as the effective
"major" once `x == 0`. Ask "could a consumer with a bare or caret version requirement upgrade into
this automatically, with no `Cargo.toml` edit, via a routine `cargo update`?" — if yes, and the
change removes or narrows anything public, it does not belong in the third-component slot.

**Pitfall:** A `0.x.y` → `0.x.(y+1)` bump is not automatically "safe" for a 0.x crate just because
the surrounding diff also adds new fields — `#[non_exhaustive]` added to a previously-exhaustive
public struct is compile-breaking on its own and must be evaluated independently of whatever
additive changes ride alongside it in the same release.

## Generalized Version

**Broken assumption:** "Adding `#[non_exhaustive]` to an existing public struct, alongside
purely-additive new fields, is safe to release as a patch-level (`0.x.y`→`0.x.(y+1)`) version bump,
because the new fields don't remove anything."

Fails whenever:
1. The struct was previously exhaustive (constructible via a struct literal from outside the
   defining crate), AND
2. Any external, non-exact-pinned consumer constructs it that way, AND
3. The version bump lands inside the range Cargo's own caret-matching rule (or the consumer's
   exact requirement string) treats as automatically compatible

**Detection invariant:**
```
for all public structs S in a 0.x crate that transition from exhaustive to #[non_exhaustive]
between release A and release B:
  semver_second_component(A) != semver_second_component(B)   // for 0.y.z, "major" = y
```

## History

| Date | Event | Notes |
|------|-------|-------|
| 2026-07-15 | filed | Discovered while bumping `family_kbase/playbook`'s workspace `cli_fmt` pin from `=0.8.0` to `=0.12.1`; traced to the 0.9.1→0.9.2 release via `changelog.md` and `task/completed/005_extend_cli_help_template_multi_section.md` |
| 2026-07-15 | verified | Tier 2 Dual-Role Self-Check. Confirming pass: cross-checked E1 (task 005 Work Procedure/Outcomes), E2 (`changelog.md:19-26`), E4 (`src/help.rs:152` + doc-comment migration example) directly against source — all matched as cited. Adversarial pass: independently reproduced the exact `error[E0639]` in an isolated external scratch crate (path dependency, separate workspace, not the crate's own doctest); searched for disconfirmation — no later changelog entry fixes/yanks 0.9.2 (changelog has no entries past v0.9.2 despite `Cargo.toml` at 0.12.1), no `cargo-semver-checks` tooling configured, not a duplicate of BUG-005/006/007, all 8 `CliHelpData` fields confirmed `pub` (no pre-existing construction barrier). No disconfirming evidence found. |

## Refs: src/

- `src/help.rs::CliHelpData` (line 152, `#[non_exhaustive]`) — struct under discussion; already
  carries doc-comment migration guidance and a `compile_fail` doctest (lines 123–151) proving the
  new pattern and rejecting the old one. No code change proposed against this file by this bug —
  the defect is in release/versioning classification, not in `help.rs`'s content.

## Refs: Cargo.toml / changelog.md

- `Cargo.toml:3` — current version `0.12.1`; the `0.9.1`→`0.9.2` transition under discussion is
  historical (not directly visible in the current file — confirmed via `changelog.md` and
  `task/completed/005_extend_cli_help_template_multi_section.md`)
- `changelog.md:19-26` — the v0.9.2 "Extend CliHelpData..." entry; documents the change correctly
  in prose, but neither the entry header nor the version number flags it as breaking

## Refs: external

- Cargo Book — "Specifying Dependencies : Caret requirements" (0.x.y compatibility rules)
- Rust API Guidelines — semver conventions for pre-1.0 crates
