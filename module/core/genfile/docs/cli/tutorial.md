# GenFile CLI Tutorial

**Version:** 1.0.0
**Target Audience:** Beginners to intermediate users
**Prerequisites:** Basic command-line familiarity
**Time to Complete:** 45-60 minutes (all 4 lessons)

---

## Overview

This tutorial guides you through GenFile CLI fundamentals with hands-on examples. By the end, you'll understand:

1. **Lesson 1:** Archive basics (create, save, load)
2. **Lesson 2:** Working with files and templates
3. **Lesson 3:** Parameters and values for dynamic content
4. **Lesson 4:** Content management and real-world workflows

**Learning Path:** Each lesson builds on previous concepts. Complete them in order for best results.

---

## Lesson 1: Archive Basics (15 minutes)

### What You'll Learn

- Create and manage template archives
- Save and load archives from disk
- Understand archive operations

### Concepts

**What is a Template Archive?**

A template archive is a structured container for file templates with parameters and dynamic content. Think of it as a "smart directory" that can be saved, shared, and materialized with different values.

**Core Operations:**
- `.archive.new` - Create archive
- `.archive.save` - Write to disk
- `.archive.load` - Read from disk
- `.archive.close` - Reset current archive

### Exercise 1.1: Your First Archive

**Goal:** Create a simple archive and save it.

```bash
# Step 1: Create new archive named "my_first_archive"
genfile .archive.new name::"my_first_archive" description::"My first template archive" verbosity::2

# Expected output:
# ✓ Archive created: my_first_archive
# Description: My first template archive
# Status: Empty (0 files, 0 parameters)
```

**What happened?**
- Created archive in memory
- Set name and description
- `verbosity::2` shows detailed output

**Try this:** Change `verbosity::3` to see even more details.

### Exercise 1.2: Save and Load

**Goal:** Persist your archive to disk and reload it.

```bash
# Step 1: Save current archive to disk
genfile .archive.save path::"./my_first.genfile" verbosity::2

# Expected output:
# ✓ Archive saved: ./my_first.genfile
# Size: 245 bytes
# Format: TOML

# Step 2: Close current archive (clear memory)
genfile .archive.close verbosity::1

# Expected output:
# ✓ Archive closed

# Step 3: Reload from disk
genfile .archive.load path::"./my_first.genfile" verbosity::2

# Expected output:
# ✓ Archive loaded: ./my_first.genfile
# Name: my_first_archive
# Files: 0
# Parameters: 0
```

**What happened?**
1. Saved in-memory archive to TOML file
2. Cleared memory state
3. Restored archive from disk

**Checkpoint:** You now understand the archive lifecycle!

### Exercise 1.3: Dry Run Mode

**Goal:** Test operations without making changes.

```bash
# Create archive with dry run enabled
genfile .archive.new name::"test_archive" dry::true verbosity::2

# Expected output:
# [DRY RUN] Would create archive: test_archive
# [DRY RUN] No changes made

# Try saving (nothing written)
genfile .archive.save path::"./test.genfile" dry::true verbosity::2

# Expected output:
# [DRY RUN] Would save to: ./test.genfile
# [DRY RUN] No file written
```

**What happened?**
- `dry::true` simulates operations
- Nothing written to disk or memory
- Perfect for testing commands

**Pro Tip:** Always use `dry::true` when testing complex operations.

### Lesson 1 Review

**What you learned:**
- ✓ Create archives with `.archive.new`
- ✓ Save archives with `.archive.save`
- ✓ Load archives with `.archive.load`
- ✓ Test safely with `dry::true`

**Next:** Lesson 2 adds files to your archives.

---

## Lesson 2: Working with Files (15 minutes)

### What You'll Learn

- Add files to archives
- Use template variables
- Define file structure

### Concepts

**Template Files vs Regular Files:**

Template files contain placeholders like `{project_name}` that get replaced during materialization. Regular files are static content.

**File Operations:**
- `.file.add` - Add template with path and content
- `.file.remove` - Remove file from archive
- `.file.list` - Show all files in archive

### Exercise 2.1: Add Your First File

**Goal:** Create archive with a template README file.

