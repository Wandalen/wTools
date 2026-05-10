# GenFile CLI Tutorial

**Version:** 1.0.0
**Target Audience:** Beginners to intermediate users
**Prerequisites:** Basic command-line familiarity
**Time to Complete:** 45-60 minutes (all 4 lessons)

---

### Overview

This tutorial guides you through GenFile CLI fundamentals with hands-on examples. By the end, you'll understand:

1. **Lesson 1:** Archive basics (create, save, load)
2. **Lesson 2:** Working with files and templates
3. **Lesson 3:** Parameters and values for dynamic content
4. **Lesson 4:** Content management and real-world workflows

**Learning Path:** Each lesson builds on previous concepts. Complete them in order for best results.

---

### Lesson 1: Archive Basics (15 minutes)

#### What You'll Learn

- Create and manage template archives
- Save and load archives from disk
- Understand archive operations

#### Concepts

**What is a Template Archive?**

A template archive is a structured container for file templates with parameters and dynamic content. Think of it as a "smart directory" that can be saved, shared, and materialized with different values.

**Core Operations:**
- `.archive.new` - Create archive
- `.archive.save` - Write to disk
- `.archive.load` - Read from disk (loading replaces any current in-memory archive)

#### Exercise 1.1: Your First Archive

**Goal:** Create a simple archive and save it.

```bash
# Step 1: Create new archive named "my_first_archive"
genfile .archive.new name::"my_first_archive" description::"My first template archive" verbosity::2

# Expected output:
# Created archive 'my_first_archive'
# Description: My first template archive
# Files: 0
# Parameters: 0
```

**What happened?**
- Created archive in memory
- Set name and description
- `verbosity::2` shows detailed output

**Try this:** Change to `verbosity::3` to see even more details.

#### Exercise 1.2: Save and Load

**Goal:** Persist your archive to disk and reload it.

```bash
# Step 1: Save current archive to disk
genfile .archive.save path::"./my_first.yaml" verbosity::2

# Expected output:
# Saved archive to ./my_first.yaml (YAML, 245 bytes)

# Step 2: Reload from disk (replaces current in-memory archive)
genfile .archive.load path::"./my_first.yaml" verbosity::2

# Expected output:
# Loaded archive 'my_first_archive' from ./my_first.yaml
# Files: 0
# Parameters: 0
```

**What happened?**
1. Saved in-memory archive to YAML file
2. Reloaded archive from disk — `.archive.load` always replaces the current in-memory state

**Checkpoint:** You now understand the archive lifecycle!

#### Exercise 1.3: Dry Run Mode

**Goal:** Test operations without making changes.

```bash
# Create archive first
genfile .archive.new name::"test_archive" verbosity::1

# Preview save without writing (dry run)
genfile .archive.save path::"./test.yaml" dry::1 verbosity::2

# Expected output:
# [DRY RUN] Would save archive to ./test.yaml
# [INFO] Format: YAML
# [INFO] Estimated size: ~200 bytes
# [DRY RUN] No changes made
```

**What happened?**
- `dry::1` simulates operations without writing anything
- Use `dry::0` (or omit `dry::`) for real execution
- Useful for validating operations before committing

**Pro Tip:** Always preview with `dry::1` when testing complex operations.

#### Lesson 1 Review

**What you learned:**
- ✓ Create archives with `.archive.new`
- ✓ Save archives with `.archive.save`
- ✓ Load archives with `.archive.load`
- ✓ Preview safely with `dry::1`

**Next:** Lesson 2 adds files to your archives.

---

### Lesson 2: Working with Files (15 minutes)

#### What You'll Learn

- Add files to archives
- Use template variables with `{{placeholder}}` syntax
- Define file structure

#### Concepts

**Template Files vs Regular Files:**

Template files contain placeholders like `{{project_name}}` that get replaced during materialization. Regular files are static content. Genfile uses double-brace mustache syntax for all placeholders.

**File Operations:**
- `.file.add` - Add template with path and content
- `.file.remove` - Remove file from archive
- `.file.list` - Show all files in archive
- `.file.show` - Display file content

#### Exercise 2.1: Add Your First File

**Goal:** Create archive with a template README file.

```bash
# Step 1: Create new archive
genfile .archive.new name::"readme_template" verbosity::1

# Step 2: Add README template with placeholder
genfile .file.add \
  path::"readme.md" \
  content::"# {{project_name}}\n\nWelcome to {{project_name}}!\n\n## Description\n\n{{description}}" \
  verbosity::2

# Expected output:
# Added file: readme.md
# Content size: 82 bytes
# Archive status: 1 file, 0 parameters
```

**What happened?**
- Added `readme.md` template to archive
- Content uses `{{project_name}}` and `{{description}}` placeholders (double braces)
- Placeholders will be replaced during `.materialize`

