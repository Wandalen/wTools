# Workflow Scenarios

Multi-command usage scenarios covering common genfile workflows from start to finish.

### Scope

- **In Scope:** Complete end-to-end workflows combining multiple commands, with rationale for each step
- **Out of Scope:** Individual command syntax (see [Commands](command/readme.md)), parameter details (see [Parameters](param.md))
- **Audience:** CLI users learning genfile patterns and integrators automating genfile workflows
- **Responsibility:** Practical reference for real-world genfile usage patterns

---

### Scenario 1: Create Template from Scratch

**Goal:** Build a new reusable template archive starting from nothing.

**When to use:** When you want to create a template without an existing directory structure to import from.

```bash
# 1. Create empty archive
genfile .archive.new name::"rust-cli-template" description::"Rust CLI project starter"

# 2. Add template files with placeholders
genfile .file.add path::"src/main.rs" content::'fn main()
{
  println!( "{{project_name}} v{{version}}" );
}'

genfile .file.add path::"Cargo.toml" content::'[package]
name = "{{project_name}}"
version = "{{version}}"
authors = ["{{author}}"]'

genfile .file.add path::"readme.md" content::"# {{project_name}}\n\n{{description}}"

# 3. Define parameters (schema)
genfile .parameter.add name::project_name mandatory::true description::"Project identifier"
genfile .parameter.add name::version mandatory::true default::"0.1.0" description::"Initial version"
genfile .parameter.add name::author mandatory::false default::"" description::"Project author"
genfile .parameter.add name::description mandatory::false default::"A Rust project" description::"Short description"

# 4. Save archive
genfile .archive.save path::"rust-cli-template.yaml" format::yaml

# 5. Verify
genfile .analyze verbosity::2
```

---

### Scenario 2: Import Existing Directory as Template

**Goal:** Convert an existing project structure into a reusable template.

**When to use:** When you have a working project and want to templatize it.

```bash
# 1. Import directory (inline mode for portability)
genfile .archive.from_directory \
  source::"./my-rust-project" \
  mode::inline \
  recursive::1 \
  exclude_pattern::"**/target/**"

# 2. Verify import
genfile .file.list verbosity::2

# 3. Auto-detect existing placeholders
genfile .discover.parameters

# 4. Review detected parameters
genfile .parameter.list

# 5. Make mandatory parameters mandatory
genfile .parameter.remove name::project_name
genfile .parameter.add name::project_name mandatory::true description::"Project name"

# 6. Save as portable archive
genfile .archive.save path::"project-template.json" pretty::1
```

---

### Scenario 3: Materialize a Template

**Goal:** Generate a concrete project from an existing template archive.

**When to use:** When you want to create a new project from a template.

```bash
# 1. Load the template
genfile .archive.load path::"project-template.json"

# 2. Inspect what parameters are needed
genfile .status
genfile .parameter.list

# 3. Set parameter values
genfile .value.set name::project_name value::"my-new-app"
genfile .value.set name::version value::"0.1.0"
genfile .value.set name::author value::"Jane Doe"
genfile .value.set name::description value::"My new Rust application"

# 4. Preview first (dry run)
genfile .materialize destination::"./my-new-app" dry::1 verbosity::2

# 5. Execute materialization
genfile .materialize destination::"./my-new-app"

# 6. Verify output
ls -la ./my-new-app/
```

---

### Scenario 4: Update and Re-save a Template

**Goal:** Modify an existing template archive by adding files, adjusting parameters, and saving a new version.

**When to use:** When a template needs updates after initial creation.

```bash
# 1. Load existing template
genfile .archive.load path::"project-template.json"

# 2. Inspect current state
genfile .analyze verbosity::2

# 3. Add new file
genfile .file.add path::".github/workflows/ci.yml" from_file::"./ci.yml.template"

# 4. Add new parameter
genfile .parameter.add name::min_rust_version mandatory::false default::"1.70"

# 5. Remove obsolete file
genfile .file.remove path::"old-config.toml"

# 6. Preview save
genfile .archive.save path::"project-template-v2.json" dry::1 verbosity::2

# 7. Save updated template
genfile .archive.save path::"project-template-v2.json" pretty::1
```

---

### Scenario 5: Pack Directory for Distribution

**Goal:** Create a single portable archive file from a template directory to share with others.

**When to use:** When you want to distribute a template as a single self-contained file.

```bash
# One-step pack (inline mode, handles directory → archive file)
genfile .pack \
  input::"./template-project" \
  output::"shareable-template.json"

# Verify the packed archive
genfile .archive.load path::"shareable-template.json"
genfile .info
genfile .file.list

# Recipients can materialize directly:
# genfile .archive.load path::"shareable-template.json"
# genfile .value.set name::project_name value::"their-app"
# genfile .materialize destination::"./their-app"
```

---

### Scenario 6: CI/CD Integration

**Goal:** Use genfile in automated pipelines for consistent project generation.

**When to use:** Scaffolding new services, generating config files, or templating infrastructure code in CI/CD.

```bash
#!/bin/bash
# Example CI scaffolding script

set -euo pipefail

TEMPLATE_PATH="templates/microservice.json"
SERVICE_NAME="${1:?Service name required}"
OUTPUT_DIR="services/${SERVICE_NAME}"

# Load template
genfile .archive.load path::"${TEMPLATE_PATH}"

# Set values
genfile .value.set name::service_name value::"${SERVICE_NAME}"
genfile .value.set name::port value::"${SERVICE_PORT:-8080}"
genfile .value.set name::team value::"${TEAM_NAME}"

# Verify all mandatory values are set
genfile .status

# Materialize (silent mode for CI)
genfile .materialize destination::"${OUTPUT_DIR}" verbosity::0

echo "Generated service at ${OUTPUT_DIR}"
```

### See Also

- [Commands](command/readme.md) — Complete command reference
- [Parameters](param.md) — Parameter specifications
- [Tutorial](tutorial.md) — Step-by-step beginner guide
- [Dictionary](dictionary.md) — Domain terminology
