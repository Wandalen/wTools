# Feature Doc Entity

### Scope

- **Purpose**: Navigate all artifacts for each user-facing capability of the crate.
- **Responsibility**: Index of feature doc instances; each instance links to the source, tests, and design docs for one capability.
- **In Scope**: All user-facing features implemented and shipped in this crate.
- **Out of Scope**: Internal algorithms (`algorithm/`); API contracts (`api/`); correctness guarantees (`invariant/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [String Splitting](001_string_splitting.md) | Core split with quoting, escaping, delimiter preservation | ✅ |
| 002 | [Text Indentation](002_text_indentation.md) | Multi-line text indentation with configurable prefix | ✅ |
| 003 | [String Isolation](003_string_isolation.md) | Left/right/between substring extraction by delimiter | ✅ |
| 004 | [Number Parsing](004_number_parsing.md) | Robust numeric string conversion with multi-format support | ✅ |
| 005 | [Command Parsing](005_command_parsing.md) | Structured parse of command-line style strings | ✅ |
| 006 | [ANSI Utilities](006_ansi_utilities.md) | ANSI escape sequence detection, strip, parse, truncate | ✅ |
| 007 | [SIMD Acceleration](007_simd_acceleration.md) | Opt-in hardware-accelerated string processing | ✅ |
| 008 | [Parser Integration](008_parser_integration.md) | Single-pass combined tokenization and transformation | ✅ |
