# genfile_core Tests

Comprehensive test suite for the genfile_core template processing library.

## Organization Principles

This test suite follows domain-based organization as specified in `test_organization.rulebook.md`:

- **Domain-Based Structure**: Tests organized by WHAT we test (functional domains), not HOW we test (unit/integration)
- **Centralized Location**: All tests in `tests/` directory at crate root
- **Environmental Independence**: Tests do not depend on environment variables, network, external services, or system state
- **Size Constraints**: Test files MUST be <1500 lines, SHOULD be 750-1000 lines
- **No Disabled Tests**: All tests must pass or be removed (see test_organization.rulebook.md § "Disabled Test Management" for exception process)

## Directory Structure

```
tests/
├── readme.md                    # This file - organizational guide
├── tests.rs                     # Test harness entry point
├── smoke_test.rs                # Minimal smoke test
├── security.rs                  # Security/path traversal validation tests
├── manual/                      # Manual testing plan and procedures
└── inc/                         # Main test modules (domain-organized)
    ├── readme.md                # Responsibility table for inc/ modules
    ├── mod.rs                   # Module declarations
    ├── basic_test.rs            # Basic smoke tests
    ├── value_test.rs            # Value type tests
    ├── parameter_test.rs        # Parameter descriptor tests
    ├── values_test.rs           # Values collection tests
    ├── renderer_test.rs         # Template renderer tests
    ├── file_descriptor_test.rs  # File descriptor tests
    ├── filesystem_test.rs       # Filesystem abstraction tests
    ├── template_test.rs         # Template tests
    ├── template_error_test.rs   # Template error handling tests
    ├── integration_test.rs      # Integration/workflow tests
    ├── archive_test.rs          # Archive core functionality tests
    ├── archive_advanced_test.rs # Advanced archive operations tests
    ├── content_source_test.rs   # Content source tests
    ├── content_source_example.rs # Content source usage examples
    └── workflow_example.rs      # Complete workflow examples
```

## Test Domain Map

| Domain | Files | Test Count | Description |
|--------|-------|------------|-------------|
| **Core Types** | value_test, parameter_test, values_test | ~30 | Value abstraction, parameter descriptors, values collection |
| **Template System** | template_test, template_error_test, renderer_test | ~35 | Template processing, error handling, rendering engine |
| **Filesystem** | filesystem_test, file_descriptor_test | ~15 | Filesystem abstraction trait, file descriptor handling |
| **Archive System** | archive_test, archive_advanced_test | ~45 | Archive CRUD operations, internalization, serialization |
| **Content Sources** | content_source_test, content_source_example | ~20 | External content resolution, storage backends |
| **Security** | security.rs | 27 | Path traversal validation, malicious path detection |
| **Integration** | integration_test, workflow_example | ~15 | End-to-end workflows, multi-component integration |
**Total**: 169 passing tests across 7 functional domains

## Scope

### Responsibilities

The test suite validates:

1. **Functional Correctness**: All public API operations produce expected results
2. **Security Compliance**: Path traversal attacks are detected and blocked (FR18)
3. **Error Handling**: Invalid inputs produce appropriate error messages
4. **Serialization**: JSON/YAML roundtrip preservation for Archive and ContentSource
5. **Feature Gates**: Conditional functionality compiles correctly under all feature combinations
6. **Integration**: Multi-component workflows operate correctly end-to-end
7. **Documentation Examples**: Example code in documentation compiles and runs

### In Scope

- ✅ All trait implementations (TemplateRenderer, FileSystem, TemplateValue, etc.)
- ✅ Core data structures (Archive, Template, Values, Parameter, FileDescriptor)
- ✅ Path traversal security validation (FR18)
- ✅ External content source resolution and storage
- ✅ Template rendering with Handlebars engine
- ✅ Archive internalization and materialization
- ✅ JSON/YAML serialization roundtrip
- ✅ Error propagation and error message quality
- ✅ Feature-gated functionality (renderer, filesystem, template, archive, external_content)
- ✅ Example code validation

### Out of Scope

- ❌ Performance benchmarking (see benchkit crate for performance tests)
- ❌ Network operations (external content sources use mock resolvers in tests)
- ❌ Concurrency/thread safety (single-threaded template processing)
- ❌ Builder pattern API (FR21 deferred until Former crate UX improves)
- ❌ Cross-platform filesystem behavior differences (tests use MemoryFileSystem)
- ❌ Unicode normalization edge cases (not a library responsibility)
- ❌ Handlebars renderer internals (third-party crate responsibility)

## Adding New Tests

Before adding new tests, follow this workflow:

### 1. Identify Test Domain

Determine which functional domain your test belongs to using the Test Domain Map above. If testing:
- Type system or value handling → Core Types domain
- Template processing or rendering → Template System domain
- File operations or abstractions → Filesystem domain
- Archive operations (CRUD, serialization) → Archive System domain
- External content loading → Content Sources domain
- Path validation or security → Security domain
- Multi-component workflows → Integration domain

### 2. Choose Target File

- **Existing Domain**: Add to corresponding file in `tests/inc/`
- **New Domain**: Create new file in `tests/inc/` and update this readme.md
- **Check File Size**: If target file >1000 lines, consider splitting by subdomain

### 3. Write Test

Follow test_organization.rulebook.md standards:
- Use descriptive test names (what behavior is validated, not implementation details)
- Ensure environmental independence (no env vars, network, or external dependencies)
- Add doc comment explaining test purpose if non-obvious
- For bug fixes: Include 5-section format (Root Cause, Why Not Caught, Fix Applied, Prevention, Pitfall)

### 4. Update Documentation

- Add test to appropriate domain count in Test Domain Map (this file)
- Update `tests/inc/readme.md` responsibility table if adding new file
- Update module declaration in `tests/inc/mod.rs`

### 5. Verify Compliance

Run full test suite:
```bash
w3 .test l::3
```

All tests must pass with zero warnings before merge.

## Test Coverage Status

**Current Coverage**: 169 tests (100% of FR1-FR20)

| Spec Requirement | Coverage | Notes |
|------------------|----------|-------|
| FR1-FR17 | 100% | Core functionality fully tested |
| FR18 (Security) | 100% | 27 dedicated path traversal tests |
| FR19 (Serialization) | 100% | JSON/YAML roundtrip tests |
| FR20 (External Content) | 100% | Content source resolution tests |

## Known Test Gaps

1. **Large Archive Performance**:
   - Current tests use small archives (<10 files, <10KB total)
   - Performance testing deferred to benchkit integration
   - Not a correctness concern, only performance characterization

2. **Real Network Content Sources**:
   - Tests use mock resolvers, not real HTTP/file I/O
   - Integration tests in downstream crates validate real I/O
   - Conscious decision to maintain environmental independence

## Maintenance

**Test Suite Owner**: genfile_core maintainers
**Review Frequency**: Weekly (automated via CI)
**Compliance Audit**: Monthly against test_organization.rulebook.md
**Last Updated**: 2026-01-05
