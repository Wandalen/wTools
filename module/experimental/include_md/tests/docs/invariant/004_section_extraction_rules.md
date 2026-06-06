# Invariant Spec: Section Extraction Rules

**Source:** `docs/invariant/004_section_extraction_rules.md`
**Test file:** `tests/section_extraction.rs`
**Case prefix:** `IN-`

## Overview Table

| Case | Name | Status |
|------|------|--------|
| IN-1 | Case-sensitive heading match enforced | ✅ |
| IN-2 | Level-aware section boundary enforced | ✅ |
| IN-3 | First occurrence wins for duplicate headings | ✅ |
| IN-4 | Code block heading never treated as section boundary | ✅ |
| IN-5 | ATX heading without space separator is not a valid heading | ✅ |
| IN-6 | Setext-style headings not recognized as section boundaries | ✅ |

---

### IN-1: Case-sensitive heading match enforced

- **Given:** `tests/fixture/multi_section.md` contains `# Introduction` (capital I); a test provides `"# introduction"` (lowercase i) as the heading argument
- **When:** Code with the lowercase heading is compiled
- **Then:** Compilation fails because no heading in the file matches `"# introduction"` verbatim; no partial or case-folded match is attempted

### IN-2: Level-aware section boundary enforced

- **Given:** `tests/fixture/multi_section.md` has a top-level `# Introduction` containing `## Overview` (H2) and `### Detail` (H3), followed by `# Usage` (H1)
- **When:** `include_md_section!(..., "# Introduction")` is evaluated
- **Then:** Extraction includes `## Overview` and `### Detail` (deeper levels) but stops at `# Usage` (same level); the level boundary is enforced by heading depth comparison, not position alone

### IN-3: First occurrence wins for duplicate headings

- **Given:** `tests/fixture/multi_section.md` contains two `# Notes` headings; the first has content `"Final section with no subsections."`, the second has `"This is the second Notes section (duplicate heading test)."`
- **When:** `include_md_section!(..., "# Notes")` is evaluated
- **Then:** Result contains `"Final section with no subsections."` and excludes `"second Notes section"`; the implementation stops scanning after the first match

### IN-4: Code block heading never treated as section boundary

- **Given:** `tests/fixture/edge_cases.md` section `## H2 With Code Block` contains a backtick fenced block with `## this looks like H2 but is code`; `## H2 With Tilde Fence` similarly uses a tilde fence
- **When:** `include_md_section!(..., "## H2 With Code Block")` and `include_md_section!(..., "## H2 With Tilde Fence")` are evaluated
- **Then:** In both cases the heading-like line inside the fence is not treated as a section boundary; the code fence content is included in the output; extraction stops only at the next REAL same-level heading after the fence closes (`code_block_heading_not_a_boundary` and `tilde_fence_heading_not_a_boundary` tests pass, BUG-005 fix)

### IN-5: ATX heading without space separator is not a valid heading

- **Given:** `tests/fixture/edge_cases.md` section `# ATX No Space Section` contains the line `##NotAHeading here.` (no space after the `##`)
- **When:** `include_md_section!(..., "# ATX No Space Section")` is evaluated
- **Then:** `##NotAHeading here.` is treated as plain content and included in the output; it does not trigger a section boundary because it lacks the mandatory space after the `#` characters; extraction stops at `# Setext Style Section` (`atx_no_space_not_a_heading_boundary` test passes)

### IN-6: Setext-style headings not recognized as section boundaries

- **Given:** `tests/fixture/edge_cases.md` section `# Setext Style Section` contains `Setext Text` followed by `==========` (setext-style H1 underline)
- **When:** `include_md_section!(..., "# Setext Style Section")` is evaluated
- **Then:** Neither `Setext Text` nor `==========` starts with `#`, so neither is recognized by `heading_level()` as an ATX heading; both lines are included as plain content; the section extends to end of file (`setext_heading_not_a_boundary` test passes)