```bash
# Step 1: Create new archive
genfile .archive.new name::"readme_template" verbosity::1

# Step 2: Add README template with placeholder
genfile .file.add \
  path::"readme.md" \
  content::"# {project_name}\n\nWelcome to {project_name}!\n\n## Description\n\n{description}" \
  verbosity::2

# Expected output:
# ✓ File added: readme.md
# Content size: 78 bytes
# Placeholders detected: project_name, description
# Archive status: 1 file, 0 parameters
```

**What happened?**
- Added `readme.md` template to archive
- Content includes `{project_name}` and `{description}` placeholders
- Placeholders detected but not yet defined

### Exercise 2.2: List Files

**Goal:** View all files in current archive.

```bash
genfile .file.list verbosity::2

# Expected output:
# Archive: readme_template
# Files (1):
#   [1] readme.md (78 bytes, 2 placeholders)
#       Placeholders: project_name, description
```

**What you see:**
- File count
- File path
- Content size
- Detected placeholders

### Exercise 2.3: Add Multiple Files

**Goal:** Create multi-file project template.

```bash
# Add main source file
genfile .file.add \
  path::"src/main.rs" \
  content::"fn main() {\n    println!(\"Hello from {project_name}!\");\n}" \
  verbosity::1

# Add configuration file
genfile .file.add \
  path::"config.toml" \
  content::"[project]\nname = \"{project_name}\"\nversion = \"{version}\"" \
  verbosity::1

# List all files
genfile .file.list verbosity::2

# Expected output:
# Archive: readme_template
# Files (3):
#   [1] readme.md (78 bytes)
#   [2] src/main.rs (56 bytes)
#   [3] config.toml (52 bytes)
# Total placeholders: project_name, description, version
```

**What happened?**
- Added 3 files to archive
- Each file can have different placeholders
- Archive now has structure: readme.md, src/, config.toml

### Exercise 2.4: Remove a File

**Goal:** Remove unwanted file from archive.

```bash
# Remove config file
genfile .file.remove path::"config.toml" verbosity::2

# Expected output:
# ✓ File removed: config.toml
# Archive status: 2 files remaining

# Verify removal
genfile .file.list verbosity::1

# Expected output:
# Files (2):
#   readme.md
#   src/main.rs
```

**What happened?**
- Removed `config.toml` from archive
- Other files unchanged
- Archive now has 2 files

### Exercise 2.5: Save Multi-File Archive

**Goal:** Persist your template for reuse.

```bash
# Save complete archive
genfile .archive.save path::"./project_template.genfile" verbosity::2

# Expected output:
# ✓ Archive saved: ./project_template.genfile
# Size: 1.2 KB
# Files: 2
# Format: TOML
```

**Checkpoint:** You can now create reusable file templates!

### Lesson 2 Review

**What you learned:**
- ✓ Add files with `.file.add`
- ✓ List files with `.file.list`
- ✓ Remove files with `.file.remove`
- ✓ Use placeholders like `{project_name}` in content
- ✓ Create multi-file templates

**Next:** Lesson 3 defines parameters for placeholders.

---

## Lesson 3: Parameters and Values (15 minutes)

### What You'll Learn

- Define parameters for placeholders
- Set parameter values
- Validate parameter constraints

### Concepts

**Parameters Define Placeholders:**

Every `{placeholder}` in your templates should have a corresponding parameter definition that specifies:
- Name (matches placeholder)
- Description (what it represents)
- Default value (optional)
- Optional flag (required vs optional)

**Parameter Operations:**
- `.parameter.add` - Create parameter definition
- `.parameter.list` - Show all parameters
- `.value.set` - Assign value to parameter
- `.value.list` - Show current values

### Exercise 3.1: Define Parameters

**Goal:** Define parameters for template placeholders.

```bash
# Load previous archive
genfile .archive.load path::"./project_template.genfile" verbosity::1

# Define project_name parameter (required)
genfile .parameter.add \
  name::"project_name" \
  description::"Name of the project" \
  optional::false \
  verbosity::2

# Expected output:
# ✓ Parameter defined: project_name
# Description: Name of the project
# Required: Yes
# Default: None

# Define description parameter with default
genfile .parameter.add \
  name::"description" \
  description::"Project description text" \
  optional::true \
  default::"A new project" \
  verbosity::2

# Expected output:
# ✓ Parameter defined: description
# Description: Project description text
# Required: No
# Default: "A new project"
```