#### Exercise 2.2: List Files

**Goal:** View all files in current archive.

```bash
genfile .file.list verbosity::2

# Expected output:
# Archive: readme_template
# Files (1):
#   [1] readme.md (82 bytes)
```

#### Exercise 2.3: Add Multiple Files

**Goal:** Create multi-file project template.

```bash
# Add main source file
genfile .file.add \
  path::"src/main.rs" \
  content::'fn main()\n{\n  println!( "Hello from {{project_name}}!" );\n}' \
  verbosity::1

# Add configuration file
genfile .file.add \
  path::"config.yaml" \
  content::"project:\n  name: {{project_name}}\n  version: {{version}}" \
  verbosity::1

# List all files
genfile .file.list verbosity::2

# Expected output:
# Archive: readme_template
# Files (3):
#   [1] readme.md (82 bytes)
#   [2] src/main.rs (60 bytes)
#   [3] config.yaml (48 bytes)
```

**What happened?**
- Added 3 files to archive
- Each file can have different placeholders
- Archive now has structure: readme.md, src/, config.yaml

#### Exercise 2.4: Remove a File

**Goal:** Remove unwanted file from archive.

```bash
# Remove config file
genfile .file.remove path::"config.yaml" verbosity::2

# Expected output:
# Removed file: config.yaml
# Archive status: 2 files remaining

# Verify removal
genfile .file.list verbosity::1

# Expected output:
# Files (2):
#   readme.md
#   src/main.rs
```

#### Exercise 2.5: Show File Content

**Goal:** Inspect template content for a specific file.

```bash
genfile .file.show path::"readme.md" verbosity::2

# Expected output:
# File: readme.md (82 bytes)
# ---
# # {{project_name}}
#
# Welcome to {{project_name}}!
#
# ## Description
#
# {{description}}
# ---
```

#### Exercise 2.6: Save Multi-File Archive

**Goal:** Persist your template for reuse.

```bash
# Save complete archive
genfile .archive.save path::"./project_template.yaml" verbosity::2

# Expected output:
# Saved archive to ./project_template.yaml (YAML, 1.2 KB)
# Files: 2
```

**Checkpoint:** You can now create reusable file templates!

#### Lesson 2 Review

**What you learned:**
- ✓ Add files with `.file.add`
- ✓ List files with `.file.list`
- ✓ Show file content with `.file.show`
- ✓ Remove files with `.file.remove`
- ✓ Use `{{placeholder}}` syntax in content (double braces)

**Next:** Lesson 3 defines parameters for placeholders.

---

### Lesson 3: Parameters and Values (15 minutes)

#### What You'll Learn

- Define parameters for placeholders
- Set parameter values
- Validate parameter constraints

#### Concepts

**Parameters Define Placeholders:**

Every `{{placeholder}}` in your templates should have a corresponding parameter definition that specifies:
- Name (matches placeholder without braces)
- Description (what it represents)
- Default value (optional)
- `mandatory::true/false` — whether a value is required for materialization

**Parameter Operations:**
- `.parameter.add` - Create parameter definition
- `.parameter.list` - Show all parameters
- `.value.set` - Assign value to parameter
- `.value.list` - Show current values

#### Exercise 3.1: Define Parameters

**Goal:** Define parameters for template placeholders.

```bash
# Load previous archive
genfile .archive.load path::"./project_template.yaml" verbosity::1

# Define project_name parameter (mandatory)
genfile .parameter.add \
  name::project_name \
  description::"Name of the project" \
  mandatory::true \
  verbosity::2

# Expected output:
# Added parameter: project_name
# Description: Name of the project
# Mandatory: Yes
# Default: (none)

# Define description parameter with default (optional)
genfile .parameter.add \
  name::description \
  description::"Project description text" \
  mandatory::false \
  default::"A new project" \
  verbosity::2

# Expected output:
# Added parameter: description
# Description: Project description text
# Mandatory: No
# Default: "A new project"
```

**What happened?**
- Defined 2 parameters matching template placeholders
- `project_name` is mandatory (no default — must be set before `.materialize`)
- `description` is optional (has default — safe to omit)

#### Exercise 3.2: List Parameters

**Goal:** View all defined parameters.

```bash
genfile .parameter.list verbosity::2

# Expected output:
# Archive: readme_template
# Parameters (2):
#   [1] project_name (mandatory)
#       Description: Name of the project
#
#   [2] description (optional)
#       Description: Project description text
#       Default: "A new project"
```

#### Exercise 3.3: Set Parameter Values

**Goal:** Assign values to parameters for materialization.

