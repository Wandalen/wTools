# docs/

This directory contains supplementary documentation for the `claude_runner_core` crate beyond the primary specification (spec.md) and test documentation (tests/readme.md).

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `claude_code_environment_reference.md` | Claude Code environment variable reference |

## Organization (1 file)

Reference documentation for Claude Code environment variables that the builder pattern API sets.

### Scope

**In Scope:**
- Environment variable reference documentation
- Variable naming conventions used by Claude Code CLI
- Tier classification (which variables have defaults vs inherit)
- Mapping between builder methods and environment variables

**Out of Scope:**
- API usage examples and tutorials (→ root readme.md, spec.md)
- Builder pattern implementation (→ src/command.rs)
- Test documentation (→ tests/readme.md)
- Specification and requirements (→ spec.md)
- Migration documentation (→ spec.md Appendix: Migration from Duplicate Execution Points)

### Knowledge Priority

Per CLAUDE.md Part 2, knowledge priority is:
1. **Test file doc comments** (highest priority) - `tests/**/*.rs`
2. **Source code doc comments** - `src/**/*.rs`
3. **Documentation files** - `docs/` (this directory)
4. **Specification** - `spec.md`

This directory contains tier 3 knowledge (reference documentation). For comprehensive knowledge about the verification framework, builder pattern API, and migration, see test file doc comments in `tests/` directory.

### Navigation

- **API Documentation**: See `readme.md` and `spec.md` in project root
- **Test Documentation**: See `tests/readme.md` for comprehensive test suite documentation
- **Implementation**: See `src/readme.md` for source code organization
- **Environment Variables**: See `claude_code_environment_reference.md` in this directory
