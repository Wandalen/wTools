# Types Reference

Type system documentation for genfile CLI parameters.

### Scope

- **In Scope:** Semantic type definitions, validation rules, and parsing behavior for all genfile CLI parameter types
- **Out of Scope:** Rust implementation details — see `src/` for source code
- **Audience:** Implementers and developers extending or integrating genfile
- **Responsibility:** Authoritative type specification backing [param.md](param.md)

### Types Index

| # | Type | Purpose | Fundamental | Constraints |
|---|------|---------|-------------|-------------|
| 1 | [VerbosityLevel](#type--1-verbositylevel) | Output detail control | integer | 0–5 range |
| 2 | [DryRunFlag](#type--2-dryrunflag) | Preview mode flag | boolean | 0 or 1 |
| 3 | [FilePath](#type--3-filepath) | File system path | path | Valid UTF-8 |
| 4 | [OutputPath](#type--4-outputpath) | Writable output path | path | Writable parent |
| 5 | [DirectoryPath](#type--5-directorypath) | Directory path | path | Must exist as dir |
| 6 | [IdentifierString](#type--6-identifierstring) | Entity identifier | string | Alphanumeric+underscore |
| 7 | [DescriptionText](#type--7-descriptiontext) | Description text | string | Any UTF-8 |
| 8 | [PatternString](#type--8-patternstring) | Glob pattern | string | Valid glob syntax |
| 9 | [ContentString](#type--9-contentstring) | Content data | string | Any UTF-8 |
| 10 | [ContentMode](#type--10-contentmode) | Storage strategy | enum | inline \| reference |
| 11 | [SerializationFormat](#type--11-serializationformat) | Data format | enum | json \| yaml |
| 12 | [WriteMode](#type--12-writemode) | Write behavior | enum | rewrite \| append \| skip |
| 13 | [RecursiveFlag](#type--13-recursiveflag) | Traversal flag | boolean | 0 or 1 |
| 14 | [PrettyPrintFlag](#type--14-prettyprintflag) | Formatting flag | boolean | 0 or 1 |
| 15 | [MandatoryFlag](#type--15-mandatoryflag) | Requirement flag | boolean | 0 or 1 |

---

### Type :: 1. VerbosityLevel

**Purpose:** Type-safe verbosity control with range validation. Prevents invalid values and provides named predicates for threshold checks.

**Fundamental Type:** integer

**Constants:**
- `0` — Silent: errors only (for scripting and CI/CD)
- `1` — Normal: summary output (default)
- `2` — Verbose: detailed progress and results
- `3` — Debug: internal operations and decisions
- `4` — Trace: function calls and data flow
- `5` — Ultra-trace: all events and state changes

**Constraints:** Value must be in range 0–5 inclusive. Construction fails for out-of-range values.

**Parsing:** Accepts decimal integer strings in the range 0–5. Rejects non-integer input and out-of-range values.

**Methods:** Provides `is_silent()` (0), `is_normal()` (1), `is_verbose()` (≥2), `is_debug()` (≥3) predicates and a raw integer accessor.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 1 | [`verbosity::`](param.md#parameter--1-verbosity) | all 24 commands |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 1 | [`.info`](command/operations.md#command--1-info) | `verbosity::` |
| 2 | [`.discover.parameters`](command/operations.md#command--2-discoverparameters) | `verbosity::` |
| 3 | [`.status`](command/operations.md#command--3-status) | `verbosity::` |
| 4 | [`.analyze`](command/operations.md#command--4-analyze) | `verbosity::` |
| 5 | [`.archive.new`](command/archive.md#command--5-archivenew) | `verbosity::` |
| 6 | [`.archive.load`](command/archive.md#command--6-archiveload) | `verbosity::` |
| 7 | [`.archive.save`](command/archive.md#command--7-archivesave) | `verbosity::` |
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | `verbosity::` |
| 9 | [`.content.internalize`](command/content.md#command--9-contentinternalize) | `verbosity::` |
| 10 | [`.content.externalize`](command/content.md#command--10-contentexternalize) | `verbosity::` |
| 11 | [`.content.list`](command/content.md#command--11-contentlist) | `verbosity::` |
| 12 | [`.file.add`](command/file.md#command--12-fileadd) | `verbosity::` |
| 13 | [`.file.remove`](command/file.md#command--13-fileremove) | `verbosity::` |
| 14 | [`.file.list`](command/file.md#command--14-filelist) | `verbosity::` |
| 15 | [`.file.show`](command/file.md#command--15-fileshow) | `verbosity::` |
| 16 | [`.materialize`](command/operations.md#command--16-materialize) | `verbosity::` |
| 17 | [`.unpack`](command/operations.md#command--17-unpack) | `verbosity::` |
| 18 | [`.pack`](command/operations.md#command--18-pack) | `verbosity::` |
| 19 | [`.parameter.add`](command/param_mgmt.md#command--19-parameteradd) | `verbosity::` |
| 20 | [`.parameter.list`](command/param_mgmt.md#command--20-parameterlist) | `verbosity::` |
| 21 | [`.parameter.remove`](command/param_mgmt.md#command--21-parameterremove) | `verbosity::` |
| 22 | [`.value.set`](command/value.md#command--22-valueset) | `verbosity::` |
| 23 | [`.value.list`](command/value.md#command--23-valuelist) | `verbosity::` |
| 24 | [`.value.clear`](command/value.md#command--24-valueclear) | `verbosity::` |

---

### Type :: 2. DryRunFlag

**Purpose:** Type-safe dry-run flag. Ensures type safety for preview-mode operations.

**Fundamental Type:** boolean

**Constants:**
- `0` / `false` — Execute: perform the real operation (default)
- `1` / `true` — Preview: simulate without making changes

**Constraints:** Must be `0`, `1`, `false`, or `true`. Rejects all other strings.

**Parsing:** Accepts `0`, `false`, `1`, `true` (case-insensitive). Rejects all other values.

**Methods:** Provides `is_dry_run()` and `is_execution()` predicates.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 2 | [`dry::`](param.md#parameter--2-dry) | write operations only |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 7 | [`.archive.save`](command/archive.md#command--7-archivesave) | `dry::` |
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | `dry::` |
| 9 | [`.content.internalize`](command/content.md#command--9-contentinternalize) | `dry::` |
| 16 | [`.materialize`](command/operations.md#command--16-materialize) | `dry::` |
| 17 | [`.unpack`](command/operations.md#command--17-unpack) | `dry::` |
| 18 | [`.pack`](command/operations.md#command--18-pack) | `dry::` |
| 24 | [`.value.clear`](command/value.md#command--24-valueclear) | `dry::` |

---

### Type :: 3. FilePath

**Purpose:** Type-safe file paths with UTF-8 validation. No existence requirement at construction time — validation is context-dependent.

**Fundamental Type:** path

**Constraints:**
- Must be valid UTF-8
- No existence requirement at parse time (checked at operation time)

**Parsing:** Accepts any valid UTF-8 string as a file path. Existence is not checked at construction.

**Methods:** Provides path accessor, existence check, and string conversion.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 3 | [`path::`](param.md#parameter--3-path) | `.archive.load`, `.archive.save`, `.file.add`, `.file.remove`, `.file.show` |
| 18 | [`from_file::`](param.md#parameter--18-from_file) | `.file.add` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 6 | [`.archive.load`](command/archive.md#command--6-archiveload) | `path::` |
| 7 | [`.archive.save`](command/archive.md#command--7-archivesave) | `path::` |
| 12 | [`.file.add`](command/file.md#command--12-fileadd) | `path::`, `from_file::` |
| 13 | [`.file.remove`](command/file.md#command--13-fileremove) | `path::` |
| 15 | [`.file.show`](command/file.md#command--15-fileshow) | `path::` |

---

### Type :: 4. OutputPath

**Purpose:** Type-safe output paths with writability validation. Parent directory must be writable or creatable.

**Fundamental Type:** path

**Constraints:**
- Must be valid UTF-8
- Parent directory must be writable or creatable at operation time

**Parsing:** Accepts any valid UTF-8 string as a path. Writability is validated at operation time, not at parse time.

**Methods:** Provides path accessor and string conversion.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 5 | [`destination::`](param.md#parameter--5-destination) | `.materialize`, `.unpack` |
| 12 | [`output_dir::`](param.md#parameter--12-output_dir) | `.content.externalize` |
| 13 | [`output::`](param.md#parameter--13-output) | `.pack` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 10 | [`.content.externalize`](command/content.md#command--10-contentexternalize) | `output_dir::` |
| 16 | [`.materialize`](command/operations.md#command--16-materialize) | `destination::` |
| 17 | [`.unpack`](command/operations.md#command--17-unpack) | `destination::` |
| 18 | [`.pack`](command/operations.md#command--18-pack) | `output::` |

---

### Type :: 5. DirectoryPath

**Purpose:** Type-safe directory paths that must exist at parse time. Validates both existence and directory nature.

**Fundamental Type:** path

**Constraints:**
- Must be valid UTF-8
- Must exist at parse time
- Must be a directory (not a file)

**Parsing:** Validates that the path exists and is a directory. Rejects non-existent paths and paths pointing to files.

**Methods:** Provides path accessor and string conversion.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 9 | [`source::`](param.md#parameter--9-source) | `.archive.from_directory` |
| 16 | [`input::`](param.md#parameter--16-input) | `.pack` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | `source::` |
| 18 | [`.pack`](command/operations.md#command--18-pack) | `input::` |

---

### Type :: 6. IdentifierString

**Purpose:** Type-safe identifiers with alphanumeric+underscore validation. Prevents identifiers with spaces or special characters.

**Fundamental Type:** string

**Constraints:**
- Non-empty
- Alphanumeric characters and underscores only
- No spaces, hyphens, or other special characters

**Parsing:** Accepts non-empty strings containing only alphanumeric characters and underscores. Rejects empty strings and strings with invalid characters.

**Methods:** Provides string accessor.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 4 | [`name::`](param.md#parameter--4-name) | `.archive.new`, `.parameter.add`, `.parameter.remove`, `.value.set` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 5 | [`.archive.new`](command/archive.md#command--5-archivenew) | `name::` |
| 19 | [`.parameter.add`](command/param_mgmt.md#command--19-parameteradd) | `name::` |
| 21 | [`.parameter.remove`](command/param_mgmt.md#command--21-parameterremove) | `name::` |
| 22 | [`.value.set`](command/value.md#command--22-valueset) | `name::` |

---

### Type :: 7. DescriptionText

**Purpose:** Type-safe descriptions with no character restrictions. Accepts any UTF-8 text including multiline.

**Fundamental Type:** string

**Constraints:** Any UTF-8 text (empty allowed).

**Parsing:** Accepts any UTF-8 string, including empty strings.

**Methods:** Provides string accessor.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 6 | [`description::`](param.md#parameter--6-description) | `.archive.new`, `.parameter.add` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 5 | [`.archive.new`](command/archive.md#command--5-archivenew) | `description::` |
| 19 | [`.parameter.add`](command/param_mgmt.md#command--19-parameteradd) | `description::` |

---

### Type :: 8. PatternString

**Purpose:** Type-safe glob patterns with syntax validation. Ensures patterns are well-formed before use.

**Fundamental Type:** string

**Constraints:** Must be valid glob pattern syntax. Rejects malformed patterns at parse time.

**Parsing:** Validates input as a legal glob pattern at parse time. Rejects malformed patterns.

**Methods:** Provides pattern accessor and match predicate.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 17 | [`include_pattern::`](param.md#parameter--17-include_pattern) | `.archive.from_directory` |
| 20 | [`filter::`](param.md#parameter--20-filter) | `.content.list` |
| 21 | [`exclude_pattern::`](param.md#parameter--21-exclude_pattern) | `.archive.from_directory` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | `include_pattern::`, `exclude_pattern::` |
| 11 | [`.content.list`](command/content.md#command--11-contentlist) | `filter::` |

---

### Type :: 9. ContentString

**Purpose:** Type-safe content strings with no constraints. Accepts any UTF-8 text for template file content, parameter values, and default values.

**Fundamental Type:** string

**Constraints:** Any UTF-8 text, including multiline content.

**Parsing:** Accepts any UTF-8 string.

**Methods:** Provides string accessor.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 8 | [`value::`](param.md#parameter--8-value) | `.value.set` |
| 22 | [`default::`](param.md#parameter--22-default) | `.parameter.add` |
| 23 | [`content::`](param.md#parameter--23-content) | `.file.add` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 12 | [`.file.add`](command/file.md#command--12-fileadd) | `content::` |
| 19 | [`.parameter.add`](command/param_mgmt.md#command--19-parameteradd) | `default::` |
| 22 | [`.value.set`](command/value.md#command--22-valueset) | `value::` |

---

### Type :: 10. ContentMode

**Purpose:** Type-safe content mode controlling whether file content is embedded inline or stored as a file path reference.

**Fundamental Type:** enum

**Constants:**
- `inline` — File content embedded directly in the archive (portable; larger archive size)
- `reference` — Archive stores file paths only (requires source files at operation time; default)

**Parsing:** Accepts `inline` and `reference` (case-insensitive). Rejects all other values.

**Methods:** Provides `is_inline()` and `is_reference()` predicates.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 14 | [`mode::`](param.md#parameter--14-mode) | `.archive.from_directory` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | `mode::` |

---

### Type :: 11. SerializationFormat

**Purpose:** Type-safe serialization format for archive persistence. Supports JSON and YAML with auto-detection from file extension.

**Fundamental Type:** enum

**Constants:**
- `json` — JSON encoding (compact or pretty-printed via `pretty::`)
- `yaml` — YAML encoding (always human-readable)

**Parsing:** Accepts `json`, `yaml`, and `yml` (case-insensitive). Also infers format from file extension when a path context is available. Rejects all other values.

**Methods:** Provides `is_json()` and `is_yaml()` predicates.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 19 | [`format::`](param.md#parameter--19-format) | `.archive.save` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 7 | [`.archive.save`](command/archive.md#command--7-archivesave) | `format::` |

---

### Type :: 12. WriteMode

**Purpose:** Type-safe write mode controlling file write behavior when the target file already exists.

**Fundamental Type:** enum

**Constants:**
- `rewrite` — Overwrite existing file (default when applicable)
- `append` — Append content to existing file
- `skip` — Leave existing file unchanged (no-op for existing files)

**Parsing:** Accepts `rewrite` (or `overwrite`), `append`, and `skip` (case-insensitive). Rejects all other values.

**Methods:** Provides `is_rewrite()`, `is_append()`, and `is_skip()` predicates.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 7 | [`write_mode::`](param.md#parameter--7-write_mode) | `.file.add` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 12 | [`.file.add`](command/file.md#command--12-fileadd) | `write_mode::` |

---

### Type :: 13. RecursiveFlag

**Purpose:** Type-safe recursive traversal flag controlling whether directory scans descend into subdirectories.

**Fundamental Type:** boolean

**Constants:**
- `0` / `false` — Flat: scan current directory only
- `1` / `true` — Recursive: descend into subdirectories (default)

**Constraints:** Must be `0`, `1`, `false`, or `true`. Rejects all other strings.

**Parsing:** Same as [DryRunFlag](#type--2-dryrunflag).

**Methods:** Provides `is_recursive()` and `is_flat()` predicates.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 10 | [`recursive::`](param.md#parameter--10-recursive) | `.archive.from_directory` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 8 | [`.archive.from_directory`](command/archive.md#command--8-archivefrom_directory) | `recursive::` |

---

### Type :: 14. PrettyPrintFlag

**Purpose:** Type-safe pretty-print flag controlling JSON formatting. Applies only to JSON output; YAML is always formatted.

**Fundamental Type:** boolean

**Constants:**
- `0` / `false` — Compact: no extra whitespace
- `1` / `true` — Pretty: human-readable indented output (default)

**Constraints:** Must be `0`, `1`, `false`, or `true`. Rejects all other strings.

**Parsing:** Same as [DryRunFlag](#type--2-dryrunflag).

**Methods:** Provides `is_pretty()` and `is_compact()` predicates.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 11 | [`pretty::`](param.md#parameter--11-pretty) | `.archive.save` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 7 | [`.archive.save`](command/archive.md#command--7-archivesave) | `pretty::` |

---

### Type :: 15. MandatoryFlag

**Purpose:** Type-safe mandatory flag controlling whether a parameter must have a value set before materialization can proceed.

**Fundamental Type:** boolean

**Constants:**
- `0` / `false` — Optional: materialization proceeds even without a value (using default if available)
- `1` / `true` — Mandatory: materialization fails if no value is set and no default is defined (default)

**Constraints:** Must be `0`, `1`, `false`, or `true`. Rejects all other strings.

**Parsing:** Same as [DryRunFlag](#type--2-dryrunflag).

**Methods:** Provides `is_mandatory()` and `is_optional()` predicates.

### Referenced Parameters

| # | Parameter | Commands |
|---|-----------|----------|
| 15 | [`mandatory::`](param.md#parameter--15-mandatory) | `.parameter.add` |

### Referenced Commands

| # | Command | Via Parameter |
|---|---------|---------------|
| 19 | [`.parameter.add`](command/param_mgmt.md#command--19-parameteradd) | `mandatory::` |
