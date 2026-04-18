# Feature Instance Procedure

### Scope

- **Trigger**: A committed task exists for a new willbe command capability (status 🔄 Planned or higher).
- **Audience**: Contributors adding, updating, or deprecating feature doc instances.

### Procedure: Create Instance

1. Confirm the feature belongs here — verify it fits **In Scope** in `readme.md`. API signatures go in `../api/`; architectural patterns go in `../pattern/`. Apply the YAGNI gate: only create when a committed task exists.
2. Assign the next unused NNN from the **Overview Table** in `readme.md`.
3. Create `docs/feature/NNN_name.md` at Progressive Documentation Level 1 minimum (H1 title + Scope section). Elaborate to Level 2 before implementation begins.
4. Add a row to the **Overview Table** in `readme.md`:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Add a note at the top of the doc instance explaining the deprecation reason.
