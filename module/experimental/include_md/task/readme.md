# Tasks

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `decisions.md` | Design decisions and resolved questions for include_md implementation |
| `unverified/` | Task files pending Verification Gate |
| `bug/` | Bug reports with root-cause analysis, MREs, and fix documentation |

## Tasks Index

| ID | File | State | Goal Summary |
|----|------|-------|--------------|
| 001 | [001_proc_macro_scaffolding.md](001_proc_macro_scaffolding.md) | 🎯 | Convert include_md to proc-macro crate with two #[proc_macro] stubs using macro_tools |
| 002 | [002_implement_include_md.md](002_implement_include_md.md) | 🎯 | Implement include_md! proc-macro: parse LitStr path, emit include_str! + const size assertion |
| 003 | [003_implement_include_md_section.md](003_implement_include_md_section.md) | 🎯 | Implement include_md_section! proc-macro: parse two LitStr args, line-by-line section extraction |
| 004 | [004_examples_and_e_criterion.md](004_examples_and_e_criterion.md) | 🎯 | Create include_md_trivial example demonstrating both macros; satisfy E criterion |
