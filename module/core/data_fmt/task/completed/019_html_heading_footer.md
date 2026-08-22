# 019 — HtmlFormatter heading and footer support

## MOST Goal

Close the one formatter task 018 deliberately deferred — `HtmlFormatter` heading/footer support — by extending `render_commented_rule_if_present` with a `comment_suffix` parameter so `<!-- -->`'s closing delimiter can be emitted safely, and confirm the resulting cfg-gate on that shared primitive compiles clean across every feature combination that touches it.

- **Motivated:** Task 018 generalized `Heading`'s titled-rule pattern to Table/Tree/Expanded/Text/Yaml/Toml/Sql, excluding `Html` because — unlike `#`/`--` line comments — `<!-- -->` is a delimited comment: an unclosed `<!--` would silently swallow all subsequent markup, including `<table>`, up to the next `-->` in the document. This needed its own design decision (how to safely close the comment) before it could be mechanically ported, which task 018 explicitly deferred rather than rushed. Closing this gap completes the formatter-coverage axis task 018 opened, using the same proven rendering primitive.
- **Observable:** `HtmlFormatter::with_heading(Heading)` / `.with_footer(Heading)` render a `<!-- ─── Title ─── -->`-shaped line before/after HTML output, always closed on the same line. Heading wraps the ENTIRE rendered output, including the optional `<!DOCTYPE>`/`<html>`/`<body>` prelude when `include_wrapper` is set — never inserted between the wrapper and the `<table>` tag. No heading/no footer configured leaves output byte-identical to the pre-feature baseline (Invariant 1). `render_commented_rule_if_present`'s cfg-gate correctly covers all 4 `html_*` sub-features alongside its pre-existing sql/toml/yaml gates, with no dead-code warning in any minimal build and no missing-symbol error in any build that needs it.
- **Scoped:** one parameter addition to the existing `render_commented_rule_if_present(output, rule, target_width, comment_prefix, comment_suffix)` (previously prefix-only) — Yaml/Toml/Sql's 3 existing call sites updated to pass `comment_suffix: ""` (byte-for-byte unchanged behavior); `HtmlFormatter` gains `heading`/`footer` fields, `with_heading()`/`with_footer()` builders, and a `wrap_with_heading_footer()` method wired into its single `Format::format()` return point, passing `"<!-- "` / `" -->"`. `render_commented_rule_if_present`'s and its `config/mod.rs` re-export's cfg-gates both extended with the 4 `html_*` features. No change to `Heading`'s own data shape, to `Json`/`Logfmt` (still excluded, no comment syntax), or to any other formatter's default output.
- **Testable:** `cargo nextest run -p data_fmt --all-features --test html_heading_test` (6/6, FT-53..FT-58); full-suite regression (`cargo nextest run -p data_fmt --all-features`) shows zero change to any pre-existing test; `cargo clippy -p data_fmt --all-targets --all-features -- -D warnings` clean; `cargo check` clean across 6 feature combinations (default, full, `format_html_basic`-only, `format_sql`-only, `format_yaml`-only, `format_toml`-only) confirming the extended cfg-gate is neither over- nor under-inclusive.

## In Scope

- `src/config/table_heading.rs` — `render_commented_rule_if_present` gains a `comment_suffix: &str` parameter; strips the rendered line's trailing newline, pushes prefix + line + suffix + a fresh newline; `inner_width` now subtracts both the prefix's and suffix's display width before sizing the rule. Cfg-gate extended with `html_minimal`/`html_bootstrap`/`html_tailwind`/`html_custom` alongside the pre-existing sql/toml/yaml features.
- `src/config/mod.rs` — matching re-export cfg-gate extension.
- `src/formatters/yaml.rs`, `src/formatters/toml_fmt.rs`, `src/formatters/sql.rs` — both call sites in each file updated to pass a trailing `comment_suffix: ""` argument (no behavioral change).
- `src/formatters/html.rs` — `HtmlFormatter` gains `heading: Option<Heading>` / `footer: Option<Heading>` `pub` fields (matching Yaml/Toml/Sql/Text/Expanded's all-`pub`-fields convention, not Table/Tree's private-field-plus-accessor convention); all 3 constructors updated; `with_heading()`/`with_footer()` builders added; new `wrap_with_heading_footer()` method computes `target_width` as the max display width across the already-rendered body's lines (same technique as Tree/Expanded/Text/Yaml/Toml/Sql) and calls `render_commented_rule_if_present` with `"<!-- "` / `" -->"`; `Format::format()`'s single return point changed from `Ok(output)` to `Ok(self.wrap_with_heading_footer(output))`.
- `tests/html_heading_test.rs` (new, 6 tests, FT-53..FT-58) mirroring `tests/sql_heading_test.rs`'s structure, plus one Html-specific structural case (FT-58, `include_wrapper` interaction) with no equivalent among the other 7 formatters.
- Docs extended (not replaced) across all 7 files task 018 itself touched: `docs/feature/007_table_heading.md`, `docs/algorithm/007_heading_rendering.md`, `docs/invariant/005_heading.md`, `docs/api/003_config_types.md` (new `#### HtmlFormatter` subsection), and their 3 `tests/docs/` counterparts (6 new `### FT-N` case sections in the feature test-surface file); `tests/readme.md` and `docs/feature/readme.md` responsibility/overview rows updated.

