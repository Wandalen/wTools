# API: Verifier

The verifier validates parsed commands against the registered grammar, performing type casting and producing typed VerifiedCommand values.

### Scope

- **Purpose**: Provides the validation layer between parsing and execution.
- **Responsibility**: Documents the Verifier interface, type casting process, and error conditions.
- **In Scope**: to_program(), to_command(), subject extraction, property extraction, internal command bypass.
- **Out of Scope**: Parsing (see api/001 via Parser), execution (see api/004).

### Abstract

The Verifier is a unit struct with two primary operations: to_program validates a full program of parsed commands, and to_command validates a single parsed command. Both require a Dictionary reference to look up the expected grammar.

### Operations

The to_command operation matches a ParsedCommand against the Dictionary by phrase name, extracts and type-casts subjects and properties according to the Command definition, and produces a VerifiedCommand. Internal commands (names ending in dot or question mark) bypass normal verification and produce a VerifiedCommand with the internal_command flag set.

Subject extraction iterates the declared subjects, casting each positional argument from string to the declared Type. Missing required subjects produce an error. Extra subjects beyond the declared count are ignored.

Property extraction iterates the declared properties, looking up each by name (or alias) in the parsed properties map, and casting the value. Unknown properties are silently ignored.

When the on_unknown_suggest feature is enabled, unknown command names trigger a similarity search across all registered commands, enriching the error message with the closest match.

### Error Handling

Verification produces errors for: unknown command name, missing required subject, type cast failure on subject or property value. All errors are wrapped in ValidationError::Verifier.

### Compatibility Guarantees

Verifier is a public unit struct. The to_program and to_command methods are the stable interface. VerifiedCommand fields (phrase, internal_command, args, props) are public and stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/verifier/verifier.rs` | Verifier struct and validation logic |
| source | `src/ca/verifier/command.rs` | VerifiedCommand, Args, Props |
| test | `tests/inc/grammar/from_program.rs` | Program-level verification tests |
| test | `tests/inc/grammar/from_command.rs` | Single command verification tests |
| doc | [feature/007_fuzzy_suggest.md](../feature/007_fuzzy_suggest.md) | Fuzzy suggestion on unknown commands |
| doc | [invariant/001_dot_prefix_required.md](../invariant/001_dot_prefix_required.md) | Dot prefix validation |
