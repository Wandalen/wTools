# Parameters Reference

Complete parameter listing for the genfile CLI.

### Quick Navigation

**By Category:**
- [Universal Parameters](#category-universal) - Used across all/most commands
- [I/O Paths](#category-io-paths) - File and directory paths
- [Entity Management](#category-entity-management) - Names, descriptions, values
- [Filtering](#category-filtering) - Pattern-based selection
- [Flags](#category-flags) - Boolean configuration
- [Enums](#category-enums) - Multiple-choice options

**By Frequency:**
- Universal (all commands): [verbosity::](#parameter--1-verbosity)
- Common (6 commands): [dry::](#parameter--2-dry)
- Moderate (3-5 commands): [path::](#parameter--3-path), [name::](#parameter--4-name)
- Command-specific (1-2 commands): All others

### Parameters Index

| # | Parameter | Type | Default | Valid Values | Purpose | Used In |
|---|-----------|------|---------|--------------|---------|---------|
| 1 | [`verbosity::`](#parameter--1-verbosity) | [VerbosityLevel](type.md#type--verbositylevel) | `1` | 0 to 5 | Output detail control | all cmds |
| 2 | [`dry::`](#parameter--2-dry) | [DryRunFlag](type.md#type--dryrunflag) | `0` | 0 or 1 | Preview mode flag | 6 cmds |
| 3 | [`path::`](#parameter--3-path) | [FilePath](type.md#type--filepath) | — | Any path | File path (input/output) | 5 cmds |
| 4 | [`name::`](#parameter--4-name) | [IdentifierString](type.md#type--identifierstring) | — | Alphanumeric+_ | Entity identifier | 4 cmds |
| 5 | [`destination::`](#parameter--5-destination) | [OutputPath](type.md#type--outputpath) | — | Any path | Output directory path | 2 cmds |
| 6 | [`description::`](#parameter--6-description) | [DescriptionText](type.md#type--descriptiontext) | `""` | Any text | Human-readable description | 2 cmds |
| 7 | [`write_mode::`](#parameter--7-write_mode) | [WriteMode](type.md#type--writemode) | — | 3 options | File conflict resolution | 1 cmd |
| 8 | [`value::`](#parameter--8-value) | [ContentString](type.md#type--contentstring) | — | Any text | Parameter value data | 1 cmd |
| 9 | [`source::`](#parameter--9-source) | [DirectoryPath](type.md#type--directorypath) | — | Any path | Source directory path | 1 cmd |
| 10 | [`recursive::`](#parameter--10-recursive) | [RecursiveFlag](type.md#type--recursiveflag) | `1` | 0 or 1 | Subdirectory traversal | 1 cmd |
| 11 | [`pretty::`](#parameter--11-pretty) | [PrettyPrintFlag](type.md#type--prettyprintflag) | `1` | 0 or 1 | JSON formatting | 1 cmd |
| 12 | [`output_dir::`](#parameter--12-output_dir) | [OutputPath](type.md#type--outputpath) | — | Any path | Output directory | 1 cmd |
| 13 | [`output::`](#parameter--13-output) | [OutputPath](type.md#type--outputpath) | — | Any path | Output file path | 1 cmd |
| 14 | [`mode::`](#parameter--14-mode) | [ContentMode](type.md#type--contentmode) | `reference` | 2 options | Content storage strategy | 1 cmd |
| 15 | [`mandatory::`](#parameter--15-mandatory) | [MandatoryFlag](type.md#type--mandatoryflag) | `0` | 0 or 1 | Parameter requirement flag | 1 cmd |
| 16 | [`input::`](#parameter--16-input) | [DirectoryPath](type.md#type--directorypath) | — | Any path | Input directory path | 1 cmd |
| 17 | [`include_pattern::`](#parameter--17-include_pattern) | [PatternString](type.md#type--patternstring) | `null` | Glob syntax | Inclusion glob pattern | 1 cmd |
| 18 | [`from_file::`](#parameter--18-from_file) | [FilePath](type.md#type--filepath) | `null` | Any path | Source file path | 1 cmd |
| 19 | [`format::`](#parameter--19-format) | [SerializationFormat](type.md#type--serializationformat) | `json` | 2 options | Serialization format | 1 cmd |
| 20 | [`filter::`](#parameter--20-filter) | [PatternString](type.md#type--patternstring) | `null` | Glob syntax | File filtering pattern | 1 cmd |
| 21 | [`exclude_pattern::`](#parameter--21-exclude_pattern) | [PatternString](type.md#type--patternstring) | `null` | Glob syntax | Exclusion glob pattern | 1 cmd |
| 22 | [`default::`](#parameter--22-default) | [ContentString](type.md#type--contentstring) | `null` | Any text | Parameter default value | 1 cmd |
| 23 | [`content::`](#parameter--23-content) | [ContentString](type.md#type--contentstring) | — | Any text | File content data | 1 cmd |

### Parameter Categories

### Category: Universal

Parameters used across all or most commands. Part of standard parameter groups.

| Parameter | Usage | Group |
|-----------|-------|-------|
| [verbosity::](#parameter--1-verbosity) | all commands | [Universal Output Control](param_group.md#group--1-universal-output-control) |
| [dry::](#parameter--2-dry) | 6 write commands | [Universal Execution Control](param_group.md#group--2-universal-execution-control) |

### Category: I/O Paths

Filesystem paths for reading or writing data.

| Parameter | Purpose | Type |
|-----------|---------|------|
| [path::](#parameter--3-path) | Generic file path | Input/Output |
| [destination::](#parameter--5-destination) | Output directory | Output |
| [source::](#parameter--9-source) | Source directory | Input |
| [output_dir::](#parameter--12-output_dir) | Output directory | Output |
| [output::](#parameter--13-output) | Output file | Output |
| [input::](#parameter--16-input) | Input directory | Input |
| [from_file::](#parameter--18-from_file) | Source file | Input |

### Category: Entity Management

Parameters for managing entities (names, descriptions, values).

| Parameter | Purpose |
|-----------|---------|
| [name::](#parameter--4-name) | Entity identifier |
| [description::](#parameter--6-description) | Entity description |
| [value::](#parameter--8-value) | Parameter value |
| [default::](#parameter--22-default) | Parameter default |
| [content::](#parameter--23-content) | File content |

### Category: Filtering

Pattern-based filtering for file selection.

| Parameter | Purpose | Pattern Type |
|-----------|---------|--------------|
| [include_pattern::](#parameter--17-include_pattern) | Include files | Glob |
| [filter::](#parameter--20-filter) | Filter files | Glob |
| [exclude_pattern::](#parameter--21-exclude_pattern) | Exclude files | Glob |

### Category: Flags

Boolean configuration flags.

| Parameter | Purpose | Default |
|-----------|---------|---------|
| [recursive::](#parameter--10-recursive) | Subdirectory traversal | `1` (enabled) |
| [pretty::](#parameter--11-pretty) | JSON formatting | `1` (enabled) |
| [mandatory::](#parameter--15-mandatory) | Parameter requirement | `0` (optional) |

### Category: Enums

Multiple-choice parameters with fixed options.

| Parameter | Options | Default |
|-----------|---------|---------|
| [write_mode::](#parameter--7-write_mode) | rewrite \| append \| skip | — |
| [mode::](#parameter--14-mode) | inline \| reference | `reference` |
| [format::](#parameter--19-format) | json \| yaml | `json` |

---

### Parameter Specifications

### Parameter :: 1. `verbosity::`

Controls output verbosity level across all commands using 0-5 scale. Higher values show more detailed information, lower values show minimal output.

- **Type:** [VerbosityLevel](type.md#type--verbositylevel)
- **Default:** `1` (normal output)
- **Required In:** None (always optional)
- **Part Of:** [Universal Output Control](param_group.md#group--1-universal-output-control) parameter group

### Constants

- `0` - Silent (errors only)
- `1` - Normal (default — summary output)
- `2` - Verbose (detailed progress)
- `3` - Debug (internal operations)
- `4` - Trace (function calls)
- `5` - Ultra-trace (all events)

### Validation

- Must be integer in range 0-5
- Invalid values rejected with error

### Examples

```bash
genfile .archive.save path::"out.json" verbosity::0  # Silent
genfile .archive.load path::"template.yaml" verbosity::1  # Normal
genfile .materialize destination::"./output" verbosity::2  # Verbose
```

### Referenced Commands

All commands accept `verbosity::`.

---

### Parameter :: 2. `dry::`

Enables preview mode where operations show what would happen without making actual changes. Essential for validating operations before execution.

- **Type:** [DryRunFlag](type.md#type--dryrunflag)
- **Default:** `0` (disabled — real execution)
- **Required In:** None (always optional)
- **Part Of:** [Universal Execution Control](param_group.md#group--2-universal-execution-control) parameter group

### Behavior

- `0` — Execute operation normally (default)
- `1` — Show preview without changes

### Validation

Must be `0` (false) or `1` (true).

### Examples

```bash
genfile .archive.save path::"test.json" dry::1  # Preview only
genfile .materialize destination::"./output" dry::1 verbosity::2  # Detailed preview
```

### Referenced Commands

| # | Command | File |
|---|---------|------|
| 7 | [.archive.save](command/archive.md) | archive.md |
| 8 | [.archive.from_directory](command/archive.md) | archive.md |
| 9 | [.content.internalize](command/content.md) | content.md |
| 16 | [.materialize](command/operations.md) | operations.md |
| 17 | [.unpack](command/operations.md) | operations.md |
| 18 | [.pack](command/operations.md) | operations.md |

---

### Parameter :: 3. `path::`

Generic file path for reading or writing archive files. Supports both input and output operations depending on command context.

- **Type:** [FilePath](type.md#type--filepath)
- **Default:** None (context-dependent)
- **Required In:** `.archive.load`, `.archive.save`, `.file.add`, `.file.show`, `.file.remove`

### Context-Dependent Behavior

- **In `.archive.load`:** Path to existing archive file
- **In `.archive.save`:** Path where archive will be saved
- **In `.file.*`:** Path within archive structure (not filesystem path)

### Validation

- Must be valid UTF-8 path string
- For input: file must exist and be readable
- For output: parent directory must be writable

### Examples

```bash
genfile .archive.load path::"./archives/template.json"
genfile .archive.save path::"./output/archive.yaml"
genfile .file.add path::"src/main.rs" content::"..."
```

---

### Parameter :: 4. `name::`

Entity identifier for archives and parameters. Must be valid identifier (alphanumeric + underscore).

- **Type:** [IdentifierString](type.md#type--identifierstring)
- **Default:** None (required when used)
- **Required In:** `.archive.new`, `.parameter.add`, `.parameter.remove`, `.value.set`

### Validation

Alphanumeric + underscore only, no spaces.

### Examples

```bash
genfile .archive.new name::"my_template"
genfile .parameter.add name::project_name mandatory::true
genfile .value.set name::version value::"1.0.0"
```

---

### Parameter :: 5. `destination::`

Output directory path for materialized or unpacked files.

- **Type:** [OutputPath](type.md#type--outputpath)
- **Default:** None (required)
- **Required In:** `.materialize`, `.unpack`

### Validation

Must be valid path, writable.

### Examples

```bash
genfile .materialize destination::"./output"
genfile .unpack destination::"./template-files"
```

---

### Parameter :: 6. `description::`

Human-readable entity description for archives and parameters.

- **Type:** [DescriptionText](type.md#type--descriptiontext)
- **Default:** `""` (empty string)
- **Required In:** None (always optional)

### Validation

Any UTF-8 text.

### Examples

```bash
genfile .archive.new name::"template" description::"REST API template"
genfile .parameter.add name::author description::"Project author name"
```

---

### Parameter :: 7. `write_mode::`

File conflict resolution mode when adding files to archive.

- **Type:** [WriteMode](type.md#type--writemode)
- **Default:** None (command-specific)
- **Required In:** None (optional)

### Constants

- `rewrite` — Overwrite existing file
- `append` — Append to existing file
- `skip` — Skip if file exists

### Examples

```bash
genfile .file.add path::"config.toml" content::"..." write_mode::rewrite
```

---

### Parameter :: 8. `value::`

Parameter value data for template substitution.

- **Type:** [ContentString](type.md#type--contentstring)
- **Default:** None (required)
- **Required In:** `.value.set`

### Validation

Any UTF-8 text.

### Examples

```bash
genfile .value.set name::project_name value::"my-app"
genfile .value.set name::version value::"1.0.0"
```

---

### Parameter :: 9. `source::`

Source directory path for scanning files.

- **Type:** [DirectoryPath](type.md#type--directorypath)
- **Default:** None (required)
- **Required In:** `.archive.from_directory`

### Validation

Must exist and be readable directory.

### Examples

```bash
genfile .archive.from_directory source::"./templates"
```

---

### Parameter :: 10. `recursive::`

Controls subdirectory traversal during filesystem scanning.

- **Type:** [RecursiveFlag](type.md#type--recursiveflag)
- **Default:** `1` (enabled)
- **Required In:** None (optional)
- **Part Of:** [Filesystem Filtering](param_group.md#group--3-filesystem-filtering) parameter group

### Validation

Must be `0` (false) or `1` (true).

### Examples

```bash
genfile .archive.from_directory source::"./src" recursive::1
genfile .archive.from_directory source::"./config" recursive::0
```

---

### Parameter :: 11. `pretty::`

Controls JSON pretty-printing (does not affect YAML).

- **Type:** [PrettyPrintFlag](type.md#type--prettyprintflag)
- **Default:** `1` (enabled)
- **Required In:** None (optional)

### Validation

Must be `0` (false) or `1` (true).

### Examples

```bash
genfile .archive.save path::"template.json" pretty::1
genfile .archive.save path::"compact.json" pretty::0
```

---

### Parameter :: 12. `output_dir::`

Output directory for externalized content.

- **Type:** [OutputPath](type.md#type--outputpath)
- **Default:** None (required)
- **Required In:** `.content.externalize`

### Validation

Must be writable.

### Examples

```bash
genfile .content.externalize output_dir::"./files"
```

---

### Parameter :: 13. `output::`

Output file path for packed archives.

- **Type:** [OutputPath](type.md#type--outputpath)
- **Default:** None (required)
- **Required In:** `.pack`

### Validation

Parent directory must be writable.

### Examples

```bash
genfile .pack input::"./templates" output::"template.json"
```

---

### Parameter :: 14. `mode::`

Content storage strategy (inline vs reference).

- **Type:** [ContentMode](type.md#type--contentmode)
- **Default:** `reference`
- **Required In:** None (optional)

### Constants

- `inline` — Embed content in archive
- `reference` — Store file paths only

### Examples

```bash
genfile .archive.from_directory source::"./src" mode::inline
genfile .archive.from_directory source::"./templates" mode::reference
```

---

### Parameter :: 15. `mandatory::`

Whether parameter is required for template materialization.

- **Type:** [MandatoryFlag](type.md#type--mandatoryflag)
- **Default:** `0` (optional)
- **Required In:** None (optional)

### Validation

Must be `0` (false) or `1` (true).

### Examples

```bash
genfile .parameter.add name::project_name mandatory::true
genfile .parameter.add name::author mandatory::false
```

---

### Parameter :: 16. `input::`

Input directory path for packing.

- **Type:** [DirectoryPath](type.md#type--directorypath)
- **Default:** None (required)
- **Required In:** `.pack`

### Validation

Must exist and be readable.

### Examples

```bash
genfile .pack input::"./templates" output::"archive.json"
```

---

### Parameter :: 17. `include_pattern::`

Glob pattern for file inclusion during directory scanning.

- **Type:** [PatternString](type.md#type--patternstring)
- **Default:** `null` (include all)
- **Required In:** None (optional)
- **Part Of:** [Filesystem Filtering](param_group.md#group--3-filesystem-filtering) parameter group

### Validation

Valid glob pattern.

### Examples

```bash
genfile .archive.from_directory source::"./src" include_pattern::"**/*.rs"
genfile .archive.from_directory source::"./docs" include_pattern::"**/*.{md,txt}"
```

---

### Parameter :: 18. `from_file::`

Source file path to read content from when adding file to archive.

- **Type:** [FilePath](type.md#type--filepath)
- **Default:** `null`
- **Required In:** None (conditional — must use `content::` OR `from_file::`)

### Interactions

Conflicts with `content::` — use one, not both in `.file.add`.

### Validation

File must exist and be readable.

### Examples

```bash
genfile .file.add path::"readme.md" from_file::"./README.md"
```

---

### Parameter :: 19. `format::`

Serialization format for archive persistence.

- **Type:** [SerializationFormat](type.md#type--serializationformat)
- **Default:** `json`
- **Required In:** None (optional)

### Constants

- `json` — JSON format
- `yaml` — YAML format

### Examples

```bash
genfile .archive.save path::"template.json" format::json
genfile .archive.save path::"template.yaml" format::yaml
```

---

### Parameter :: 20. `filter::`

Pattern for filtering file listings.

- **Type:** [PatternString](type.md#type--patternstring)
- **Default:** `null` (no filter)
- **Required In:** None (optional)

### Examples

```bash
genfile .content.list filter::inline
genfile .content.list filter::reference
```

---

### Parameter :: 21. `exclude_pattern::`

Glob pattern for file exclusion during directory scanning.

- **Type:** [PatternString](type.md#type--patternstring)
- **Default:** `null` (exclude none)
- **Required In:** None (optional)
- **Part Of:** [Filesystem Filtering](param_group.md#group--3-filesystem-filtering) parameter group

### Validation

Valid glob pattern.

### Examples

```bash
genfile .archive.from_directory source::"./src" exclude_pattern::"**/target/**"
genfile .archive.from_directory source::"./project" exclude_pattern::"**/*.{log,tmp}"
```

---

### Parameter :: 22. `default::`

Default value for parameter when not explicitly set.

- **Type:** [ContentString](type.md#type--contentstring)
- **Default:** `null` (no default)
- **Required In:** None (optional)

### Validation

Any UTF-8 text.

### Examples

```bash
genfile .parameter.add name::port default::"8080"
genfile .parameter.add name::author default::""
```

---

### Parameter :: 23. `content::`

File content data for adding file to archive.

- **Type:** [ContentString](type.md#type--contentstring)
- **Default:** None (required when not using `from_file::`)
- **Required In:** None (conditional — must use `content::` OR `from_file::`)

### Interactions

Conflicts with `from_file::` — use one, not both in `.file.add`.

### Validation

Any UTF-8 text.

### Examples

```bash
genfile .file.add path::"main.rs" content::"fn main() {}"
genfile .file.add path::"readme.md" content::"# {{project_name}}"
```

---

### Parameter Interaction Matrix

| Parameter 1 | Parameter 2 | Relationship | Behavior |
|-------------|-------------|--------------|----------|
| `include_pattern::` | `exclude_pattern::` | Cooperative | Exclusions applied after inclusions |
| `content::` | `from_file::` | Conflict | Use one, not both in `.file.add` |
| `mode::inline` | `from_file::` | Enhancement | Inline mode embeds from_file content |
| `dry::1` | `verbosity::2+` | Enhancement | Detailed preview output |
| `format::json` | `pretty::1` | Enhancement | Pretty-printed JSON |

### See Also

- [Commands](command/readme.md) - Command specifications
- [Types](type.md) - Type system and validation
- [Parameter Groups](param_group.md) - Shared parameter sets
- [Dictionary](dictionary.md) - Domain terminology
