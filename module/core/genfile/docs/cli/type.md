# Types Reference

Type system documentation for genfile CLI parameters.

### Scope

- **In Scope:** Semantic type definitions, validation rules, and parsing behavior for all genfile CLI parameter types
- **Out of Scope:** Rust implementation details — see `src/` for source code
- **Audience:** Implementers and developers extending or integrating genfile
- **Responsibility:** Authoritative type specification backing [param.md](param.md)

### Quick Navigation

**By Category:**
- [Integer Types](#category-integer-types) — Constrained numeric types
- [Boolean Types](#category-boolean-types) — Flag types
- [String Types](#category-string-types) — Text types with validation
- [Path Types](#category-path-types) — Filesystem path types
- [Enum Types](#category-enum-types) — Multiple-choice types

### Types Index

| # | Type | Purpose | Fundamental | Constraints | Used By |
|---|------|---------|-------------|-------------|---------|
| 1 | [VerbosityLevel](#type-verbositylevel) | Output detail control | integer | 0–5 range | verbosity:: |
| 2 | [DryRunFlag](#type-dryrunflag) | Preview mode flag | boolean | 0 or 1 | dry:: |
| 3 | [FilePath](#type-filepath) | File system path | path | Valid UTF-8 | path::, from_file:: |
| 4 | [OutputPath](#type-outputpath) | Writable output path | path | Writable parent | destination::, output_dir::, output:: |
| 5 | [DirectoryPath](#type-directorypath) | Directory path | path | Must exist as dir | source::, input:: |
| 6 | [IdentifierString](#type-identifierstring) | Entity identifier | string | Alphanumeric+underscore | name:: |
| 7 | [DescriptionText](#type-descriptiontext) | Description text | string | Any UTF-8 | description:: |
| 8 | [PatternString](#type-patternstring) | Glob pattern | string | Valid glob syntax | include_pattern::, exclude_pattern::, filter:: |
| 9 | [ContentString](#type-contentstring) | Content data | string | Any UTF-8 | value::, default::, content:: |
| 10 | [ContentMode](#type-contentmode) | Storage strategy | enum | inline \| reference | mode:: |
| 11 | [SerializationFormat](#type-serializationformat) | Data format | enum | json \| yaml | format:: |
| 12 | [WriteMode](#type-writemode) | Write behavior | enum | rewrite \| append \| skip | write_mode:: |
| 13 | [RecursiveFlag](#type-recursiveflag) | Traversal flag | boolean | 0 or 1 | recursive:: |
| 14 | [PrettyPrintFlag](#type-prettyprintflag) | Formatting flag | boolean | 0 or 1 | pretty:: |
| 15 | [MandatoryFlag](#type-mandatoryflag) | Requirement flag | boolean | 0 or 1 | mandatory:: |

---

### Category: Integer Types

Numeric types with constrained ranges.

| Type | Range | Default | Purpose |
|------|-------|---------|---------|
| [VerbosityLevel](#type-verbositylevel) | 0–5 | 1 | Output verbosity |

### Category: Boolean Types

Binary flag types accepting `0`/`1` or `false`/`true`.

| Type | Default | Purpose |
|------|---------|---------|
| [DryRunFlag](#type-dryrunflag) | 0 | Preview mode |
| [RecursiveFlag](#type-recursiveflag) | 1 | Directory traversal |
| [PrettyPrintFlag](#type-prettyprintflag) | 1 | JSON formatting |
| [MandatoryFlag](#type-mandatoryflag) | 0 | Parameter requirement |

### Category: String Types

Text types with validation.

| Type | Validation | Purpose |
|------|------------|---------|
| [IdentifierString](#type-identifierstring) | Alphanumeric+underscore | Entity names |
| [DescriptionText](#type-descriptiontext) | Any UTF-8 | Descriptions |
| [PatternString](#type-patternstring) | Valid glob | File patterns |
| [ContentString](#type-contentstring) | Any UTF-8 | Content data |

### Category: Path Types

Filesystem path types.

| Type | Validation | Purpose |
|------|------------|---------|
| [FilePath](#type-filepath) | Valid path | File references |
| [OutputPath](#type-outputpath) | Writable parent | Output targets |
| [DirectoryPath](#type-directorypath) | Existing directory | Input sources |

### Category: Enum Types

Multiple-choice types.

| Type | Options | Purpose |
|------|---------|---------|
| [ContentMode](#type-contentmode) | inline \| reference | Storage strategy |
| [SerializationFormat](#type-serializationformat) | json \| yaml | Data format |
| [WriteMode](#type-writemode) | rewrite \| append \| skip | Write behavior |

---

### Type :: `VerbosityLevel`

Semantic integer type representing CLI output verbosity with a constrained range. Prevents invalid values and provides named predicates for threshold checks.

- **Purpose:** Type-safe verbosity control with range validation
- **Fundamental Type:** integer

**Valid Values:** 0 (silent — errors only) to 5 (ultra-trace). Default: 1 (normal output).

**Constraints:**
- Value must be in range 0–5 inclusive
- Construction fails for out-of-range values

**Parsing:** Accepts decimal integer strings in the range 0–5. Rejects non-integer input and out-of-range values.

**Behavior:** Provides semantic predicates — silent (0), normal (1), verbose (≥2), debug (≥3) — and a raw integer accessor.

**Used By:** [verbosity::](param.md#parameter--1-verbosity) parameter (all commands)

---

### Type :: `DryRunFlag`

Boolean type for preview mode control. Ensures type safety for dry-run operations.

- **Purpose:** Type-safe dry-run flag
- **Fundamental Type:** boolean

**Valid Values:** `0`/`false` (execute) or `1`/`true` (preview). Default: `0` (execute).

**Parsing:** Accepts `0`, `false`, `1`, `true`. Rejects all other strings.

**Behavior:** Provides `is_dry_run()` and `is_execution()` predicates.

**Used By:** [dry::](param.md#parameter--2-dry) parameter (write operations only)

---

### Type :: `FilePath`

Path type for file system paths with UTF-8 validation.

- **Purpose:** Type-safe file paths
- **Fundamental Type:** path

**Constraints:**
- Must be valid UTF-8
- No existence requirement at parse time (context-dependent)

**Parsing:** Accepts any string as a file path. Existence is not checked at construction time.

**Behavior:** Provides path accessor, existence check, and string conversion.

**Used By:** [path::](param.md#parameter--3-path), [from_file::](param.md#parameter--18-from_file)

---

### Type :: `OutputPath`

Path type for output locations with writability validation.

- **Purpose:** Type-safe output paths
- **Fundamental Type:** path

**Constraints:**
- Must be valid UTF-8
- Parent directory must be writable or creatable

**Parsing:** Accepts any string as a path. Writability of the parent directory is validated at operation time, not at parse time.

**Used By:** [destination::](param.md#parameter--5-destination), [output_dir::](param.md#parameter--12-output_dir), [output::](param.md#parameter--13-output)

---

### Type :: `DirectoryPath`

Path type for directory paths that must exist at parse time.

- **Purpose:** Type-safe directory paths
- **Fundamental Type:** path

**Constraints:**
- Must be valid UTF-8
- Must exist at parse time
- Must be a directory (not a file)

**Parsing:** Validates that the path exists and is a directory. Rejects non-existent paths and paths pointing to files.

**Used By:** [source::](param.md#parameter--9-source), [input::](param.md#parameter--16-input)

---

### Type :: `IdentifierString`

String type for identifiers with alphanumeric+underscore validation.

- **Purpose:** Type-safe identifiers
- **Fundamental Type:** string

**Constraints:**
- Non-empty
- Alphanumeric characters and underscores only
- No spaces, hyphens, or other special characters

**Parsing:** Accepts non-empty strings containing only alphanumeric characters and underscores. Rejects empty strings and strings with invalid characters.

**Used By:** [name::](param.md#parameter--4-name)

---

### Type :: `DescriptionText`

String type for description text with no character restrictions.

- **Purpose:** Type-safe descriptions
- **Fundamental Type:** string

**Constraints:** Any UTF-8 text (empty allowed).

**Parsing:** Accepts any UTF-8 string, including empty strings.

**Used By:** [description::](param.md#parameter--6-description)

---

### Type :: `PatternString`

String type for glob patterns with syntax validation.

- **Purpose:** Type-safe glob patterns
- **Fundamental Type:** string

**Constraints:** Must be valid glob pattern syntax.

**Parsing:** Validates input as a legal glob pattern at parse time. Rejects malformed patterns.

**Used By:** [include_pattern::](param.md#parameter--17-include_pattern), [exclude_pattern::](param.md#parameter--21-exclude_pattern), [filter::](param.md#parameter--20-filter)

---

### Type :: `ContentString`

String type for content data with no constraints.

- **Purpose:** Type-safe content strings
- **Fundamental Type:** string

**Constraints:** Any UTF-8 text, including multiline content.

**Parsing:** Accepts any UTF-8 string.

**Used By:** [value::](param.md#parameter--8-value), [default::](param.md#parameter--22-default), [content::](param.md#parameter--23-content)

---

### Type :: `ContentMode`

Enum type for content storage strategy.

- **Purpose:** Type-safe content mode
- **Fundamental Type:** enum

**Valid Values:** `inline` (content embedded in archive) or `reference` (content stored as file paths). Default: `reference`.

**Parsing:** Accepts `inline` and `reference` (case-insensitive). Rejects all other values.

**Behavior:** Provides `is_inline()` and `is_reference()` predicates.

**Used By:** [mode::](param.md#parameter--14-mode)

---

### Type :: `SerializationFormat`

Enum type for archive serialization format.

- **Purpose:** Type-safe serialization format
- **Fundamental Type:** enum

**Valid Values:** `json` or `yaml`. Default: `json`.

**Parsing:** Accepts `json`, `yaml`, and `yml` (case-insensitive). Also infers format from file extension when a path is provided. Rejects all other values.

**Used By:** [format::](param.md#parameter--19-format)

---

### Type :: `WriteMode`

Enum type for file write behavior.

- **Purpose:** Type-safe write mode
- **Fundamental Type:** enum

**Valid Values:** `rewrite` (overwrite existing), `append` (add to existing), or `skip` (leave existing unchanged). No universal default — depends on command context.

**Parsing:** Accepts `rewrite`/`overwrite`, `append`, and `skip` (case-insensitive). Rejects all other values.

**Used By:** [write_mode::](param.md#parameter--7-write_mode)

---

### Type :: `RecursiveFlag`

Boolean type for recursive directory traversal control.

- **Purpose:** Type-safe recursive flag
- **Fundamental Type:** boolean

**Valid Values:** `0`/`false` (flat — no subdirectories) or `1`/`true` (recursive). Default: `1` (recursive).

**Parsing:** Same as [DryRunFlag](#type-dryrunflag).

**Used By:** [recursive::](param.md#parameter--10-recursive)

---

### Type :: `PrettyPrintFlag`

Boolean type for JSON formatting control.

- **Purpose:** Type-safe pretty-print flag
- **Fundamental Type:** boolean

**Valid Values:** `0`/`false` (compact) or `1`/`true` (pretty). Default: `1` (pretty).

**Parsing:** Same as [DryRunFlag](#type-dryrunflag).

**Used By:** [pretty::](param.md#parameter--11-pretty)

---

### Type :: `MandatoryFlag`

Boolean type for parameter requirement control.

- **Purpose:** Type-safe mandatory flag
- **Fundamental Type:** boolean

**Valid Values:** `0`/`false` (optional) or `1`/`true` (mandatory). Default: `0` (optional).

**Parsing:** Same as [DryRunFlag](#type-dryrunflag).

**Used By:** [mandatory::](param.md#parameter--15-mandatory)

---

### Type Safety Principles

Semantic newtypes prevent misuse that primitive types allow.

**Why Semantic Newtypes:**
1. **Prevents mixing incompatible values** — a function accepting `VerbosityLevel` and `DryRunFlag` cannot receive them in the wrong order; the compiler enforces correctness at build time
2. **Centralizes validation** — parse once at the input boundary; all downstream code receives already-validated values
3. **Self-documenting signatures** — function signatures show semantic intent, not raw primitive types
4. **Compile-time safety** — type mismatches are caught at build time, not at runtime

### See Also

- [Parameters](param.md) — Parameter specifications
- [Commands](command/readme.md) — Command documentation
- [Parameter Groups](param_group.md) — Shared parameter sets
