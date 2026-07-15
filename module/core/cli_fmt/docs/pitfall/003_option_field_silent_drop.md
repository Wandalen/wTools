# Pitfall: Option Field Silent Drop

### Scope

- **Purpose**: Document the silently-unread `Option` field trap discovered in example rendering.
- **Responsibility**: Trap, failure mode, and mitigation for adding an `Option`-typed data field without a test proving the renderer consumes it.
- **In Scope**: `emit_examples`'s handling of `ExampleEntry.desc`.
- **Out of Scope**: Other renderer sections — see `feature/002_cli_help_template.md`.

### Trap

Adding an `Option<String>`-typed field to a data struct consumed by a renderer, and assuming the renderer emits it because the struct compiles and the field is documented as part of the API.

### Failure

The renderer's emit function never reads the new field — it silently drops the `Some(text)` annotation with no error, warning, or panic. Existing tests that only check the *input fixture* contains the field (not that the *rendered output string* contains it) pass without detecting the drop.

### Mitigation

Every `Option`-typed field consumed by a renderer needs at least one test asserting the `Some` branch's value is present in the rendered output string — not just present in the input fixture. Compile success is not evidence of correct rendering.

### Features

| File | Relationship |
|------|-------------|
| [`../feature/002_cli_help_template.md`](../feature/002_cli_help_template.md) | Example rendering section this pitfall applies to |

### Sources

| File | Relationship |
|------|-------------|
| `src/help.rs` | `emit_examples` — `Some`/`None` branch enforcing this mitigation |

### Tests

| File | Relationship |
|------|-------------|
| [`../../tests/docs/pitfall/003_option_field_silent_drop.md`](../../tests/docs/pitfall/003_option_field_silent_drop.md) | Test specification verifying this mitigation holds |
| `tests/help.rs` | `test_example_desc_rendered` (T09), `bug_reproducer(BUG-007)` |
