# Invariant: Auto-Wrap Backward Compatibility

### Scope

- **Purpose**: Guarantee that disabling auto-wrap produces byte-identical output to pre-auto-fit behavior for all table style presets.
- **Responsibility**: Documents the backward compatibility invariant for the auto-wrap feature.
- **In Scope**: `auto_wrap(false)` behavior guarantee, all 9 table presets, byte-level identity.
- **Out of Scope**: Auto-fold backward compatibility (separate invariant), auto-wrap-enabled rendering behavior (see `feature/005_auto_fit.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/table/mod.rs` | `should_auto_wrap` guard, `format_internal` pipeline |
| test | `tests/auto_wrap_test.rs` | T06 (`auto_wrap_false_is_byte_identical`) |
| doc | `../feature/005_auto_fit.md` | Auto-fit feature: "when auto_wrap is false — no wrapping triggered" |
| task | `../../task/019_cell_auto_wrapping_with_budget_allocation.md` | Implementation task — Acceptance Criteria |

### Invariant Statement

When `TableConfig::auto_wrap(false)` is set, the output of `TableFormatter::format()` must be **byte-identical** to the output produced before the auto-wrap feature was added, for all 9 table style presets (`plain`, `minimal`, `bordered`, `markdown`, `grid`, `unicode_box`, `csv`, `tsv`, `compact`) and for any input data.

Formally: for any `headers`, `rows`, and preset `P`:

```
let old_config = P();              // pre-auto-wrap config
let new_config = P().auto_wrap(false);
assert_eq!( format(old_config, data), format(new_config, data) );
```

### Enforcement Mechanism

1. **Guard clause**: `should_auto_wrap()` returns `false` when `auto_wrap` is disabled, short-circuiting the entire budget allocation and wrapping pipeline
2. **Default preservation**: `auto_wrap` defaults to `true`, so opt-out is explicit
3. **CSV/TSV bypass**: CSV and TSV presets auto-disable wrapping regardless of `auto_wrap` setting (data format integrity)
4. **Test enforcement**: Test T06 in `auto_wrap_test.rs` compares byte-identical output with `auto_wrap(false)` against known golden output

### Violation Consequences

- Silent output breakage for all existing users who upgrade without changing their code
- Tables that previously rendered correctly would unexpectedly change layout
- Scripts parsing table output would break on altered column alignment
