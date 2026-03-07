# src/

This directory contains the core implementation of the `claude_runner_core` crate, providing builder pattern API for executing Claude Code commands programmatically.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `lib.rs` | Crate entry point and public API surface |
| `command.rs` | ClaudeCommand builder implementation |
| `types.rs` | Enum type definitions and conversions |

## Organization (3 files)

Files organized by responsibility following Rust module conventions.

### Module Structure

```
src/
├── lib.rs          # Crate root, public API
├── command.rs      # ClaudeCommand builder
└── types.rs        # ActionMode, LogLevel enums
```

### Scope

**In Scope:**
- Builder pattern API for Claude Code command construction
- Environment variable automation (tier 1 defaults: bash_timeout=3.6M, bash_max_timeout=7.2M, auto_continue=true, telemetry=false, max_output_tokens=200K)
- Type safety via enums (ActionMode, LogLevel)
- Private field encapsulation (prevents direct construction)
- Single execution point (execute() method)
- Test-only helpers for verification without actual execution

**Out of Scope:**
- Session lifecycle management (→ claude_session crate)
- Context injection from wplan (→ dream_agent crate)
- Actual process execution (→ dream_agent crate, command.rs only builds commands)
- Interactive terminal UI (→ terminal-based tools)
- Configuration hierarchy (→ config_hierarchy crate)

### Design Principles

1. **Single Execution Point**: All commands go through execute()
2. **Builder Pattern**: Configuration via chainable with_*() methods
3. **Private Fields**: Cannot construct with struct literals
4. **No Session Logic**: Pure execution, no state management
5. **Migration Complete**: Old factory pattern impossible (from_message/create/generate removed)
6. **Type Safety**: Enums replace string literals (ActionMode, LogLevel)

### Test Coverage

Comprehensive test suite in `tests/` directory:
- Builder pattern API (4 test files): edge cases, methods, defaults, environment variables
- Type definitions (1 test file): enum conversions and defaults
- Migration validation (2 test files): factory pattern removal, single execution point
- Verification framework (5 test files): 231 validation assertions across 6 layers (spec.md NFR-6)
- Inspection methods (1 test file): describe() and describe_env()
- Execution output (1 test file): ExecutionOutput struct and Display
- Skip permissions (1 test file): --dangerously-skip-permissions flag
- Manual execution (2 test files): real Claude binary tests (skipped in CI)
- **Total**: 20 test files, all passing; see tests/readme.md and spec.md NFR-6 for counts

See `tests/readme.md` for complete test documentation.
