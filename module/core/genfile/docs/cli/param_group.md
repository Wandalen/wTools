# Parameter Groups

Semantically coherent parameter sets shared across commands. Groups reduce duplication and clarify common patterns.

**Governance Principle:** Semantic Coherence — parameters share PURPOSE, not just frequency.

### Groups Index

| # | Group | Purpose | Parameters | Applicability |
|---|-------|---------|------------|---------------|
| 1 | [Universal Output Control](#group--1-universal-output-control) | Output presentation detail | 1 | Universal |
| 2 | [Universal Execution Control](#group--2-universal-execution-control) | Preview vs execution | 1 | Write operations |
| 3 | [Filesystem Filtering](#group--3-filesystem-filtering) | File traversal and filtering | 3 | Command-specific |

---

### Group :: 1. Universal Output Control

Controls output presentation verbosity and detail level across all commands.

- **Parameters:** [`verbosity::`](param.md#parameter--1-verbosity) — Output detail level (0-5 scale)
- **Applicability:** Universal (all commands)
- **Semantic Coherence Test:** "Does this parameter control output presentation detail?" — YES for `verbosity::` ✅

### Referenced Commands

All commands implement this group.

### Excluded Parameters

- `dry::` — Controls execution mode (preview vs real), NOT output detail
- `format::` — Controls serialization format (JSON vs YAML), NOT verbosity
- `pretty::` — Controls JSON formatting (compact vs pretty), NOT verbosity level
- `filter::` — Controls content filtering, NOT output presentation

### Notes

- Default value (1) provides balanced output for most use cases
- Silent mode (0) useful for scripting (errors only)
- Debug modes (3-5) intended for development/troubleshooting
- Every command respects this parameter consistently

---

### Group :: 2. Universal Execution Control

Controls whether operations execute normally or run in preview mode without making changes.

- **Parameters:** [`dry::`](param.md#parameter--2-dry) — Preview mode flag (0=execute, 1=preview)
- **Applicability:** Write operations only
- **Semantic Coherence Test:** "Does this parameter control execution vs preview mode?" — YES for `dry::` ✅

### Referenced Commands

| # | Command | File |
|---|---------|------|
| 7 | [.archive.save](command/archive.md) | archive.md |
| 8 | [.archive.from_directory](command/archive.md) | archive.md |
| 9 | [.content.internalize](command/content.md) | content.md |
| 16 | [.materialize](command/operations.md) | operations.md |
| 17 | [.unpack](command/operations.md) | operations.md |
| 18 | [.pack](command/operations.md) | operations.md |

### Excluded Parameters

- `verbosity::` — Controls output detail, NOT execution mode
- `mandatory::` — Parameter metadata, NOT execution control
- Read operations (`.info`, `.status`, `.file.list`, etc.) — No side effects, dry mode meaningless

### Notes

- Only appears in commands with side effects (file writes, archive modifications)
- Default (0) ensures real execution unless explicitly previewed
- Combines well with `verbosity::2+` for detailed previews

### Typical Workflow

```bash
# 1. Preview operation
genfile .materialize destination::"./output" dry::1 verbosity::2

# 2. Review preview output

# 3. Execute for real
genfile .materialize destination::"./output" dry::0
```

---

### Group :: 3. Filesystem Filtering

Controls filesystem traversal depth and file inclusion/exclusion patterns for directory scanning.

- **Parameters:**
  - [`recursive::`](param.md#parameter--10-recursive) — Subdirectory traversal flag (0=flat, 1=recursive)
  - [`include_pattern::`](param.md#parameter--17-include_pattern) — Glob pattern for file inclusion
  - [`exclude_pattern::`](param.md#parameter--21-exclude_pattern) — Glob pattern for file exclusion
- **Applicability:** `.archive.from_directory` only
- **Semantic Coherence Test:** "Does this parameter control filesystem discovery scope or filtering?" — YES for all three ✅

### Referenced Commands

| # | Command | File |
|---|---------|------|
| 8 | [.archive.from_directory](command/archive.md) | archive.md |

### Excluded Parameters

- `source::` — Specifies starting directory, NOT filtering strategy
- `mode::` — Controls content storage (inline vs reference), NOT filesystem discovery
- `filter::` — Used in `.content.list` for archive filtering, NOT filesystem filtering

### Interaction Model

1. `recursive::1` enables subdirectory traversal
2. `include_pattern::` applied first (whitelist)
3. `exclude_pattern::` applied second (blacklist)
4. Result: Only files matching include AND NOT matching exclude

### Examples

```bash
# Include only Rust files
genfile .archive.from_directory \
  source::"./project" \
  recursive::1 \
  include_pattern::"**/*.rs"

# Exclude build artifacts
genfile .archive.from_directory \
  source::"./workspace" \
  recursive::1 \
  exclude_pattern::"**/target/**"

# Complex filtering
genfile .archive.from_directory \
  source::"./src" \
  recursive::1 \
  include_pattern::"**/*.{rs,toml,md}" \
  exclude_pattern::"**/target/**"
```

---

### Semantic Coherence Principle

Parameters belong in the same group ONLY if they share semantic purpose.

**Test:** "Does parameter X control [group purpose]?"
**Rule:** Answer must be YES for ALL parameters in the group.

**Counter-example (hypothetical "Common Parameters"):**
- Question: "Does this parameter appear frequently?"
- `verbosity::`, `dry::`, `path::` all answer YES — but they mix output control, execution control, and I/O paths.
- Co-occurrence ≠ semantic relationship.

### See Also

- [Parameters](param.md) - Individual parameter specifications
- [Commands](command/readme.md) - Command usage patterns
- [Types](type.md) - Type system and validation
