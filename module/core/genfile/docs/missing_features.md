# Missing Features - Intentionally Out of Scope

This document explains features that are **intentionally excluded** from genfile_core to maintain simplicity.

**Note:** genfiles (TemplateArchive) are self-contained - parameter values are stored INSIDE the genfile (JSON/YAML), never in external files.

**Design Principle:** genfile_core is a simple, focused template processing library. Complex features are intentionally excluded.

---

## Out of Scope Features

The following features were considered but explicitly excluded to keep genfile_core simple and focused:

### 1. Interactive Prompting

**What it is:** Automatically prompting users via stdin for missing mandatory parameters.

**Why excluded:**
- Not the library's responsibility - calling applications should handle user interaction
- Keeps library pure (no I/O besides file operations)
- Simple for applications to implement themselves:
  ```rust
  let undefined = archive.get_undefined_parameters();
  for param in undefined {
    let value = prompt_user(param); // App's code
    archive.set_value(param, value);
  }
  ```

**Verdict:** Applications should implement their own prompting logic.

---

### 2. TomlExtend Write Mode

**What it is:** Smart TOML file merging that preserves user edits while adding new template fields.

**Why excluded:**
- User explicitly requested simplicity over this feature
- Complex merge algorithm with edge cases
- Only useful for project scaffolding/regeneration tools
- genfile_core is a template library, not a project regeneration tool
- Adds toml_edit dependency and significant complexity

**Verdict:** Out of scope. Applications needing this can implement custom merging logic.

---

### 3. Builder Patterns (via `former` crate)

**What it is:** Fluent API for constructing TemplateArchive and related types.

**Why excluded:**
- Not core functionality, just convenience
- Current direct construction API is simple enough:
  ```rust
  let mut archive = TemplateArchive::new("name");
  archive.add_parameter(ParameterDescriptor { ... });
  archive.set_value("key", value);
  ```
- Builder pattern adds complexity with derive macros
- Would require `former` dependency

**Verdict:** Direct construction is sufficient for a simple library.

---

### 4. Other Excluded Features

The following features are also out of scope for simplicity:

**Performance Optimizations:**
- Template caching
- Streaming large files
- Arena allocation
- *Verdict:* Premature optimization. No evidence these are needed.

**Quality Metrics:**
- Test coverage metrics (cargo tarpaulin)
- Performance benchmarks (criterion)
- Memory profiling (valgrind)
- *Verdict:* These are development tools, not library features. Run manually when needed.

**Enhanced Reporting:**
- Detailed MaterializationReport with per-file details
- File hashes in reports
- *Verdict:* Basic reporting is sufficient. Can add later if users request.

**TOML Security:**
- TOML bomb protection (file size limits)
- Template injection sanitization
- *Verdict:* No longer relevant without TOML parameter files. Trust Handlebars.

---

## What Remains (Core Library Features)

After excluding the above, genfile_core focuses on:

✅ **Template Processing:**
- TemplateValue trait for custom types
- Handlebars renderer
- Parameter discovery and validation
- Template rendering

✅ **File Operations:**
- Binary and text file support (all bytes 0x00-0xFF)
- JSON/YAML serialization (self-contained genfiles)
- Directory packing/unpacking
- External content references (FileRef/UrlRef)

✅ **Archive System:**
- Self-contained TemplateArchive
- Parameter definitions and values inside genfile
- Materialization to filesystem
- Real and in-memory filesystem abstractions

✅ **Security:**
- Path traversal validation (prevents `..` attacks)
- Typed error handling

**Result:** Simple, focused library with ~95% of planned features complete.

---

## For Application Developers

If you need the excluded features:

**Interactive Prompting:** Implement in your application:
```rust
let undefined = archive.get_undefined_parameters();
for param_name in undefined {
  print!("Enter value for '{}': ", param_name);
  let value = read_user_input();
  archive.set_value(param_name, value);
}
archive.save_to_file("template.yaml")?; // Save values back to genfile
```

**TOML Merging:** Use `toml_edit` directly in your application if needed.

**Builder Patterns:** Use direct construction - it's simpler for this library's scope.

---

**Last updated:** 2025-10-19