**What happened?**
- Defined 2 parameters matching template placeholders
- `project_name` is required (no default)
- `description` is optional (has default)

### Exercise 3.2: List Parameters

**Goal:** View all defined parameters.

```bash
genfile .parameter.list verbosity::2

# Expected output:
# Archive: readme_template
# Parameters (2):
#   [1] project_name (required)
#       Description: Name of the project
#       Used in: readme.md, src/main.rs
#
#   [2] description (optional)
#       Description: Project description text
#       Default: "A new project"
#       Used in: readme.md
```

**What you see:**
- All defined parameters
- Required vs optional status
- Default values
- Which files use each parameter

### Exercise 3.3: Set Parameter Values

**Goal:** Assign values to parameters for materialization.

```bash
# Set project name
genfile .value.set \
  name::"project_name" \
  value::"my_awesome_app" \
  verbosity::2

# Expected output:
# ✓ Value set: project_name = "my_awesome_app"
# Status: Valid
# Required parameter: Satisfied

# Set custom description
genfile .value.set \
  name::"description" \
  value::"An amazing application" \
  verbosity::1

# List all values
genfile .value.list verbosity::2

# Expected output:
# Parameter Values (2):
#   project_name = "my_awesome_app"
#   description = "An amazing application"
#
# Validation: ✓ All required parameters set
```

**What happened?**
- Set values for both parameters
- Values validated against definitions
- Archive ready for materialization

### Exercise 3.4: Materialization Preview

**Goal:** See what files would be generated.

```bash
# Preview materialization (dry run)
genfile .materialize \
  destination::"./output" \
  dry::true \
  verbosity::3

# Expected output:
# [DRY RUN] Would create: ./output/readme.md
# Content preview:
# ---
# # my_awesome_app
#
# Welcome to my_awesome_app!
#
# ## Description
#
# An amazing application
# ---
#
# [DRY RUN] Would create: ./output/src/main.rs
# Content preview:
# ---
# fn main() {
#     println!("Hello from my_awesome_app!");
# }
# ---
#
# [DRY RUN] Summary:
# - 2 files would be created
# - 0 directories would be created
# - No actual changes made
```

**What you see:**
- Full preview of generated files
- All placeholders replaced with values
- Directory structure
- No files actually created (dry run)

**Checkpoint:** You understand the full parameter → value → materialization flow!

### Lesson 3 Review

**What you learned:**
- ✓ Define parameters with `.parameter.add`
- ✓ Set required vs optional parameters
- ✓ Assign values with `.value.set`
- ✓ Preview materialization with `dry::true`

**Next:** Lesson 4 covers real-world workflows.

---

## Lesson 4: Real-World Workflows (15 minutes)

### What You'll Learn

- Complete end-to-end workflows
- Content management strategies
- Archive organization patterns

### Concepts

**Production Workflow:**

1. Create or load archive
2. Define structure (files + parameters)
3. Set values for specific instance
4. Materialize to disk
5. Save archive for reuse

**Content Operations:**
- `.content.get` - Retrieve file content
- `.content.inspect` - Analyze content
- `.content.replace` - Update file content

### Exercise 4.1: Create Rust Project Template

**Goal:** Build reusable Rust project template from scratch.

