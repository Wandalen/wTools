# Formatter Entity Procedure

## Trigger

Any of: new Rust formatter struct added to `src/formatters/`, existing formatter's trait or input type changes, formatter deprecated or removed.

## Operations

### Create

**Trigger:** New formatter struct added to `src/formatters/`.

1. Assign the next sequential ID (next after current highest in Overview Table).
2. Create `docs/formatter/NNN_formatter_name.md` with all required sections: Scope, Cross-References, Trait, Input, Variants.
3. Add a row to `formatter/readme.md` Overview Table.
4. Add the formatter to `docs/entities.md` Master Doc Instances Table.
5. Create corresponding variant doc instances in `docs/variant/` for each variant the formatter produces.

### Update

**Trigger:** Formatter's trait implementation, input type, variants, or feature flags change.

1. Open the corresponding `docs/formatter/NNN_formatter_name.md`.
2. Update the Trait table (add/remove/change status).
3. Update the Input table if accepted types change.
4. Update the Variants table if variants are added, removed, or selection mechanism changes.
5. Update Cross-References if new source files or doc instances become relevant.

### Deprecate

**Trigger:** Formatter struct deprecated or marked for removal.

1. Mark the formatter's trait entries with `⚠️ Deprecated since X.Y.Z` in the Trait table.
2. Update the Overview Table status to `❌` in `formatter/readme.md`.
3. Preserve the doc instance file — ID is permanently retired, file remains as historical record.
