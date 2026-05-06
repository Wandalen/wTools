# Feature: Parser Integration

### Scope

- **Purpose**: Provide a single-pass tokenizing parser that simultaneously splits, classifies, and transforms string input, enabling complex parsing pipelines without multiple traversal passes.
- **Responsibility**: Documents the parser integration capability and links to its source, tests, and API contract.
- **In Scope**: Single-pass tokenization, token classification, transformation pipeline, iterator-based output.
- **Out of Scope**: Basic string splitting (`feature/001`); command parsing (`feature/005`); API operation signatures (`api/003`).

### Design

The parser integration layer composes the split iterator with a transformation stage in a single traversal. Rather than splitting into segments and then post-processing the resulting collection, the parser yields fully classified and transformed tokens as it walks the source string.

Each token carries a classification indicating whether it represents a delimited segment, a delimiter, or a structural boundary. Transformation callbacks registered on the parser may modify token content before it is yielded, enabling operations such as trimming, substitution, or tagging without an additional collection pass.

The parser exposes an iterator interface, making it composable with standard Rust iterator adapters. Internal state is stack-allocated where possible to minimize allocation pressure in tight loops.

This feature is the result of the Task 008 parser integration work, which unified the separate tokenization and transformation sub-systems.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/string/parser.rs` | Parser, token types, and transformation pipeline |
| test | `tests/parser_integration_comprehensive_test.rs` | Single-pass parsing correctness and pipeline tests |
| doc | `docs/api/003_parser_integration_api.md` | Parser integration public API contract |
| task | `task/completed/008_parser_integration.md` | Parser integration implementation task |
