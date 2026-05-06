# Output Format Catalog

Output format specifications for genfile CLI commands across verbosity levels.

### Scope

- **In Scope:** Output structure, verbosity level behavior, exit codes, and format conventions for all command categories
- **Out of Scope:** Archive serialization formats (JSON/YAML) — see [Dictionary: Serialization Format](dictionary.md#serialization-format); type validation formats — see [Types](type.md)
- **Audience:** CLI users, integrators parsing genfile output, and tool authors building on top of genfile
- **Responsibility:** Authoritative reference for what genfile prints and when

---

### Verbosity Level Behavior

All commands respect the `verbosity::` parameter. Higher levels add more detail without removing lower-level output.

| Level | Name | Output Includes |
|-------|------|----------------|
| 0 | Silent | Errors only |
| 1 | Normal | Summary line(s) per operation (default) |
| 2 | Verbose | Per-item progress, totals, and counts |
| 3 | Debug | Internal decisions and state changes |
| 4 | Trace | Function-level call tracking |
| 5 | Ultra-trace | All events and data flow |

Levels 3–5 are intended for development and troubleshooting. Normal use requires only 0–2.

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

---

### Format by Command Category

### Read Commands (`.info`, `.status`, `.file.list`, `.parameter.list`, `.value.list`, `.analyze`, `.content.list`, `.file.show`)

Read commands produce structured summary output. They never modify state.

**Normal (verbosity 1):**
```
Archive: <name>
Files: <N> (inline: <N>, reference: <N>)
Parameters: <N> (mandatory: <N>, optional: <N>)
```

**Verbose (verbosity 2):** Adds per-item listings with metadata.

**Silent (verbosity 0):** Produces no output on success (exit code 0 only).

---

### Write Commands (`.archive.save`, `.content.internalize`, `.content.externalize`, `.materialize`, `.unpack`, `.pack`)

Write commands report what was written.

**Normal (verbosity 1):**
```
<Action> <count> files to <destination>
```

**Verbose (verbosity 2):** Lists each file processed with size.

**Dry run (any verbosity):**
```
[DRY RUN] Would <action> <count> files to <destination>
[DRY RUN] No changes made
```

---

### Creation Commands (`.archive.new`, `.archive.from_directory`, `.file.add`, `.parameter.add`, `.value.set`)

Creation commands confirm what was created or set.

**Normal (verbosity 1):**
```
Created <entity>: <name>
```
or
```
Added <entity>: <name> (<metadata>)
```

**Verbose (verbosity 2):** Adds field-by-field confirmation.

---

### Discovery Commands (`.discover.parameters`)

Reports discovered items and actions taken.

**Normal (verbosity 1):**
```
Discovered <N> parameters: <name1>, <name2>, ...
Added <N> parameter definitions
```

**Verbose (verbosity 2):** Lists which files each parameter was found in.

---

### Stderr vs Stdout

| Stream | Content |
|--------|---------|
| stdout | All normal output (summaries, listings, progress) |
| stderr | All errors and warnings |

This separation allows piping stdout for programmatic processing while errors remain visible.

### See Also

- [Parameters](param.md) — `verbosity::` and `dry::` parameter specifications
- [Types](type.md) — `VerbosityLevel` type definition
- [Parameter Groups](param_group.md) — Universal Output Control and Universal Execution Control groups
- [Workflow Scenarios](workflow_scenario.md) — Output in context of complete workflows
