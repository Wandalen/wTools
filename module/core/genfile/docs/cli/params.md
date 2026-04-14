# Parameters Reference

Complete parameter listing for genfile CLI (23 unique parameters).

## Quick Navigation

**By Category:**
- [Universal Parameters](#category-universal) (2) - Used across all/most commands
- [I/O Paths](#category-io-paths) (7) - File and directory paths
- [Entity Management](#category-entity-management) (5) - Names, descriptions, values
- [Filtering](#category-filtering) (3) - Pattern-based selection
- [Flags](#category-flags) (3) - Boolean configuration
- [Enums](#category-enums) (3) - Multiple-choice options

**By Frequency:**
- Universal (24 commands): [verbosity::](#parameter-1-verbosity)
- Common (6 commands): [dry::](#parameter-2-dry)
- Moderate (3-5 commands): [path::](#parameter-3-path), [name::](#parameter-4-name)
- Command-specific (1-2 commands): All others

## Parameters Index

| # | Parameter | Type | Default | Commands | Category | Purpose |
|---|-----------|------|---------|----------|----------|---------|
| 1 | [`verbosity::`](#parameter-1-verbosity) | [VerbosityLevel](types.md#type-verbositylevel) | `1` | 24 | Universal | Output detail control |
| 2 | [`dry::`](#parameter-2-dry) | [DryRunFlag](types.md#type-dryrunflag) | `0` | 6 | Universal | Preview mode flag |
| 3 | [`path::`](#parameter-3-path) | [FilePath](types.md#type-filepath) | - | 5 | I/O Path | File path (input/output) |
| 4 | [`name::`](#parameter-4-name) | [IdentifierString](types.md#type-identifierstring) | - | 4 | Entity | Entity identifier |
| 5 | [`destination::`](#parameter-5-destination) | [OutputPath](types.md#type-outputpath) | - | 2 | I/O Path | Output directory path |
| 6 | [`description::`](#parameter-6-description) | [DescriptionText](types.md#type-descriptiontext) | `""` | 2 | Entity | Human-readable description |
| 7 | [`write_mode::`](#parameter-7-writemode) | [WriteMode](types.md#type-writemode) | - | 1 | Enum | File conflict resolution |
| 8 | [`value::`](#parameter-8-value) | [ContentString](types.md#type-contentstring) | - | 1 | Entity | Parameter value data |
| 9 | [`source::`](#parameter-9-source) | [DirectoryPath](types.md#type-directorypath) | - | 1 | I/O Path | Source directory path |
| 10 | [`recursive::`](#parameter-10-recursive) | [RecursiveFlag](types.md#type-recursiveflag) | `1` | 1 | Flag | Subdirectory traversal |
| 11 | [`pretty::`](#parameter-11-pretty) | [PrettyPrintFlag](types.md#type-prettyprintflag) | `1` | 1 | Flag | JSON formatting |
| 12 | [`output_dir::`](#parameter-12-outputdir) | [OutputPath](types.md#type-outputpath) | - | 1 | I/O Path | Output directory |
| 13 | [`output::`](#parameter-13-output) | [OutputPath](types.md#type-outputpath) | - | 1 | I/O Path | Output file path |
| 14 | [`mode::`](#parameter-14-mode) | [ContentMode](types.md#type-contentmode) | `reference` | 1 | Enum | Content storage strategy |
| 15 | [`mandatory::`](#parameter-15-mandatory) | [MandatoryFlag](types.md#type-mandatoryflag) | `0` | 1 | Flag | Parameter requirement flag |
| 16 | [`input::`](#parameter-16-input) | [DirectoryPath](types.md#type-directorypath) | - | 1 | I/O Path | Input directory path |
| 17 | [`include_pattern::`](#parameter-17-includepattern) | [PatternString](types.md#type-patternstring) | `null` | 1 | Filtering | Inclusion glob pattern |
| 18 | [`from_file::`](#parameter-18-fromfile) | [FilePath](types.md#type-filepath) | `null` | 1 | I/O Path | Source file path |
| 19 | [`format::`](#parameter-19-format) | [SerializationFormat](types.md#type-serializationformat) | `json` | 1 | Enum | Serialization format |
| 20 | [`filter::`](#parameter-20-filter) | [PatternString](types.md#type-patternstring) | `null` | 1 | Filtering | File filtering pattern |
| 21 | [`exclude_pattern::`](#parameter-21-excludepattern) | [PatternString](types.md#type-patternstring) | `null` | 1 | Filtering | Exclusion glob pattern |
| 22 | [`default::`](#parameter-22-default) | [ContentString](types.md#type-contentstring) | `null` | 1 | Entity | Parameter default value |
| 23 | [`content::`](#parameter-23-content) | [ContentString](types.md#type-contentstring) | - | 1 | Entity | File content data |

## Parameter Categories

### Category: Universal

Parameters used across all or most commands. Part of standard parameter groups.

| Parameter | Usage | Group |
|-----------|-------|-------|
| [verbosity::](#parameter-1-verbosity) | 24/24 (100%) | [Universal Output Control](parameter_groups.md#group-1-universal-output-control) |
| [dry::](#parameter-2-dry) | 6/24 (25%) | [Universal Execution Control](parameter_groups.md#group-2-universal-execution-control) |

### Category: I/O Paths

Filesystem paths for reading or writing data.

| Parameter | Purpose | Type |
|-----------|---------|------|
| [path::](#parameter-3-path) | Generic file path | Input/Output |
| [destination::](#parameter-5-destination) | Output directory | Output |
| [source::](#parameter-9-source) | Source directory | Input |
| [output_dir::](#parameter-12-outputdir) | Output directory | Output |
| [output::](#parameter-13-output) | Output file | Output |
| [input::](#parameter-16-input) | Input directory | Input |
| [from_file::](#parameter-18-fromfile) | Source file | Input |

### Category: Entity Management

Parameters for managing entities (names, descriptions, values).

| Parameter | Purpose |
|-----------|---------|
| [name::](#parameter-4-name) | Entity identifier |
| [description::](#parameter-6-description) | Entity description |
| [value::](#parameter-8-value) | Parameter value |
| [default::](#parameter-22-default) | Parameter default |
| [content::](#parameter-23-content) | File content |

### Category: Filtering

Pattern-based filtering for file selection.

| Parameter | Purpose | Pattern Type |
|-----------|---------|--------------|
| [include_pattern::](#parameter-17-includepattern) | Include files | Glob |
| [filter::](#parameter-20-filter) | Filter files | Glob |
| [exclude_pattern::](#parameter-21-excludepattern) | Exclude files | Glob |

### Category: Flags

Boolean configuration flags.

| Parameter | Purpose | Default |
|-----------|---------|---------|
| [recursive::](#parameter-10-recursive) | Subdirectory traversal | `1` (enabled) |
| [pretty::](#parameter-11-pretty) | JSON formatting | `1` (enabled) |
| [mandatory::](#parameter-15-mandatory) | Parameter requirement | `0` (optional) |

### Category: Enums

Multiple-choice parameters with fixed options.

| Parameter | Options | Default |
|-----------|---------|---------|
| [write_mode::](#parameter-7-writemode) | rewrite \| append \| skip | - |
| [mode::](#parameter-14-mode) | inline \| reference | `reference` |
| [format::](#parameter-19-format) | json \| yaml | `json` |

---

## Parameter Specifications

### Parameter :: 1. `verbosity::`

Controls output verbosity level across all commands using 0-5 scale. Higher values show more detailed information, lower values show minimal output.

**Type:** [VerbosityLevel](types.md#type-verbositylevel) (u8)
**Default:** `1` (normal output)
**Required In:** None (always optional)
**Part Of:** [Universal Output Control](parameter_groups.md#group-1-universal-output-control) parameter group

**Used By:** ALL 24 commands

**Validation:**
- Must be integer in range 0-5
- Invalid values rejected with error

**Verbosity Levels:**
- `0` - Silent (errors only)
- `1` - Normal (default - summary output)
- `2` - Verbose (detailed progress)
- `3` - Debug (internal operations)
- `4` - Trace (function calls)
- `5` - Ultra-trace (all events)

**Examples:**
```bash
genfile .archive.save path::"out.json" verbosity::0  # Silent
genfile .archive.load path::"template.yaml" verbosity::1  # Normal
genfile .materialize destination::"./output" verbosity::2  # Verbose
```

---

### Parameter :: 2. `dry::`

Enables preview mode where operations show what would happen without making actual changes. Essential for validating operations before execution.

**Type:** [DryRunFlag](types.md#type-dryrunflag) (bool)
**Default:** `0` (disabled - real execution)
**Required In:** None (always optional)
**Part Of:** [Universal Execution Control](parameter_groups.md#group-2-universal-execution-control) parameter group

**Used By:**
- [.archive.save](commands/archive.md#command-7-archivesave)
- [.archive.from_directory](commands/archive.md#command-8-archivefromdirectory)
- [.content.internalize](commands/content.md#command-9-contentinternalize)
- [.materialize](commands/operations.md#command-16-materialize)
- [.unpack](commands/operations.md#command-17-unpack)
- [.pack](commands/operations.md#command-18-pack)

**Validation:** Must be `0` (false) or `1` (true)

**Behavior:**
- `0` - Execute operation normally (default)
- `1` - Show preview without changes

**Examples:**
```bash
genfile .archive.save path::"test.json" dry::1  # Preview only
genfile .materialize destination::"./output" dry::1 verbosity::2  # Detailed preview
```

---

### Parameter :: 3. `path::`

Generic file path for reading or writing archive files. Supports both input and output operations depending on command context.

**Type:** [FilePath](types.md#type-filepath) (PathBuf)
**Default:** None (context-dependent - required in most commands)
**Required In:** .archive.load, .archive.save, .file.add, .file.show, .file.remove

**Used By:**
- [.archive.load](commands/archive.md#command-6-archiveload) - Input path
- [.archive.save](commands/archive.md#command-7-archivesave) - Output path
- [.file.add](commands/file.md#command-12-fileadd) - Archive-internal path
- [.file.show](commands/file.md#command-15-fileshow) - Archive-internal path
- [.file.remove](commands/file.md#command-13-fileremove) - Archive-internal path

**Validation:**
- Must be valid UTF-8 path string
- For input: file must exist and be readable
- For output: parent directory must be writable

**Context-Dependent Behavior:**
- **In .archive.load:** Path to existing archive file
- **In .archive.save:** Path where archive will be saved
- **In .file.*:** Path within archive structure (not filesystem path)

**Examples:**
```bash
genfile .archive.load path::"./archives/template.json"  # Load
genfile .archive.save path::"./output/archive.yaml"  # Save
genfile .file.add path::"src/main.rs" content::"..."  # Archive path
```

---

### Parameter :: 4. `name::`

Entity identifier for archives and parameters. Must be valid identifier (alphanumeric + underscore).

**Type:** [IdentifierString](types.md#type-identifierstring) (String)
**Default:** None (required when used)
**Required In:** .archive.new, .parameter.add, .parameter.remove, .value.set

**Used By:**
- [.archive.new](commands/archive.md#command-5-archivenew)
- [.parameter.add](commands/param_mgmt.md#command-19-parameteradd)
- [.parameter.remove](commands/param_mgmt.md#command-21-parameterremove)
- [.value.set](commands/value.md#command-22-valueset)

**Validation:** Alphanumeric + underscore only, no spaces

**Examples:**
```bash
genfile .archive.new name::"my_template"
genfile .parameter.add name::project_name mandatory::true
genfile .value.set name::version value::"1.0.0"
```

---

### Parameter :: 5. `destination::`

Output directory path for materialized or unpacked files.

**Type:** [OutputPath](types.md#type-outputpath) (PathBuf)
**Default:** None (required)
**Required In:** .materialize, .unpack

**Used By:**
- [.materialize](commands/operations.md#command-16-materialize)
- [.unpack](commands/operations.md#command-17-unpack)

**Validation:** Must be valid path, writable

**Examples:**
```bash
genfile .materialize destination::"./output"
genfile .unpack destination::"./template-files"
```

---

### Parameter :: 6. `description::`

Human-readable entity description for archives and parameters.

**Type:** [DescriptionText](types.md#type-descriptiontext) (String)
**Default:** `""` (empty string)
**Required In:** None (always optional)

**Used By:**
- [.archive.new](commands/archive.md#command-5-archivenew)
- [.parameter.add](commands/param_mgmt.md#command-19-parameteradd)

**Validation:** Any UTF-8 text

**Examples:**
```bash
genfile .archive.new name::"template" description::"REST API template"
genfile .parameter.add name::author description::"Project author name"
```

---

### Parameter :: 7. `write_mode::`

File conflict resolution mode when adding files to archive.

**Type:** [WriteMode](types.md#type-writemode) (enum)
**Default:** None (command-specific)
**Required In:** None (optional)

**Used By:**
- [.file.add](commands/file.md#command-12-fileadd)

**Options:**
- `rewrite` - Overwrite existing file
- `append` - Append to existing file
- `skip` - Skip if file exists

**Examples:**
```bash
genfile .file.add path::"config.toml" content::"..." write_mode::rewrite
```

---

### Parameter :: 8. `value::`

Parameter value data for template substitution.

**Type:** [ContentString](types.md#type-contentstring) (String)
**Default:** None (required)
**Required In:** .value.set

**Used By:**
- [.value.set](commands/value.md#command-22-valueset)

**Validation:** Any UTF-8 text

**Examples:**
```bash
genfile .value.set name::project_name value::"my-app"
genfile .value.set name::version value::"1.0.0"
```

---

### Parameter :: 9. `source::`

Source directory path for scanning files.

**Type:** [DirectoryPath](types.md#type-directorypath) (PathBuf)
**Default:** None (required)
**Required In:** .archive.from_directory

**Used By:**
- [.archive.from_directory](commands/archive.md#command-8-archivefromdirectory)

**Validation:** Must exist and be readable directory

**Examples:**
```bash
genfile .archive.from_directory source::"./templates"
```

---

### Parameter :: 10. `recursive::`

Controls subdirectory traversal during filesystem scanning.

**Type:** [RecursiveFlag](types.md#type-recursiveflag) (bool)
**Default:** `1` (enabled)
**Required In:** None (optional)
**Part Of:** [Filesystem Filtering](parameter_groups.md#group-3-filesystem-filtering) parameter group

**Used By:**
- [.archive.from_directory](commands/archive.md#command-8-archivefromdirectory)

**Validation:** Must be `0` (false) or `1` (true)

**Examples:**
```bash
genfile .archive.from_directory source::"./src" recursive::1  # Scan subdirs
genfile .archive.from_directory source::"./config" recursive::0  # Flat only
```

---

### Parameter :: 11. `pretty::`

Controls JSON pretty-printing (does not affect YAML).

**Type:** [PrettyPrintFlag](types.md#type-prettyprintflag) (bool)
**Default:** `1` (enabled)
**Required In:** None (optional)

**Used By:**
- [.archive.save](commands/archive.md#command-7-archivesave)

**Validation:** Must be `0` (false) or `1` (true)

**Examples:**
```bash
genfile .archive.save path::"template.json" pretty::1  # Pretty JSON
genfile .archive.save path::"compact.json" pretty::0  # Compact JSON
```

---

### Parameter :: 12. `output_dir::`

Output directory for externalized content.

**Type:** [OutputPath](types.md#type-outputpath) (PathBuf)
**Default:** None (required)
**Required In:** .content.externalize

**Used By:**
- [.content.externalize](commands/content.md#command-10-contentexternalize)

**Validation:** Must be writable

**Examples:**
```bash
genfile .content.externalize output_dir::"./files"
```

---

### Parameter :: 13. `output::`

Output file path for packed archives.

**Type:** [OutputPath](types.md#type-outputpath) (PathBuf)
**Default:** None (required)
**Required In:** .pack

**Used By:**
- [.pack](commands/operations.md#command-18-pack)

**Validation:** Parent directory must be writable

**Examples:**
```bash
genfile .pack input::"./templates" output::"template.json"
```

---

### Parameter :: 14. `mode::`

Content storage strategy (inline vs reference).

**Type:** [ContentMode](types.md#type-contentmode) (enum)
**Default:** `reference`
**Required In:** None (optional)

**Used By:**
- [.archive.from_directory](commands/archive.md#command-8-archivefromdirectory)

**Options:**
- `inline` - Embed content in archive
- `reference` - Store file paths only

**Examples:**
```bash
genfile .archive.from_directory source::"./src" mode::inline
genfile .archive.from_directory source::"./templates" mode::reference
```

---

### Parameter :: 15. `mandatory::`

Whether parameter is required for template materialization.

**Type:** [MandatoryFlag](types.md#type-mandatoryflag) (bool)
**Default:** `0` (optional)
**Required In:** None (optional)

**Used By:**
- [.parameter.add](commands/param_mgmt.md#command-19-parameteradd)

**Validation:** Must be `0` (false) or `1` (true)

**Examples:**
```bash
genfile .parameter.add name::project_name mandatory::true
genfile .parameter.add name::author mandatory::false
```

---

### Parameter :: 16. `input::`

Input directory path for packing.

**Type:** [DirectoryPath](types.md#type-directorypath) (PathBuf)
**Default:** None (required)
**Required In:** .pack

**Used By:**
- [.pack](commands/operations.md#command-18-pack)

**Validation:** Must exist and be readable

**Examples:**
```bash
genfile .pack input::"./templates" output::"archive.json"
```

---

### Parameter :: 17. `include_pattern::`

Glob pattern for file inclusion during directory scanning.

**Type:** [PatternString](types.md#type-patternstring) (String)
**Default:** `null` (include all)
**Required In:** None (optional)
**Part Of:** [Filesystem Filtering](parameter_groups.md#group-3-filesystem-filtering) parameter group

**Used By:**
- [.archive.from_directory](commands/archive.md#command-8-archivefromdirectory)

**Validation:** Valid glob pattern

**Examples:**
```bash
genfile .archive.from_directory source::"./src" include_pattern::"**/*.rs"
genfile .archive.from_directory source::"./docs" include_pattern::"**/*.{md,txt}"
```

---

### Parameter :: 18. `from_file::`

Source file path to read content from when adding file to archive.

**Type:** [FilePath](types.md#type-filepath) (PathBuf)
**Default:** `null`
**Required In:** None (conditional - must use `content::` OR `from_file::`)

**Used By:**
- [.file.add](commands/file.md#command-12-fileadd)

**Validation:** File must exist and be readable

**Interactions:** Conflicts with `content::` parameter (use one or the other)

**Examples:**
```bash
genfile .file.add path::"readme.md" from_file::"./README.md"
```

---

### Parameter :: 19. `format::`

Serialization format for archive persistence.

**Type:** [SerializationFormat](types.md#type-serializationformat) (enum)
**Default:** `json`
**Required In:** None (optional)

**Used By:**
- [.archive.save](commands/archive.md#command-7-archivesave)

**Options:**
- `json` - JSON format
- `yaml` - YAML format

**Examples:**
```bash
genfile .archive.save path::"template.json" format::json
genfile .archive.save path::"template.yaml" format::yaml
```

---

### Parameter :: 20. `filter::`

Pattern for filtering file listings.

**Type:** [PatternString](types.md#type-patternstring) (String)
**Default:** `null` (no filter)
**Required In:** None (optional)

**Used By:**
- [.content.list](commands/content.md#command-11-contentlist)

**Validation:** Valid pattern

**Examples:**
```bash
genfile .content.list filter::inline
genfile .content.list filter::reference
```

---

### Parameter :: 21. `exclude_pattern::`

Glob pattern for file exclusion during directory scanning.

**Type:** [PatternString](types.md#type-patternstring) (String)
**Default:** `null` (exclude none)
**Required In:** None (optional)
**Part Of:** [Filesystem Filtering](parameter_groups.md#group-3-filesystem-filtering) parameter group

**Used By:**
- [.archive.from_directory](commands/archive.md#command-8-archivefromdirectory)

**Validation:** Valid glob pattern

**Examples:**
```bash
genfile .archive.from_directory source::"./src" exclude_pattern::"**/target/**"
genfile .archive.from_directory source::"./project" exclude_pattern::"**/*.{log,tmp}"
```

---

### Parameter :: 22. `default::`

Default value for parameter when not explicitly set.

**Type:** [ContentString](types.md#type-contentstring) (String)
**Default:** `null` (no default)
**Required In:** None (optional)

**Used By:**
- [.parameter.add](commands/param_mgmt.md#command-19-parameteradd)

**Validation:** Any UTF-8 text

**Examples:**
```bash
genfile .parameter.add name::port default::"8080"
genfile .parameter.add name::author default::""
```

---

### Parameter :: 23. `content::`

File content data for adding file to archive.

**Type:** [ContentString](types.md#type-contentstring) (String)
**Default:** None (required when not using `from_file::`)
**Required In:** None (conditional - must use `content::` OR `from_file::`)

**Used By:**
- [.file.add](commands/file.md#command-12-fileadd)

**Validation:** Any UTF-8 text

**Interactions:** Conflicts with `from_file::` parameter (use one or the other)

**Examples:**
```bash
genfile .file.add path::"main.rs" content::"fn main() {}"
genfile .file.add path::"readme.md" content::"# {{project_name}}"
```

---

## Parameter Interaction Matrix

| Parameter 1 | Parameter 2 | Relationship | Behavior |
|-------------|-------------|--------------|----------|
| `include_pattern::` | `exclude_pattern::` | Cooperative | Exclusions applied after inclusions |
| `content::` | `from_file::` | Conflict | Use one, not both in .file.add |
| `mode::inline` | `from_file::` | Enhancement | Inline mode embeds from_file content |
| `dry::1` | `verbosity::2+` | Enhancement | Detailed preview output |
| `format::json` | `pretty::1` | Enhancement | Pretty-printed JSON |

---

## See Also

- [Commands Reference](commands.md) - Command specifications
- [Types Reference](types.md) - Type system and validation
- [Parameter Groups](parameter_groups.md) - Shared parameter sets
- [Dictionary](dictionary.md) - Domain terminology
