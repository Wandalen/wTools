# Feature: Command Pipeline

The core architecture processes CLI input through three sequential stages: parsing raw strings into structured commands, verifying them against a registered grammar, and executing matched routines.

### Scope

- **Purpose**: Defines the fundamental processing model that all command input traverses.
- **Responsibility**: Documents the three-stage pipeline and its data flow boundaries.
- **In Scope**: Parser, Verifier, Executor stages and their intermediate representations.
- **Out of Scope**: Individual stage APIs (see api/003, api/004), builder registration (see feature/002).

### Design

Input enters as a raw string or token sequence. The Parser splits it into one or more ParsedCommand values containing a name, positional subjects, and key-value properties as untyped strings. The Verifier matches each ParsedCommand against a Dictionary of registered Commands, performs type casting on subjects and properties, and produces VerifiedCommand values with typed arguments. The Executor dispatches each VerifiedCommand to its registered Routine, optionally providing a shared Context.

CommandsAggregator orchestrates all three stages in a single perform call, hiding the pipeline seams from callers. Consumers needing finer control can drive Parser, Verifier, and Executor independently, as unitore does.

The pipeline supports multi-command programs: a single input string may contain multiple dot-prefixed commands separated by arguments, all parsed into a Program container and executed sequentially.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/parser/parser.rs` | Parser stage implementation |
| source | `src/ca/verifier/verifier.rs` | Verifier stage implementation |
| source | `src/ca/executor/executor.rs` | Executor stage implementation |
| source | `src/ca/aggregator.rs` | Pipeline orchestration in perform() |
| test | `tests/inc/grammar/from_program.rs` | Multi-command program verification |
| test | `tests/inc/executor/program.rs` | Program-level execution tests |
| doc | [api/001_commands_aggregator.md](../api/001_commands_aggregator.md) | Facade wrapping the pipeline |
| doc | [feature/002_fluent_builder.md](002_fluent_builder.md) | Builder for pipeline configuration |
| doc | [invariant/001_dot_prefix_required.md](../invariant/001_dot_prefix_required.md) | Dot prefix enforced at parser stage |
| doc | [invariant/002_colon_property_syntax.md](../invariant/002_colon_property_syntax.md) | Colon property syntax enforced at parser stage |
