# API: Parser Integration API

### Scope

- **Purpose**: Define the public operations, token types, and transformation hooks for the single-pass parser integration layer.
- **Responsibility**: Contracts the observable behaviour callers depend on for the parser integration feature.
- **In Scope**: Parser entry point, token classification variants, transformation callback registration, iterator output.
- **Out of Scope**: Basic split API (`api/001`); string utility API (`api/002`); internal tokenizer implementation (`feature/008`).

### Operations

The parser integration API exposes a builder that accepts a source string and produces a classified, transformed token iterator.

The caller registers zero or more transformation callbacks before executing the parser. Each callback receives a mutable token and may modify its content. Callbacks are applied in registration order.

The iterator yields tokens in source order. Each token is classified as a delimited segment, a delimiter, or a structural boundary. Token content reflects any transformations applied.

The parser does not collect tokens into a container; it yields one token per call to `next`, keeping memory usage proportional to the largest single token rather than the entire source.

### Sources

- [src/string/parser.rs](../../src/string/parser.rs) — Parser, token types, and transformation pipeline

### Tests

- [tests/parser_integration_comprehensive_test.rs](../../tests/parser_integration_comprehensive_test.rs) — Parser API behaviour and transformation tests

### Features

- [008_parser_integration.md](../feature/008_parser_integration.md) — Parser integration feature design
