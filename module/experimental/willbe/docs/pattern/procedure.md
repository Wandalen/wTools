# Pattern Instance Procedure

### Scope

- **Trigger**: A new architectural pattern or significant design decision needs documentation.
- **Audience**: Contributors adding, updating, or deprecating pattern doc instances.

### Procedure: Create Instance

1. Confirm the decision belongs here — architectural patterns, module organization rules, design rationale. Feature behavior goes in `../feature/`; API contracts go in `../api/`.
2. Assign the next unused NNN from the **Overview Table** in `readme.md`.
3. Create `docs/pattern/NNN_name.md` with `### Scope`, `### Cross-References`, and pattern-specific sections (Problem, Solution, Applicability, Consequences).
4. Add a row to the **Overview Table** in `readme.md`:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Add a note at the top of the doc instance explaining the deprecation reason.
