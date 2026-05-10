# Parameter Spec: mode::

### Scope

- **Element:** `parameter/mode`
- **Source:** `docs/cli/param.md#parameter--14-mode`
- **Prefix:** `EC-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-36 | inline_mode_embeds_file_content | nominal | ✅ |
| EC-37 | reference_mode_stores_path_only | nominal | ✅ |

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
