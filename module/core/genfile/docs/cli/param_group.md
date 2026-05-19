# Parameter Groups

Semantically coherent parameter sets shared across commands. Groups reduce duplication and clarify common patterns.

### Scope

- **In Scope:** Semantic parameter groups with shared purpose, invariants, and cross-reference tables
- **Out of Scope:** Individual parameter specifications (see [Parameters](param.md)), command-specific parameters not part of any group
- **Audience:** CLI users and integrators who rely on consistent parameter behavior across commands
- **Governance Principle:** Semantic Coherence — parameters belong in the same group ONLY if they share PURPOSE, not just frequency of co-occurrence.

### Groups Index

| # | Group | Purpose | Parameters | Applicability |
|---|-------|---------|------------|---------------|
| 1 | [Universal Output Control](#group--1-universal-output-control) | Output presentation detail | 1 | Universal |
| 2 | [Universal Execution Control](#group--2-universal-execution-control) | Preview vs execution | 1 | Write operations |
| 3 | [Filesystem Filtering](#group--3-filesystem-filtering) | File traversal and filtering | 3 | Command-specific |

---

### Group :: 1. Universal Output Control

**Pattern:** Every command accepts `verbosity::` to control how much output it produces. The value is an integer 0–5. The default (1) provides balanced output for interactive use.

**Purpose:** Controls output presentation verbosity and detail level across all commands.

**Applicability:** Universal — all 24 commands implement this group.

#### Semantic Coherence Test

**Test:** "Does this parameter control output presentation detail?"

- `verbosity::` — YES ✅ — directly controls how much output is produced

#### Why NOT Included

- `dry::` — Controls execution mode (preview vs real), NOT output detail
- `format::` — Controls serialization format (JSON vs YAML), NOT verbosity
- `pretty::` — Controls JSON formatting (compact vs pretty), NOT verbosity level
- `filter::` — Controls content filtering, NOT output presentation

#### Invariants

- Every command must accept `verbosity::` — no command may exclude it
- Default value (1) is consistent across all commands
- Level 0 produces no output except errors — safe for scripting
- Levels 3-5 are for development/troubleshooting only; not suitable for CI/CD output parsing

#### Referenced Parameters

| # | Parameter | Type | Default | Role in Group |
|---|-----------|------|---------|---------------|
| 1 | [`verbosity::`](param.md#parameter--1-verbosity) | [VerbosityLevel](type.md#type--1-verbositylevel) | `1` | Output detail level (0=silent, 5=ultra-trace) |

#### Referenced Commands

| # | Command | Membership | Excluded Params | Notes |
|---|---------|------------|-----------------|-------|
| 1 | [`.info`](command/operations.md#command--1-info) | Full | — | — |
| 2 | [`.discover.parameters`](command/operations.md#command--2-discoverparameters) | Full | — | — |
| 3 | [`.status`](command/operations.md#command--3-status) | Full | — | — |
| 4 | [`.analyze`](command/operations.md#command--4-analyze) | Full | — | — |
| 5 | [`.archive.new`](command/archive.md#command--5-archivenew) | Full | — | — |
| 6 | [`.archive.load`](command/archive.md#command--6-archiveload) | Full | — | — |
| 7 | [`.archive.save`](command/archive.md#command--7-archivesave) | Full | — | — |
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | Full | — | — |
| 9 | [`.content.internalize`](command/content.md#command--9-contentinternalize) | Full | — | — |
| 10 | [`.content.externalize`](command/content.md#command--10-contentexternalize) | Full | — | — |
| 11 | [`.content.list`](command/content.md#command--11-contentlist) | Full | — | — |
| 12 | [`.file.add`](command/file.md#command--12-fileadd) | Full | — | — |
| 13 | [`.file.remove`](command/file.md#command--13-fileremove) | Full | — | — |
| 14 | [`.file.list`](command/file.md#command--14-filelist) | Full | — | — |
| 15 | [`.file.show`](command/file.md#command--15-fileshow) | Full | — | — |
| 16 | [`.materialize`](command/operations.md#command--16-materialize) | Full | — | — |
| 17 | [`.unpack`](command/operations.md#command--17-unpack) | Full | — | — |
| 18 | [`.pack`](command/operations.md#command--18-pack) | Full | — | — |
| 19 | [`.parameter.add`](command/param_mgmt.md#command--19-parameteradd) | Full | — | — |
| 20 | [`.parameter.list`](command/param_mgmt.md#command--20-parameterlist) | Full | — | — |
| 21 | [`.parameter.remove`](command/param_mgmt.md#command--21-parameterremove) | Full | — | — |
| 22 | [`.value.set`](command/value.md#command--22-valueset) | Full | — | — |
| 23 | [`.value.list`](command/value.md#command--23-valuelist) | Full | — | — |
| 24 | [`.value.clear`](command/value.md#command--24-valueclear) | Full | — | — |

#### Referenced Tests

| # | Test Spec | Scope |
|---|-----------|-------|
| 1 | [001_universal_output_control.md](../../tests/docs/cli/param_group/001_universal_output_control.md) | Universal Output Control group invariants |

#### Typical Patterns

```bash
# Silent mode for scripting
genfile .archive.save path::"out.json" verbosity::0

# Default interactive mode (1 is the default, can be omitted)
genfile .archive.load path::"template.yaml"

# Verbose for understanding operations
genfile .materialize destination::"./output" verbosity::2

# Debug for troubleshooting
genfile .archive.from_directory source::"./src" verbosity::3
```

---

### Group :: 2. Universal Execution Control

**Pattern:** Write operations accept `dry::` to toggle between preview mode (1) and real execution (0). Dry mode shows what would happen without making changes.

**Purpose:** Controls whether operations execute normally or run in preview mode without making changes.

**Applicability:** Write operations only — commands with side effects (file writes, archive modifications).

#### Semantic Coherence Test

**Test:** "Does this parameter control execution vs preview mode?"

- `dry::` — YES ✅ — directly toggles between real and preview execution

#### Why NOT Included

- `verbosity::` — Controls output detail, NOT execution mode
- `mandatory::` — Parameter metadata, NOT execution control
- Read operations (`.info`, `.status`, `.file.list`, etc.) — No side effects; dry mode is meaningless for these

#### Invariants

- Only appears in commands with side effects (file writes, archive modifications)
- Default (0) ensures real execution unless explicitly previewed — no accidental dry runs
- Exit codes are identical between dry and real execution (both reflect what would happen)
- Dry mode performs full validation — errors in dry mode predict errors in real execution

#### Referenced Parameters

| # | Parameter | Type | Default | Role in Group |
|---|-----------|------|---------|---------------|
| 2 | [`dry::`](param.md#parameter--2-dry) | [DryRunFlag](type.md#type--2-dryrunflag) | `0` | Preview mode toggle (0=execute, 1=preview) |

#### Referenced Commands

| # | Command | Membership | Excluded Params | Notes |
|---|---------|------------|-----------------|-------|
| 2 | [`.discover.parameters`](command/operations.md#command--2-discoverparameters) | Full | — | Previews parameter detection |
| 4 | [`.analyze`](command/operations.md#command--4-analyze) | Full | — | Previews analysis |
| 7 | [`.archive.save`](command/archive.md#command--7-archivesave) | Full | — | Previews file write |
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | Full | — | Previews directory scan |
| 9 | [`.content.internalize`](command/content.md#command--9-contentinternalize) | Full | — | Previews content read |
| 10 | [`.content.externalize`](command/content.md#command--10-contentexternalize) | Full | — | Previews content externalization |
| 16 | [`.materialize`](command/operations.md#command--16-materialize) | Full | — | Previews file generation |
| 17 | [`.unpack`](command/operations.md#command--17-unpack) | Full | — | Previews file extraction |
| 18 | [`.pack`](command/operations.md#command--18-pack) | Full | — | Previews pack operation |
| 24 | [`.value.clear`](command/value.md#command--24-valueclear) | Full | — | Previews value deletion |

#### Referenced Tests

| # | Test Spec | Scope |
|---|-----------|-------|
| 1 | [002_universal_execution_control.md](../../tests/docs/cli/param_group/002_universal_execution_control.md) | Universal Execution Control group invariants |

#### Typical Patterns

```bash
# 1. Preview operation
genfile .materialize destination::"./output" dry::1 verbosity::2

# 2. Review preview output, then execute
genfile .materialize destination::"./output"

# CI/CD: enable dry-run globally via environment
export GENFILE_DRY=1
genfile .archive.save path::"out.json"   # preview only
```

---

### Group :: 3. Filesystem Filtering

**Pattern:** Directory scan commands accept filtering parameters to control which files are included. Include patterns act as a whitelist; exclude patterns act as a blacklist. Both can be combined.

**Purpose:** Controls filesystem traversal depth and file inclusion/exclusion patterns for directory scanning.

**Applicability:** `.archive.from_directory` only — the sole command that scans a filesystem directory.

#### Semantic Coherence Test

**Test:** "Does this parameter control filesystem discovery scope or filtering?"

- `recursive::` — YES ✅ — controls traversal depth (flat vs recursive)
- `include_pattern::` — YES ✅ — controls which files are included (whitelist glob)
- `exclude_pattern::` — YES ✅ — controls which files are excluded (blacklist glob)

#### Why NOT Included

- `source::` — Specifies starting directory, NOT filtering strategy
- `mode::` — Controls content storage (inline vs reference), NOT filesystem discovery
- `filter::` — Used in `.content.list` for archive filtering, NOT filesystem filtering

#### Invariants

- `include_pattern::` is applied before `exclude_pattern::` (whitelist then blacklist)
- A file must match `include_pattern::` AND NOT match `exclude_pattern::` to be included
- When both patterns are absent, all files are included (no filtering)
- `recursive::0` (flat mode) disregards any `include_pattern::` that uses `**` — patterns still applied but subdirectories not traversed

#### Referenced Parameters

| # | Parameter | Type | Default | Role in Group |
|---|-----------|------|---------|---------------|
| 10 | [`recursive::`](param.md#parameter--10-recursive) | [RecursiveFlag](type.md#type--13-recursiveflag) | `1` | Subdirectory traversal (0=flat, 1=recursive) |
| 17 | [`include_pattern::`](param.md#parameter--17-include_pattern) | [PatternString](type.md#type--8-patternstring) | `null` | Glob pattern for file inclusion (whitelist) |
| 21 | [`exclude_pattern::`](param.md#parameter--21-exclude_pattern) | [PatternString](type.md#type--8-patternstring) | `null` | Glob pattern for file exclusion (blacklist) |

#### Referenced Commands

| # | Command | Membership | Excluded Params | Notes |
|---|---------|------------|-----------------|-------|
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | Full | — | Only command using this group |

#### Referenced Tests

| # | Test Spec | Scope |
|---|-----------|-------|
| 1 | [003_filesystem_filtering.md](../../tests/docs/cli/param_group/003_filesystem_filtering.md) | Filesystem Filtering group invariants |

#### Typical Patterns

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

# Complex filtering: Rust/TOML/Markdown, excluding target
genfile .archive.from_directory \
  source::"./src" \
  recursive::1 \
  include_pattern::"**/*.{rs,toml,md}" \
  exclude_pattern::"**/target/**"

# Flat scan (top-level only)
genfile .archive.from_directory \
  source::"./templates" \
  recursive::0
```