```bash
# Step 1: Create archive
genfile .archive.new \
  name::"rust_binary_template" \
  description::"Template for Rust binary projects" \
  verbosity::2

# Step 2: Add Cargo.toml
genfile .file.add \
  path::"Cargo.toml" \
  content::"[package]\nname = \"{crate_name}\"\nversion = \"{version}\"\nedition = \"2021\"\n\n[dependencies]" \
  verbosity::1

# Step 3: Add main.rs
genfile .file.add \
  path::"src/main.rs" \
  content::"fn main()\n{\n  println!( \"Welcome to {crate_name} v{version}!\" );\n}" \
  verbosity::1

# Step 4: Add README
genfile .file.add \
  path::"readme.md" \
  content::"# {crate_name}\n\n{description}\n\n## Installation\n\n```bash\ncargo install {crate_name}\n```" \
  verbosity::1

# Step 5: Define parameters
genfile .parameter.add \
  name::"crate_name" \
  description::"Rust crate name" \
  optional::false \
  verbosity::1

genfile .parameter.add \
  name::"version" \
  description::"Initial version" \
  optional::true \
  default::"0.1.0" \
  verbosity::1

genfile .parameter.add \
  name::"description" \
  description::"Crate description" \
  optional::true \
  default::"A Rust binary application" \
  verbosity::1

# Step 6: Save reusable template
genfile .archive.save \
  path::"./templates/rust_binary.genfile" \
  verbosity::2

# Expected output:
# ✓ Archive saved: ./templates/rust_binary.genfile
# Files: 3 (Cargo.toml, src/main.rs, readme.md)
# Parameters: 3 (1 required, 2 optional)
# Ready for reuse
```

**What you created:**
- Complete Rust project template
- 3 files with proper structure
- 3 parameters (1 required, 2 with defaults)
- Saved for reuse

### Exercise 4.2: Use Template for New Project

**Goal:** Generate actual project from template.

```bash
# Step 1: Load template
genfile .archive.load \
  path::"./templates/rust_binary.genfile" \
  verbosity::1

# Step 2: Set values for specific project
genfile .value.set name::"crate_name" value::"hello_world" verbosity::1
genfile .value.set name::"version" value::"0.1.0" verbosity::1
genfile .value.set name::"description" value::"A friendly greeting app" verbosity::1

# Step 3: Preview (optional but recommended)
genfile .materialize \
  destination::"./hello_world" \
  dry::true \
  verbosity::2

# Step 4: Actually create files
genfile .materialize \
  destination::"./hello_world" \
  dry::false \
  verbosity::3

# Expected output:
# ✓ Created: ./hello_world/Cargo.toml
# ✓ Created: ./hello_world/src/main.rs
# ✓ Created: ./hello_world/readme.md
#
# Summary:
# - 3 files created
# - 1 directory created (src/)
# - Total size: 428 bytes
#
# Next steps:
# cd hello_world && cargo build
```

**What happened:**
1. Loaded reusable template
2. Customized for specific project
3. Generated complete project structure
4. Ready to build and run

**Try it:** Run `cd hello_world && cargo build && cargo run`

### Exercise 4.3: Content Management

**Goal:** Inspect and modify template content.

```bash
# Load template again
genfile .archive.load \
  path::"./templates/rust_binary.genfile" \
  verbosity::1

# Inspect main.rs content
genfile .content.get \
  path::"src/main.rs" \
  verbosity::2

# Expected output:
# File: src/main.rs
# Content (78 bytes):
# ---
# fn main()
# {
#   println!( "Welcome to {crate_name} v{version}!" );
# }
# ---
# Placeholders: crate_name, version

# Analyze content structure
genfile .content.inspect \
  path::"src/main.rs" \
  verbosity::3

# Expected output:
# File: src/main.rs
# Analysis:
#   Size: 78 bytes
#   Lines: 4
#   Placeholders: 2 (crate_name, version)
#   Language: Rust (detected from extension)
#   Syntax valid: Yes

# Update content
genfile .content.replace \
  path::"src/main.rs" \
  content::"fn main()\n{\n  println!( \"=== {crate_name} v{version} ===\"  );\n  println!( \"Status: Ready!\" );\n}" \
  verbosity::2

# Expected output:
# ✓ Content replaced: src/main.rs
# Old size: 78 bytes
# New size: 112 bytes
# Change: +34 bytes

# Save updated template
genfile .archive.save \
  path::"./templates/rust_binary.genfile" \
  verbosity::1
```

**What happened:**
- Retrieved file content
- Analyzed content structure
- Updated template
- Saved changes

### Exercise 4.4: Advanced Workflow

**Goal:** Combine operations for efficient workflow.

