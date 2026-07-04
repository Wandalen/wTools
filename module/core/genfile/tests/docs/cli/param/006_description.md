# Parameter Spec: description::

### Scope

- **Element:** `parameter/description`
- **Source:** `docs/cli/param.md#parameter--6-description`
- **Prefix:** `EC-`
- **Minimum cases:** 6

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| EC-17 | empty_string_is_default | nominal | ✅ |
| EC-18 | non_empty_description_stored | nominal | ✅ |
| EC-76 | parameter_add_stores_description | nominal | 🚧 |
| EC-77 | unicode_characters_in_description_accepted | nominal | 🚧 |
| EC-78 | description_persists_through_save_and_load | nominal | 🚧 |
| EC-79 | very_long_description_accepted | nominal | 🚧 |

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

### EC-76: parameter add stores description

- **Given:** An archive is loaded
- **When:** `.parameter.add name::port description::"HTTP port number"` is run
- **Then:** Exit code 0; parameter `port` created with description `"HTTP port number"`
- **Tests:** `tests/parameter_commands_test.rs`

### EC-77: unicode characters in description accepted

- **Given:** An archive is loaded
- **When:** `.archive.new name::test description::"Шаблон для сервиса"` is run
- **Then:** Exit code 0; description stored and retrieved correctly including non-ASCII characters
- **Tests:** `tests/archive_commands_test.rs`

### EC-78: description persists through save and load

- **Given:** An archive is created with description `"test desc"`; then saved and reloaded
- **When:** After `.archive.load` the description is inspected
- **Then:** Exit code 0; description value `"test desc"` preserved across serialization round-trip
- **Tests:** `tests/archive_commands_test.rs`

### EC-79: very long description accepted

- **Given:** A description string of 1000+ characters is provided
- **When:** `.archive.new name::test description::"<1000-char string>"` is run
- **Then:** Exit code 0; full description stored without truncation
- **Tests:** `tests/archive_commands_test.rs`
