# API: Input

The input abstraction provides polymorphic entry points for feeding command strings or token sequences into the pipeline.

### Scope

- **Purpose**: Decouples callers from a specific input format, accepting strings, string vectors, or interactive prompts.
- **Responsibility**: Documents the IntoInput trait, Input newtype, and supported conversion paths.
- **In Scope**: IntoInput trait, Input(Vec<String>) newtype, &str splitting, Vec<String> passthrough, ask() prompt.
- **Out of Scope**: How input is parsed into commands (see api/003 via Parser).

### Abstract

Input is a newtype wrapping a vector of strings, representing the raw tokens to be parsed. The IntoInput trait provides conversion from multiple source types, enabling callers to pass input in the most convenient form.

### Operations

String reference conversion splits the input on whitespace boundaries, producing one token per word. This is the simplest path used when passing a single command string.

String vector conversion passes tokens through unchanged. This is the safe path for subjects containing spaces or colons, as it preserves token boundaries that whitespace splitting would destroy.

The ask function provides an interactive stdin prompt, reading a line of input from the user and returning it as an Input value.

### Error Handling

Input conversion is infallible. Parsing errors are deferred to the Parser stage.

### Compatibility Guarantees

IntoInput and Input are public types. The conversion semantics (split-on-space for strings, passthrough for vectors) are stable behavior.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/input.rs` | Input, IntoInput, ask() |
| doc | [api/001_commands_aggregator.md](001_commands_aggregator.md) | perform() accepts IntoInput |
| doc | [invariant/002_colon_property_syntax.md](../invariant/002_colon_property_syntax.md) | Why Vec input is needed for colon subjects |
