# Output Format Catalog

Output format specifications for genfile CLI commands: archive serialization formats and output presentation conventions.

### Scope

- **In Scope:** Output structure, verbosity level behavior, exit codes, format conventions for all command categories, and archive serialization format catalog
- **Out of Scope:** Type validation rules — see [Types](type.md); parameter group semantics — see [Parameter Groups](param_group.md)
- **Audience:** CLI users, integrators parsing genfile output, and tool authors building on top of genfile
- **Responsibility:** Authoritative reference for what genfile prints and when, and how archives are encoded on disk

---

### Exit Codes

All commands use a consistent exit code convention:

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | User error (invalid input, file not found, missing value) |
| 2 | Runtime error (I/O failure, internal error) |

Exit code 1 always includes a human-readable error message on stderr. Exit code 2 may include additional context.

---

### Output Conventions

**Prefix tags (verbosity ≥ 2):**
- `[INFO]` — Informational progress message
- `[DEBUG]` — Internal operation detail (verbosity ≥ 3)
- `[DRY RUN]` — Preview-mode action description
- `[WARN]` — Non-fatal issue worth noting
- `[ERROR]` — Fatal error (always shown regardless of verbosity)

**Dry run prefix:** All output lines in dry-run mode begin with `[DRY RUN]`. No filesystem changes occur.

**Counts:** Human-readable counts use plain integers without padding. File sizes use human-readable units (B, KB, MB).

**Streams:**

| Stream | Content |
|--------|---------|
| stdout | All normal output (summaries, listings, progress) |
| stderr | All errors and warnings |

This separation allows piping stdout for programmatic processing while errors remain visible.

---

### Output Presentation Format Catalog

