# Parameter Groups

Semantically coherent parameter sets shared across commands. Groups reduce duplication and clarify common patterns.

**Total Groups:** 3
**Governance Principle:** Semantic Coherence (parameters share PURPOSE, not just frequency)

## Groups Index

| # | Group | Purpose | Parameters | Applicability | Commands |
|---|-------|---------|------------|---------------|----------|
| 1 | [Universal Output Control](#group-1-universal-output-control) | Output presentation detail | 1 | Universal (100%) | 24 |
| 2 | [Universal Execution Control](#group-2-universal-execution-control) | Preview vs execution | 1 | Write operations (25%) | 6 |
| 3 | [Filesystem Filtering](#group-3-filesystem-filtering) | File traversal and filtering | 3 | Command-specific | 1 |

---

### Group :: 1. Universal Output Control

**Purpose:** Controls output presentation verbosity and detail level across all commands.

**Parameters:**
- [`verbosity::`](params.md#parameter-1-verbosity) - Output detail level (0-5 scale)

**Semantic Coherence Test:**
**Question:** "Does this parameter control output presentation detail?"
**Answer:** YES for `verbosity::` ✅

**Applicability:** Universal (appears in 24/24 commands = 100%)

**Used By:**
- [.info](commands/operations.md#command-1-info)
- [.discover.parameters](commands/operations.md#command-2-discoverparameters)
- [.status](commands/operations.md#command-3-status)
- [.analyze](commands/operations.md#command-4-analyze)
- [.archive.new](commands/archive.md#command-5-archivenew)
- [.archive.load](commands/archive.md#command-6-archiveload)
- [.archive.save](commands/archive.md#command-7-archivesave)
- [.archive.from_directory](commands/archive.md#command-8-archivefromdirectory)
- [.content.internalize](commands/content.md#command-9-contentinternalize)
- [.content.externalize](commands/content.md#command-10-contentexternalize)
- [.content.list](commands/content.md#command-11-contentlist)
- [.file.add](commands/file.md#command-12-fileadd)
- [.file.remove](commands/file.md#command-13-fileremove)
- [.file.list](commands/file.md#command-14-filelist)
- [.file.show](commands/file.md#command-15-fileshow)
- [.materialize](commands/operations.md#command-16-materialize)
- [.unpack](commands/operations.md#command-17-unpack)
- [.pack](commands/operations.md#command-18-pack)
- [.parameter.add](commands/param_mgmt.md#command-19-parameteradd)
- [.parameter.list](commands/param_mgmt.md#command-20-parameterlist)
- [.parameter.remove](commands/param_mgmt.md#command-21-parameterremove)
- [.value.set](commands/value.md#command-22-valueset)
- [.value.list](commands/value.md#command-23-valuelist)
- [.value.clear](commands/value.md#command-24-valueclear)

**Why NOT Included:**
- ❌ `dry::` - Controls execution mode (preview vs real), NOT output detail
- ❌ `format::` - Controls serialization format (JSON vs YAML), NOT verbosity
- ❌ `pretty::` - Controls JSON formatting (compact vs pretty), NOT verbosity level
- ❌ `filter::` - Controls content filtering, NOT output presentation

**Pattern:** Single-parameter universal group (100% applicability)

**Notes:**
- Default value (1) provides balanced output for most use cases
- Silent mode (0) useful for scripting (errors only)
- Debug modes (3-5) intended for development/troubleshooting
- Every command respects this parameter consistently

---

### Group :: 2. Universal Execution Control

**Purpose:** Controls whether operations execute normally or run in preview mode without making changes.

**Parameters:**
- [`dry::`](params.md#parameter-2-dry) - Preview mode flag (0=execute, 1=preview)

**Semantic Coherence Test:**
**Question:** "Does this parameter control execution vs preview mode?"
**Answer:** YES for `dry::` ✅

**Applicability:** Write operations only (6/24 commands = 25%)

**Used By:**
- [.archive.save](commands/archive.md#command-7-archivesave) - Preview file write
- [.archive.from_directory](commands/archive.md#command-8-archivefromdirectory) - Preview directory scan (note: creates archive in memory but doesn't persist)
- [.content.internalize](commands/content.md#command-9-contentinternalize) - Preview content conversion
- [.materialize](commands/operations.md#command-16-materialize) - Preview template rendering
- [.unpack](commands/operations.md#command-17-unpack) - Preview file extraction
- [.pack](commands/operations.md#command-18-pack) - Preview archive creation

**Why NOT Included:**
- ❌ `verbosity::` - Controls output detail, NOT execution mode
- ❌ `mandatory::` - Parameter metadata, NOT execution control
- ❌ Read operations (`.info`, `.status`, `.file.list`, etc.) - No side effects, dry mode meaningless

**Pattern:** Single-parameter selective group (applies only to write operations)

**Notes:**
- Only appears in commands with side effects (file writes, archive modifications)
- Default (0) ensures real execution unless explicitly previewed
- Safe to use always (no harm in dry-running read operations, just no effect)
- Combines well with `verbosity::2+` for detailed previews

**Typical Workflow:**
```bash
# 1. Preview operation
genfile .materialize destination::"./output" dry::1 verbosity::2

# 2. Review preview output

# 3. Execute for real
genfile .materialize destination::"./output" dry::0
```

---

### Group :: 3. Filesystem Filtering

**Purpose:** Controls filesystem traversal depth and file inclusion/exclusion patterns for directory scanning.

**Parameters:**
- [`recursive::`](params.md#parameter-10-recursive) - Subdirectory traversal flag (0=flat, 1=recursive)
- [`include_pattern::`](params.md#parameter-17-includepattern) - Glob pattern for file inclusion
- [`exclude_pattern::`](params.md#parameter-21-excludepattern) - Glob pattern for file exclusion

**Semantic Coherence Test:**
**Question:** "Does this parameter control filesystem discovery scope or filtering?"
**Answer:** YES for all three parameters ✅

**Applicability:** Command-specific (.archive.from_directory only)

**Used By:**
- [.archive.from_directory](commands/archive.md#command-8-archivefromdirectory)

**Why NOT Included:**
- ❌ `source::` - Specifies starting directory, NOT filtering strategy
- ❌ `mode::` - Controls content storage (inline vs reference), NOT filesystem discovery
- ❌ `filter::` - Used in `.content.list` for archive filtering, NOT filesystem filtering

**Pattern:** Multi-parameter command-specific group (semantic coherence across related features)

**Interaction Model:**
1. `recursive::1` enables subdirectory traversal
2. `include_pattern::` applied first (whitelist)
3. `exclude_pattern::` applied second (blacklist)
4. Result: Only files matching include AND NOT matching exclude

**Examples:**

**Include only Rust files:**
```bash
genfile .archive.from_directory \
  source::"./project" \
  recursive::1 \
  include_pattern::"**/*.rs"
# Scans all subdirectories, includes only .rs files
```

**Exclude build artifacts:**
```bash
genfile .archive.from_directory \
  source::"./workspace" \
  recursive::1 \
  exclude_pattern::"**/target/**"
# Scans all subdirectories except target/
```

**Complex filtering:**
```bash
genfile .archive.from_directory \
  source::"./src" \
  recursive::1 \
  include_pattern::"**/*.{rs,toml,md}" \
  exclude_pattern::"**/target/**"
# Include only Rust/TOML/Markdown, exclude target directories
```

**Notes:**
- All three parameters work together for precise file selection
- Default: recursive enabled, no patterns (include all files)
- Patterns use glob syntax (`**` for subdirs, `*` for wildcards)
- Exclusions override inclusions (if file matches both, it's excluded)

---

## Semantic Coherence Principle

**Definition:** Parameters belong in same group ONLY if they share semantic purpose.

**Test:** "Does parameter X control [group purpose]?"
**Rule:** Answer must be YES for ALL parameters in group.

**Good Example (Filesystem Filtering):**
- Question: "Does this parameter control filesystem discovery/filtering?"
- `recursive::` → YES (controls traversal depth) ✅
- `include_pattern::` → YES (controls file inclusion) ✅
- `exclude_pattern::` → YES (controls file exclusion) ✅
- **Result:** Semantically coherent group ✅

**Bad Example (Hypothetical "Common Parameters"):**
- Question: "Does this parameter appear frequently?"
- `verbosity::` → YES (24 commands) ✅
- `dry::` → YES (6 commands) ✅
- `path::` → YES (5 commands) ✅
- **Problem:** Frequency ≠ semantic relationship ❌
- **Result:** NOT semantically coherent (would mix output control + execution control + I/O paths)

**Key Insight:** Co-occurrence ≠ semantic relationship. Groups must share PURPOSE, not just frequency.

---

## Parameter Group Benefits

**For Users:**
- **Consistency:** Same parameters work same way across commands
- **Predictability:** Universal groups expected everywhere
- **Learnability:** Learn once, use everywhere

**For Implementers:**
- **Reusability:** Shared validation and handling logic
- **Maintainability:** Change group behavior in one place
- **Testability:** Test group behavior once, applies everywhere

**For Documentation:**
- **Reduced Duplication:** Document group once, reference from commands
- **Clear Patterns:** Explicit grouping clarifies design intent
- **Easier Navigation:** Users understand parameter organization

---

## See Also

- [Parameters Reference](params.md) - Individual parameter specifications
- [Commands Reference](commands.md) - Command usage patterns
- [Types Reference](types.md) - Type system and validation