```bash
# One-liner: Create archive, add files, save
genfile .archive.new name::"quick_template" verbosity::0 && \
  genfile .file.add path::"file.txt" content::"Hello {name}!" verbosity::0 && \
  genfile .parameter.add name::"name" mandatory::true verbosity::0 && \
  genfile .archive.save path::"./quick.genfile" verbosity::1

# One-liner: Load, set values, materialize
genfile .archive.load path::"./quick.genfile" verbosity::0 && \
  genfile .value.set name::"name" value::"World" verbosity::0 && \
  genfile .materialize destination::"./output" verbosity::2

# Expected output:
# ✓ Created: ./output/file.txt
# Content: Hello World!
```

**What you learned:**
- Chain commands with `&&`
- Use `verbosity::0` for quiet mode
- Create rapid workflows

### Lesson 4 Review

**What you learned:**
- ✓ Build complete project templates
- ✓ Reuse templates for multiple projects
- ✓ Manage content with `.content.*` commands
- ✓ Chain commands for efficient workflows

**You're now proficient with GenFile CLI!**

---

## What's Next?

### Additional Resources

1. **[Commands Reference](command/readme.md)** - Complete command documentation
2. **[Parameters Reference](param.md)** - All parameter specifications
3. **[Type System](type.md)** - For implementers and advanced users
4. **[Dictionary](dictionary.md)** - Domain terminology

### Common Patterns

**Pattern 1: Team Template Library**
```bash
# Create shared templates directory
mkdir -p ~/.genfile/templates

# Save team templates
genfile .archive.save path::"~/.genfile/templates/rust_lib.genfile"
genfile .archive.save path::"~/.genfile/templates/react_component.genfile"

# Team members can load
genfile .archive.load path::"~/.genfile/templates/rust_lib.genfile"
```

**Pattern 2: Versioned Archives**
```bash
# Save with version in filename
genfile .archive.save path::"./templates/api_v1.0.0.genfile"
genfile .archive.save path::"./templates/api_v1.1.0.genfile"

# Load specific version
genfile .archive.load path::"./templates/api_v1.1.0.genfile"
```

**Pattern 3: Batch Materialization**
```bash
# Create multiple projects from same template
for project in app1 app2 app3; do
  genfile .archive.load path::"./template.genfile" verbosity::0
  genfile .value.set name::"project_name" value::"$project" verbosity::0
  genfile .materialize destination::"./$project" verbosity::1
done
```

### Practice Exercises

**Exercise A: Web API Template**

Create a template for REST API projects with:
- `server.py` with `{api_name}` and `{port}` placeholders
- `readme.md` with documentation
- `requirements.txt` with dependencies
- Parameters: api_name (required), port (default: 8000), description (optional)

**Exercise B: Multi-Language Template**

Create a template that generates the same project in multiple languages:
- `rust/` directory with Rust implementation
- `python/` directory with Python implementation
- Shared `readme.md`
- Parameter: `project_name`

**Exercise C: Archive Migration**

Practice loading, modifying, and resaving archives:
1. Load existing archive
2. Add new file
3. Update parameter definitions
4. Save as new version

### Getting Help

**Documentation:** All commands documented in [command/readme.md](command/readme.md)

**Examples:** See [command/operations.md](command/operations.md) for `.materialize` workflows

**Issues:** Common problems and solutions in [maintenance.md](maintenance.md)

**Verbosity Levels:**
- `verbosity::0` - Silent (only errors)
- `verbosity::1` - Minimal (success/failure)
- `verbosity::2` - Standard (detailed info)
- `verbosity::3` - Verbose (all details)
- `verbosity::4` - Debug (internal state)
- `verbosity::5` - Trace (everything)

---

## Congratulations!

You've completed the GenFile CLI tutorial. You now understand:

- ✓ Archive lifecycle (create, save, load)
- ✓ File management (add, remove, list)
- ✓ Parameter definitions and values
- ✓ Content operations
- ✓ Complete workflows from template to materialization

**You're ready to create production templates!**

---

**Tutorial Version:** 1.0.0
**Last Updated:** 2026-02-08
**Feedback:** Report issues or suggestions via project repository
