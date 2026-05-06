# Invariant Instance Procedure

### Scope

- **Trigger**: A correctness property of `file_tools` is identified that must be enforced and tested.
- **Audience**: Contributors adding, updating, or deprecating invariant doc instances.

### Procedure: Create Instance

1. Confirm the invariant belongs here — a measurable correctness property, not a feature or design decision.
2. Assign the next unused NNN from the **Overview Table** in `readme.md`.
3. Create `docs/invariant/NNN_name.md` with `### Scope`, `### Invariant Statement`, `### Enforcement Mechanism`, `### Violation Consequences`, and `### Cross-References`.
4. Add a row to the **Overview Table** in `readme.md`:
   `| NNN | [Name](NNN_name.md) | One-line purpose | ✅ |`

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Add a note at the top of the doc instance explaining the deprecation reason.
