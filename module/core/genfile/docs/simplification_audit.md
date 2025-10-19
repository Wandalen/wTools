# Simplification Audit - Feature Removal Analysis

**Date:** 2025-10-19
**Goal:** Identify features to remove for simplicity
**Principle:** genfile_core should be a simple, focused template library

---

## All Unimplemented/Partial Features (30 total)

| # | Feature | Category | Verdict | Reason |
|---|---------|----------|---------|--------|
| 1 | Path traversal validation | Security | ✅ **KEEP** | Simple security check, prevents `..` attacks |
| 2 | README.md improvements | Documentation | ✅ **KEEP** | Documentation, not code |
| 3 | **TomlExtend write mode** | Complex Feature | ❌ **REMOVE** | User explicitly doesn't want this complexity |
| 4 | **TOML merging with toml_edit** | Complex Feature | ❌ **REMOVE** | Only needed for TomlExtend |
| 5 | TOML bomb protection | Security | ⚠️ **MAYBE KEEP** | Simple file size check, prevents DoS |
| 6 | toml_edit dependency | Dependency | ❌ **REMOVE** | Only needed for TomlExtend |
| 7 | Test coverage metrics | Quality Metric | ❌ **REMOVE** | Nice but not core functionality |
| 8 | FR10: Write Mode Support | Complex Feature | ❌ **REMOVE** | Partial only because TomlExtend missing |
| 9 | FR11: TOML Merging Logic | Complex Feature | ❌ **REMOVE** | Related to TomlExtend |
| 10 | **FR6: Interactive Prompting** | UX Feature | ❌ **REMOVE** | Calling app's responsibility |
| 11 | Interactive prompting stdin | UX Feature | ❌ **REMOVE** | Duplicate of #10 |
| 12 | Detailed MaterializationReport | Enhanced Reporting | ❌ **REMOVE** | Nice but not core |
| 13 | Per-file error details | Enhanced Reporting | ❌ **REMOVE** | Nice but not core |
| 14 | Performance benchmarks | Quality Metric | ❌ **REMOVE** | Nice but not core |
| 15 | Memory profiling | Quality Metric | ❌ **REMOVE** | Nice but not core |
| 16 | API documentation | Documentation | ✅ **KEEP** | Documentation, not code |
| 17 | Examples | Documentation | ✅ **KEEP** | Documentation, not code |
| 18 | **FR19: Builder Pattern Support** | Convenience | ❌ **REMOVE** | Adds complexity, not essential |
| 19 | TemplateArchive::former() | Convenience | ❌ **REMOVE** | Part of builder patterns |
| 20 | TemplateFile::former() | Convenience | ❌ **REMOVE** | Part of builder patterns |
| 21 | ParameterDescriptor::former() | Convenience | ❌ **REMOVE** | Part of builder patterns |
| 22 | Parameters::former() | Convenience | ❌ **REMOVE** | Part of builder patterns |
| 23 | former dependency | Dependency | ❌ **REMOVE** | Only needed for builders |
| 24 | File hashes in report | Enhanced Reporting | ❌ **REMOVE** | Nice but not core |
| 25 | spec.md outdated | Documentation | ✅ **KEEP** | Documentation task |
| 26 | Template injection sanitization | Security | ⚠️ **MAYBE KEEP** | Security warning, low priority |
| 27 | Template caching | Performance Opt | ❌ **REMOVE** | Premature optimization |
| 28 | Migration guide | Documentation | ❌ **REMOVE** | Not needed for library |
| 29 | Streaming large files | Performance Opt | ❌ **REMOVE** | Premature optimization |
| 30 | Arena allocation | Performance Opt | ❌ **REMOVE** | Premature optimization |

---

## Summary by Category

### ❌ REMOVE (21 features)

