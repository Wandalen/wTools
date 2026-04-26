# API Doc Entity

### Scope

- **Purpose**: Documents the public programmatic interfaces exposed by the wca crate.
- **Responsibility**: Describes operations, error handling, and compatibility for each API surface.
- **In Scope**: CommandsAggregator facade, grammar types, verifier, executor, input trait.
- **Out of Scope**: User-facing feature navigation (see feature/), correctness contracts (see invariant/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Commands Aggregator](001_commands_aggregator.md) | Top-level facade orchestrating the full pipeline | ✅ |
| 002 | [Grammar](002_grammar.md) | Command, Dictionary, Type, and Value definitions | ✅ |
| 003 | [Verifier](003_verifier.md) | Parsed-to-verified command validation interface | ✅ |
| 004 | [Executor](004_executor.md) | Command execution with context and routine dispatch | ✅ |
| 005 | [Input](005_input.md) | Input polymorphism via IntoInput trait | ✅ |
