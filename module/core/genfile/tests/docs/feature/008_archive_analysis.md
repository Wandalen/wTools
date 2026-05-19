# Feature Spec: Archive Analysis

### Scope

- **Element:** `feature/008_archive_analysis`
- **Source:** `docs/feature/008_archive_analysis.md`
- **Prefix:** `FT-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | info_displays_archive_metadata | nominal | ✅ |
| FT-02 | status_shows_readiness_summary | nominal | ✅ |
| FT-03 | discover_parameters_finds_placeholders | nominal | ✅ |
| FT-04 | analyze_produces_comprehensive_report | nominal | ✅ |
| FT-05 | discover_finds_undefined_placeholders | nominal | ✅ |

---

### FT-01: info displays archive metadata

- **Given:** An archive with name `my-template` and description `A test template` is loaded
- **When:** `.info` is run
- **Then:** Exit code 0; output contains `my-template` and `A test template`
- **Tests:** `tests/analysis_test.rs`

### FT-02: status shows readiness summary

- **Given:** An archive with one mandatory parameter `name` with no value set is loaded
- **When:** `.status` is run
- **Then:** Exit code 0; output indicates the archive is not ready (missing mandatory value)
- **Tests:** `tests/analysis_test.rs`

### FT-03: discover.parameters finds template placeholders

- **Given:** An archive with a file containing `{{author}}` and `{{version}}` placeholders
- **When:** `.discover.parameters` is run
- **Then:** Exit code 0; output lists both `author` and `version` as discovered parameters
- **Tests:** `tests/analysis_test.rs`

### FT-04: analyze produces comprehensive report

- **Given:** An archive with files, parameters, and values loaded
- **When:** `.analyze` is run
- **Then:** Exit code 0; output includes file count, parameter list, value status, and readiness summary
- **Tests:** `tests/analysis_test.rs`

### FT-05: discover finds placeholders not formally defined

- **Given:** An archive with `{{undeclared}}` in a template file but no `.parameter.add name::undeclared` defined
- **When:** `.discover.parameters` is run
- **Then:** Exit code 0; `undeclared` appears in output marked as discovered but not defined
- **Tests:** `tests/analysis_test.rs`