```bash
# Set project name
genfile .value.set \
  name::project_name \
  value::"my_awesome_app" \
  verbosity::2

# Expected output:
# Set value: project_name = "my_awesome_app"

# Set custom description
genfile .value.set \
  name::description \
  value::"An amazing application" \
  verbosity::1

# List all values
genfile .value.list verbosity::2

# Expected output:
# Parameter Values (2):
#   project_name = "my_awesome_app"
#   description = "An amazing application"
```

#### Exercise 3.4: Materialization Preview

**Goal:** See what files would be generated.

```bash
# Preview materialization (dry run)
genfile .materialize \
  destination::"./output" \
  dry::1 \
  verbosity::2

# Expected output:
# [DRY RUN] Would create: ./output/readme.md
# [DRY RUN] Would create: ./output/src/main.rs
# [DRY RUN] Summary: 2 files, no changes made

# Execute for real
genfile .materialize destination::"./output" verbosity::2

# Expected output:
# Created: ./output/readme.md
# Created: ./output/src/main.rs
# Materialized 2 files to ./output
```

**What you see:**
- Full preview of generated files
- All `{{placeholders}}` replaced with values
- No files created during dry run

**Checkpoint:** You understand the full parameter → value → materialization flow!

#### Lesson 3 Review

**What you learned:**
- ✓ Define parameters with `.parameter.add`
- ✓ Use `mandatory::true` for required and `mandatory::false` for optional
- ✓ Assign values with `.value.set`
- ✓ Preview materialization with `dry::1`

**Next:** Lesson 4 covers real-world workflows.

---

### Lesson 4: Real-World Workflows (15 minutes)

#### What You'll Learn

- Complete end-to-end workflows
- Content management strategies
- Archive organization patterns

#### Concepts

**Production Workflow:**

1. Create or load archive
2. Define structure (files + parameters)
3. Set values for specific instance
4. Materialize to disk
5. Save archive for reuse

#### Exercise 4.1: Create Rust Project Template

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
  content::'[package]\nname = "{{crate_name}}"\nversion = "{{version}}"\nedition = "2021"\n\n[dependencies]' \
  verbosity::1

# Step 3: Add main.rs
genfile .file.add \
  path::"src/main.rs" \
  content::'fn main()\n{\n  println!( "Welcome to {{crate_name}} v{{version}}!" );\n}' \
  verbosity::1

# Step 4: Add README
genfile .file.add \
  path::"readme.md" \
  content::'# {{crate_name}}\n\n{{description}}\n\n## Installation\n\n```bash\ncargo install {{crate_name}}\n```' \
  verbosity::1

# Step 5: Define parameters
genfile .parameter.add \
  name::crate_name \
  description::"Rust crate name" \
  mandatory::true \
  verbosity::1

genfile .parameter.add \
  name::version \
  description::"Initial version" \
  mandatory::false \
  default::"0.1.0" \
  verbosity::1

genfile .parameter.add \
  name::description \
  description::"Crate description" \
  mandatory::false \
  default::"A Rust binary application" \
  verbosity::1

# Step 6: Save reusable template
genfile .archive.save \
  path::"./templates/rust_binary.yaml" \
  verbosity::2

# Expected output:
# Saved archive to ./templates/rust_binary.yaml (YAML)
# Files: 3 (Cargo.toml, src/main.rs, readme.md)
# Parameters: 3 (1 mandatory, 2 optional)
```

**What you created:**
- Complete Rust project template
- 3 files with proper structure
- 3 parameters (1 mandatory, 2 with defaults)

#### Exercise 4.2: Use Template for New Project

**Goal:** Generate actual project from template.

```bash
# Step 1: Load template
genfile .archive.load \
  path::"./templates/rust_binary.yaml" \
  verbosity::1

# Step 2: Set values for specific project
genfile .value.set name::crate_name value::"hello_world" verbosity::1
genfile .value.set name::version value::"0.1.0" verbosity::1
genfile .value.set name::description value::"A friendly greeting app" verbosity::1

# Step 3: Preview (recommended)
genfile .materialize \
  destination::"./hello_world" \
  dry::1 \
  verbosity::2

# Step 4: Actually create files
genfile .materialize \
  destination::"./hello_world" \
  verbosity::2

# Expected output:
# Created: ./hello_world/Cargo.toml
# Created: ./hello_world/src/main.rs
# Created: ./hello_world/readme.md
# Materialized 3 files to ./hello_world
```

**What happened:**
1. Loaded reusable template
2. Customized for specific project
3. Generated complete project structure
4. Ready to build: `cd hello_world && cargo build`

#### Exercise 4.3: Inspect and Update Template Content

**Goal:** Review a template file and update its content.

```bash
# Load template
genfile .archive.load \
  path::"./templates/rust_binary.yaml" \
  verbosity::1

