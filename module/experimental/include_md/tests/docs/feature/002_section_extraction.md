# Feature Spec: Section Extraction

**Source:** `docs/feature/002_section_extraction.md`
**Test file:** `tests/section_extraction.rs`
**Case prefix:** `FT-`

## Overview Table

| Case | Name | Status |
|------|------|--------|
| FT-1 | Exact heading match extracts correct section | ✅ |
| FT-2 | Section boundary stops at next equal-or-higher heading | ✅ |
| FT-3 | Nested subsections included in extracted section | ✅ |
| FT-4 | First occurrence wins for duplicate heading | ✅ |
| FT-5 | Case-sensitive match — wrong case is compile error | ✅ |
| FT-6 | Heading not found is compile error | ✅ |
| FT-7 | Missing file is compile error | ✅ |
| FT-8 | One argument (missing heading) is compile error | ✅ |
| FT-9 | Backtick fenced code block heading not a boundary | ✅ |
| FT-10 | Tilde fenced code block heading not a boundary | ✅ |
| FT-11 | Empty section body returns heading and blank line | ✅ |

---

### FT-1: Exact heading match extracts correct section

- **Given:** `tests/fixture/multi_section.md` contains `## Configuration` with known content
- **When:** `include_md_section!("tests/fixture/multi_section.md", "## Configuration")` is evaluated at compile time
- **Then:** The macro returns the section starting at `## Configuration` and ending before `# Notes`; content matches `"## Configuration\n\nSet options here.\n\n"`

### FT-2: Section boundary stops at next equal-or-higher heading

- **Given:** `tests/fixture/multi_section.md` has `## Installation` followed by sibling `## Configuration`
- **When:** `include_md_section!("tests/fixture/multi_section.md", "## Installation")` is evaluated
- **Then:** Extraction terminates at `## Configuration` (same level); result is `"## Installation\n\nRun cargo add.\n\n"` without `## Configuration` content

### FT-3: Nested subsections included in extracted section

- **Given:** `tests/fixture/multi_section.md` has `# Introduction` containing `## Overview` and `### Detail`
- **When:** `include_md_section!("tests/fixture/multi_section.md", "# Introduction")` is evaluated
- **Then:** Result includes `## Overview` and `### Detail` content; all nested content is present up to the next `# Usage` heading

### FT-4: First occurrence wins for duplicate heading

- **Given:** `tests/fixture/multi_section.md` contains two `# Notes` headings with different content
- **When:** `include_md_section!("tests/fixture/multi_section.md", "# Notes")` is evaluated
- **Then:** The macro returns the first `# Notes` section only; the second occurrence's content (`"second Notes section"`) is absent from the result

### FT-5: Case-sensitive match — wrong case is compile error

- **Given:** `tests/fixture/multi_section.md` contains `# Introduction` (capital I)
- **When:** Code containing `include_md_section!(..., "# introduction")` (lowercase i) is compiled
- **Then:** Compilation fails because the heading argument does not match any heading verbatim; no section is extracted

### FT-6: Heading not found is compile error

- **Given:** A valid file path and a heading string that does not appear in the file
- **When:** Code containing `include_md_section!(path, "## DoesNotExist__XYZ")` is compiled
- **Then:** Compilation fails with a "heading not found" error at the macro invocation site

### FT-7: Missing file is compile error

- **Given:** A file path that does not refer to any existing file
- **When:** Code containing `include_md_section!("does_not_exist.md", "# Heading")` is compiled
- **Then:** Compilation fails with a file-not-found error at the macro invocation site

### FT-8: One argument (missing heading) is compile error

- **Given:** Only one string literal argument (file path) is provided; the heading argument is omitted
- **When:** Code containing `include_md_section!("some.md")` is compiled
- **Then:** Compilation fails; the macro requires exactly two string literal arguments

### FT-9: Backtick fenced code block heading not a boundary

- **Given:** `tests/fixture/edge_cases.md` contains `## H2 With Code Block` which encloses a backtick fenced block containing `## this looks like H2 but is code`
- **When:** `include_md_section!("tests/fixture/edge_cases.md", "## H2 With Code Block")` is evaluated
- **Then:** The heading line inside the fence is not treated as a section boundary; the full section — including the code fence opener, the heading-like line, the fence closer, and the content after the fence — is returned correctly; `code_block_heading_not_a_boundary` test function passes (BUG-005 fix)

### FT-10: Tilde fenced code block heading not a boundary

- **Given:** `tests/fixture/edge_cases.md` contains `## H2 With Tilde Fence` which encloses a tilde-fenced block (`~~~python`) containing `## tilde fence heading`
- **When:** `include_md_section!("tests/fixture/edge_cases.md", "## H2 With Tilde Fence")` is evaluated
- **Then:** The heading line inside the tilde fence is not treated as a boundary; the full section including the tilde fence content and trailing content is returned; `tilde_fence_heading_not_a_boundary` test passes (BUG-005 fix)

### FT-11: Empty section body returns heading and blank line

- **Given:** `tests/fixture/edge_cases.md` contains `# Empty Section` with no body lines — only a blank line separating it from the next H1
- **When:** `include_md_section!("tests/fixture/edge_cases.md", "# Empty Section")` is evaluated
- **Then:** The macro returns `"# Empty Section\n\n"` — the heading line plus the blank line before the next heading; no error is raised for an empty section body
