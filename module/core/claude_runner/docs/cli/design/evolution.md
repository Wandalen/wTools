# CLI Evolution

Historical record of CLI design decisions, changes, and evolution for `claude_runner`.

## Table of Contents

- [Version History](#version-history) — Design changes by version
- [Decision Log](#decision-log) — Major architectural decisions with rationale
- [Future Roadmap](#future-roadmap) — Planned improvements and features

---

## Version History

### Version 0.1.0 (Initial Design)

**Date:** 2024-03-03

**Characteristics:**
- Single command: `.run`
- Implicit routing: `--help` flag → `.help`, everything else → `.run`
- Parameter syntax: `--flag value` adapter → `key::value` unilang tokens
- Help system: Custom `print_help()` function

**Design Rationale:**
- Familiarity: Users know POSIX-style flags
- Simplicity: Single primary command, no complex routing
- Adapter layer: Converts familiar syntax to unilang for validation

**Known Limitations:**
- `--help` flag is semantically a command selector but parsed as flag
- Adding new commands requires special routing logic
- Implicit routing not discoverable (users can't list available commands)

**Files:** `src/main.rs` (327 lines)

---

### Version 0.2.0 (Dual Support — Phase 1)

**Date:** 2024-03-03 (Target)

**Characteristics:**
- Explicit commands: `.help` and `.run` recognized as first tokens
- Backward compatible: `--help` and `-h` flags still work
- Command discovery: Users can type `.help` or `.run` explicitly
- Migration path: Dual support → Deprecation → Removal

**Design Rationale:**
- Enable extensibility while maintaining compatibility
- Clearer semantic model: commands are first-class
- Smooth transition: Users can adapt at their pace
- Future-proof: Ready for `.status`, `.list`, etc.

**Changes:**
- `argv_to_unilang_tokens()`: Add dot-command detection (first token check)
- `print_help()`: Add COMMANDS section
- Tests: Add explicit `.help` and `.run` invocation tests
- Documentation: Update all docs to show both syntaxes

**Migration Documentation:** [Migration Guide](migration_guide.md)

---

### Version 0.3.0 (Deprecation — Phase 2)

**Date:** 2026-Q2 (Target)

**Characteristics:**
- Deprecation: `--help` and `-h` flags marked as deprecated
- Warning message printed when deprecated flags used
- Clear guidance: Use `.help` command instead
- Still functional: Deprecated flags continue to work

**Design Rationale:**
- Give users warning period to adapt
- Avoid breaking changes immediately
- Clear deprecation message in help output
- Documentation updated to recommend new syntax

**Changes:**
- `argv_to_unilang_tokens()`: Add deprecation notice output
- `print_help()`: Add DEPRECATION NOTICE section
- Documentation: All docs updated with deprecation warnings

**Migration Documentation:** [Migration Guide](migration_guide.md)

---

### Version 1.0.0 (Explicit Commands — Phase 3)

**Date:** 2026-Q3 (Target)

**Characteristics:**
- `--help` and `-h` flags removed entirely
- Only `.help` command accepted
- `.run` command optional (explicit or implicit)
- Clean separation: Commands vs parameters
- Semantic clarity: No more "help flag" confusion

**Design Rationale:**
- Cleanest semantic model
- Consistent with unilang patterns (`.namespace.action`)
- Enables clean extensibility for new commands
- Reduced maintenance burden (single help path)

**Changes:**
- `argv_to_unilang_tokens()`: Remove `--help`/`-h` parsing
- `print_help()`: Remove deprecated flags section
- Routing: Simplified to explicit-or-implicit logic
- Tests: All `--help` tests replaced with `.help` tests
- Documentation: All docs updated to remove `--help` references

**Migration Documentation:** [Migration Guide](migration_guide.md)

---

## Decision Log

### Decision 1: Use unilang Framework

**Date:** 2024-02-15

**Context:** Initial CLI development

**Decision:** Use `unilang` crate for command routing and validation rather than building custom parser.

**Options Considered:**
- Build custom parser from scratch
- Use `clap` (popular Rust CLI library)
- Use `wca` (aggregation framework in workspace)

**Rationale for unilang:**
- **Semantic validation**: unilang provides `SemanticAnalyzer` for argument validation
- **Command registry**: Clean separation between commands and handlers
- **Type safety**: `Kind` enum prevents type errors
- **Integration**: Already used by `genfile` in workspace
- **Extensibility**: Designed for external binary consumers

**Tradeoffs Accepted:**
- Higher learning curve vs homegrown solution
- Some adapter layer needed for user-familiar syntax
- More dependencies

**Outcome:** Correct decision — provides needed validation and extensibility.

---

### Decision 2: Adapter Layer for User Familiarity

**Date:** 2024-02-20

**Context:** After choosing unilang framework

**Decision:** Implement adapter layer that converts `--flag value` syntax to unilang `key::value` format.

**Options Considered:**
- Require users to learn unilang syntax directly
- Teach users both syntaxes without conversion
- Provide only unilang syntax (no adapter)

**Rationale for adapter:**
- **User experience**: POSIX flags are familiar from other tools
- **Adoption**: Users don't need to learn new syntax
- **Best of both**: Framework power + user familiarity
- **Future migration**: Can gradually expose unilang syntax

**Implementation:**
- `argv_to_unilang_tokens()` function converts flags to tokens
- Maintains all parameter semantics
- Handles routing (implicit `.run` → explicit `.run`)

**Outcome:** Correct decision — balances power with usability.

---

### Decision 3: Dot-Prefixed Commands

**Date:** 2024-02-25

**Context:** Planning CLI architecture and extensibility

**Decision:** Use dot prefix for command names (`.help`, `.run`, etc.) to distinguish from parameters.

**Options Considered:**
- Plain command names (`help`, `run`)
- Verbs only (`execute`, `list`)
- No prefix (let unilang infer)

**Rationale for dot prefix:**
- **Visual distinction**: Commands (`.name`) vs parameters (`key::value`)
- **Namespace hierarchy**: `.namespace.action` pattern enables grouping
- **Unilang convention**: Matches framework's command naming patterns
- **Parser optimization**: Tokens starting with `.` quickly identified

**Outcome:** Correct decision — enables clean extensibility and namespace organization.

---

### Decision 4: Boolean Flags Are Idempotent

**Date:** 2024-02-28

**Context:** Defining parameter behavior for boolean flags

**Decision:** Allow boolean flags to be specified multiple times with no effect (idempotent).

**Options Considered:**
- Error on duplicate boolean flags
- Warning on duplicate boolean flags
- Last-wins for boolean flags

**Rationale for idempotent:**
- **User experience**: Accidental duplication shouldn't error
- **No ambiguity**: String parameters error on duplicates, booleans shouldn't be different
- **Simplicity**: No special handling needed
- **Convention**: Matches unilang's boolean handling

**Implementation:**
- `--verbose --verbose` → `verbose::1` (same as once)
- `--continue --continue` → `continue::1`
- No error or warning printed

**Outcome:** Correct decision — provides predictable, user-friendly behavior.

---

### Decision 5: Help Before Parsing

**Date:** 2024-03-05

**Context:** Designing error handling and help system

**Decision:** Check for `.help` command BEFORE parsing rest of arguments.

**Options Considered:**
- Parse all arguments first, then check for help flag
- Check for help after parsing
- Always run full pipeline

**Rationale for early help:**
- **Efficiency**: Don't parse if user wants help
- **Clear error messages**: Help output appears even with bad flags before it
- **User intent**: "claude_runner --help badflag" should show help, not "unknown argument"

**Implementation:**
```rust
// Phase 2: Help check
if tokens.first().map(String::as_str) == Some(".help") {
    print_help();
    return;
}

// Phase 3+: Parse and execute
// ... rest of pipeline
```

**Outcome:** Correct decision — handles "help" case gracefully.

---

## Future Roadmap

### Planned Features

#### Short Term (Next 3-6 months)

| Feature | Description | Priority | Complexity |
|---------|-------------|----------|-------------|
| **Session management** | `.status`, `.list`, `.switch` commands | Medium | Medium |
| **Configuration** | `.config set/get/list/reset` commands | Low | Low |
| **Export/Import** | `.export`, `.import` for session state | Low | Medium |
| **REPL mode** | Interactive command prompt with history | Medium | High |
| **Shell completion** | Generate completion scripts for bash/zsh/fish | High | High |
| **Rich error messages** | Structured errors with suggestion and code | Medium | Medium |

#### Medium Term (6-12 months)

| Feature | Description | Priority | Complexity |
|---------|-------------|----------|-------------|
| **Session templates** | Pre-defined session configurations | Low | Low |
| **Plugin system** | Load external command extensions | High | Very High |
| **Multi-project support** | Work with multiple projects simultaneously | Medium | Medium |
| **Workflow templates** | Pre-defined command sequences | Low | Medium |
| **Telemetry** | Optional usage analytics | Low | Low |

#### Long Term (12+ months)

| Feature | Description | Priority | Complexity |
|---------|-------------|----------|-------------|
| **Web UI** | Browser-based interface | Low | Very High |
| **Command aliases** | User-defined command shortcuts | Medium | Low |
| **Profiles** | Named configuration sets | Low | Low |
| **Local cache** | Cache Claude responses for reuse | Medium | Medium |

### Proposed Breaking Changes

| Version | Change | Rationale | Migration Path |
|---------|--------|-----------|---------------|
| **1.0.0** | Initial design | Baseline for single-command tool | N/A |
| **0.2.0** | Add explicit commands | Enable extensibility | 3-phase migration |
| **0.3.0** | Remove `--help` flag | Clean semantic model | 3-phase migration |
| **1.1.0** | Add session commands | Richer workflows | Planned feature |
| **1.2.0** | Add REPL mode | Interactive development | Planned feature |

---

## Architectural Principles

### Core Principles Guiding Future Development

1. **Semantic Clarity**: Commands and parameters should clearly indicate their purpose
2. **Extensibility First**: Design for future commands, not just current needs
3. **User Familiarity**: Balance power with ease of learning
4. **Backward Compatibility**: Use phased migrations for breaking changes
5. **Type Safety**: Leverage framework validation over manual checking
6. **Clear Error Messages**: Users should understand what went wrong and how to fix
7. **Idempotency**: Operations should produce consistent results when repeated
8. **Documentation**: Every feature should be documented with examples

### Anti-Patterns to Avoid

1. Don't embed help as a flag
2. Don't require implicit commands when explicit form exists
3. Don't create hidden state between invocations
4. Don't use magic numbers without documentation
5. Don't silently ignore user input
6. Don't break changes without migration path
7. Don't make boolean flags non-idempotent
8. Don't create commands with unclear responsibility

---

## Summary

| Aspect | Current Status | Future Direction |
|---------|---------------|------------------|
| **Commands** | Single `.run` (implicit) | Multiple with namespaces (planned) |
| **Routing** | Flag-based (with explicit support planned) | Explicit commands (3-phase migration) |
| **Help** | `--help` flag (dual support) | Deprecate → Remove → Help system |
| **Parameters** | `--flag value` syntax | Keep current syntax |
| **Extensibility** | Limited (needs adapter changes) | Rich session management planned |
| **Documentation** | Comprehensive but evolving | Maintain and expand |

**Evolution Path:**
```
Current (v0.1.0) → Dual Support (v0.2.0) → Explicit (v0.3.0) → Rich Sessions (v1.1.0) → ...
```

---

## References

- [Migration Guide](migration_guide.md) — 3-phase migration path
- [Command Design](command_design.md) — Design recommendations
- [Unilang Exploration](unilang_exploration.md) — Framework details
- [Best Practices](best_practices.md) — Patterns and anti-patterns
