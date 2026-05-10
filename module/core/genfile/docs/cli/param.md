# Parameters Reference

Complete parameter listing for the genfile CLI.

### Scope

- **In Scope:** All 23 CLI parameters — types, defaults, constraints, command usage, and group membership
- **Out of Scope:** Type implementation details (see [type.md](type.md)), parameter group semantics (see [param_group.md](param_group.md))
- **Audience:** CLI users, integrators, and developers extending genfile
- **Responsibility:** Authoritative parameter specification backing all command documentation

### Parameters Index

| # | Parameter | Type | Default | Commands | Purpose |
|---|-----------|------|---------|----------|---------|
| 1 | [`verbosity::`](#parameter--1-verbosity) | [VerbosityLevel](type.md#type--1-verbositylevel) | `1` | 24 | Output detail control |
| 2 | [`dry::`](#parameter--2-dry) | [DryRunFlag](type.md#type--2-dryrunflag) | `0` | 10 | Preview mode flag |
| 3 | [`path::`](#parameter--3-path) | [FilePath](type.md#type--3-filepath) | — | 5 | File path (input/output) |
| 4 | [`name::`](#parameter--4-name) | [IdentifierString](type.md#type--6-identifierstring) | — | 4 | Entity identifier |
| 5 | [`destination::`](#parameter--5-destination) | [OutputPath](type.md#type--4-outputpath) | — | 2 | Output directory path |
| 6 | [`description::`](#parameter--6-description) | [DescriptionText](type.md#type--7-descriptiontext) | `""` | 2 | Human-readable description |
| 7 | [`write_mode::`](#parameter--7-write_mode) | [WriteMode](type.md#type--12-writemode) | `rewrite` | 1 | File conflict resolution |
| 8 | [`value::`](#parameter--8-value) | [ContentString](type.md#type--9-contentstring) | — | 1 | Parameter value data |
| 9 | [`source::`](#parameter--9-source) | [DirectoryPath](type.md#type--5-directorypath) | — | 1 | Source directory path |
| 10 | [`recursive::`](#parameter--10-recursive) | [RecursiveFlag](type.md#type--13-recursiveflag) | `1` | 1 | Subdirectory traversal |
| 11 | [`pretty::`](#parameter--11-pretty) | [PrettyPrintFlag](type.md#type--14-prettyprintflag) | `1` | 1 | JSON formatting |
| 12 | [`output_dir::`](#parameter--12-output_dir) | [OutputPath](type.md#type--4-outputpath) | — | 1 | Externalized content directory |
| 13 | [`output::`](#parameter--13-output) | [OutputPath](type.md#type--4-outputpath) | — | 1 | Output archive file path |
| 14 | [`mode::`](#parameter--14-mode) | [ContentMode](type.md#type--10-contentmode) | `reference` | 1 | Content storage strategy |
| 15 | [`mandatory::`](#parameter--15-mandatory) | [MandatoryFlag](type.md#type--15-mandatoryflag) | `0` | 1 | Parameter requirement flag |
| 16 | [`input::`](#parameter--16-input) | [DirectoryPath](type.md#type--5-directorypath) | — | 1 | Input directory for packing |
| 17 | [`include_pattern::`](#parameter--17-include_pattern) | [PatternString](type.md#type--8-patternstring) | `null` | 1 | Inclusion glob pattern |
| 18 | [`from_file::`](#parameter--18-from_file) | [FilePath](type.md#type--3-filepath) | `null` | 1 | Source file for content |
| 19 | [`format::`](#parameter--19-format) | [SerializationFormat](type.md#type--11-serializationformat) | `json` | 1 | Serialization format |
| 20 | [`filter::`](#parameter--20-filter) | [PatternString](type.md#type--8-patternstring) | `null` | 1 | Content listing filter |
| 21 | [`exclude_pattern::`](#parameter--21-exclude_pattern) | [PatternString](type.md#type--8-patternstring) | `null` | 1 | Exclusion glob pattern |
| 22 | [`default::`](#parameter--22-default) | [ContentString](type.md#type--9-contentstring) | `null` | 1 | Parameter default value |
| 23 | [`content::`](#parameter--23-content) | [ContentString](type.md#type--9-contentstring) | — | 1 | File content data |

---

### Parameter Specifications

### Parameter :: 1. `verbosity::`

Controls output verbosity level across all commands using a 0–5 scale. Higher values show more detailed information, lower values show minimal output. Default (1) provides balanced output for interactive use; 0 silences all non-error output for scripting.

- **Fundamental Type:** integer
- **Constraints:** Integer in range 0–5 inclusive; out-of-range values are rejected at parse time
- **Default:** `1` (normal output)
- **Purpose:** Output detail level control — universal parameter applied to all commands

- **Sources:**
  - **CLI:** `verbosity::2`
  - **Env:** `GENFILE_VERBOSITY=2`
  - **Config:** `[defaults] verbosity = 2`
  - **Resolution:** CLI > Env > Config > built-in default (1)

### Examples

```bash
genfile .archive.save path::"out.json" verbosity::0      # Silent (scripting)
genfile .archive.load path::"template.yaml"              # Normal (default)
genfile .materialize destination::"./output" verbosity::2  # Verbose
genfile .analyze verbosity::3                            # Debug
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [VerbosityLevel](type.md#type--1-verbositylevel) | Integer | integer | Range 0–5 inclusive |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 1–24 | All commands | `1` | Universal — every command accepts `verbosity::` |

### Referenced Parameter Groups

| # | Group | Membership | Co-members |
|---|-------|------------|------------|
| 1 | [Universal Output Control](param_group.md#group--1-universal-output-control) | Full | — (sole member) |

---

### Parameter :: 2. `dry::`

Enables preview mode where operations show what would happen without making actual changes. Essential for validating complex write operations before execution. Exit codes are identical between dry and real runs.

- **Fundamental Type:** boolean
- **Constraints:** Accepts `0`/`false` (execute) or `1`/`true` (preview); all other values rejected
- **Default:** `0` (real execution)
- **Purpose:** Preview-vs-execution toggle for write operations

- **Sources:**
  - **CLI:** `dry::1`
  - **Env:** `GENFILE_DRY=1`
  - **Config:** `[defaults] dry = 1`
  - **Resolution:** CLI > Env > Config > built-in default (0)

### Examples

```bash
genfile .archive.save path::"test.json" dry::1                  # Preview save
genfile .materialize destination::"./output" dry::1 verbosity::2  # Detailed preview
genfile .materialize destination::"./output"                    # Execute (dry::0 default)
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [DryRunFlag](type.md#type--2-dryrunflag) | Boolean | boolean | 0 or 1 only |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 2 | [`.discover.parameters`](command/operations.md#command--2-discoverparameters) | `0` | Previews parameter detection |
| 4 | [`.analyze`](command/operations.md#command--4-analyze) | `0` | Previews analysis |
| 7 | [`.archive.save`](command/archive.md#command--7-archivesave) | `0` | Previews file write |
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | `0` | Previews directory scan |
| 9 | [`.content.internalize`](command/content.md#command--9-contentinternalize) | `0` | Previews content read |
| 10 | [`.content.externalize`](command/content.md#command--10-contentexternalize) | `0` | Previews content externalization |
| 16 | [`.materialize`](command/operations.md#command--16-materialize) | `0` | Previews file generation |
| 17 | [`.unpack`](command/operations.md#command--17-unpack) | `0` | Previews file extraction |
| 18 | [`.pack`](command/operations.md#command--18-pack) | `0` | Previews pack operation |
| 24 | [`.value.clear`](command/value.md#command--24-valueclear) | `0` | Previews value deletion |

### Referenced Parameter Groups

| # | Group | Membership | Co-members |
|---|-------|------------|------------|
| 2 | [Universal Execution Control](param_group.md#group--2-universal-execution-control) | Full | — (sole member) |

---

### Parameter :: 3. `path::`

Generic file path parameter used by both input and output operations depending on command context. For archive commands, refers to an on-disk file; for file commands, refers to the path within the archive structure.

- **Fundamental Type:** path
- **Constraints:** Must be valid UTF-8; for input commands, file must exist; for output commands, parent directory must be writable or creatable
- **Default:** None — required in all commands that accept it
- **Purpose:** File path for archive I/O and archive-internal file addressing

- **Sources:**
  - **CLI:** `path::"./template.yaml"`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .archive.load path::"./archives/template.json"    # Input: read file
genfile .archive.save path::"./output/archive.yaml"       # Output: write file
genfile .file.add path::"src/main.rs" content::"..."      # Archive-internal path
genfile .file.show path::"readme.md"                      # Archive-internal path
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [FilePath](type.md#type--3-filepath) | Path | path | Valid UTF-8; existence checked at operation time |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 6 | [`.archive.load`](command/archive.md#command--6-archiveload) | — | On-disk path to read |
| 7 | [`.archive.save`](command/archive.md#command--7-archivesave) | — | On-disk path to write |
| 12 | [`.file.add`](command/file.md#command--12-fileadd) | — | Archive-internal path |
| 13 | [`.file.remove`](command/file.md#command--13-fileremove) | — | Archive-internal path |
| 15 | [`.file.show`](command/file.md#command--15-fileshow) | — | Archive-internal path |

---

### Parameter :: 4. `name::`

Entity identifier for archives, parameters, and values. Must be a valid identifier: alphanumeric characters and underscores only, no spaces or special characters.

- **Fundamental Type:** string
- **Constraints:** Non-empty; alphanumeric + underscore only; no spaces, hyphens, or special characters
- **Default:** None — required in all commands that accept it
- **Purpose:** Entity identification for archives, parameter definitions, and value assignments

- **Sources:**
  - **CLI:** `name::project_name`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .archive.new name::"my_template"
genfile .parameter.add name::project_name mandatory::true
genfile .parameter.remove name::old_param
genfile .value.set name::version value::"1.0.0"
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [IdentifierString](type.md#type--6-identifierstring) | String | string | Alphanumeric + underscore; non-empty |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 5 | [`.archive.new`](command/archive.md#command--5-archivenew) | — | Archive name |
| 19 | [`.parameter.add`](command/param_mgmt.md#command--19-parameteradd) | — | Parameter name |
| 21 | [`.parameter.remove`](command/param_mgmt.md#command--21-parameterremove) | — | Parameter name to remove |
| 22 | [`.value.set`](command/value.md#command--22-valueset) | — | Parameter name to set value for |

---

### Parameter :: 5. `destination::`

Output directory path where materialized or unpacked files will be written. The directory is created if it does not exist; parent must be writable.

- **Fundamental Type:** path
- **Constraints:** Must be valid UTF-8; parent directory must be writable or creatable
- **Default:** None — required
- **Purpose:** Output location for template materialization and file extraction

- **Sources:**
  - **CLI:** `destination::"./output"`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .materialize destination::"./output"
genfile .materialize destination::"./output" dry::1 verbosity::2
genfile .unpack destination::"./template-files"
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [OutputPath](type.md#type--4-outputpath) | Path | path | Parent must be writable or creatable |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 16 | [`.materialize`](command/operations.md#command--16-materialize) | — | Required |
| 17 | [`.unpack`](command/operations.md#command--17-unpack) | — | Required |

---

### Parameter :: 6. `description::`

Human-readable description for archives and parameter definitions. Accepts any UTF-8 text including empty strings.

- **Fundamental Type:** string
- **Constraints:** Any UTF-8 text; empty string allowed
- **Default:** `""` (empty string)
- **Purpose:** Human-readable metadata for archives and parameter definitions

- **Sources:**
  - **CLI:** `description::"REST API template"`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .archive.new name::"template" description::"REST API template"
genfile .parameter.add name::author description::"Project author name"
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [DescriptionText](type.md#type--7-descriptiontext) | String | string | Any UTF-8; empty allowed |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 5 | [`.archive.new`](command/archive.md#command--5-archivenew) | `""` | Archive description |
| 19 | [`.parameter.add`](command/param_mgmt.md#command--19-parameteradd) | `""` | Parameter description |

---

### Parameter :: 7. `write_mode::`

Controls file conflict resolution when adding a file to the archive that already exists at the target path.

- **Fundamental Type:** enum
- **Constraints:** Must be `rewrite`, `append`, or `skip` (case-insensitive)
- **Default:** `rewrite` (overwrite existing)
- **Purpose:** File conflict resolution strategy for `.file.add`

- **Sources:**
  - **CLI:** `write_mode::skip`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Constants

- `rewrite` — Overwrite existing file in archive
- `append` — Append content to existing file in archive
- `skip` — Leave existing file unchanged (no-op)

### Examples

```bash
genfile .file.add path::"config.toml" content::"..." write_mode::rewrite
genfile .file.add path::"notes.md" content::"## Addendum" write_mode::append
genfile .file.add path::"readme.md" content::"..." write_mode::skip
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [WriteMode](type.md#type--12-writemode) | Enum | enum | rewrite \| append \| skip |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 12 | [`.file.add`](command/file.md#command--12-fileadd) | `rewrite` | Only command using write_mode:: |

---

### Parameter :: 8. `value::`

Parameter value data for template substitution. This is the actual value that will replace `{{name}}` placeholders during materialization.

- **Fundamental Type:** string
- **Constraints:** Any UTF-8 text; may be empty
- **Default:** None — required
- **Purpose:** Value to assign to a named parameter for template substitution

- **Sources:**
  - **CLI:** `value::"my-project"`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .value.set name::project_name value::"my-app"
genfile .value.set name::version value::"1.0.0"
genfile .value.set name::description value::"A new Rust application"
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [ContentString](type.md#type--9-contentstring) | String | string | Any UTF-8; empty allowed |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 22 | [`.value.set`](command/value.md#command--22-valueset) | — | Required |

---

### Parameter :: 9. `source::`

Source directory path for filesystem scanning. Must exist and be a readable directory at invocation time.

- **Fundamental Type:** path
- **Constraints:** Must be valid UTF-8; must exist as a directory at parse time; must be readable
- **Default:** None — required
- **Purpose:** Starting directory for archive creation from filesystem

- **Sources:**
  - **CLI:** `source::"./templates"`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .archive.from_directory source::"./templates"
genfile .archive.from_directory source::"./src" mode::inline recursive::1
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [DirectoryPath](type.md#type--5-directorypath) | Path | path | Must exist as directory at parse time |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | — | Required |

---

### Parameter :: 10. `recursive::`

Controls whether directory scanning descends into subdirectories. Default is recursive (1) to capture nested files. Use flat mode (0) for top-level-only scans.

- **Fundamental Type:** boolean
- **Constraints:** Accepts `0`/`false` (flat) or `1`/`true` (recursive); all other values rejected
- **Default:** `1` (recursive)
- **Purpose:** Subdirectory traversal depth control for filesystem scanning

- **Sources:**
  - **CLI:** `recursive::0`
  - **Env:** `GENFILE_RECURSIVE=0`
  - **Config:** `[defaults] recursive = 0`
  - **Resolution:** CLI > Env > Config > built-in default (1)

### Examples

```bash
genfile .archive.from_directory source::"./src" recursive::1   # Recursive (default)
genfile .archive.from_directory source::"./config" recursive::0  # Top-level only
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [RecursiveFlag](type.md#type--13-recursiveflag) | Boolean | boolean | 0 or 1 only |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | `1` | — |

### Referenced Parameter Groups

| # | Group | Membership | Co-members |
|---|-------|------------|------------|
| 3 | [Filesystem Filtering](param_group.md#group--3-filesystem-filtering) | Full | [`include_pattern::`](#parameter--17-include_pattern), [`exclude_pattern::`](#parameter--21-exclude_pattern) |

---

### Parameter :: 11. `pretty::`

Controls JSON pretty-printing. When enabled (1), output is indented and human-readable. When disabled (0), output is compact. Has no effect on YAML output (YAML is always human-readable).

- **Fundamental Type:** boolean
- **Constraints:** Accepts `0`/`false` (compact) or `1`/`true` (pretty); all other values rejected
- **Default:** `1` (pretty)
- **Purpose:** JSON formatting control for archive serialization

- **Sources:**
  - **CLI:** `pretty::0`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .archive.save path::"template.json" pretty::1   # Human-readable JSON
genfile .archive.save path::"compact.json" pretty::0    # Compact JSON
genfile .archive.save path::"template.yaml" format::yaml  # pretty:: ignored for YAML
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [PrettyPrintFlag](type.md#type--14-prettyprintflag) | Boolean | boolean | 0 or 1 only; no effect on YAML |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 7 | [`.archive.save`](command/archive.md#command--7-archivesave) | `1` | Affects JSON only |

---

### Parameter :: 12. `output_dir::`

Output directory where externalized content files will be written. Content previously embedded inline in the archive will be written here as individual files.

- **Fundamental Type:** path
- **Constraints:** Must be valid UTF-8; parent directory must be writable or creatable
- **Default:** None — required
- **Purpose:** Target directory for content externalization

- **Sources:**
  - **CLI:** `output_dir::"./files"`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .content.externalize output_dir::"./extracted-files"
genfile .content.externalize output_dir::"./files" verbosity::2
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [OutputPath](type.md#type--4-outputpath) | Path | path | Parent must be writable or creatable |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 10 | [`.content.externalize`](command/content.md#command--10-contentexternalize) | — | Required |

---

### Parameter :: 13. `output::`

Output file path for packed archives. The archive is written to this path as a single self-contained JSON file.

- **Fundamental Type:** path
- **Constraints:** Must be valid UTF-8; parent directory must be writable or creatable
- **Default:** None — required
- **Purpose:** Target file path for one-step pack operation

- **Sources:**
  - **CLI:** `output::"archive.json"`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .pack input::"./templates" output::"template.json"
genfile .pack input::"./src" output::"../dist/archive.json"
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [OutputPath](type.md#type--4-outputpath) | Path | path | Parent must be writable or creatable |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 18 | [`.pack`](command/operations.md#command--18-pack) | — | Required |

---

### Parameter :: 14. `mode::`

Content storage strategy for archive files. Inline mode embeds file content directly in the archive (portable, larger). Reference mode stores only file paths (smaller, requires source files at materialization time).

- **Fundamental Type:** enum
- **Constraints:** Must be `inline` or `reference` (case-insensitive)
- **Default:** `reference`
- **Purpose:** Content storage strategy selection for directory import

- **Sources:**
  - **CLI:** `mode::inline`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Constants

- `inline` — Embed file content in archive (portable; larger archive size)
- `reference` — Store file paths only (requires source files; smaller archive)

### Examples

```bash
genfile .archive.from_directory source::"./src" mode::inline      # Portable
genfile .archive.from_directory source::"./templates" mode::reference  # Lightweight
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [ContentMode](type.md#type--10-contentmode) | Enum | enum | inline \| reference |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | `reference` | — |

---

### Parameter :: 15. `mandatory::`

Whether a parameter requires a value to be set before template materialization can proceed. Mandatory parameters without a value or default cause `.materialize` to fail with a clear error.

- **Fundamental Type:** boolean
- **Constraints:** Accepts `0`/`false` (optional) or `1`/`true` (mandatory); all other values rejected
- **Default:** `0` (optional)
- **Purpose:** Parameter requirement declaration for materialization validation

- **Sources:**
  - **CLI:** `mandatory::true`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .parameter.add name::project_name mandatory::true   # Must be set
genfile .parameter.add name::author mandatory::false        # Uses default or empty
genfile .parameter.add name::version mandatory::1           # Integer form also accepted
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [MandatoryFlag](type.md#type--15-mandatoryflag) | Boolean | boolean | 0 or 1 only |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 19 | [`.parameter.add`](command/param_mgmt.md#command--19-parameteradd) | `0` | — |

---

### Parameter :: 16. `input::`

Input directory path for one-step pack operations. The directory is scanned recursively and all files are embedded inline in the output archive.

- **Fundamental Type:** path
- **Constraints:** Must be valid UTF-8; must exist as a directory at parse time; must be readable
- **Default:** None — required
- **Purpose:** Source directory for one-step directory-to-archive packing

- **Sources:**
  - **CLI:** `input::"./templates"`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .pack input::"./templates" output::"archive.json"
genfile .pack input::"./src" output::"src-archive.json"
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [DirectoryPath](type.md#type--5-directorypath) | Path | path | Must exist as directory at parse time |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 18 | [`.pack`](command/operations.md#command--18-pack) | — | Required |

---

### Parameter :: 17. `include_pattern::`

Glob pattern for file inclusion during directory scanning. Acts as a whitelist: only files matching this pattern are included. Applied before `exclude_pattern::`.

- **Fundamental Type:** string
- **Constraints:** Must be valid glob pattern syntax; `null` means include all files
- **Default:** `null` (include all files)
- **Purpose:** File inclusion whitelist filter for directory scanning

- **Sources:**
  - **CLI:** `include_pattern::"**/*.rs"`
  - **Env:** `GENFILE_INCLUDE_PATTERN=**/*.rs`
  - **Config:** `[defaults] include_pattern = "**/*.rs"`
  - **Resolution:** CLI > Env > Config > built-in default (null)

### Examples

```bash
genfile .archive.from_directory source::"./src" include_pattern::"**/*.rs"
genfile .archive.from_directory source::"./docs" include_pattern::"**/*.{md,txt}"
genfile .archive.from_directory source::"./project" \
  include_pattern::"**/*.{rs,toml,md}" \
  exclude_pattern::"**/target/**"
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [PatternString](type.md#type--8-patternstring) | String | string | Valid glob syntax; null = include all |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | `null` | Applied before exclude_pattern:: |

### Referenced Parameter Groups

| # | Group | Membership | Co-members |
|---|-------|------------|------------|
| 3 | [Filesystem Filtering](param_group.md#group--3-filesystem-filtering) | Full | [`recursive::`](#parameter--10-recursive), [`exclude_pattern::`](#parameter--21-exclude_pattern) |

---

### Parameter :: 18. `from_file::`

Source file path to read content from when adding a file to the archive. Alternative to `content::` — use one or the other, not both. The file must exist at invocation time.

- **Fundamental Type:** path
- **Constraints:** Must be valid UTF-8; file must exist and be readable; mutually exclusive with `content::`
- **Default:** `null`
- **Purpose:** File content source from filesystem (alternative to inline `content::`)

- **Sources:**
  - **CLI:** `from_file::"./README.md"`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .file.add path::"readme.md" from_file::"./README.md"
genfile .file.add path::"ci.yml" from_file::".github/workflows/ci.yml"
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [FilePath](type.md#type--3-filepath) | Path | path | Must exist and be readable; mutually exclusive with content:: |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 12 | [`.file.add`](command/file.md#command--12-fileadd) | `null` | Mutually exclusive with content:: |

---

### Parameter :: 19. `format::`

Serialization format for archive persistence. When not specified, format is auto-detected from the file extension (`.json` → JSON, `.yaml`/`.yml` → YAML). Overrides auto-detection when specified.

- **Fundamental Type:** enum
- **Constraints:** Must be `json`, `yaml`, or `yml` (case-insensitive)
- **Default:** `json` (or auto-detected from file extension)
- **Purpose:** Explicit serialization format selection; overrides extension-based auto-detection

- **Sources:**
  - **CLI:** `format::yaml`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Constants

- `json` — JSON encoding (pretty or compact via `pretty::`)
- `yaml` — YAML encoding (always human-readable)

### Examples

```bash
genfile .archive.save path::"template.json" format::json   # Explicit JSON
genfile .archive.save path::"template.yaml" format::yaml   # Explicit YAML
genfile .archive.save path::"archive.dat" format::json     # Override extension
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [SerializationFormat](type.md#type--11-serializationformat) | Enum | enum | json \| yaml; auto-detected from extension |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 7 | [`.archive.save`](command/archive.md#command--7-archivesave) | `json` | Auto-detects from extension if omitted |

---

### Parameter :: 20. `filter::`

Glob pattern for filtering content listing results. Used to show only files with a specific content mode or path pattern.

- **Fundamental Type:** string
- **Constraints:** Must be valid glob pattern syntax; `null` means no filtering (show all)
- **Default:** `null` (no filter)
- **Purpose:** Content listing filter for `.content.list`

- **Sources:**
  - **CLI:** `filter::inline`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .content.list                        # Show all files
genfile .content.list filter::inline         # Show inline files only
genfile .content.list filter::reference      # Show reference files only
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [PatternString](type.md#type--8-patternstring) | String | string | Valid glob; null = no filter |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 11 | [`.content.list`](command/content.md#command--11-contentlist) | `null` | — |

---

### Parameter :: 21. `exclude_pattern::`

Glob pattern for file exclusion during directory scanning. Acts as a blacklist: files matching this pattern are excluded even if they match `include_pattern::`. Applied after `include_pattern::`.

- **Fundamental Type:** string
- **Constraints:** Must be valid glob pattern syntax; `null` means exclude no files
- **Default:** `null` (exclude no files)
- **Purpose:** File exclusion blacklist filter for directory scanning

- **Sources:**
  - **CLI:** `exclude_pattern::"**/target/**"`
  - **Env:** `GENFILE_EXCLUDE_PATTERN=**/target/**`
  - **Config:** `[defaults] exclude_pattern = "**/target/**"`
  - **Resolution:** CLI > Env > Config > built-in default (null)

### Examples

```bash
genfile .archive.from_directory source::"./src" exclude_pattern::"**/target/**"
genfile .archive.from_directory source::"./project" exclude_pattern::"**/*.{log,tmp}"
genfile .archive.from_directory source::"./workspace" \
  include_pattern::"**/*.rs" \
  exclude_pattern::"**/tests/**"
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [PatternString](type.md#type--8-patternstring) | String | string | Valid glob; null = exclude nothing |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | `null` | Applied after include_pattern:: |

### Referenced Parameter Groups

| # | Group | Membership | Co-members |
|---|-------|------------|------------|
| 3 | [Filesystem Filtering](param_group.md#group--3-filesystem-filtering) | Full | [`recursive::`](#parameter--10-recursive), [`include_pattern::`](#parameter--17-include_pattern) |

---

### Parameter :: 22. `default::`

Default value for a parameter when no explicit value is set via `.value.set`. If a mandatory parameter has no default and no value is set, materialization fails. Optional parameters with a default use it automatically.

- **Fundamental Type:** string
- **Constraints:** Any UTF-8 text; `null` means no default (parameter must be explicitly set if mandatory)
- **Default:** `null` (no default)
- **Purpose:** Fallback value for parameter definitions

- **Sources:**
  - **CLI:** `default::"0.1.0"`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .parameter.add name::port default::"8080"
genfile .parameter.add name::version default::"0.1.0"
genfile .parameter.add name::author default::""   # Empty string default
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [ContentString](type.md#type--9-contentstring) | String | string | Any UTF-8; null = no default |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 19 | [`.parameter.add`](command/param_mgmt.md#command--19-parameteradd) | `null` | — |

---

### Parameter :: 23. `content::`

File content data for adding a file to the archive. Alternative to `from_file::` — use one or the other, not both. Supports multiline content and `{{placeholder}}` syntax.

- **Fundamental Type:** string
- **Constraints:** Any UTF-8 text; mutually exclusive with `from_file::` in `.file.add`
- **Default:** None — required when `from_file::` is not provided
- **Purpose:** Inline file content for archive file addition

- **Sources:**
  - **CLI:** `content::"fn main() {}"`
  - **Env:** not applicable (command-specific)
  - **Config:** not applicable (command-specific)
  - **Resolution:** CLI only

### Examples

```bash
genfile .file.add path::"main.rs" content::'fn main() {}'
genfile .file.add path::"readme.md" content::"# {{project_name}}"
genfile .file.add path::"config.yaml" \
  content::"project:\n  name: {{project_name}}\n  version: {{version}}"
```

### Referenced Type

| Type | Kind | Fundamental | Key Constraint |
|------|------|-------------|----------------|
| [ContentString](type.md#type--9-contentstring) | String | string | Any UTF-8; mutually exclusive with from_file:: |

### Referenced Commands

| # | Command | Default | Notes |
|---|---------|---------|-------|
| 12 | [`.file.add`](command/file.md#command--12-fileadd) | — | Mutually exclusive with from_file:: |

---

### Parameter Interaction Matrix

| Parameter 1 | Parameter 2 | Relationship | Behavior |
|-------------|-------------|--------------|----------|
| `include_pattern::` | `exclude_pattern::` | Cooperative | Exclusions applied after inclusions |
| `content::` | `from_file::` | Conflict | Use one, not both in `.file.add` |
| `mode::inline` | `from_file::` | Enhancement | Inline mode embeds from_file content |
| `dry::1` | `verbosity::2+` | Enhancement | Detailed preview output |
| `format::json` | `pretty::1` | Enhancement | Pretty-printed JSON |
