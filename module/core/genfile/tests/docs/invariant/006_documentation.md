# Invariant Spec: Documentation

### Scope

- **Element:** `invariant/006_documentation`
- **Source:** `docs/invariant/006_documentation.md`
- **Prefix:** `IN-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IN-01 | readme_contains_quick_start_section | nominal | 🔶 deferred |
| IN-02 | doc_tests_compile_without_warnings | nominal | 🔶 deferred |

---

### IN-01: readme.md contains a quick start section

- **Given:** `readme.md` at the crate root is read
- **When:** The file is checked for a quick start section
- **Then:** A section headed "Quick Start" (or equivalent) is present with at least one end-to-end example
- **Tests:** none — see task/001_fill_test_surface_gaps.md

### IN-02: doc tests compile without warnings

- **Given:** The crate is built with `RUSTDOCFLAGS="-D warnings"`
- **When:** `cargo test --doc --all-features` is run
- **Then:** Exit code 0; no documentation warnings; all doc test examples compile and pass
- **Behavioral Divergence:** doc with broken example → compile error; valid doc → passes
- **Tests:** CI enforced via `w3 .test level::2`
