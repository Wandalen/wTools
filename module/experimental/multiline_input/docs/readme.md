# Documentation

## Scope

Persistent development knowledge for the multiline_input crate.

This directory contains architecture decisions, design explorations, and investigation results that explain WHY the crate is designed the way it is.

## Organization

- `architecture.md` - Core design decisions and their rationale

## Navigation

**For understanding the crate**:
- **Why trait-based DI?** See `architecture.md` → Trait-Based Dependency Injection
- **Why MockTerminal?** See `architecture.md` → Non-Fragile Testing Solution
- **Testing strategy?** See `../tests/readme.md`
- **Requirements?** See `../spec.md`

## Content Guidelines

**Suitable for docs/**:
- Architecture decision records (ADRs)
- Design explorations and investigations
- Performance analysis
- Security considerations
- Historical context for design decisions

**NOT suitable for docs/** (belongs elsewhere):
- Project onboarding → `../readme.md`
- Requirements → `../spec.md`
- Testing strategy → `../tests/readme.md`
- Implementation details → source code comments
- Temporary investigations → `./-knowledge/`
