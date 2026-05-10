# Parameter Spec: recursive::

### Scope

- **Element:** `parameter/recursive`
- **Source:** `docs/cli/param.md#parameter--10-recursive`
- **Prefix:** `EC-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-27 | recursive_one_includes_subdirs | nominal | ✅ |
| EC-28 | recursive_zero_scans_top_level_only | nominal | ✅ |

---

### EC-27: recursive one includes subdirs

- **Given:** Directory `./src` contains `main.rs` at top level and `util/helper.rs` in a subdirectory
- **When:** `.archive.from_directory source::"./src" recursive::1` is run
- **Then:** Exit code 0; archive contains both `main.rs` and `util/helper.rs`
- **Tests:** `tests/archive_commands_test.rs`

### EC-28: recursive zero scans top level only

- **Given:** Directory `./src` contains `main.rs` at top level and `util/helper.rs` in a subdirectory
- **When:** `.archive.from_directory source::"./src" recursive::0` is run
- **Then:** Exit code 0; archive contains only `main.rs` (subdirectory files excluded)
- **Tests:** `tests/archive_commands_test.rs`
