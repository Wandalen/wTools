# API Spec: include_md_section Macro

**Source:** `docs/api/002_include_md_section.md`
**Test file:** `tests/section_extraction.rs`
**Case prefix:** `AP-`

## Overview Table

| Case | Name | Status |
|------|------|--------|
| AP-1 | Two string literal arguments (path, heading) accepted | ✅ |
| AP-2 | Zero arguments rejected at compile time | ✅ |
| AP-3 | One argument rejected at compile time | ✅ |
| AP-4 | Valid file and heading → section returned as `&'static str` | ✅ |
| AP-5 | Heading not found produces compile error at invocation site | ✅ |
| AP-6 | Section includes all nested subsections within level boundary | ✅ |

---

### AP-1: Two string literal arguments (path, heading) accepted

- **Given:** A valid file and a heading string that exists in that file
- **When:** `include_md_section!("tests/fixture/multi_section.md", "# Introduction")` is evaluated at compile time
- **Then:** The macro expands without error; the matched section is returned as a `&'static str` constant

### AP-2: Zero arguments rejected at compile time

- **Given:** No arguments are passed to the macro
- **When:** Code containing `include_md_section!()` is compiled
- **Then:** `cargo check` exits non-zero; a syn parse error is reported at the invocation site; no binary is produced

### AP-3: One argument rejected at compile time

- **Given:** Only the path argument is provided; the heading argument is absent
- **When:** Code containing `include_md_section!("some.md")` is compiled
- **Then:** `cargo check` exits non-zero; the missing comma or heading token is rejected; no binary is produced

### AP-4: Valid file and heading → section returned as `&'static str`

- **Given:** `tests/fixture/multi_section.md` exists and contains `# Notes`
- **When:** `include_md_section!("tests/fixture/multi_section.md", "# Notes")` is used in a context requiring `&'static str`
- **Then:** The expanded value is accepted by the compiler as a `&'static str`; it equals the first `# Notes` section content

### AP-5: Heading not found produces compile error at invocation site

- **Given:** The heading argument does not match any heading in the file (including wrong-case and absent headings)
- **When:** Code containing `include_md_section!(path, "## DoesNotExist__XYZ")` is compiled
- **Then:** `cargo check` exits non-zero; a "heading not found" error is reported at the macro invocation site; no binary is produced

### AP-6: Section includes all nested subsections within level boundary

- **Given:** `tests/fixture/multi_section.md` has `# Introduction` containing `## Overview` and `### Detail`
- **When:** `include_md_section!("tests/fixture/multi_section.md", "# Introduction")` is evaluated
- **Then:** The returned string contains `## Overview` and `### Detail` content; extraction boundary is level-aware, inclusive of all nested subsections until the next H1