## Out of Scope

- A visible `<caption>`-element (or similar rendered-markup) banner for HTML — rejected in favor of matching every other adopting formatter's "invisible to the rendered document, visible in source" comment-line pattern; a `<caption>` element would be visible in a browser, which none of Table/Tree/Expanded/Text/Yaml/Toml/Sql's headings are (they're all either terminal text or source-level comments).
- `Json` (no comment syntax to host a non-data line), `Logfmt` (no comment convention) — unchanged from task 018's own exclusion.
- Any change to `Heading`'s own fields, `content_str()` format, or the `·`/`─` constants.
- Any change to existing no-heading/no-footer output on any of the other 7 formatters (Invariant 1 — passthrough — holds throughout; Yaml/Toml/Sql's `comment_suffix: ""` calls are byte-for-byte unchanged).
- A crate version bump — consistent with task 018's own non-decision (no external consumer has requested a pinned version for this specific feature).

## Work Procedure

1. ~~Design: resolve HTML's delimited-comment problem (`<!-- -->` needs an explicit close, unlike `#`/`--` line comments) by adding a `comment_suffix` parameter to `render_commented_rule_if_present`, defaulting to empty-string semantics at the 3 existing call sites (Rust has no default parameters, so each Yaml/Toml/Sql call site passes an explicit `""`).~~ Done.
2. ~~Implement `HtmlFormatter` heading/footer: fields, builders, `wrap_with_heading_footer()`, wired into the single `Format::format()` return point; confirmed `HtmlFormatter::format()` has exactly one return point (simplest case among all 8 adopting formatters), so no per-branch wiring was needed the way Sql's BUG-020 branch or Tree's leaf-only-root branch required.~~ Done.
3. ~~Extend both cfg-gates (`render_commented_rule_if_present` itself and its `config/mod.rs` re-export) with the 4 `html_*` sub-features, mirroring the exact set that gates `HtmlFormatter`'s own module (`formatters/mod.rs`).~~ Done.
4. ~~Write `tests/html_heading_test.rs` (6 tests, FT-53..FT-58) matching the existing Html test files' cfg-gate convention (`#[cfg(feature = "format_html")]`, the aggregate meta-feature, not the individual `html_minimal` etc.).~~ Done.
5. ~~Verify the cfg-gate fix compiles clean across 6 feature combinations (default, full, `format_html_basic`-only, `format_sql`-only, `format_yaml`-only, `format_toml`-only) — confirms neither over-inclusion (dead-code warning in a build without any gated feature) nor under-inclusion (missing-symbol error in a build that needs it).~~ Done.
6. ~~Doc sweep across all 7 files task 018 touched, plus `docs/feature/readme.md` (found and fixed one stale formatter-list enumeration missing Html — "across Table/Tree/Expanded/Text/Yaml/Toml/Sql" — while checking every collection-level readme for the same drift task 018's own `POST_CLOSURE_CONSISTENCY_FIX` entry warned about).~~ Done.
7. ~~Final live verification: full suite + clippy, fresh, confirming zero regression from the doc-only tail of the sweep.~~ Done — 1004/1004 pass, clippy clean.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|--------------------|
| `HtmlFormatter::new().with_heading(Heading::new("Users"))` | title-only heading | first line starts `<!-- ─── Users` AND ends `-->`; `<table>` and body content survive uncorrupted |
| `HtmlFormatter::new().with_footer(Heading::new("2 records"))` | title-only footer | last line starts `<!-- ─── 2 records` and ends `-->`, on its own line — not glued onto `</table>`'s line |
| heading on Html output with ragged line lengths | target-width derivation | commented heading's total width (`<!-- ` prefix + rule + ` -->` suffix) fills to the widest rendered body line's display width — the first case where both a prefix AND a suffix width are subtracted before sizing the rule |
| no heading/footer set on `HtmlFormatter` | passthrough | output does not start with `<!--`; starts with `<table` and ends with `</table>` (Invariant 1) |
| heading + footer both set on Html output | coexistence | first line is heading, last line is footer, both fill to the same width, `<table>` body between them unaffected |
| `HtmlFormatter::new().with_include_wrapper(true).with_heading(...)` | wrapper interaction | heading is the very first line, before `<!DOCTYPE html>`, which itself precedes `<table>` — heading wraps the ENTIRE output, not just the table fragment |
| Yaml/Toml/Sql's 3 existing call sites after the `comment_suffix` parameter addition | regression | byte-for-byte unchanged output — `comment_suffix: ""` makes the new suffix-push and suffix-width-subtraction steps no-ops |
| `cargo check` under `format_html_basic`-only / `format_sql`-only / `format_yaml`-only / `format_toml`-only / default / full | cfg-gate correctness | clean in every combination — no dead-code warning, no missing-symbol error |

## Execution State

- **State:** ✅ (Done)
- **ID:** 019
- **Slug:** html_heading_footer
- **Executor:** ai
- **Priority:** 3
- **Value:** 6
- **Easiness:** 8
- **Safety:** 9
- **Dir:** `module/core/data_fmt`
- **Closes:** null
- **Reopen Count:** 0

## History

- **[2026-08-22]** `CREATED` — Filed retroactively to document work completed earlier in the same session, approved via "good. do all that" (referring to two previously-proposed next-step items: Html formatter heading/footer support, and verifying the in-flight cfg-gating fix). Closes the one formatter task 018 explicitly deferred, reusing `render_commented_rule_if_present` — extended with a `comment_suffix` parameter — rather than reimplementing the rendering primitive.
- **[2026-08-22]** `VERIFIED` — Readiness Verification Gate PASS (Tier 2 Dual-Role Self-Check, self-administered by ai; see Verification Record). State ❓ → 🎯; registered in the data_fmt Tasks Index.
- **[2026-08-22]** `IMPLEMENTATION_COMPLETE` — `HtmlFormatter` heading/footer shipped: `comment_suffix` parameter added to `render_commented_rule_if_present` (Yaml/Toml/Sql's 3 call sites updated to pass `""`, byte-for-byte unchanged); `HtmlFormatter` gained `heading`/`footer` `pub` fields, `with_heading()`/`with_footer()` builders, and `wrap_with_heading_footer()` wired into its single `Format::format()` return point — the simplest wiring case among all 8 adopting formatters (no per-branch convergence needed, unlike Tree's leaf-only-root, Expanded's empty-headers, Text's empty-cli-help-rows, or Sql's BUG-020 empty-rows branches). Confirmed by reading the source directly (not assumed) that Html's body, like Sql's, ends with no trailing newline (`</table>` with nothing after it) — reused the identical `footer.is_some() && !body.is_empty() && !body.ends_with('\n')` guard Phase 5 of task 018 introduced for Sql. Both `render_commented_rule_if_present`'s own cfg-gate and its `config/mod.rs` re-export's matching gate extended with `html_minimal`/`html_bootstrap`/`html_tailwind`/`html_custom`, mirroring the exact feature set that gates `HtmlFormatter`'s own module in `formatters/mod.rs`. 6 new tests (`tests/html_heading_test.rs`, FT-53..FT-58, cfg-gated `#[cfg(feature = "format_html")]` matching the convention already established by the pre-existing `formatter_008_html_test.rs`/`tests/html.rs`), including FT-58 — a genuinely distinct structural case with no equivalent among the other 7 formatters, proving the heading wraps the entire output (including the optional `<!DOCTYPE>`/`<html>`/`<body>` `include_wrapper` prelude) rather than being inserted between the wrapper and the `<table>` tag. `cargo check -p data_fmt` confirmed clean across 6 feature combinations (default, full, `format_html_basic`-only, `format_sql`-only, `format_yaml`-only, `format_toml`-only), verifying the extended cfg-gate is neither over- nor under-inclusive. Doc sweep completed across `docs/feature/007_table_heading.md` (new "Html usage" construction example; Out-of-Scope corrected to drop the "Html deferred" clause and cite this task for the rejected `<caption>` alternative instead), `docs/algorithm/007_heading_rendering.md` (§ Comment-Wrapped Rendering retitled to include Html, steps rewritten for the strip-then-append-suffix mechanic), `docs/invariant/005_heading.md` (Invariant 3's enforcement description corrected — the old text assumed `render_line()`'s own trailing newline survived unmodified, which stopped being fully true once suffix-stripping was added), `docs/api/003_config_types.md` (new `#### HtmlFormatter` subsection), all 3 `tests/docs/` counterparts (6 new `### FT-53`..`### FT-58` case sections in the feature test-surface file, `Yaml/Toml/Sql Coverage` sections retitled to include Html across the algorithm and invariant test-surface files), `tests/readme.md`, and `docs/feature/readme.md` (found and fixed a stale formatter-list enumeration missing Html — "across Table/Tree/Expanded/Text/Yaml/Toml/Sql" — discovered while checking every collection-level readme for the same class of drift task 018's own `POST_CLOSURE_CONSISTENCY_FIX` entry had warned about; `docs/api/readme.md`, `tests/docs/feature/readme.md`, `docs/readme.md`'s heading/footer glossary, and `docs/entity.md`/`docs/doc_graph.yml`'s node tables were all checked and confirmed already formatter-agnostic — no edit needed). Live verification: `cargo nextest run -p data_fmt --all-features --test html_heading_test` → 6/6 pass; `cargo nextest run -p data_fmt --all-features` (full suite) → 1004/1004 pass; `cargo clippy -p data_fmt --all-targets --all-features -- -D warnings` → clean.
- **[2026-08-22]** `TASK_COMPLETE` — All Work Procedure items (1–7) done, nothing remains In Scope. Final fresh verification re-run after the doc-only tail of the sweep: `cargo nextest run -p data_fmt --all-features` → 1004/1004 pass; `cargo clippy -p data_fmt --all-targets --all-features -- -D warnings` → clean. Confirmed: `Heading` itself unchanged; Yaml/Toml/Sql's own no-heading/no-footer and heading/footer output remain byte-identical to their task-018 baseline (Invariant 1, re-verified via the full regression run); `Json`/`Logfmt` remain excluded exactly as scoped; no external consumer has requested a pinned version for this feature, so no version bump requested or made. State 🎯 → ✅; filed directly to `completed/` (work was already fully done and verified at filing time — no open-state window); updating the Tasks Index.

## Related Documentation

- `src/config/table_heading.rs` — `render_commented_rule_if_present`'s `comment_suffix` parameter addition; the `Heading` type itself untouched.
- `src/config/mod.rs` — re-export cfg-gate extension, mirroring the source function's own gate.
- `src/formatters/yaml.rs`, `src/formatters/toml_fmt.rs`, `src/formatters/sql.rs` — 3 call sites each updated with a trailing `comment_suffix: ""` argument.
- `src/formatters/html.rs` — `HtmlFormatter`'s heading/footer fields, builders, and `wrap_with_heading_footer()`.
- `tests/html_heading_test.rs` — 6 new tests, FT-53..FT-58.
- `docs/feature/007_table_heading.md`, `docs/algorithm/007_heading_rendering.md`, `docs/invariant/005_heading.md`, `docs/api/003_config_types.md`, their three `tests/docs/` counterparts, `tests/readme.md`, and `docs/feature/readme.md` — extended (not replaced) to describe Html alongside the 7 formatters task 018 already covered.
- `task/completed/018_heading_footer_multi_formatter.md` — the task this one completes; task 018's own Out of Scope explicitly deferred Html pending its own header/footer design, which this task resolves (`comment_suffix` parameter, closed-delimiter comment).
- `task/readme.md` — Tasks Index row for 019, added at filing (already ✅ Completed, no separate index update needed later).

## Verification Record

**Gate Check** · Tier: 2 · Type: Full · Verdict: PASS · Agents: 0 (self, dual-role) · 1/1

Readiness gate: confirming pass — this task closes a clearly-scoped, already-identified gap (task 018's own deferred Html item); the design question (delimited-comment closing) has a concrete, low-risk answer (`comment_suffix` parameter, empty-string-compatible at all existing call sites) rather than an open-ended one; fully additive (`Option<Heading>` fields defaulting to `None`, byte-for-byte-unchanged Yaml/Toml/Sql behavior). Adversarial pass — checked whether extending a shared primitive's signature could silently change Yaml/Toml/Sql's existing output: traced every one of their 6 call sites (2 each) and confirmed all pass the new parameter as a literal `""`, which is a no-op through both the width-subtraction (`unicode_visual_len("") == 0`) and the push (`push_str("")` is a no-op) — verified by the full-suite regression showing zero change to any pre-existing Yaml/Toml/Sql test; checked whether the `<caption>`-element alternative was dismissed too quickly: it was rejected because it is visible in a rendered browser, unlike every other adopting formatter's heading (all either terminal text or a source-level comment invisible to the "rendered" output), which is a substantive, non-arbitrary distinction, not a convenience shortcut.

| Gate | Name | Prev | Now | Issues | Fixes |
|------|------|------|-----|--------|-------|
| D1 | Scope Coherence — closes exactly task 018's deferred item, nothing more | — | 🟢 | — | — |
| D2 | Design Soundness — `comment_suffix` addition is additive and backward-compatible | — | 🟢 | — | — |
| D3 | Value / YAGNI — completes a documented, previously-scoped gap rather than speculative new surface | — | 🟢 | — | — |
| D4 | Execution Scope — all touched paths inside `module/core/data_fmt` (`$SCOPE_DIR` = package root) | — | 🟢 | — | — |
| D5 | Crate Scope Unity — single crate, no workspace-root or cross-crate file touched | — | 🟢 | — | — |
| **Total** | | — | 🟢 | 0 open | — |

**Gate Check** · Tier: 2 · Type: Full · Verdict: PASS · Agents: 0 (self, dual-role) · 1/1

Implementation completion: confirming pass — compiled, 6/6 new Html tests green (FT-53..FT-58), full suite 1004/1004 green (998 task-018 baseline + 6 new), clippy clean, cfg-gate verified clean across 6 feature combinations, docs extended across all 7 files task 018 touched plus `docs/feature/readme.md`. Adversarial pass — checked for silent scope creep (none: `Heading` itself untouched, Yaml/Toml/Sql behavior byte-for-byte unchanged, `Json`/`Logfmt` still excluded); checked the trailing-newline guard was genuinely re-derived for Html rather than blindly copy-pasted from Sql — read `html.rs`'s own `Format::format()` source directly and confirmed `output.push_str("</table>")` has no following `push('\n')`, the same shape as Sql's bare `;`, before reusing the identical guard condition; checked whether the cfg-gate extension could either under-gate (missing-symbol error in a `html_*`-only build) or over-gate (dead-code warning in a build with none of the 10 gating features) — verified both directions via 6 explicit `cargo check` invocations rather than reasoning about the `#[cfg(any(...))]` list in the abstract; checked the collection-level readme sweep was genuinely exhaustive rather than stopping at the first hit — grepped every `docs/*/readme.md` and `tests/docs/*/readme.md` for the same stale-enumeration pattern that produced the one real fix (`docs/feature/readme.md`), confirming `docs/api/readme.md`, `tests/docs/feature/readme.md`, `docs/readme.md`'s glossary, `docs/entity.md`, and `docs/doc_graph.yml` were all already formatter-agnostic (no formatter-by-formatter list to go stale) rather than assuming they were clean.

| Gate | Name | Prev | Now | Issues | Fixes |
|------|------|------|-----|--------|-------|
| G1 | Compiles clean | — | 🟢 | — | — |
| G2 | Full test pass (1004/1004 incl. 6 new Html tests) | — | 🟢 | — | — |
| G3 | Clippy clean | — | 🟢 | — | — |
| G4 | cfg-gate correct across 6 feature combinations (neither over- nor under-inclusive) | — | 🟢 | — | — |
| G5 | Docs extended, not replaced, across all 7 task-018 files | — | 🟢 | — | — |
| G6 | Collection-level readme sweep for stale formatter-list enumerations | 🔴 | 🟢 | `docs/feature/readme.md` line 20 still read "across Table/Tree/Expanded/Text/Yaml/Toml/Sql" — missing Html | Added `/Html` to the enumeration |
| G7 | No pre-existing test regressed | — | 🟢 | — | — |
| G8 | Comment-wrapped rule stays valid HTML — closing delimiter always present on the same line | — | 🟢 | — | — |
| **Total** | | 🔴 | 🟢 | 0 open | 1/1 fixed |
