# Parameter Spec: name::

### Scope

- **Element:** `parameter/name`
- **Source:** `docs/cli/param.md#parameter--4-name`
- **Prefix:** `EC-`
- **Minimum cases:** 3

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-11 | valid_identifier_accepted | nominal | ✅ |
| EC-12 | underscore_in_name_accepted | nominal | ✅ |
| EC-13 | name_with_spaces_rejected | error | ✅ |

---

### EC-11: valid identifier accepted

- **Given:** A name consisting of alphanumeric characters
- **When:** `.archive.new name::"mytemplate"` is run
- **Then:** Exit code 0; archive created with that name
- **Tests:** `tests/archive_commands_test.rs`

### EC-12: underscore in name accepted

- **Given:** A name with underscores (valid identifier syntax)
- **When:** `.archive.new name::"my_template_v2"` is run
- **Then:** Exit code 0; archive created with that name
- **Tests:** `tests/archive_commands_test.rs`

### EC-13: name with spaces rejected

- **Given:** A name containing a space character
- **When:** `.archive.new name::"my template"` is run
- **Then:** Exit code 1; error message indicates invalid identifier format
- **Tests:** `tests/archive_commands_test.rs`