# Inspect main.rs content
genfile .file.show \
  path::"src/main.rs" \
  verbosity::2

# Expected output:
# File: src/main.rs
# ---
# fn main()
# {
#   println!( "Welcome to {{crate_name}} v{{version}}!" );
# }
# ---

# Update content: remove old file and add updated version
genfile .file.remove path::"src/main.rs" verbosity::1

genfile .file.add \
  path::"src/main.rs" \
  content::'fn main()\n{\n  println!( "=== {{crate_name}} v{{version}} ===" );\n  println!( "Status: Ready!" );\n}' \
  verbosity::2

# Verify updated content
genfile .file.show path::"src/main.rs" verbosity::1

# Save updated template
genfile .archive.save \
  path::"./templates/rust_binary.yaml" \
  verbosity::1
```

**What happened:**
- Inspected file content with `.file.show`
- Replaced content by removing and re-adding with `.file.remove` + `.file.add`
- Saved changes

#### Exercise 4.4: Advanced Workflow

**Goal:** Combine operations for efficient workflow.

```bash
# Chain: Create archive, add files, save
genfile .archive.new name::"quick_template" verbosity::0 && \
  genfile .file.add path::"file.txt" content::"Hello {{name}}!" verbosity::0 && \
  genfile .parameter.add name::name mandatory::true verbosity::0 && \
  genfile .archive.save path::"./quick.yaml" verbosity::1

# Chain: Load, set values, materialize
genfile .archive.load path::"./quick.yaml" verbosity::0 && \
  genfile .value.set name::name value::"World" verbosity::0 && \
  genfile .materialize destination::"./output" verbosity::2

# Expected output:
# Created: ./output/file.txt
# Materialized 1 file to ./output

# Result file content:
# Hello World!
```

**What you learned:**
- Chain commands with `&&`
- Use `verbosity::0` for quiet mode
- Create rapid workflows

#### Lesson 4 Review

**What you learned:**
- ✓ Build complete project templates
- ✓ Reuse templates for multiple projects
- ✓ Inspect content with `.file.show`
- ✓ Update content with `.file.remove` + `.file.add`
- ✓ Chain commands for efficient workflows

**You're now proficient with GenFile CLI!**

---

### What's Next?

#### Additional Resources

1. **[Commands Reference](command/readme.md)** - Complete command documentation
2. **[Parameters Reference](param.md)** - All parameter specifications
3. **[Type System](type.md)** - For implementers and advanced users
4. **[Dictionary](dictionary.md)** - Domain terminology
5. **[Workflow Scenarios](workflow_scenario.md)** - End-to-end usage patterns

#### Common Patterns

**Pattern 1: Team Template Library**
```bash
# Create shared templates directory
mkdir -p ~/.genfile/templates

# Save team templates
genfile .archive.save path::"~/.genfile/templates/rust_lib.yaml"

# Team members can load
genfile .archive.load path::"~/.genfile/templates/rust_lib.yaml"
```

**Pattern 2: Versioned Archives**
```bash
# Save with version in filename
genfile .archive.save path::"./templates/api_v1.0.0.yaml"
genfile .archive.save path::"./templates/api_v1.1.0.yaml"

# Load specific version
genfile .archive.load path::"./templates/api_v1.1.0.yaml"
```

**Pattern 3: Batch Materialization**
```bash
# Create multiple projects from same template
for project in app1 app2 app3; do
  genfile .archive.load path::"./template.yaml" verbosity::0
  genfile .value.set name::project_name value::"$project" verbosity::0
  genfile .materialize destination::"./$project" verbosity::1
done
```

#### Practice Exercises

**Exercise A: Web API Template**

Create a template for REST API projects with:
- `server.py` with `{{api_name}}` and `{{port}}` placeholders
- `readme.md` with documentation
- `requirements.txt` with dependencies
- Parameters: api_name (mandatory), port (default: 8000), description (optional)

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

#### Getting Help

**Documentation:** All commands documented in [command/readme.md](command/readme.md)

**Examples:** See [command/operations.md](command/operations.md) for `.materialize` workflows

**Issues:** Common problems and solutions in [maintenance.md](maintenance.md)

**Verbosity:** See [VerbosityLevel](type.md#type--1-verbositylevel) for the complete 0-5 level reference

---

### Congratulations!

You've completed the GenFile CLI tutorial. You now understand:

- ✓ Archive lifecycle (create, save, load)
- ✓ File management (add, remove, list, show)
- ✓ Parameter definitions (`mandatory::true/false`) and values
- ✓ `{{placeholder}}` mustache syntax
- ✓ Complete workflows from template to materialization

**You're ready to create production templates!**

---

**Tutorial Version:** 1.0.0
**Last Updated:** 2026-05-10
