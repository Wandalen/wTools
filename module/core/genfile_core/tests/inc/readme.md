# tests/inc/

Test modules for genfile_core crate, organized by functional domain.

### Responsibility Table

| File | Responsibility | Notes |
|------|----------------|-------|
| mod.rs | Module declarations and feature gate configuration | - |
| value_test.rs | Value type abstraction and TemplateValue trait | - |
| parameter_test.rs | ParameterDescriptor creation and metadata handling | - |
| values_test.rs | Values collection operations and serialization | - |
| renderer_test.rs | TemplateRenderer trait and Handlebars integration | feature-gated |
| file_descriptor_test.rs | FileDescriptor and WriteMode handling | - |
| filesystem_test.rs | FileSystem trait and MemoryFileSystem implementation | feature-gated |
| template_test.rs | Template creation and materialization | feature-gated |
| template_error_test.rs | Template error handling and edge cases | feature-gated |
| integration_test.rs | End-to-end multi-component workflows | feature-gated |
| archive_test.rs | Archive CRUD operations and serialization | feature-gated |
| archive_advanced_test.rs | Archive internalization and external sources | feature-gated |
| content_source_test.rs | ContentSource resolution and custom storage | feature-gated |
| content_source_example.rs | ContentSource usage examples and patterns | feature-gated |
| workflow_example.rs | Complete workflow demonstrations and serialization | feature-gated |

## Unique Responsibility Verification

Each test file has a distinct responsibility answering ONE question:

- **value_test.rs**: "How do Value types work?" (type abstraction)
- **parameter_test.rs**: "How do parameter descriptors work?" (metadata)
- **values_test.rs**: "How do Values collections work?" (collection operations)
- **renderer_test.rs**: "How does template rendering work?" (rendering engine)
- **file_descriptor_test.rs**: "How do file descriptors work?" (file metadata)
- **filesystem_test.rs**: "How does filesystem abstraction work?" (filesystem trait)
- **template_test.rs**: "How do templates work?" (template operations)
- **template_error_test.rs**: "How does template error handling work?" (error cases)
- **integration_test.rs**: "How do components integrate?" (multi-component)
- **archive_test.rs**: "How do archives work?" (core CRUD)
- **archive_advanced_test.rs**: "How does archive internalization work?" (advanced operations)
- **content_source_test.rs**: "How do content sources work?" (resolution/storage)
- **content_source_example.rs**: "How to use content sources?" (usage patterns)
- **workflow_example.rs**: "How to build complete workflows?" (end-to-end examples)

**One-Second Test Result**: ✅ No overlap detected - each file has unique Input→Output signature

## File Size Compliance

Per test_organization.rulebook.md:
- **MUST**: All files <1500 lines ✅
- **SHOULD**: Files 750-1000 lines (1 file at 830 lines)

**Status**: All files compliant with size constraints

## Feature Gates

| Feature | Files Affected | Purpose |
|---------|----------------|---------|
| `renderer` | renderer_test.rs | Handlebars template rendering |
| `filesystem` | filesystem_test.rs | FileSystem trait implementation |
| `template` | template_test.rs, template_error_test.rs | Template processing |
| `archive` | archive_test.rs, archive_advanced_test.rs, integration_test.rs, workflow_example.rs | Archive operations |
| `external_content` | content_source_test.rs, content_source_example.rs | External content resolution |

## Adding New Test Modules

Before creating new test file in `tests/inc/`:

1. **Check Responsibility Table**: Does any existing file already test this domain?
2. **Apply One-Second Test**: Does new file have identical Input→Output to existing file?
3. **If overlap detected**: Add tests to existing file instead
4. **If unique**: Add row to table above with single-sentence responsibility (3-10 words)
5. **Update both files**: Add module declaration to `mod.rs` AND update this table

See `../readme.md` for complete test organization guide.