Named output rendering modes controlled by `verbosity::` and `dry::`. See [VerbosityLevel](type.md#type--1-verbositylevel) for the complete level constant table.

---

### Format :: F03. Silent Mode

- **ID:** F03
- **Output context:** All 24 commands
- **Trigger:** `verbosity::0`
- **Structure:** No stdout output on success; errors and warnings still appear on stderr
- **Rendering source:** TBD
- **Example:**
```
(no output — exit code signals success or failure)
```

**Notes:** Safe for scripting and CI pipelines where only exit codes matter.

---

### Format :: F04. Standard Mode

- **ID:** F04
- **Output context:** All 24 commands (default)
- **Trigger:** `verbosity::1` — default; parameter may be omitted
- **Structure:** One or a few summary lines per operation; command-category conventions:
  - *Read commands* (`.info`, `.status`, `.file.list`, `.parameter.list`, `.value.list`, `.analyze`, `.content.list`, `.file.show`): structured summary block
  - *Write commands* (`.archive.save`, `.content.internalize`, `.content.externalize`, `.materialize`, `.unpack`, `.pack`): action + count line
  - *Creation commands* (`.archive.new`, `.archive.from_directory`, `.file.add`, `.parameter.add`, `.value.set`): confirmation line per created entity
  - *Discovery commands* (`.discover.parameters`): discovered item count with names
- **Rendering source:** TBD
- **Example (read — `.info`):**
```
Archive: my-template
Files: 12 (inline: 4, reference: 8)
Parameters: 3 (mandatory: 1, optional: 2)
```
- **Example (write — `.materialize`):**
```
Materialized 12 files to ./output
```
- **Example (creation — `.archive.new`):**
```
Created archive 'rust-cli-template'
Files: 0
Parameters: 0
```

---

### Format :: F05. Verbose Mode

- **ID:** F05
- **Output context:** All 24 commands
- **Trigger:** `verbosity::2`
- **Structure:** Extends F04 with per-item listings, size metadata, content mode, and `[INFO]` prefixed progress lines
- **Rendering source:** TBD
- **Example (`.archive.load`):**
```
[INFO] Reading archive from backup.yaml
[INFO] Detected format: YAML
[INFO] Loaded archive 'backup-template'
Files: 15 (inline: 3, reference: 12)
Parameters: 6 (mandatory: 2, optional: 4)
```
- **Example (`.archive.from_directory`):**
```
[INFO] Scanning ./src
[INFO] Include: **/*.{rs,toml,md} | Exclude: **/target/**
[INFO] Mode: inline
Matched 67 files — Added 67 files (234 KB)
```

---

### Format :: F06. Debug Mode

- **ID:** F06
- **Output context:** All 24 commands (intended for development and troubleshooting only)
- **Trigger:** `verbosity::3` (Debug), `verbosity::4` (Trace), `verbosity::5` (Ultra-trace)
- **Structure:** Extends F05 with internal state, decisions, and function-level call tracking; `[DEBUG]` prefix on each internal detail line; levels 4-5 add progressively lower-level event detail
- **Rendering source:** TBD
- **Example:**
```
[DEBUG] archive::load called with path="backup.yaml"
[DEBUG] detected format from extension: YAML
[INFO] Reading archive from backup.yaml
[INFO] Detected format: YAML
[INFO] Loaded archive 'backup-template'
Files: 15 (inline: 3, reference: 12)
```

**Notes:** Levels 3-5 are not suitable for CI/CD output parsing. Use verbosity::0-2 in automation.

---

### Format :: F07. Dry Run Overlay

- **ID:** F07
- **Output context:** Write commands only — `.archive.save`, `.content.internalize`, `.content.externalize`, `.materialize`, `.unpack`, `.pack`, `.value.clear`, `.discover.parameters`
- **Trigger:** `dry::1` — combinable with any verbosity level (F03-F06)
- **Structure:** All action lines prefixed with `[DRY RUN]`; closing line `[DRY RUN] No changes made`; no filesystem writes or archive modifications occur; full validation still runs (errors in dry mode predict errors in real execution)
- **Rendering source:** TBD
- **Example (`.materialize dry::1 verbosity::2`):**
```
[DRY RUN] Would materialize to ./preview
[INFO] Files to create: src/main.rs, src/lib.rs, ...
[INFO] Substitutions: project_name -> "my-app", version -> "1.0.0"
[DRY RUN] No files created
```
- **Example (`.archive.save dry::1`):**
```
[DRY RUN] Would save archive to test.json
[DRY RUN] No changes made
```

---

### Archive Serialization Format Catalog

The following formats are supported for archive persistence (`.archive.save`, `.archive.load`, `.pack`).

---

### Format :: F01. JSON

- **ID:** F01
- **Output context:** `.archive.save`, `.archive.load`, `.pack` — JSON serialization of the archive on disk
- **Trigger:** `.json` file extension, or `format::json` parameter (default format)
- **Structure:** UTF-8 JSON object with `name`, `description`, `files` array, and `parameters` array at top level
- **Rendering source:** `serde_json` — pretty-printed by default (`pretty::1`), compact with `pretty::0`
- **Example:**
```json
{
  "name": "rust-cli-template",
  "description": "Rust CLI project template",
  "files": [
    {
      "path": "src/main.rs",
      "content": "fn main() { println!(\"Hello, {{project_name}}!\"); }",
      "mode": "inline"
    }
  ],
  "parameters": [
    { "name": "project_name", "mandatory": true, "default": null, "description": "Project name" }
  ]
}
```

**Notes:**
- Compact mode (`pretty::0`) removes all whitespace — suitable for CI and embedded use
- Reference-mode files store `"path"` instead of `"content"`

---

### Format :: F02. YAML

- **ID:** F02
- **Output context:** `.archive.save`, `.archive.load`, `.pack` — YAML serialization of the archive on disk
- **Trigger:** `.yaml` or `.yml` file extension, or `format::yaml` parameter
- **Structure:** UTF-8 YAML mapping with `name`, `description`, `files` sequence, and `parameters` sequence at top level
- **Rendering source:** `serde_yaml` — always human-readable (no compact mode)
- **Example:**
```yaml
name: rust-cli-template
description: Rust CLI project template
files:
  - path: src/main.rs
    content: "fn main() { println!(\"Hello, {{project_name}}!\"); }"
    mode: inline
parameters:
  - name: project_name
    mandatory: true
    default: null
    description: Project name
```

**Notes:**
- YAML is always human-readable — no compact/pretty distinction
- Better suited for hand-editing and version control review than JSON
- Slower to parse than JSON for large archives
