# Feature Spec: Archive Serialization

### Scope

- **Element:** `feature/007_archive_serialization`
- **Source:** `docs/feature/007_archive_serialization.md`
- **Prefix:** `FT-`
- **Minimum cases:** 4

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| FT-01 | pack_internalizes_refs_before_saving | nominal | 🔶 deferred |
| FT-02 | pack_from_directory_creates_portable_archive | nominal | ✅ |
| FT-03 | pack_dry_run_shows_planned_operations | nominal | ✅ |
| FT-04 | pack_output_loads_with_no_external_deps | nominal | ✅ |

---

### FT-01: pack internalizes references before saving

- **Given:** An archive with external file references is loaded
- **When:** `.pack output::<file>.json` is run
- **Then:** Exit code 0; the output archive file is self-contained (all content inline)
- **Tests:** `tests/archive_commands_test.rs`

### FT-02: pack from directory creates portable archive

- **Given:** A directory with template files exists; no archive loaded
- **When:** `.pack input::<dir> output::<file>.json` is run
- **Then:** Exit code 0; output archive contains all directory files inline
- **Tests:** `tests/archive_commands_test.rs`

### FT-03: pack dry run shows planned operations without writing

- **Given:** An archive or directory exists as input
- **When:** `.pack output::<file>.json dry::1` is run
- **Then:** Exit code 0; output describes planned pack; no file written to `<file>.json`
- **Tests:** `tests/archive_commands_test.rs`

### FT-04: pack output loads with no external dependencies

- **Given:** `.pack` has produced an output file
- **When:** The output is loaded via `.archive.load path::<file>.json` on a different machine (no original files)
- **Then:** Archive loads successfully; all file contents accessible inline
- **Tests:** `tests/archive_commands_test.rs`
