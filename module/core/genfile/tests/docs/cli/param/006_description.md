# Parameter Spec: description::

### Scope

- **Element:** `parameter/description`
- **Source:** `docs/cli/param.md#parameter--6-description`
- **Prefix:** `EC-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-17 | empty_string_is_default | nominal | ✅ |
| EC-18 | non_empty_description_stored | nominal | ✅ |

---

### EC-17: empty string is default

- **Given:** No `description::` is provided
- **When:** `.archive.new name::"test"` is run
- **Then:** Exit code 0; archive description is empty string `""`
- **Tests:** `tests/archive_commands_test.rs`

### EC-18: non empty description stored

- **Given:** A description string is provided
- **When:** `.archive.new name::"test" description::"REST API scaffold"` is run
- **Then:** Exit code 0; archive description is stored as `"REST API scaffold"`
- **Tests:** `tests/archive_commands_test.rs`
