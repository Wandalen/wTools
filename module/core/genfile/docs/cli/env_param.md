# Environment Variable Parameters

Environment variable mechanism for configuring genfile CLI parameters without command-line flags.

### Scope

- **In Scope:** Environment variable naming convention, precedence relative to CLI flags, supported parameters, and override behavior
- **Out of Scope:** Shell-specific syntax, OS environment management — this documents the genfile-specific mechanism only
- **Audience:** CLI users, integrators, and CI/CD pipeline authors
- **Responsibility:** Complete reference for environment-based parameter configuration

### Mechanism

Environment variables provide a persistent, session-wide alternative to passing parameters on every command invocation. They are read at startup and applied as defaults that CLI flags override.

**Precedence (highest to lowest):**
1. CLI flag (e.g., `verbosity::2`) — highest priority
2. Environment variable (e.g., `GENFILE_VERBOSITY=2`) — session default
3. Built-in default (defined per parameter) — lowest priority

### Naming Convention

Every CLI parameter maps to an environment variable following the pattern:

```
GENFILE_<PARAMETER_NAME_UPPER>
```

Where `<PARAMETER_NAME_UPPER>` is the CLI parameter name converted to uppercase with underscores preserved.

**Examples:**

| CLI Parameter | Environment Variable |
|---------------|---------------------|
| `verbosity::` | `GENFILE_VERBOSITY` |
| `dry::` | `GENFILE_DRY` |
| `recursive::` | `GENFILE_RECURSIVE` |
| `include_pattern::` | `GENFILE_INCLUDE_PATTERN` |
| `exclude_pattern::` | `GENFILE_EXCLUDE_PATTERN` |

### Supported Parameters

All CLI parameters that accept scalar values support environment variable overrides. Parameters that are inherently command-specific (like `destination::` or `source::`) are typically set per-invocation rather than via environment variables.

**Most commonly set via environment:**

| Parameter | Environment Variable | Typical Use |
|-----------|---------------------|-------------|
| `verbosity::` | `GENFILE_VERBOSITY` | Set output level for entire session |
| `dry::` | `GENFILE_DRY` | Enable preview mode globally (e.g., in CI review phase) |
| `recursive::` | `GENFILE_RECURSIVE` | Change default traversal behavior |

### Usage Examples

**Set verbosity for entire session:**
```bash
export GENFILE_VERBOSITY=2
genfile .archive.load path::"template.yaml"  # uses verbosity 2
genfile .materialize destination::"./out"    # uses verbosity 2
```

**Enable dry run globally (CI review phase):**
```bash
export GENFILE_DRY=1
genfile .materialize destination::"./out"   # preview only
genfile .archive.save path::"out.json"      # preview only
```

**Override environment variable with CLI flag:**
```bash
export GENFILE_VERBOSITY=0
genfile .info verbosity::2   # CLI flag overrides: uses verbosity 2
```

**CI/CD configuration:**
```bash
# .env or CI pipeline environment
GENFILE_VERBOSITY=0          # silent mode for scripting
GENFILE_DRY=0                # real execution (explicit)
```

### Validation

Environment variable values are validated using the same rules as CLI flags. Invalid values produce an error at startup.

```bash
GENFILE_VERBOSITY=99 genfile .info
# Error: invalid value for GENFILE_VERBOSITY: '99' — valid range is 0–5
```

### See Also

- [Parameters](param.md) — Complete parameter specifications
- [Configuration File Parameters](config_param.md) — File-based configuration mechanism
- [Parameter Groups](param_group.md) — Semantic parameter groupings
