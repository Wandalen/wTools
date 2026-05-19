# GenFile CLI Documentation

### Scope

- **Purpose**: Authoritative CLI documentation for the genfile template archive management tool
- **Responsibility**: Documents all commands, parameters, types, parameter groups, and workflow scenarios
- **In Scope**: Commands, parameters, types, parameter groups, and workflow documentation for genfile CLI
- **Out of Scope**: Implementation internals, Rust API, test infrastructure — see `src/` and `tests/`

### Completion Matrix

| File | L1 | L2 | L3 | L4 | L5 | Status |
|------|----|----|----|----|----|----|
| readme.md | ✅ | ✅ | ➖ | ➖ | ➖ | Complete |
| command/readme.md | ✅ | ✅ | ✅ | ➖ | ➖ | Complete |
| command/archive.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| command/content.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| command/file.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| command/param_mgmt.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| command/value.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| command/operations.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| param.md | ✅ | ✅ | ✅ | ➖ | ➖ | Complete |
| dictionary.md | ➖ | ✅ | ✅ | ➖ | ➖ | Complete |
| type.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| param_group.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| env_param.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| config_param.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| workflow_scenario.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| format.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| tutorial.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |
| maintenance.md | ➖ | ➖ | ✅ | ➖ | ➖ | Complete |

**Current Level:** L3 (Specification Complete)
**Design Completeness:** 75%
**Implementation Status:** 100% ✅ all commands implemented

### Quick Navigation

**By Layer:**
- [Commands](command/readme.md) - All available operations
- [Parameters](param.md) - Input controls and configuration
- [Types](type.md) - Type system and validation rules
- [Parameter Groups](param_group.md) - Reusable parameter sets
- [Dictionary](dictionary.md) - Domain terminology glossary

**By Namespace:**
- [Archive Operations](command/archive.md) - Create, load, save archives
- [Content Management](command/content.md) - Inline/reference content control
- [File Operations](command/file.md) - Add, remove, list files
- [Parameter Management](command/param_mgmt.md) - Define parameters
- [Value Operations](command/value.md) - Set parameter values
- [Core Operations](command/operations.md) - Materialize, pack, analyze

**By Use Case:**
- Creating archives: [.archive.new](command/archive.md#command--5-archivenew), [.archive.from_directory](command/archive.md#command--8-archivefrom_directory)
- Managing content: [.file.add](command/file.md#command--12-fileadd), [.content.internalize](command/content.md#command--9-contentinternalize)
- Rendering templates: [.materialize](command/operations.md#command--16-materialize), [.unpack](command/operations.md#command--17-unpack)

**Configuration:**
- [Environment Variables](env_param.md) - Session-level parameter defaults
- [Config File](config_param.md) - Persistent parameter defaults

**Learning:**
- [Tutorial](tutorial.md) - Hands-on lessons for beginners
- [Workflow Scenarios](workflow_scenario.md) - End-to-end usage patterns
- [Parameter Groups](param_group.md) - Shared parameter sets and semantic coherence

### Documentation Structure

```
docs/cli/
├── readme.md              # This file (navigation hub + Completion Matrix)
├── tutorial.md            # Hands-on lessons for beginners
├── command/               # Namespace-organized command specifications
│   ├── readme.md          # Commands index (all commands) + Quick Reference
│   ├── archive.md         # Archive lifecycle
│   ├── content.md         # Content management
│   ├── file.md            # File operations
│   ├── param_mgmt.md      # Parameter definitions
│   ├── value.md           # Parameter values
│   └── operations.md      # Core operations
├── param.md               # Parameter specifications
├── type.md                # Type system definitions
├── param_group.md         # Shared parameter sets
├── dictionary.md          # Domain terminology
├── env_param.md           # Environment variable parameter mechanism
├── config_param.md        # Configuration file parameter mechanism
├── workflow_scenario.md   # Multi-command usage scenarios
├── format.md              # Output format catalog
└── maintenance.md         # Update procedures when commands change
```

### Responsibility Table

| File | Responsibility |
|------|----------------|
| [command/readme.md](command/readme.md) | Command index and Quick Reference |
| [param.md](param.md) | Parameter specifications and constraints |
| [type.md](type.md) | Semantic type definitions and validation rules |
| [param_group.md](param_group.md) | Shared parameter group definitions |
| [dictionary.md](dictionary.md) | Domain terminology glossary |
| [tutorial.md](tutorial.md) | Hands-on beginner lessons |
| [env_param.md](env_param.md) | Environment variable configuration mechanism |
| [config_param.md](config_param.md) | Configuration file mechanism and discovery |
| [workflow_scenario.md](workflow_scenario.md) | End-to-end multi-command usage scenarios |
| [format.md](format.md) | Output format catalog and verbosity behavior |
| [maintenance.md](maintenance.md) | Update procedures when commands change |
| [command/](command/readme.md) | Namespace-organized command specifications |

### Quick Start

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

### Documentation Principles

1. **Three-Layer Separation:** Commands, parameters, and types in orthogonal layers
2. **Type Safety:** All parameters use semantic newtypes (VerbosityLevel not u8)
3. **Bidirectional Cross-References:** Commands ↔ parameters ↔ types
4. **Semantic Parameter Groups:** Universal Output Control, Universal Execution Control, Filesystem Filtering
5. **Namespace Structure:** Entity Signals score ≥8 for command entity → namespace structure selected

### Further Reading

- [GenFile Core](../../src/lib.rs) - Implementation
- [Tests](../../tests/) - Usage examples
