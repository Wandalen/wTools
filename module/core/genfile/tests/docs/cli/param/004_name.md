# Parameter Spec: name::

### Scope

- **Element:** `parameter/name`
- **Source:** `docs/cli/param.md#parameter--4-name`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-11 | valid_identifier_accepted | nominal | ✅ |
| EC-12 | underscore_in_name_accepted | nominal | ✅ |
| EC-13 | name_with_spaces_rejected | error | ✅ |
| EC-70 | name_in_value_set_requires_existing_parameter | Behavioral Divergence | 🚧 |
| EC-71 | empty_name_rejected | error | 🚧 |
| EC-72 | hyphen_in_name_rejected | error | 🚧 |

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

### EC-70: name in value set requires existing parameter

- **Given:** An archive is loaded; no parameter named `port` is defined
- **When:** `.value.set name::port value::"8080"` is run
- **Then:** Exit code 1; error indicates no parameter named `port` exists (in `.value.set`, `name::` must match a declared parameter; in `.archive.new`, `name::` creates a new archive — different semantic)
- **Tests:** `tests/value_commands_test.rs`

### EC-71: empty name rejected

- **Given:** An empty string is provided as `name::`
- **When:** `.archive.new name::""` is run
- **Then:** Exit code 1; error message indicates name cannot be empty
- **Tests:** `tests/archive_commands_test.rs`

### EC-72: hyphen in name rejected

- **Given:** A name containing a hyphen character
- **When:** `.archive.new name::"my-template"` is run
- **Then:** Exit code 1; error message indicates hyphens are not valid in identifier names
- **Tests:** `tests/archive_commands_test.rs`
