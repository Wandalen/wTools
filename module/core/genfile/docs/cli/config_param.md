# Configuration File Parameters

Configuration file mechanism for persisting genfile CLI defaults across sessions.

### Scope

- **In Scope:** Configuration file format, discovery locations, supported parameters, and precedence relative to environment variables and CLI flags
- **Out of Scope:** Archive files (`.json`/`.yaml`) — those are data files, not configuration; shell environment management
- **Audience:** CLI users, integrators, and system administrators
- **Responsibility:** Complete reference for file-based parameter configuration

### Mechanism

A configuration file provides persistent defaults that survive shell session restarts. Values in the config file are applied after built-in defaults and before environment variables.

**Precedence (highest to lowest):**
1. CLI flag (e.g., `verbosity::2`) — highest priority
2. Environment variable (e.g., `GENFILE_VERBOSITY=2`)
3. Configuration file value — persistent user defaults
4. Built-in default — lowest priority

### Discovery Locations

Genfile searches for configuration files in the following order, using the first one found:

| Priority | Location | Purpose |
|----------|----------|---------|
| 1 | `$GENFILE_CONFIG` (if set) | Explicit override |
| 2 | `.genfile.toml` (current directory) | Project-local config |
| 3 | `~/.config/genfile/config.toml` | User config (XDG) |
| 4 | `~/.genfile.toml` | User config (legacy) |

### File Format

Configuration files use TOML format.

**Example `~/.config/genfile/config.toml`:**
```toml
[defaults]
verbosity = 1
dry = 0
recursive = 1
```

**Example `.genfile.toml` (project-local):**
```toml
[defaults]
verbosity = 2
exclude_pattern = "**/target/**"
```

### Supported Keys

All scalar CLI parameters are supported as configuration keys under the `[defaults]` section.

| Config Key | CLI Equivalent | Example Value |
|------------|----------------|---------------|
| `verbosity` | `verbosity::` | `1` |
| `dry` | `dry::` | `0` |
| `recursive` | `recursive::` | `1` |
| `include_pattern` | `include_pattern::` | `"**/*.rs"` |
| `exclude_pattern` | `exclude_pattern::` | `"**/target/**"` |

### Project-Local vs User Config

**Project-local (`.genfile.toml` in working directory):**
- Applies to all genfile invocations from that directory
- Commit to version control to share with team
- Useful for: project-specific `exclude_pattern`, default `verbosity`

**User config (`~/.config/genfile/config.toml`):**
- Applies to all genfile invocations by the current user
- Not committed to version control
- Useful for: personal verbosity preference, default dry-run behavior

### Validation

Config file values are validated using the same rules as CLI flags. Invalid values produce an error at startup.

```
Error: invalid value for 'verbosity' in .genfile.toml: '99' — valid range is 0–5
```

### See Also

- [Parameters](param.md) — Complete parameter specifications
- [Environment Variable Parameters](env_param.md) — Session-based configuration mechanism
- [Parameter Groups](param_group.md) — Semantic parameter groupings
