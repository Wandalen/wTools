# Parameter Spec: mode::

### Scope

- **Element:** `parameter/mode`
- **Source:** `docs/cli/param.md#parameter--14-mode`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-36 | inline_mode_embeds_file_content | nominal | ✅ |
| EC-37 | reference_mode_stores_path_only | nominal | ✅ |
| EC-105 | default_is_reference | nominal | 🚧 |
| EC-106 | invalid_mode_value_rejected | error | 🚧 |
| EC-107 | case_insensitive_mode_accepted | nominal | 🚧 |
| EC-108 | reference_mode_path_stored_relative_to_source | nominal | 🚧 |

---

### EC-36: inline mode embeds file content

- **Given:** A directory `./src` contains `main.rs` with content `"fn main() {}"`
- **When:** `.archive.from_directory source::"./src" mode::inline` is run
- **Then:** Exit code 0; archive entry for `main.rs` contains the file content embedded directly
- **Tests:** `tests/archive_commands_test.rs`

### EC-37: reference mode stores path only

- **Given:** A directory `./src` contains `main.rs`
- **When:** `.archive.from_directory source::"./src" mode::reference` is run
- **Then:** Exit code 0; archive entry for `main.rs` stores the path, not the content
- **Tests:** `tests/archive_commands_test.rs`

### EC-105: default is reference

- **Given:** A directory `./src` contains `main.rs`; no `mode::` parameter specified
- **When:** `.archive.from_directory source::"./src"` is run
- **Then:** Exit code 0; archive entry for `main.rs` stores the path only (default is `reference`)
- **Tests:** `tests/archive_commands_test.rs`

### EC-106: invalid mode value rejected

- **Given:** An unrecognized mode value is provided
- **When:** `.archive.from_directory source::"./src" mode::copy` is run
- **Then:** Exit code 1; error message indicates `copy` is not a valid mode value
- **Tests:** `tests/archive_commands_test.rs`

### EC-107: case insensitive mode accepted

- **Given:** A directory `./src` contains `main.rs`
- **When:** `.archive.from_directory source::"./src" mode::INLINE` is run
- **Then:** Exit code 0; inline mode applied (value matched case-insensitively)
- **Tests:** `tests/archive_commands_test.rs`

### EC-108: reference mode path stored relative to source

- **Given:** Directory `./project/src` contains `main.rs`
- **When:** `.archive.from_directory source::"./project/src" mode::reference` is run
- **Then:** Exit code 0; archive entry path is `main.rs` (relative to source root, not absolute)
- **Tests:** `tests/archive_commands_test.rs`
