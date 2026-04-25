# Feature Doc Entity

### Scope

- **Purpose**: Documents functional requirements and user-facing capabilities of the genfile_core library.
- **Responsibility**: Index of all feature doc instances for genfile_core.
- **In Scope**: Functional requirements and capabilities of the genfile_core library.
- **Out of Scope**: Non-functional constraints (→ `invariant/`), API contracts (→ `api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Template Value Trait](001_template_value_trait.md) | Defines how values convert to template strings | ✅ |
| 002 | [Default Value Type](002_default_value_type.md) | Built-in value enum for common data types | ✅ |
| 003 | [Parameter Definition](003_parameter_definition.md) | Parameter metadata with mandatory/default attributes | ✅ |
| 004 | [Parameter Collection](004_parameter_collection.md) | Collection of parameter descriptors with validation | ✅ |
| 005 | [Value Storage](005_value_storage.md) | Generic runtime storage for template parameter values | ✅ |
| 006 | [Template Renderer Trait](006_template_renderer_trait.md) | Pluggable template engine abstraction | ✅ |
| 007 | [Handlebars Renderer](007_handlebars_renderer.md) | Default Handlebars template engine implementation | ✅ |
| 008 | [File Descriptor](008_file_descriptor.md) | Specification for a single file to be generated | ✅ |
| 009 | [Write Mode Support](009_write_mode_support.md) | File write strategy for generated output | ✅ |
| 010 | [File System Trait](010_file_system_trait.md) | Abstraction over file I/O for testability | ✅ |
| 011 | [Real File System](011_real_file_system.md) | Production filesystem implementation | ✅ |
| 012 | [Memory File System](012_memory_file_system.md) | In-memory filesystem for testing | ✅ |
| 013 | [Template Holder](013_template_holder.md) | Low-level generic template processor | ✅ |
| 014 | [Template Generation](014_template_generation.md) | End-to-end file generation from template | ✅ |
| 015 | [Missing Mandatory Detection](015_missing_mandatory_detection.md) | Detection of unfilled required parameters | ✅ |
| 016 | [Typed Errors](016_typed_errors.md) | Comprehensive typed error enum for all failure modes | ✅ |
| 017 | [Archive Self-Containment](017_archive_self_containment.md) | Self-contained archive with embedded parameter values | ✅ |
