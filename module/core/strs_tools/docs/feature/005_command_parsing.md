# Feature: Command Parsing

### Scope

- **Purpose**: Parse shell-like command strings into structured request objects, decomposing a raw string into a command name and an ordered list of argument tokens.
- **Responsibility**: Documents the command parsing capability and links to its source, tests, and API contract.
- **In Scope**: Command name extraction, positional argument tokenization, whitespace-aware token boundary detection.
- **Out of Scope**: String splitting (`feature/001`); ANSI processing (`feature/006`); API operation signatures (`api/002`).

### Design

Command parsing operates on a single input string representing a shell-like command invocation. The parser identifies the first whitespace-delimited token as the command name and treats subsequent tokens as positional arguments.

Token boundaries follow whitespace conventions: sequences of whitespace characters between tokens are consumed as separators and do not appear in the output. Quoted tokens are not supported by this feature; for quote-aware tokenization see the parser integration feature (`feature/008`).

The result is a structured request value containing the command name and the argument list. Both the name and each argument are owned strings.

### Sources

- [src/string/parse_request.rs](../../src/string/parse_request.rs) — Command parsing implementation and ParseRequest type

### Tests

- [tests/inc/parse_test.rs](../../tests/inc/parse_test.rs) — Command parsing correctness and token boundary tests

### APIs

- [002_string_utilities_api.md](../api/002_string_utilities_api.md) — Command parsing operation contract
