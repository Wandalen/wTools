# 015 — Add `Heading::render(width)` public method

## MOST Goal

Add a public standalone render method to `Heading` so external callers can emit a titled rule of a specific display width without reimplementing the rendering arithmetic.

- **Motivated:** `Heading::content_str()` is `pub(crate)` and no public `render()` method exists. External callers that need a heading at a specific width must duplicate the fill arithmetic (`HEADING_LEAD_WIDTH` + content + `─` fill), which risks diverging from the internal logic and breaks when display-width arithmetic (BUG-015) is updated internally.
- **Observable:** `data_fmt::Heading::new("Title").with_field("5").render(40)` compiles in an external crate and returns a string whose display width equals exactly 40 terminal columns (measured via `unicode_visual_len`), filling with `─` characters. The existing test suite passes without regression.
- **Scoped:** One new `pub` method on `Heading` in `src/config/table_heading.rs`. No new modules, no doc entity directories, no version bump beyond a patch. `content_str()` remains `pub(crate)`.
- **Testable:** Unit test `heading_render_fills_to_display_width` in `tests/` asserts `unicode_visual_len(result) == width` for a representative title+field combination; `heading_render_width_zero` asserts width-0 produces an empty string or a string with no fill characters; all existing tests still pass.

## In Scope

- `src/config/table_heading.rs` — add `pub fn render(&self, width: usize) -> String`
  - Reuse internal arithmetic: emit `─── {content} ──...` filling to `width` display columns
  - Use `unicode_visual_len` for width arithmetic (matches BUG-015 fix; must NOT use `chars().count()`)
  - If `width` is less than or equal to the lead prefix length, emit only the prefix with no fill
- `tests/heading_render_test.rs` (new) — two unit tests: `heading_render_fills_to_display_width`, `heading_render_width_zero`
- `Cargo.toml` — patch version bump (e.g. `0.6.1 → 0.6.2`)

## Out of Scope

- Creating `pub mod decorator` or any new module namespace — no concrete consumer in scope
- Changing `content_str()` visibility
- Modifying internal rendering paths (`row_rendering.rs`)
- Any other public API additions
- Major or minor version bump (new method on existing pub struct = patch)

## Work Procedure

1. Add `pub fn render(&self, width: usize) -> String` to `impl Heading` in `src/config/table_heading.rs`
   - Call `self.content_str()` to build the content string
   - Compute `content_display_len = unicode_visual_len(&content)`
   - Compute `used = HEADING_LEAD_WIDTH + 1 + content_display_len + 1` (leading `─── ` + content + ` `)
   - Compute `fill = width.saturating_sub(used)`
   - Emit: `"─".repeat(HEADING_LEAD_WIDTH) + " " + content + " " + "─".repeat(fill)`
   - If `width <= HEADING_LEAD_WIDTH`: return `"─".repeat(HEADING_LEAD_WIDTH)` with no fill
2. Create `tests/heading_render_test.rs` with two test functions
3. Bump `Cargo.toml` version `0.6.1 → 0.6.2`
4. Run `w3 .test level::3` — all tests pass, clippy clean

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| `Heading::new("Title").render(40)` | Standard width | `unicode_visual_len(result) == 40` |
| `Heading::new("Title").with_field("5").render(40)` | With field | `unicode_visual_len(result) == 40`, field visible |
| `Heading::new("Title").render(0)` | Edge: width 0 | No fill; no panic |
| `Heading::new("Title").render(3)` | Edge: equals HEADING_LEAD_WIDTH | Only lead prefix; no fill |

## MAAV Gate Result

**Date:** 2026-06-27
**Verdict: FAIL — 4 blocking issues must be resolved before moving to Verified**

| Dimension | Result | Key Finding |
|-----------|--------|-------------|
| Scope Coherence | FAIL | Observable clause names `unicode_visual_len` (a `pub(crate)` function) as the measurement tool for external callers — it is not accessible externally |
| MOST Goal Quality | FAIL | (a) Observable outcome is not externally verifiable; (b) `render(0)` Work Procedure mandates `"───"` (3 chars) but Testable permits "empty string" — direct contradiction |
| Value / YAGNI | FAIL | No concrete external consumer of `Heading::render()` exists or is blocked; `Heading` is used only inside `data_fmt` and its own examples; this is speculative convenience API |
| Implementation Feasibility | PARTIAL PASS | Internal access path works; arithmetic is sound. One defect: `render(0)` per Work Procedure returns `"───"` but Testable says "empty string or no fill" — contradictory |

**Required fixes before re-verification:**

1. **YAGNI (BLOCKING):** Identify a concrete external consumer currently blocked by the missing method, or remove this task. A hypothetical future caller is not sufficient. Consider: is there a crate in the workspace that needs standalone heading rendering today?
2. **Observable clause:** Replace `measured via unicode_visual_len` with an externally-usable measurement (e.g., `unicode_width::UnicodeWidthStr::width()` — which IS a public dep). Alternatively, for ASCII-only test inputs, `result.chars().count()` suffices and requires no note.
3. **render(0) contradiction:** Choose one behavior and make Work Procedure and Testable clause agree:
   - Option A: `render(0)` returns `"───"` (Work Procedure literal) — remove "empty string" from Testable
   - Option B: `render(0)` returns `""` — add guard `if width == 0 { return String::new(); }` before the HEADING_LEAD_WIDTH guard
4. **Content-drop documentation:** Document that `render(width <= HEADING_LEAD_WIDTH)` silently drops title and fields, and note the divergence from `render_heading_if_present` (which always emits content).
