# Invariant: Section Extraction Rules

### Scope

- **Purpose**: Define the three deterministic rules that govern how a section is identified and extracted from a markdown file.
- **Responsibility**: Documents the section extraction invariant — heading matching, boundary detection, and duplicate handling rules.
- **In Scope**: Heading match semantics, section boundary definition, duplicate heading behavior.
- **Out of Scope**: Path resolution (see invariant/001), error reporting (see invariant/002), size enforcement (see invariant/003), full-file inclusion.

### Invariant Statement

Section extraction follows exactly three rules, applied unconditionally:

1. **Heading match is case-sensitive and exact.** The heading argument must match the file heading verbatim, including all leading marker characters and any trailing whitespace. No normalization, case folding, or partial matching is performed.

2. **Section boundary is level-aware and inclusive.** Extraction begins at the matched heading line and ends immediately before the next heading of equal or greater depth. All content between those boundaries — including all nested subsections of any depth — is included in the output.

3. **First occurrence wins for duplicate headings.** When the same heading string appears more than once in the file, the first occurrence is extracted without error. No error is raised for duplicates.

### Enforcement Mechanism

The macro implementation scans the file line by line from the start. On the first line that matches the heading argument exactly, it records the heading level and begins accumulating output. Accumulation stops on the next line that is a heading of equal or lesser depth value (equal or greater significance level). Scanning stops at first match; subsequent occurrences are never examined.

### Violation Consequences

Case-insensitive matching would introduce ambiguity for markdown files that contain mixed-case headings with identical text. Level-unaware boundaries would truncate nested subsections, producing incomplete section output. Raising errors on duplicate headings would break on common documentation patterns where multiple sections share a subheading name (such as "Examples" or "Notes" appearing under different top-level sections).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/_blank/standard_lib.rs` | Placeholder; future home of macro entry points |
| doc | [api/002_include_md_section.md](../api/002_include_md_section.md) | Section macro contract referencing these rules |
| doc | [feature/002_section_extraction.md](../feature/002_section_extraction.md) | User-facing rationale for these design choices |
| doc | [invariant/002_compile_time_errors.md](002_compile_time_errors.md) | Error contract for heading-not-found condition |

### Sources

| File | Notes |
|------|-------|
| `spec.md` | Original specification §Section Extraction Behavior; deleted commit `c13cf485` (not migrated); recoverable from git history |
