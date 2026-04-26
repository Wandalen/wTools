# Invariant: Colon Property Syntax

Properties are expressed as `key:value` pairs separated by a colon character with no spaces around the separator. This syntax is the exclusive mechanism for passing named parameters to commands.

### Scope

- **Purpose**: Defines the canonical property syntax and its interaction with subject parsing.
- **Responsibility**: Documents the colon separator rule, ambiguity with subjects, and the Vec input escape hatch.
- **In Scope**: Parser colon detection, subject-versus-property ambiguity, Vec passthrough for colon-containing subjects.
- **Out of Scope**: Property type casting (see api/003), type system (see feature/003).

### Invariant Statement

Any token containing a colon character is interpreted as a property assignment where the substring before the first colon is the property name and the substring after it is the property value. This interpretation is unconditional at the parser level; there is no quoting or escaping mechanism to include a literal colon in a subject value when input is provided as a single string.

### Enforcement Mechanism

The parser iterates tokens after the command phrase. For each token, it checks whether the token contains a colon. If a colon is present, the token is split into a property key-value pair and added to the parsed properties map. If no colon is present, the token is treated as a positional subject. When a subject value must contain a colon (for example, a URL), callers must provide input as a pre-split vector of strings via the Vec conversion path in IntoInput, which preserves token boundaries and avoids the colon-splitting heuristic.

### Violation Consequences

There is no runtime error for colon presence in a token — the parser always succeeds by treating it as a property. The violation manifests as semantic incorrectness: a subject value containing a colon is silently misinterpreted as a property, causing the command to receive fewer subjects than intended and an unexpected property entry. This silent misparse is the primary motivation for the Vec input path.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/parser/parser.rs` | Colon-based property extraction |
| source | `src/ca/input.rs` | Vec passthrough avoids colon splitting |
| test | `tests/inc/commands_aggregator/basic.rs` | Colon-in-subject test via Vec input |
| doc | [api/005_input.md](../api/005_input.md) | Vec input conversion path |
| doc | [feature/001_command_pipeline.md](../feature/001_command_pipeline.md) | Parser stage in pipeline |
