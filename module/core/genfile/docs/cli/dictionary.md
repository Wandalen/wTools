# CLI Terminology Dictionary

Domain-specific terminology for genfile template archive management.

**Purpose:** Remove terminology barriers for new users and clarify domain concepts.

---

### Archive

**Definition:** Template archive data structure containing files (templates), parameters (variables), and metadata.

**Characteristics:**
- In-memory representation of template project
- Can be serialized to JSON or YAML files
- Contains file collection + parameter definitions + archive metadata
- Exists independently of filesystem

**File Formats:**
- JSON: `.json` extension, human-readable or compact
- YAML: `.yaml`/`.yml` extension, always human-readable

**Related:**
- [.archive.* commands](command/archive.md) - Archive lifecycle management
- [template](#template) - Files within archive
- [materialization](#materialization) - Rendering archive to filesystem

**Example:**
```json
{
  "name": "rust-cli-template",
  "description": "Rust CLI project template",
  "files": [
    { "path": "src/main.rs", "content": "..." },
    { "path": "Cargo.toml", "content": "..." }
  ],
  "parameters": [
    { "name": "project_name", "mandatory": true },
    { "name": "author", "default": "" }
  ]
}
```

---

### Template

**Definition:** File with parameter placeholders (`{{param_name}}`) that get substituted during materialization.

**Characteristics:**
- Text file with mustache-style placeholders
- Stored in archive with metadata (path, content mode)
- Rendered to concrete file during materialization
- Can contain arbitrary text (code, config, documentation)

**Placeholder Syntax:**
- `{{parameter_name}}` - Simple substitution
- `{{project_name}}_file.rs` - Embedded in filename
- Nested placeholders not supported

**Related:**
- [materialization](#materialization) - Rendering process
- [parameter](#parameter) - Variables for substitution
- [.file.* commands](command/file.md) - File management

**Example:**
```rust
// Template file: src/main.rs
fn main()
{
  println!( "Welcome to {{project_name}}!" );
  println!( "Version: {{version}}" );
}

// After materialization (project_name="my-app", version="1.0.0"):
fn main()
{
  println!( "Welcome to my-app!" );
  println!( "Version: 1.0.0" );
}
```

---

### Parameter

**Definition:** Named variable used for template substitution with optional default value and mandatory flag.

**Characteristics:**
- Has name (identifier), description, default value, mandatory flag
- Defined in archive metadata (schema)
- Values set before materialization (instances)
- Used for placeholder substitution in templates

**Metadata:**
- `name`: Parameter identifier (alphanumeric + underscore)
- `description`: Human-readable explanation
- `default`: Optional default value
- `mandatory`: Whether value required for materialization

**Related:**
- [.parameter.* commands](command/param_mgmt.md) - Parameter definition management
- [.value.* commands](command/value.md) - Parameter value management
- [template](#template) - Where parameters are used

**Example:**
```yaml
parameters:
  - name: project_name
    description: "Name of the generated project"
    default: "my-project"
    mandatory: true
  - name: author
    description: "Project author name"
    default: ""
    mandatory: false
```

---

### Materialization

**Definition:** Process of rendering template files to destination directory with parameter value substitution.

**Steps:**
1. Load archive (if not already in memory)
2. Set parameter values (`.value.set` for each required parameter)
3. Execute `.materialize` command
4. Templates rendered with placeholders replaced by values
5. Files written to destination directory

**Characteristics:**
- Converts archive (abstract) → filesystem (concrete)
- Preserves directory structure from archive
- Placeholder substitution atomic (all or nothing)
- Can be previewed with `dry::1` flag

**Related:**
- [.materialize command](command/operations.md#command--16-materialize) - Materialization command
- [template](#template) - What gets rendered
- [parameter](#parameter) - What gets substituted

**Workflow:**
```bash
# 1. Load archive
genfile .archive.load path::"template.yaml"

# 2. Set values
genfile .value.set name::project_name value::"my-app"
genfile .value.set name::version value::"1.0.0"

# 3. Materialize
genfile .materialize destination::"./output"

# Result: ./output/ contains rendered files with substitutions
```

---

### Content Mode

**Definition:** Storage strategy for file data in archives - inline (embedded) or reference (file paths).

**Modes:**

**Inline Mode:**
- File content embedded directly in archive
- Portable (archive file contains all data)
- Larger archive size
- Use when: sharing templates, no source files available

**Reference Mode:**
- Archive stores file paths only
- Smaller archive size
- Requires source files present during materialization
- Use when: working with local templates, minimizing archive size

**Conversion:**
- [.content.internalize](command/content.md#command--9-contentinternalize) - Reference → Inline
- [.content.externalize](command/content.md#command--10-contentexternalize) - Inline → Reference

**Related:**
- [.archive.from_directory mode::](command/archive.md#command--8-archivefrom_directory) - Choose mode during creation
- [.content.* commands](command/content.md) - Content management

**Comparison:**

| Aspect | Inline | Reference |
|--------|--------|-----------|
| Archive size | Large | Small |
| Portability | Portable | Requires source files |
| Use case | Sharing, distribution | Local development |
| File access | From archive | From filesystem |

**Example:**
```bash
# Create portable archive (inline mode)
genfile .archive.from_directory source::"./templates" mode::inline
genfile .archive.save path::"portable.json"
# Archive contains all file contents - can be shared standalone

# Create lightweight archive (reference mode)
genfile .archive.from_directory source::"./templates" mode::reference
genfile .archive.save path::"lightweight.json"
# Archive contains only file paths - needs source files present
```

---

### Dry Run

**Definition:** Preview mode that shows what operation would do without executing changes.

**Characteristics:**
- Controlled by `dry::1` parameter
- Validates inputs and shows planned actions
- No filesystem writes or modifications
- Exit codes match real execution
- Useful for validation before actual execution

**Applies To:**
- Write operations only (commands with side effects)
- Read operations ignore dry flag (no side effects anyway)

**Related:**
- [dry:: parameter](param.md#parameter--2-dry) - Parameter documentation
- [Universal Execution Control](param_group.md#group--2-universal-execution-control) - Parameter group

**Example:**
```bash
# Preview materialization
genfile .materialize destination::"./output" dry::1
# Output: [DRY RUN] Would create 12 files in ./output (no files created)

# Actual execution
genfile .materialize destination::"./output" dry::0
# Output: Created 12 files in ./output (files actually created)
```

---

### Verbosity Level

**Definition:** Output detail control using 0-5 scale where higher values show more information.

**Characteristics:**
- Universal parameter (appears in all commands)
- Scale from 0 (silent/errors only) to 5 (ultra-trace/all events); default is 1
- Controlled by `verbosity::` parameter

**Related:**
- [VerbosityLevel](type.md#type--1-verbositylevel) - Complete level definitions with constants
- [verbosity:: parameter](param.md#parameter--1-verbosity) - Parameter documentation
- [Universal Output Control](param_group.md#group--1-universal-output-control) - Parameter group

**Use Cases:**
- `verbosity::0` - CI/CD pipelines (quiet)
- `verbosity::1` - Interactive use (default)
- `verbosity::2` - Understanding operations
- `verbosity::3+` - Debugging issues

**Example:**
```bash
# Silent mode (errors only)
genfile .archive.save path::"out.json" verbosity::0
# Output: (none unless error)

# Normal mode (default)
genfile .archive.load path::"template.yaml"
# Output: Loaded archive 'my-template' from template.yaml

# Verbose mode (detailed)
genfile .materialize destination::"./output" verbosity::2
# Output:
# [INFO] Rendering 12 files...
# [INFO] Rendered: src/main.rs
# [INFO] Rendered: src/lib.rs
# ...
# [INFO] Materialized 12 files successfully
```

---

### Glob Pattern

**Definition:** Wildcard pattern for file matching using `*` and `**` syntax.

**Syntax:**
- `*` - Matches any characters except path separator
- `**` - Matches any characters including path separators (subdirectories)
- `?` - Matches single character
- `[abc]` - Matches one character from set
- `{a,b}` - Matches one alternative

**Examples:**
- `*.rs` - All Rust files in current directory
- `**/*.rs` - All Rust files recursively
- `src/**/*.{rs,toml}` - All Rust and TOML files under src/
- `**/target/**` - All files in target directories (any depth)

**Related:**
- [include_pattern:: parameter](param.md#parameter--17-include_pattern) - Inclusion patterns
- [exclude_pattern:: parameter](param.md#parameter--21-exclude_pattern) - Exclusion patterns
- [Filesystem Filtering](param_group.md#group--3-filesystem-filtering) - Pattern group

**Notes:**
- Case-sensitive on Unix, case-insensitive on Windows
- Patterns evaluated using glob crate (Rust)

**Example:**
```bash
# Include only Rust files
genfile .archive.from_directory \
  source::"./project" \
  include_pattern::"**/*.rs"

# Exclude build artifacts
genfile .archive.from_directory \
  source::"./workspace" \
  exclude_pattern::"**/target/**"

# Complex filtering
genfile .archive.from_directory \
  source::"./src" \
  include_pattern::"**/*.{rs,toml,md}" \
  exclude_pattern::"**/target/**"
```

---

### Serialization Format

**Definition:** Data encoding format for archive persistence - JSON or YAML.

**Formats:**

**JSON:**
- Extension: `.json`
- Compact or pretty-printed (controlled by `pretty::` parameter)
- Faster parsing
- More verbose (explicit syntax)

**YAML:**
- Extension: `.yaml`, `.yml`
- Always human-readable (no compact mode)
- Slower parsing
- More concise (implicit syntax)

**Auto-Detection:**
- Based on file extension in `.archive.load` and `.archive.save`
- Override with `format::` parameter

**Related:**
- [format:: parameter](param.md#parameter--19-format) - Format selection
- [.archive.save](command/archive.md#command--7-archivesave) - Saving with format control

**Recommendation:**
- **JSON:** Machine consumption, CI/CD, performance-critical
- **YAML:** Human editing, documentation, readability

**Example:**
```bash
# Save as JSON (default)
genfile .archive.save path::"template.json"
# Output: Saved archive to template.json (JSON, 2.4 KB)

# Save as YAML
genfile .archive.save path::"template.yaml" format::yaml
# Output: Saved archive to template.yaml (YAML, 1.8 KB)

# Compact JSON (no pretty-print)
genfile .archive.save path::"compact.json" pretty::0
# Output: Saved archive to compact.json (JSON compact, 1.2 KB)
```

---

### See Also

- [Commands Reference](command/readme.md) - Command documentation
- [Parameters Reference](param.md) - Parameter documentation
- [Types Reference](type.md) - Type system
- [Parameter Groups](param_group.md) - Shared parameter sets
