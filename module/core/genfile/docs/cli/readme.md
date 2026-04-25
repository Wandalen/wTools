# GenFile CLI Documentation

Complete CLI reference for the genfile template archive management tool.

## Quick Navigation

**By Layer:**
- [Commands](commands.md) - All available operations (24 commands)
- [Parameters](params.md) - Input controls and configuration (23 parameters)
- [Types](types.md) - Type system and validation rules (15 types)
- [Parameter Groups](parameter_groups.md) - Reusable parameter sets (3 groups)
- [Dictionary](dictionary.md) - Domain terminology glossary (9 terms)

**By Namespace:**
- [Archive Operations](commands/archive.md) - Create, load, save archives (4 commands)
- [Content Management](commands/content.md) - Inline/reference content control (3 commands)
- [File Operations](commands/file.md) - Add, remove, list files (4 commands)
- [Parameter Management](commands/param_mgmt.md) - Define parameters (3 commands)
- [Value Operations](commands/value.md) - Set parameter values (3 commands)
- [Core Operations](commands/operations.md) - Materialize, pack, analyze (7 commands)

**By Use Case:**
- Creating archives: [.archive.new](commands/archive.md#command-5-archivenew), [.archive.from_directory](commands/archive.md#command-8-archivefromdirectory)
- Managing content: [.file.add](commands/file.md#command-12-fileadd), [.content.internalize](commands/content.md#command-9-contentinternalize)
- Rendering templates: [.materialize](commands/operations.md#command-16-materialize), [.unpack](commands/operations.md#command-17-unpack)

**Learning:**
- [Tutorial](tutorial.md) - Hands-on lessons for beginners (4 lessons, 45-60 minutes)

## Documentation Principles

This documentation follows systematic CLI design principles:

1. **Three-Layer Separation:** Commands, parameters, and types in orthogonal layers
2. **Type Safety:** All parameters use semantic newtypes (VerbosityLevel not u8)
3. **Bidirectional Cross-References:** Commands ↔ parameters ↔ types
4. **Semantic Parameter Groups:** Universal Output Control, Universal Execution Control, Filesystem Filtering
5. **Namespace Structure:** Entity-driven organization (archive, content, file, parameter, value)

## Documentation Structure

```
docs/cli/
├── readme.md              # This file (navigation hub)
├── tutorial.md            # Hands-on lessons for beginners
├── commands.md            # Commands index table
├── commands/              # Namespace-organized command specs
│   ├── readme.md          # Namespace index
│   ├── archive.md         # Archive lifecycle (4 commands)
│   ├── content.md         # Content management (3 commands)
│   ├── file.md            # File operations (4 commands)
│   ├── param_mgmt.md      # Parameter definitions (3 commands)
│   ├── value.md           # Parameter values (3 commands)
│   └── operations.md      # Core operations (7 commands)
├── params.md              # Parameter specifications (23 parameters)
├── types.md               # Type system definitions (15 types)
├── parameter_groups.md    # Shared parameter sets (3 groups)
├── dictionary.md          # Domain terminology (9 terms)
└── maintenance.md         # Update procedures when commands change
```

### Responsibility Table

| File | Responsibility |
|------|----------------|
| [commands.md](commands.md) | Index of all 24 commands by namespace |
| [params.md](params.md) | Specifications for all 23 parameters |
| [types.md](types.md) | Semantic type definitions for 15 types |
| [parameter_groups.md](parameter_groups.md) | Three shared parameter group definitions |
| [dictionary.md](dictionary.md) | Domain terminology glossary (9 terms) |
| [tutorial.md](tutorial.md) | Hands-on beginner lessons (4 lessons) |
| [maintenance.md](maintenance.md) | Update procedures when commands change |
| [commands/](commands/readme.md) | Namespace-organized command specifications |

## Quick Start

**Create archive from directory:**
```bash
genfile .archive.from_directory source::"./templates" mode::reference
genfile .archive.save path::"template.yaml"
```

**Materialize template:**
```bash
genfile .archive.load path::"template.yaml"
genfile .value.set name::project_name value::"my-app"
genfile .materialize destination::"./output"
```

**Preview changes (dry run):**
```bash
genfile .materialize destination::"./preview" dry::1 verbosity::2
```

## Common Workflows

**Create archive from scratch:**
```bash
# 1. Create empty archive
genfile .archive.new name::"my-template"

# 2. Add files
genfile .file.add path::"main.rs" from_file::"src/main.rs"
genfile .file.add path::"readme.md" content::"# {{project_name}}"

# 3. Add parameters
genfile .parameter.add name::project_name mandatory::true

# 4. Save
genfile .archive.save path::"template.yaml"
```

**Convert directory to portable archive:**
```bash
# One-step approach
genfile .pack input::"./templates" output::"template.json"

# OR two-step approach
genfile .archive.from_directory source::"./templates" mode::inline
genfile .archive.save path::"template.json" pretty::1
```

**Load, modify, and materialize:**
```bash
# Load existing archive
genfile .archive.load path::"template.yaml"

# Set parameter values
genfile .value.set name::project_name value::"my-app"
genfile .value.set name::version value::"1.0.0"
genfile .value.set name::author value::"John Doe"

# Check readiness
genfile .status

# Materialize
genfile .materialize destination::"./output" verbosity::2
```

## Implementation Notes

**For Developers:** See [types.md](types.md) for newtype validation logic and parsing rules.

**For Users:** See [dictionary.md](dictionary.md) for domain terminology explanations.

## Parameter Groups Overview

### Universal Output Control
- **Parameter:** [`verbosity::`](params.md#parameter-1-verbosity)
- **Used By:** ALL 24 commands
- **Purpose:** Control output detail level (0-5)
- **Default:** 1 (normal)

### Universal Execution Control
- **Parameter:** [`dry::`](params.md#parameter-2-dry)
- **Used By:** 6 write operations
- **Purpose:** Preview mode (show changes without executing)
- **Default:** 0 (execute)

### Filesystem Filtering
- **Parameters:** [`recursive::`](params.md#parameter-10-recursive), [`include_pattern::`](params.md#parameter-17-includepattern), [`exclude_pattern::`](params.md#parameter-21-excludepattern)
- **Used By:** [.archive.from_directory](commands/archive.md#command-8-archivefromdirectory)
- **Purpose:** Control file discovery and filtering

## Navigation Tips

**Find Commands:**
1. By name: Use [commands index](commands.md#commands-index)
2. By namespace: Use [namespace groups](commands.md#namespace-groups)
3. By functionality: Use [quick navigation](commands.md#quick-navigation)

**Find Parameters:**
1. By name: Use [parameters index](params.md#parameters-index)
2. By category: Use [parameter categories](params.md#parameter-categories)
3. By frequency: Use [by frequency](params.md#quick-navigation)

**Find Types:**
1. By name: Use [types index](types.md#types-index)
2. By category: Use [type categories](types.md#type-categories)

**Find Concepts:**
1. Use [dictionary](dictionary.md) for domain terminology

## Documentation Statistics

| Layer | Files | Entities | Total Lines |
|-------|-------|----------|-------------|
| Commands | 7 | 24 commands | ~2,600 |
| Parameters | 1 | 23 parameters | ~1,500 |
| Types | 1 | 15 types | ~1,200 |
| Parameter Groups | 1 | 3 groups | ~400 |
| Dictionary | 1 | 9 terms | ~500 |
| Tutorial | 1 | 4 lessons | ~650 |
| Navigation | 2 | - | ~200 |
| **Total** | **14** | **77 entities** | **~7,050** |

## Architecture Compliance

**Principle 1: Structure Follows Scale**
- ✅ 24 commands ≥20 threshold → namespace structure used
- ✅ 6 namespace files (archive, content, file, param_mgmt, value, operations)

**Principle 2: Type Safety is Universal**
- ✅ 15 semantic newtypes defined
- ✅ All parameters wrapped in semantic types
- ✅ No bare primitives (String, u8, bool) in parameter specifications

**Principle 3: Cross-References are Bidirectional**
- ✅ Commands list parameters used
- ✅ Parameters list commands using them
- ✅ Types list parameters using them
- ✅ All cross-references include links

**Principle 4: Parameter Groups Must Be Semantically Coherent**
- ✅ 3 groups defined with semantic coherence test
- ✅ "Why NOT X" rationale for excluded parameters
- ✅ All groups pass semantic coherence test

**Principle 5: Documentation Serves Two Audiences**
- ✅ types.md for implementers (validation, parsing)
- ✅ commands.md, params.md for users (usage, examples)
- ✅ Clear separation maintained

## Further Reading

- [CLI Design Rulebook](https://github.com/Wandalen/wTools/blob/master/module/experimental/willbe/asset/template/module_template/docs/cli_design.md) - Design principles
- [GenFile Core](../../src/lib.rs) - Implementation
- [Tests](../../tests/) - Usage examples

---

**Documentation Version:** 1.0.0
**Last Updated:** 2026-04-25
**CLI Version:** genfile 0.4.0
