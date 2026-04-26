# Invariant Doc Entity

### Scope

- **Purpose**: Formal correctness properties that must always hold in the wca framework.
- **Responsibility**: Documents invariant statements, enforcement mechanisms, and violation consequences.
- **In Scope**: Syntax rules, type constraints, execution safety guarantees, regression contracts.
- **Out of Scope**: Feature navigation (see feature/), API interface details (see api/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Dot Prefix Required](001_dot_prefix_required.md) | All command names must start with dot character | ✅ |
| 002 | [Colon Property Syntax](002_colon_property_syntax.md) | Properties use colon separator exclusively | ✅ |
| 003 | [Bool Accepted Values](003_bool_accepted_values.md) | Bool type accepts only four string literals | ✅ |
| 004 | [Help No Execute](004_help_no_execute.md) | Help display must not trigger command execution | ✅ |
| 005 | [Routine Required](005_routine_required.md) | Command execution fails when routine is absent | ✅ |
