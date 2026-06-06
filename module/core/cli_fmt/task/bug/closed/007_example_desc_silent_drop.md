# BUG-007: ExampleEntry.desc silently dropped in help renderer

- **Severity:** Medium
- **State:** Fixed
- **Affects:** `CliHelpTemplate::render()` — any `ExampleEntry` with `desc: Some(text)` in `CliHelpData::examples`
- **Component:** `src/help.rs::emit_examples`
- **Filed:** 2026-05-17
- **Updated:** 2026-05-17
- **Validated By:** `test_example_desc_rendered` (T09) in `tests/help.rs`
- **Validation Date:** 2026-05-17

## Symptom

```bash
# Before fix — ExampleEntry.desc Some("run one") was silently dropped:
# CliHelpTemplate::render() with ExampleEntry { invocation: "myapp cmd", desc: Some("run one") }
# Got:  "  myapp cmd\n"              (WRONG — desc dropped)
# Want: "  myapp cmd  # run one\n"  (CORRECT — desc appended)
```

## Impact

Any `CliHelpTemplate` user who supplies `desc: Some(text)` on an `ExampleEntry` receives output
that silently omits the annotation. The field is documented as "Optional annotation line appended
after the invocation" — this contract was broken. Silent wrong result — no error raised, no
warning emitted. Affects every invocation where any example has `desc: Some(_)`.
Entity Scope: None.

## How Discovered

```bash
# Discovered during test surface audit (task 003) of cli_help_template module.
# Code review of emit_examples() found the desc field was never read.
$ cargo test test_example_desc_rendered -- --nocapture
```

## Minimum Reproducible Example

```bash
mkdir -p /tmp/mre007 && cat > /tmp/mre007/test.rs << 'EOF'
// Reproducer: ExampleEntry.desc=Some must appear in rendered output
#[test]
fn mre_007() {
    use cli_fmt::help::*;
    let data = CliHelpData {
        binary: "app".into(), tagline: "t".into(), groups: vec![],
        options: vec![],
        examples: vec![ ExampleEntry { invocation: "app run".into(), desc: Some("do it".into()) } ],
    };
    let style = CliHelpStyle { tty_detect: false, ..CliHelpStyle::default() };
    let out = CliHelpTemplate::new(style, data).render();
    // Before fix: out does not contain "# do it"
    // After fix:  out contains "# do it"
    assert!(out.contains("# do it"), "desc=Some must appear in output, got:\n{out}");
}
EOF
# Run: cargo test mre_007
```

## Hypothesis Table

| ID | Hypothesis | State | Summary | Evidence |
|----|-----------|-------|---------|----------|
| H1 | `emit_examples()` never reads the `desc` field | ✅ Root Cause | Implementation emitted only `ex.invocation` unconditionally; `ex.desc` was never accessed | E1, E2 |
| H2 | The `desc` field is rendered but ANSI codes cause it to be invisible | ❌ Refuted | Bug reproduces with `tty_detect=false` — no ANSI codes emitted | E3 |
| H3 | The field is populated correctly but the test fixture omits it | ❌ Refuted | `two_group_data()` fixture includes `desc: Some("run one")` yet the string is absent from render output | E4 |

## Evidence Table

| # | Location | What it shows | Hypothesis |
|---|----------|---------------|------------|
| E1 | `src/help.rs::emit_examples` (pre-fix) | `writeln!(out, "{ei}{ex_color}{}{rst}", ex.invocation)` — `ex.desc` never referenced | H1 ✅ Root Cause |
| E2 | Terminal output | `render()` output contains invocation lines but no `# text` annotations | H1 ✅ Root Cause |
| E3 | `tests/help.rs::test_no_ansi_codes` (T02) | With `tty_detect=false` no ANSI codes emitted — bug is independent of ANSI | H2 ❌ |
| E4 | `tests/help.rs::two_group_data()` | `desc: Some("run one")` supplied; `render()` output does not contain "run one" | H3 ❌ |

## Root Cause

```
emit_examples(self, out, bold, ex_color, rst)
  → for ex in &self.data.examples:
      writeln!(out, "{ei}{ex_color}{}{rst}", ex.invocation)   ← ex.desc never read
                                                                ← Some("run one") dropped
```

`emit_examples()` was written to emit only the invocation string. The `desc: Option<String>` field
was added to the data model as documented API but the renderer was never updated to consume it.
Any `Option`-typed renderer field requires an explicit branch — compiling without error is not proof it renders.

## Why Not Caught

Tests T01–T08 used the `two_group_data()` fixture which includes `desc: Some("run one")` but no
test asserted that "run one" appeared in the rendered output. Presence of the field in the
fixture data does not imply presence in the output — this assertion gap allowed the silent drop
to go undetected across 8 tests.

## Fix Location

`src/help.rs::emit_examples`:

```rust
// Before — invocation only, desc ignored:
let _ = writeln!( out, "{ei}{ex_color}{}{rst}", ex.invocation );

// After — branch on desc:
if let Some( ref desc ) = ex.desc
{
  let _ = writeln!( out, "{ei}{ex_color}{}  # {desc}{rst}", ex.invocation );
}
else
{
  let _ = writeln!( out, "{ei}{ex_color}{}{rst}", ex.invocation );
}
```

## Prevention

Any new `Option`-typed field on a data struct used by a renderer must have at least one test
asserting that the `Some` branch value is visible in the rendered output. Compile success is
not evidence of correct rendering — add `assert!(output.contains(desc_text))`.

**Pitfall:** Option-typed renderer fields need a test asserting the Some branch appears in output — compiling without error is not proof it renders.

## Generalized Version

**Broken assumption:** Adding a field to a data struct used by a renderer automatically causes the renderer to emit that field.

Fails whenever:
1. A data struct has an `Option`-typed field, AND
2. The renderer body accesses a different set of fields (e.g., from a copy of the struct), AND
3. No test asserts the `Some` branch value appears in the rendered output

**Detection invariant:**
```
for all ExampleEntry where desc = Some(text):
  render().contains(text)  == true
```

## History

| Date | Event | Notes |
|------|-------|-------|
| 2026-05-17 | filed | Discovered during test surface audit of cli_help_template |
| 2026-05-17 | fix_applied | `emit_examples` branching on `ex.desc` added in `src/help.rs` |
| 2026-05-17 | verified | Verified by `test_example_desc_rendered` (T09) in `tests/help.rs` |
| 2026-05-17 | closed | |

## Refs: src/

- `src/help.rs` — `emit_examples`: fix applied; `Fix(BUG-007)` backreference present

## Refs: tests/

- `tests/help.rs` — `test_example_desc_rendered` (T09): full 5-section bug doc present; `Some` branch assertion covers the exact dropped field
