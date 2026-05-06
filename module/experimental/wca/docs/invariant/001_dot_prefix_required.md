# Invariant: Dot Prefix Required

All command phrases registered in the Dictionary must begin with a dot character. The parser enforces this at the boundary between raw input and parsed representation.

### Scope

- **Purpose**: Ensures unambiguous command recognition in mixed input streams.
- **Responsibility**: Documents the dot-prefix contract, where it is enforced, and what happens on violation.
- **In Scope**: Parser dot detection, error on missing prefix, internal command dot/question-mark suffixes.
- **Out of Scope**: Property syntax (see invariant/002), command verification (see api/003).

### Invariant Statement

Every command phrase must start with the `.` character. The parser splits input on dot boundaries, treating each dot-prefixed token as a new command. Input that contains no dot-prefixed token produces a parser error rather than silently succeeding with zero commands.

### Enforcement Mechanism

The parser scans input tokens for the dot prefix. When splitting a raw string, it uses the dot character as the command delimiter. If the first non-whitespace character of a command token is not a dot, the parser rejects it. Internal commands extend this convention with suffix characters: a trailing dot (`.cmd.`) triggers prefix search, and a trailing question mark (`.cmd?`) triggers detail display.

### Violation Consequences

Submitting input without a dot-prefixed command phrase produces a validation error at the parser stage. The pipeline halts before verification or execution. No routine is invoked, no side effects occur.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/parser/parser.rs` | Dot-prefix splitting logic |
| test | `tests/inc/commands_aggregator/basic.rs` | Dot-prefixed command tests |
| doc | [feature/001_command_pipeline.md](../feature/001_command_pipeline.md) | Pipeline stage where enforcement occurs |
| doc | [feature/005_command_routing.md](../feature/005_command_routing.md) | Internal command dot/question-mark suffixes |
| doc | [api/003_verifier.md](../api/003_verifier.md) | Internal command bypass during verification |
