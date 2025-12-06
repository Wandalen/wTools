# Unilang Documentation

This directory contains persistent development knowledge, design decisions, and implementation guidance for the Unilang crate.

## Organization Principles

Documentation is organized by topic to reflect the nature of the content rather than rigid categorization. Each subdirectory contains related documentation with clear separation of concerns.

## Navigation

### Directory Structure

| Directory/File | Purpose | Description |
|----------------|---------|-------------|
| `analysis/` | Codebase analysis reports | API analysis, usability studies, pattern identification |
| `architecture/` | Architectural documentation | Design decisions, component structure, system architecture |
| `cli_definition_approaches.md` | CLI design patterns | Comprehensive guide to command definition approaches |
| `design_principles.md` | Core design philosophy | Guiding principles and design values |
| `development_rules.md` | Development guidelines | Rules and standards for contributors |
| `doc_test_status.md` | Documentation testing status | Current state of doc tests across the crate |
| `optimization_guide.md` | Performance optimization | Guidelines for improving performance |
| `performance.md` | Performance characteristics | Performance analysis and benchmarks |
| `quick_start.md` | Getting started guide | Quick introduction for new users |
| `roadmap.md` | Future development plans | Planned features and improvements |
| `features/` | Feature design documents | Detailed feature design and implementation guides |

### Quick Access

**New to Unilang?**
- Start with `quick_start.md` for a practical introduction
- Review `design_principles.md` to understand core philosophy

**Contributing?**
- Read `development_rules.md` for contribution guidelines
- Check `roadmap.md` to align with planned work

**Improving Performance?**
- Consult `optimization_guide.md` for best practices
- Review `performance.md` for current characteristics

**Understanding the Design?**
- Browse `architecture/` for system structure
- See `cli_definition_approaches.md` for command patterns
- Read `analysis/` for codebase insights

## Content Guidelines

### Belongs in `docs/`
- Architecture decision records and rationale
- Design explorations and investigations
- Performance analysis and optimization notes
- Integration guides and implementation patterns
- Development guidelines and best practices

### Does NOT Belong in `docs/`
- **Project onboarding** → See `readme.md` in project root
- **Requirements/specifications** → See `spec.md` in project root
- **Test plans** → See `tests/readme.md` or `tests/manual/readme.md`
- **Temporary investigation notes** → Use `./-knowledge/` directory

## Subdirectory Details

### `analysis/`
Contains comprehensive codebase analysis reports focusing on API usability, common patterns, and improvement opportunities. The analysis index (`analysis/index.md`) provides navigation to all analysis documents.

**Key Documents:**
- `api_analysis.md` - Detailed API surface analysis with boilerplate identification
- `usability_improvements.md` - User experience enhancement opportunities
- `index.md` - Navigation guide for all analysis documents

### `architecture/`
Documents architectural decisions, component relationships, and system design patterns. Focuses on WHY the system is structured as it is.

**Key Documents:**
- `benchmark_separation.md` - Rationale for benchmark organization

### `features/`
Contains detailed feature design documents that explain implementation approaches, usage patterns, and design decisions for specific features. These documents combine implementation details with usage guidance.

**Important Distinction:** This directory contains feature **design and implementation** documentation, not requirements specifications. Requirements specifications belong in `spec.md` at the project root.

**Current Contents:**
- `repl.md` - REPL feature design, implementation details, and usage guide

## Document Relationships

```
spec.md (root)
  └─ Defines WHAT the system must do (requirements)
     └─ docs/
        ├─ architecture/ → WHY design decisions were made
        ├─ features/ → HOW features are implemented
        ├─ analysis/ → HOW current patterns work
        ├─ optimization_guide.md → HOW TO improve performance
        └─ cli_definition_approaches.md → HOW TO define commands
```

## Maintenance

This readme must be updated when:
- New documentation files or directories are added
- Existing documentation is reorganized
- Documentation organization principles change
- Navigation patterns evolve

---

*For requirements and specifications, see `spec.md` in the project root.*
*For test documentation, see `tests/readme.md`.*
*For project onboarding, see `readme.md` in the project root.*
