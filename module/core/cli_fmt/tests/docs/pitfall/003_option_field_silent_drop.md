# Pitfall Test: Option Field Silent Drop

### Scope

- **Purpose**: Verify the mitigation documented in `docs/pitfall/003_option_field_silent_drop.md` holds — the silently-unread Option field trap does not recur.
- **Responsibility**: Test spec proving `emit_examples` reads and renders `ExampleEntry.desc`'s `Some` branch, asserted against rendered output, not just input fixture presence.
- **In Scope**: PF-1..PF-2 — `Some(text)` renders the annotation (the historical failure scenario) and `Some("")` still renders the marker (proves the check is on the Option variant, not string truthiness).
- **Out of Scope**: Other renderer sections — see `tests/docs/feature/002_cli_help_template.md`.

### PF-1: ExampleEntry.desc = Some(text) renders the inline annotation — the historical failure scenario

- **Given:** an `ExampleEntry` with `desc: Some("run one")`
- **When:** the example is rendered
- **Then:** the rendered output line contains `"# run one"` — asserted against the output string, not merely the input fixture
- **Note:** Regression guard for BUG-007 — `emit_examples` previously never read the `desc` field, silently dropping the annotation.

### PF-2: ExampleEntry.desc = Some("") still renders the marker — proves the check is on the Option variant, not string truthiness

- **Given:** an `ExampleEntry` with `desc: Some("")` (present but empty)
- **When:** the example is rendered
- **Then:** the rendered output line still contains the `"# "` marker — the renderer branches on `is_some()`, not on whether the contained string is non-empty; a truthiness-based check would have silently passed BUG-007's original defect too

### Pitfalls

| File | Relationship |
|------|-------------|
| [`../../../docs/pitfall/003_option_field_silent_drop.md`](../../../docs/pitfall/003_option_field_silent_drop.md) | Authoritative trap/failure/mitigation for this spec |

### Sources

| File | Relationship |
|------|-------------|
| `../../../src/help.rs` | `emit_examples` — `Some`/`None` branch under test |

### Tests

| File | Relationship |
|------|-------------|
| `../../../tests/help.rs` | PF-1: `test_example_desc_rendered` (`bug_reproducer(BUG-007)`, T09); PF-2: `test_example_empty_desc_some_renders_marker` (T-B05) |
