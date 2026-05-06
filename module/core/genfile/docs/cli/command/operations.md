# Core Operations

Core operational commands for materialization, analysis, and utility functions. These commands perform primary archive operations that don't fit entity-specific namespaces.

- **Namespace:** operations (mixed)
- **Status:** All ✅ Implemented

### Commands in this Namespace

| # | Command | Purpose | Params | Complexity |
|---|---------|---------|--------|------------|
| 1 | [.info](#command-1-info) | Show metadata | 1 | Low |
| 2 | [.discover.parameters](#command-2-discoverparameters) | Auto-detect params | 2 | Low |
| 3 | [.status](#command-3-status) | Readiness check | 1 | Low |
| 4 | [.analyze](#command-4-analyze) | Analyze structure | 2 | Low |
| 16 | [.materialize](#command-16-materialize) | Render templates | 3 | Low |
| 17 | [.unpack](#command-17-unpack) | Unpack raw files | 3 | Low |
| 18 | [.pack](#command-18-pack) | Pack to archive | 4 | Medium |

---

### Command :: 1. `.info`

Displays archive metadata and statistics. Use this to inspect archive state.

**Syntax:**
```bash
genfile .info
genfile .info verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Shows archive name, description, file count, parameter count
- Displays content mode distribution (inline vs reference)
- Shows parameter readiness status

**Examples:**

**Basic info:**
```bash
genfile .info
# Output:
# Archive: my-template
# Description: Rust CLI template
# Files: 12 (inline: 4, reference: 8)
# Parameters: 3 (mandatory: 1, optional: 2)
```

**Exit Codes:** 0 (success) | 1 (no archive loaded) | 2 (runtime error)

**Related Commands:**
- [.status](#command-3-status) - Readiness check
- [.analyze](#command-4-analyze) - Detailed analysis

---

### Command :: 2. `.discover.parameters`

Auto-detects template parameters in archive files by scanning for placeholder syntax (`{{param_name}}`). Use this to automatically discover parameters.

**Syntax:**
```bash
genfile .discover.parameters
genfile .discover.parameters verbosity::2
genfile .discover.parameters dry::1
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |
| `dry::` | [DryRunFlag](../type.md#type-dryrunflag) | Preview mode (0 or 1) | `0` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>
<small>*`dry::` is part of [Universal Execution Control](../param_group.md#group-2-universal-execution-control) parameter group*</small>

**Behavior:**
- Scans all file content for `{{parameter}}` placeholders
- Adds discovered parameters to archive (non-mandatory, no defaults)
- Skips parameters already defined

**Examples:**

**Discover parameters:**
```bash
genfile .discover.parameters
# Output:
# Scanning 12 files for parameters...
# Discovered 5 parameters:
#   project_name (found in 3 files)
#   version (found in 2 files)
#   author (found in 1 file)
#   license (found in 1 file)
#   year (found in 2 files)
# Added 5 parameter definitions
```

**Exit Codes:** 0 (success) | 1 (no archive loaded) | 2 (scan error)

**Related Commands:**
- [.parameter.add](param_mgmt.md#command-19-parameteradd) - Manual parameter addition
- [.parameter.list](param_mgmt.md#command-20-parameterlist) - List discovered parameters

---

### Command :: 3. `.status`

Shows archive readiness and completeness status. Use this to check if archive is ready for materialization.

**Syntax:**
```bash
genfile .status
genfile .status verbosity::2
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>

**Behavior:**
- Checks if all mandatory parameters have values
- Reports missing values
- Shows overall readiness status

**Examples:**

**Ready status:**
```bash
genfile .status
# Output:
# Archive Status: READY
# Files: 12
# Parameters: 3 (all mandatory values set)
# Ready for materialization
```

**Not ready status:**
```bash
genfile .status
# Output:
# Archive Status: NOT READY
# Files: 12
# Parameters: 3
# Missing mandatory parameter values:
#   - project_name
# Set missing values before materialization
```

**Exit Codes:** 0 (ready) | 1 (not ready - missing values) | 2 (runtime error)

**Related Commands:**
- [.value.set](value.md#command-22-valueset) - Set missing values
- [.info](#command-1-info) - General archive info

---

### Command :: 4. `.analyze`

Performs comprehensive archive analysis including all insights. Use this for detailed archive inspection.

**Syntax:**
```bash
genfile .analyze
genfile .analyze verbosity::2
genfile .analyze dry::1
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |
| `dry::` | [DryRunFlag](../type.md#type-dryrunflag) | Preview mode (0 or 1) | `0` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>
<small>*`dry::` is part of [Universal Execution Control](../param_group.md#group-2-universal-execution-control) parameter group*</small>

**Behavior:**
- Analyzes file structure and parameter usage
- Detects unused parameters
- Identifies missing parameter definitions
- Reports content mode distribution

**Examples:**

**Basic analysis:**
```bash
genfile .analyze
# Output:
# Archive Analysis:
#
# Files: 12 total
#   - Inline: 4 (18 KB)
#   - Reference: 8 (links)
#
# Parameters: 3 defined
#   - Used in templates: 2
#   - Unused: 1 (port)
#   - Undefined (found in templates): 0
#
# Recommendations:
#   - Remove unused parameter: port
```

**Exit Codes:** 0 (success) | 1 (no archive loaded) | 2 (analysis error)

**Related Commands:**
- [.info](#command-1-info) - Basic info
- [.discover.parameters](#command-2-discoverparameters) - Auto-detect parameters

---

### Command :: 16. `.materialize`

Renders template archive to destination directory with parameter substitution. Use this to generate concrete files from templates.

**Syntax:**
```bash
genfile .materialize destination::"./output"
genfile .materialize destination::"./my-project" verbosity::2
genfile .materialize destination::"./preview" dry::1
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `destination::` | [OutputPath](../type.md#type-outputpath) | Output directory for materialized files | - | ✅ Yes |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |
| `dry::` | [DryRunFlag](../type.md#type-dryrunflag) | Preview mode (0 or 1) | `0` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>
<small>*`dry::` is part of [Universal Execution Control](../param_group.md#group-2-universal-execution-control) parameter group*</small>

**Behavior:**
- Renders all templates with parameter substitution
- Creates directory structure
- Writes materialized files to destination
- Requires all mandatory parameter values set

**Examples:**

**Basic materialization:**
```bash
genfile .materialize destination::"./output"
# Output:
# Materializing 12 files to ./output...
# Rendered: src/main.rs
# Rendered: src/lib.rs
# ...
# Materialized 12 files successfully
```

**Dry run preview:**
```bash
genfile .materialize destination::"./preview" dry::1 verbosity::2
# Output:
# [DRY RUN] Would materialize to ./preview
# [INFO] Files to create:
#   ./preview/src/main.rs
#   ./preview/src/lib.rs
#   ...
# [INFO] Parameter substitutions:
#   project_name -> "my-app"
#   version -> "1.0.0"
# [DRY RUN] No files created
```

**Exit Codes:** 0 (success) | 1 (missing mandatory values) | 2 (write error)

**Interactions:**
- Dependencies: All mandatory parameters must have values
- Typical workflow: `.archive.load` → `.value.set` → `.materialize`

**Related Commands:**
- [.unpack](#command-17-unpack) - Unpack without rendering
- [.value.set](value.md#command-22-valueset) - Set values before materialization
- [.status](#command-3-status) - Check readiness

---

### Command :: 17. `.unpack`

Unpacks raw template files to destination without rendering (no parameter substitution). Use this to extract template source files.

**Syntax:**
```bash
genfile .unpack destination::"./template-files"
genfile .unpack destination::"./output" verbosity::2
genfile .unpack destination::"./preview" dry::1
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `destination::` | [OutputPath](../type.md#type-outputpath) | Output directory for unpacked files | - | ✅ Yes |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |
| `dry::` | [DryRunFlag](../type.md#type-dryrunflag) | Preview mode (0 or 1) | `0` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>
<small>*`dry::` is part of [Universal Execution Control](../param_group.md#group-2-universal-execution-control) parameter group*</small>

**Behavior:**
- Extracts all files without parameter substitution
- Preserves `{{placeholders}}` in output
- Creates directory structure
- Does not require parameter values

**Examples:**

**Basic unpack:**
```bash
genfile .unpack destination::"./templates"
# Output:
# Unpacking 12 files to ./templates...
# Extracted: src/main.rs (with placeholders)
# Extracted: src/lib.rs (with placeholders)
# ...
# Unpacked 12 files successfully
```

**Exit Codes:** 0 (success) | 1 (destination error) | 2 (write error)

**Related Commands:**
- [.materialize](#command-16-materialize) - Render with substitution
- [.archive.save](archive.md#command-7-archivesave) - Save archive instead

---

### Command :: 18. `.pack`

Creates portable archive from directory with inline content in one step. Use this as shortcut for `.archive.from_directory` + `.archive.save` with inline mode.

**Syntax:**
```bash
genfile .pack input::"./templates" output::"template.json"
genfile .pack input::"./src" output::"backup.yaml" verbosity::2
genfile .pack input::"./code" output::"archive.json" dry::1
```

**Parameters:**

| Parameter | Type | Description | Default | Required |
|-----------|------|-------------|---------|----------|
| `input::` | [DirectoryPath](../type.md#type-directorypath) | Source directory to pack | - | ✅ Yes |
| `output::` | [OutputPath](../type.md#type-outputpath) | Output file path (JSON or YAML) | - | ✅ Yes |
| `verbosity::` | [VerbosityLevel](../type.md#type-verbositylevel) | Output detail level (0-5) | `1` | No |
| `dry::` | [DryRunFlag](../type.md#type-dryrunflag) | Preview mode (0 or 1) | `0` | No |

<small>*`verbosity::` is part of [Universal Output Control](../param_group.md#group-1-universal-output-control) parameter group*</small>
<small>*`dry::` is part of [Universal Execution Control](../param_group.md#group-2-universal-execution-control) parameter group*</small>

**Behavior:**
- Scans input directory recursively
- Creates archive with inline content (portable)
- Saves directly to output file
- One-step operation (no intermediate archive in memory)

**Examples:**

**Pack directory:**
```bash
genfile .pack input::"./templates" output::"template.json"
# Output:
# Scanning ./templates...
# Found 24 files
# Reading file contents...
# Creating portable archive...
# Saved to template.json (125 KB)
```

**Exit Codes:** 0 (success) | 1 (input directory not found) | 2 (write error)

**Related Commands:**
- [.archive.from_directory](archive.md#command-8-archivefromdirectory) - Create archive (two-step)
- [.archive.save](archive.md#command-7-archivesave) - Save archive (two-step)

---

### See Also

- [Archive Operations](archive.md) - Archive lifecycle
- [Value Operations](value.md) - Parameter values
- [Dictionary: Materialization](../dictionary.md#materialization) - Materialization concept
- [Parameters Reference](../param.md) - Parameter documentation