**Complex Features (user doesn't want):**
- TomlExtend write mode
- TOML merging with toml_edit
- FR10: Write Mode Support (only partial because TomlExtend missing)
- FR11: TOML Merging Logic
- toml_edit dependency

**Interactive Prompting (application's responsibility):**
- FR6: Interactive Prompting
- Interactive prompting stdin

**Builder Patterns (adds complexity):**
- FR19: Builder Pattern Support
- TemplateArchive::former()
- TemplateFile::former()
- ParameterDescriptor::former()
- Parameters::former()
- former dependency

**Performance Optimizations (premature):**
- Template caching
- Streaming large files
- Arena allocation

**Quality Metrics (nice but not core):**
- Test coverage metrics
- Performance benchmarks
- Memory profiling

**Enhanced Reporting (nice but not core):**
- Detailed MaterializationReport
- Per-file error details
- File hashes in report

**Documentation (not features):**
- Migration guide

### ✅ KEEP (7 items)

**Security (simple checks):**
- Path traversal validation

**Documentation (not code features):**
- README.md improvements
- API documentation
- Examples
- spec.md rewrite

### ⚠️ MAYBE KEEP (2 features)

**Simple Security:**
- TOML bomb protection (simple file size limit)
- Template injection sanitization (documentation warning)

---

## Detailed Verdicts

### ❌ REMOVE: TomlExtend & Related (6 features)

**Features:**
1. TomlExtend write mode
2. TOML merging with toml_edit
3. FR10: Write Mode Support
4. FR11: TOML Merging Logic
5. toml_edit dependency
6. (Related spec sections)

**Reason:**
- User explicitly said "we don't need such features for now. we need simplicity"
- Complex merge algorithm with edge cases
- Only useful for project scaffolding tools
- genfile_core is a template library, not a project regeneration tool
- Adds dependency and complexity

**Impact:**
- Removes most complex unimplemented feature
- Eliminates toml_edit dependency
- Simplifies WriteMode to just `Rewrite`
- Removes 4 FR requirements from spec

**Current code:** WriteMode enum exists with only Rewrite implemented, easy to clean up

---

### ❌ REMOVE: Interactive Prompting (2 features)

**Features:**
1. FR6: Interactive Prompting
2. Interactive prompting stdin

**Reason:**
- Not library's responsibility - application should prompt
- genfile_core is for template processing, not user interaction
- Calling app can easily implement:
  ```rust
  // Application's responsibility:
  let undefined = archive.get_undefined_parameters();
  for param in undefined {
    let value = prompt_user(param); // App's code
    archive.set_value(param, value);
  }
  ```
- Keeps library focused on template processing

**Impact:**
- Removes stdin dependency
- Removes user interaction logic
- Library stays pure (no I/O besides file operations)

---

### ❌ REMOVE: Builder Patterns (6 features)

**Features:**
1. FR19: Builder Pattern Support
2. TemplateArchive::former()
3. TemplateFile::former()
4. ParameterDescriptor::former()
5. Parameters::former()
6. former dependency

**Reason:**
- Not core functionality, just convenience
- Adds complexity with derive macros
- Current API with direct field access works fine
- Violates simplicity goal
- Builders useful for complex APIs, but genfile_core API is simple

**Current usage:**
```rust
// Current (simple):
let mut archive = TemplateArchive::new("name");
archive.add_parameter(ParameterDescriptor { ... });
archive.set_value("key", value);

// Builder (more complex, not needed):
let archive = TemplateArchive::former()
  .name("name")
  .parameter().name("key").end()
  .form();
```

**Impact:**
- Removes former dependency
- Simplifies API surface
- Less code to maintain

---

### ❌ REMOVE: Performance Optimizations (3 features)

**Features:**
1. Template caching
2. Streaming large files
3. Arena allocation

**Reason:**
- Premature optimization
- No evidence these are needed
- Adds complexity without proven benefit
- Simple library should be simple, optimize later if needed

**Current performance:** No complaints, 141 tests run in 0.03s

**Impact:**
- Keeps codebase simple
- Can add later if benchmarks show need

---

### ❌ REMOVE: Quality Metrics (3 features)

**Features:**
1. Test coverage metrics
2. Performance benchmarks
3. Memory profiling

**Reason:**
- These are development tools, not library features
- Not tracked as "features" - these are process improvements
- Can run manually when needed (cargo tarpaulin, criterion, valgrind)
- Don't belong in feature list

**Impact:**
- Cleaner feature list
- Still can run these tools, just not tracking as features

---

### ❌ REMOVE: Enhanced Reporting (3 features)

**Features:**
1. Detailed MaterializationReport
2. Per-file error details
3. File hashes in report

**Reason:**
- Basic MaterializationReport exists and works
- File-by-file details add complexity
- Hashes not needed for simple template library
- Can add later if users request

**Current:** Basic report returns file count, works fine

**Impact:**
- Keeps reporting simple
- Reduces feature scope

---

### ✅ KEEP: Path Traversal Validation

**Feature:** Reject paths with `..` to prevent directory traversal

**Reason:**
- Simple security check (~5 lines of code)
- Prevents malicious templates from writing outside target directory
- Easy to implement:
  ```rust
  fn validate_path(path: &Path) -> Result<(), Error> {
    if path.components().any(|c| c == Component::ParentDir) {
      return Err(Error::InvalidPath(".. not allowed".into()));
    }
    Ok(())
  }
  ```

**Verdict:** ✅ KEEP - simple, important security feature

---

### ✅ KEEP: Documentation Tasks (4 items)

**Items:**
1. README.md improvements
2. API documentation
3. Examples
4. spec.md rewrite

**Reason:**
- These are documentation, not code features
- Important for usability
- Don't add code complexity

**Verdict:** ✅ KEEP - but these are documentation tasks, not features

---

### ⚠️ MAYBE KEEP: TOML Bomb Protection

**Feature:** File size limit validation

**Reason to keep:**
- Simple check (~3 lines)
- Prevents DoS from huge TOML files
- No complexity:
  ```rust
  const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
  if file_size > MAX_FILE_SIZE {
    return Err(Error::FileTooLarge);
  }
  ```

**Reason to remove:**
- Not reading TOML for parameters anymore (removed that feature)
- Only relevant if user provides TOML template content
- Unlikely attack vector

**Verdict:** ⚠️ **SUGGEST REMOVE** - no longer relevant without TOML parameter files

---

### ⚠️ MAYBE KEEP: Template Injection Sanitization

**Feature:** Security warnings for malicious template syntax

**Reason to keep:**
- Handlebars allows some expressions that could be dangerous
- Good to warn users

**Reason to remove:**
- Very low priority (Score: 6)
- Complex to implement properly
- Handlebars is designed for this, trust it
- Users should control their templates

**Verdict:** ⚠️ **SUGGEST REMOVE** - trust Handlebars, users control templates

---

## Recommendations

### Phase 1: Remove Complex Features (High Priority)

**Remove from spec.md:**
- FR6 (Interactive Prompting)
- FR10 (Write Mode Support - change to "Single write mode: Rewrite")
- FR11 (TOML Merging Logic)
- FR19 (Builder Pattern Support)

**Remove from docs/missing_features.md:**
- Section 2: TomlExtend Mode
- Section 3: Builder Patterns
- Update section 1: Interactive Prompting (or remove entirely)

**Remove from docs/status.md:**
- 11 feature rows about TomlExtend, Interactive, Builders, Quality Metrics, Performance Opts

**Remove from Cargo.toml dependencies:**
- `former` (not used)
- `toml_edit` (not used)

### Phase 2: Simplify Documentation (Medium Priority)

**Update spec.md:**
- Simplify FR requirements from 20 → ~12
- Remove all references to removed features
- Update success metrics

**Update docs/status.md:**
- Remove quality metrics (coverage, benchmarks, profiling) from feature list
- Remove enhanced reporting features
- Remove performance optimizations
- Keep only: Path traversal validation + documentation tasks

### Phase 3: What Remains (Core Library)

**After removal, genfile_core features:**

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
- Path traversal validation (to implement)
- Typed error handling

**Total:** ~130 features implemented, ~3 to implement, ~23 removed

---

## Statistics After Removal

| Metric | Before | After Removal | Change |
|--------|--------|---------------|--------|
| Total features | 156 | ~133 | -23 removed |
| Complete | 126 (80.8%) | 126 (94.7%) | +13.9% |
| Missing/Partial | 30 (19.2%) | 7 (5.3%) | -14.1% |
| Functional Requirements | 20 | ~12 | -8 removed |
| Dependencies | 10 | 8 | -2 (former, toml_edit) |

**Result:** Simple, focused library with ~95% feature completion

---

## Conclusion

**Recommend removing 21-23 features:**
- TomlExtend & TOML merging (6 features)
- Interactive prompting (2 features)
- Builder patterns (6 features)
- Performance optimizations (3 features)
- Quality metrics (3 features)
- Enhanced reporting (3 features)
- TOML bomb protection (1 feature, no longer relevant)
- Template injection sanitization (1 feature, low value)

**Keep only:**
- Path traversal validation (1 feature to implement)
- Documentation tasks (4 tasks, not code features)

**Result:** genfile_core becomes a simple, focused template processing library without unnecessary complexity.
