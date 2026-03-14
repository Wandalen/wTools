# CLI Documentation

Complete reference for `claude_runner` command-line interface design, unilang framework integration, and architectural patterns.

## Quick Navigation

**By Topic:**
- [Unilang Exploration](unilang_exploration.md) - Deep dive into unilang crate architecture and patterns
- [Command Design](command_design.md) - CLI design principles and recommendations for `claude_runner`
- [Tutorial](tutorial.md) - Hands-on lessons for CLI development and extension
- [Implementation Guide](implementation_guide.md) - Step-by-step implementation with checklists
- [Quick Reference](quick_reference.md) - Fast lookup card for common patterns
- [Architecture](architecture.md) - System diagrams, layers, and data flow
- [API Reference](api_reference.md) - Complete API documentation for unilang and core library
- [Migration Guide](migration_guide.md) - 3-phase path from implicit to explicit commands
- [Best Practices](best_practices.md) - Patterns, anti-patterns, and guidelines
- [Troubleshooting](troubleshooting.md) - Common issues, solutions, and debugging techniques
- [Examples Gallery](examples.md) - Real-world usage patterns and workflows
- [Comparison](comparison.md) - Framework and tool comparisons with tradeoff analysis
- [Evolution](evolution.md) - Historical record of design decisions and future roadmap

**By Layer:**
- **User Layer** - What users type and see
- **Adapter Layer** - How `--flag value` maps to `key::value`
- **Unilang Layer** - Command registry, parsing, execution pipeline

## Key Concepts

### Dot-Prefixed Commands
Commands in unilang use dot prefix: `.run`, `.help`, `.archive.new`
- Distinguishes commands from parameters
- Enables namespace hierarchy (`.namespace.action`)
- Parser optimization for early routing

### Parameter Syntax: `key::value`
All parameters use double-colon format after adapter conversion:
```bash
message::"value"
dir::/path/to/project
verbosity::2
dry::1
```

### Hybrid Design Pattern
`claude_runner` uses adapter layer for user familiarity:

```bash
# User types (Claude-style)
claude_runner --message "hi" --dir /path --dry-run

# Adapter converts to unilang
[".run", "message::hi", "dir::/path", "dry::1"]
```

This preserves familiar `--flag value` syntax while leveraging unilang's command routing and validation.

## Documentation Principles

1. **Evidence-Based**: All patterns backed by actual code analysis
2. **Cross-Reference**: Bidirectional links between concepts
3. **Progressive Disclosure**: Concepts build on each other
4. **Practical Focus**: Real-world examples, not theory

**By Layer:**
- **User Layer** - What users type and see
- **Adapter Layer** - How `--flag value` maps to `key::value`
- **Unilang Layer** - Command registry, parsing, execution pipeline

## Architecture Overview

```
User Input (CLI)
    ↓ argv_to_unilang_tokens()
Tokens (`.run message::hi dir::/path`)
    ↓ Parser → SemanticAnalyzer
VerifiedCommands
    ↓ Interpreter
Execution
```

## Key Concepts

### Dot-Prefixed Commands
Commands in unilang use dot prefix: `.run`, `.help`, `.archive.new`
- Distinguishes commands from parameters
- Enables namespace hierarchy (`.namespace.action`)
- Parser optimization for early routing

### Parameter Syntax: `key::value`
All parameters use double-colon format after adapter conversion:
```bash
message::"Fix the bug"
dir::/path/to/project
verbosity::2
dry::1
```

### Hybrid Design Pattern
`claude_runner` uses adapter layer for user familiarity:

```bash
# User types (Claude-style)
claude_runner --message "hi" --dir /path

# Adapter converts to unilang
[".run", "message::hi", "dir::/path"]
```

This preserves familiar `--flag value` syntax while leveraging unilang's command routing.

## Documentation Principles

1. **Evidence-Based**: All patterns backed by actual code analysis
2. **Cross-Reference**: Bidirectional links between concepts
3. **Progressive Disclosure**: Concepts build on each other
4. **Practical Focus**: Real-world examples, not theory

## Related Resources

- [genfile CLI](../../../../../genfile/docs/cli/readme.md) - Reference implementation of unilang patterns
- [claude_runner CLI User Reference](../readme.md) - User-facing CLI reference
- [unilang crate](https://docs.rs/unilang) - Published documentation on crates.io

## Migration Resources

- [Architecture Migration Plan](../../architecture_migration_plan.md) - Subprocess migration plan: remove backward cross-repo dep, move routines to dream_agent
