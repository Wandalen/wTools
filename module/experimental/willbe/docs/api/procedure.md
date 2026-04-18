# API Instance Procedure

### Scope

- **Trigger**: A new public API surface needs documentation (new command, new parameter, new library export).
- **Audience**: Contributors adding, updating, or deprecating API doc instances.

### Procedure: Create Instance

1. Confirm the surface belongs here — public type signatures, CLI command parameters, method contracts. Behavioral guarantees go in `../invariant/`; usage guides go in `../feature/`.
2. Assign the next unused NNN from the **Overview Table** in `readme.md`.
3. Create `docs/api/NNN_name.md` with `### Scope`, `### Cross-References`, and type-specific sections.
4. Add a row to the **Overview Table** in `readme.md`:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Add a note at the top of the doc instance explaining the deprecation reason.
