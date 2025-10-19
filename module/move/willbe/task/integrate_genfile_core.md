# Change Proposal for `willbe`

### Task ID
* TASK-20251019-180000-IntegrateGenfileCore

### Requesting Context
* **Requesting Crate/Project:** `module/core/genfile_core`
* **Driving Feature/Task:** US1 - willbe Integration, SM1 - Adoption Success Metric
* **Link to Requester's Plan:** `module/core/genfile/spec.md` (User Stories section, Success Metrics section)
* **Date Proposed:** 2025-10-19

### Overall Goal of Proposed Change
* Replace willbe's existing `template.rs` module (472 lines) with `genfile_core` library to eliminate code duplication and provide a reusable, well-tested template processing foundation for the wTools ecosystem.

### Problem Statement / Justification
* **Code Duplication:** willbe currently contains 472 lines of template processing code in `template.rs` that duplicates functionality now available in the production-ready `genfile_core` library.
* **Maintainability:** Template logic is embedded in willbe rather than being a reusable library, making it harder to maintain and test.
* **Limited Functionality:** The current implementation lacks features that `genfile_core` provides:
  - Parameter discovery and analysis
  - Binary file support
  - JSON/YAML serialization of templates
  - External content sources (FileRef, UrlRef)
  - Pluggable renderers and filesystems
  - Comprehensive security validation (path traversal prevention)
* **Testing Difficulty:** Current template.rs is tightly coupled to willbe, making unit testing complex. genfile_core has 215 passing tests with 134% test-to-code ratio.

### Proposed Solution / Specific Changes

**1. Add genfile_core dependency**

File: `module/move/willbe/Cargo.toml`

```diff
[dependencies]
+ genfile_core = { workspace = true, features = [ "default" ] }
```

**2. Remove or deprecate template.rs module**

File: `module/move/willbe/src/template.rs`
- Mark as deprecated with pointer to genfile_core
- Or remove entirely and update imports

**3. Update code using template.rs**

Identify all locations in willbe that use the template module and migrate them to use genfile_core's API:

- Replace custom template types with `TemplateArchive` or `Template<V,R,FS>`
- Use `HandlebarsRenderer` instead of custom rendering
- Use `MemoryFileSystem` for testing, `RealFileSystem` for production
- Migrate parameter handling to use `ParameterDescriptor` and `Values`

**4. Run full test suite**

Ensure all existing willbe tests continue passing (SM3: Zero Regressions requirement).

### Expected Behavior & Usage Examples

**Before (current willbe code):**
```rust
// Custom template.rs implementation
use crate::template::{ Template, ... };
let tmpl = Template::new( ... );
// 472 lines of custom implementation
```

**After (using genfile_core):**
```rust
use genfile_core::
{
  TemplateArchive,
  WriteMode,
  Value,
  HandlebarsRenderer,
  RealFileSystem,
};

let mut archive = TemplateArchive::new( "project-template" );
archive.add_text_file( path, content, WriteMode::Rewrite );
archive.set_value( "project_name", Value::String( name ) );

let renderer = HandlebarsRenderer::new();
let mut fs = RealFileSystem::new();
archive.materialize_with_components( output_path, &renderer, &mut fs )?;
```

### Acceptance Criteria

1. ✅ genfile_core added as dependency to willbe
2. ✅ All code using template.rs migrated to genfile_core
3. ✅ template.rs removed or marked deprecated
4. ✅ All existing willbe tests pass (zero regressions)
5. ✅ New tests added for genfile_core integration if needed
6. ✅ Documentation updated to reflect the change

### Potential Impact & Considerations

**Breaking Changes:**
- Internal API changes only - willbe's external CLI interface should remain unchanged
- Template file formats should remain compatible

**Benefits:**
- Eliminate 472 lines of duplicated code
- Gain access to genfile_core's 215 passing tests
- Inherit security features (path traversal validation with 27 tests)
- Access to advanced features (binary files, serialization, external content)
- Better maintainability through separation of concerns

**Dependencies:**
- Adds genfile_core workspace dependency
- genfile_core is production-ready (94% complete, 215 tests passing)
- No breaking changes to willbe's external API expected

**Performance:**
- genfile_core designed for performance (NFR1 target: <100ms for 10KB templates)
- No significant performance impact expected

**Security:**
- Improved security through genfile_core's path traversal validation
- 27 dedicated security tests prevent directory escape attacks

**Testing:**
- Existing willbe tests must continue passing
- genfile_core brings 215 additional tests covering the template functionality

### Alternatives Considered

1. **Keep current template.rs implementation**
   - Rejected: Perpetuates code duplication and maintenance burden
   - Misses opportunity to create reusable library for wTools ecosystem

2. **Partial migration (use genfile_core alongside template.rs)**
   - Rejected: Creates confusion and doesn't eliminate duplication
   - Half measures don't achieve the goal of code reuse

3. **Extract template.rs into separate library instead of using genfile_core**
   - Rejected: genfile_core already exists, is tested, and production-ready
   - Would duplicate effort already completed

### Notes & Open Questions

**genfile_core Status:**
- Version: 0.1.0
- Completion: 94% (141/149 features)
- Tests: 215 passing (169 unit/integration + 46 doc tests)
- Documentation: Complete (README, API docs, 7 runnable examples)
- Security: Path traversal validation with 27 tests

**Migration Strategy:**
1. Add genfile_core dependency
2. Create migration branch
3. Update one willbe component at a time
4. Test incrementally
5. Verify zero regressions
6. Remove template.rs when migration complete

**Open Questions:**
1. Are there any willbe-specific template features not covered by genfile_core?
2. What is the timeline for this integration?
3. Who will own the migration work?
4. Should we version-lock genfile_core or track latest?

**Links:**
- genfile_core spec: `module/core/genfile/spec.md`
- genfile_core features: `module/core/genfile/docs/features.md`
- genfile_core examples: `module/core/genfile/examples/`
- Current template.rs: `module/move/willbe/src/template.rs`
