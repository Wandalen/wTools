# API: Verifier

The verifier validates parsed commands against the registered grammar, performing type casting and producing verified command values.

### Scope

- **Purpose**: Provides the validation layer between parsing and execution.
- **Responsibility**: Documents the Verifier interface, type casting process, and error conditions.
- **In Scope**: Program validation, command validation, subject extraction, property extraction, internal command bypass.
- **Out of Scope**: Parsing (see api/001 via Parser), execution (see api/004).

### Abstract

The Verifier exposes two primary operations: program validation and command validation. Both require a Dictionary reference to look up the expected grammar.

### Operations

The command validation operation matches a parsed command against the Dictionary by phrase name, extracts and type-casts subjects and properties according to the command definition, and produces a verified command. Internal commands (names ending in dot or question mark) bypass normal verification and produce a verified command with the internal flag set.

Subject extraction iterates the declared subjects, casting each positional argument from string to the declared type. Missing required subjects produce an error. Extra subjects beyond the declared count are ignored.

Property extraction iterates the declared properties, looking up each by name (or alias) in the parsed properties map, and casting the value. Unknown properties are silently ignored.

When the on_unknown_suggest feature is enabled, unknown command names trigger a similarity search across all registered commands, enriching the error message with the closest match.

### Error Handling

Verification produces errors for: unknown command name, missing required subject, type cast failure on subject or property value. All errors are wrapped in a single verifier error category.

### Compatibility Guarantees

Verifier is a public type with a stable validation interface. The verified command output type and its fields (phrase, internal flag, args, props) are public and stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/verifier/verifier.rs` | Verifier struct and validation logic |
| source | `src/ca/verifier/command.rs` | VerifiedCommand, Args, Props |
| test | `tests/inc/grammar/from_program.rs` | Program-level verification tests |
| test | `tests/inc/grammar/from_command.rs` | Single command verification tests |
| doc | [feature/007_fuzzy_suggest.md](../feature/007_fuzzy_suggest.md) | Fuzzy suggestion on unknown commands |
| doc | [invariant/001_dot_prefix_required.md](../invariant/001_dot_prefix_required.md) | Dot prefix validation |
