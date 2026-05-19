# CLI Spec: Analysis Commands

### Scope

- **Element:** Commands `1–4` (`.info`, `.discover.parameters`, `.status`, `.analyze`)
- **Source:** `docs/cli/command/operations.md`
- **Prefix:** `IT-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IT-11 | info_shows_archive_metadata | nominal | ✅ |
| IT-12 | status_ready_when_all_mandatory_set | nominal | ✅ |
| IT-13 | status_not_ready_when_mandatory_missing | nominal | ✅ |
| IT-14 | analyze_detects_unused_parameters | nominal | ✅ |
| IT-15 | discover_parameters_finds_placeholders | nominal | ✅ |
| IT-16 | discover_parameters_dry_run_no_changes | nominal | ✅ |

---

### IT-11: info shows archive metadata

- **Given:** An archive with 3 files and 2 parameters is loaded
- **When:** `.info` is run
- **Then:** Exit code 0; output shows archive name, file count (3), and parameter count (2)
- **Tests:** `tests/analysis_test.rs`

### IT-12: status ready when all mandatory values set

- **Given:** An archive with one mandatory parameter; value is set via `.value.set`
- **When:** `.status` is run
- **Then:** Exit code 0; output shows `READY`
- **Tests:** `tests/analysis_test.rs`

### IT-13: status not ready when mandatory value missing

- **Given:** An archive with one mandatory parameter; no value set
- **When:** `.status` is run
- **Then:** Exit code 1; output shows missing parameter name
- **Tests:** `tests/analysis_test.rs`

### IT-14: analyze detects unused parameters

- **Given:** An archive with parameter `port` defined but no `{{port}}` placeholder in any file
- **When:** `.analyze` is run
- **Then:** Exit code 0; output identifies `port` as unused
- **Tests:** `tests/analysis_test.rs`

### IT-15: discover.parameters finds placeholders

- **Given:** An archive file containing `{{project_name}}` and `{{version}}`; no parameters defined
- **When:** `.discover.parameters` is run
- **Then:** Exit code 0; output lists 2 discovered parameters; both added as definitions
- **Tests:** `tests/analysis_test.rs`

### IT-16: discover.parameters dry run makes no changes

- **Given:** An archive file containing `{{project_name}}`; no parameters defined
- **When:** `.discover.parameters dry::1` is run
- **Then:** Exit code 0; output shows discovered parameter; no definition added (archive unchanged)
- **Tests:** `tests/analysis_test.rs`
