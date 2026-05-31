# Test Spec: Typed Errors

- **Source**: `docs/feature/016_typed_errors.md`
- **Prefix**: `FT-16`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-16-1 | missing_parameters_message_includes_param_names | ⏳ |
| FT-16-2 | render_failure_message_is_non_empty | ⏳ |
| FT-16-3 | filesystem_io_message_is_non_empty | ⏳ |
| FT-16-4 | all_variants_implement_display | ⏳ |

---

### FT-16-1: missing_parameters_message_includes_param_names

- **Given:** `Error::MissingParameters(vec!["foo", "bar"])`
- **When:** `to_string()` is called
- **Then:** The output contains both `"foo"` and `"bar"`

---

### FT-16-2: render_failure_message_is_non_empty

- **Given:** An `Error::RenderFailure` variant with a non-empty engine error string
- **When:** `to_string()` is called
- **Then:** The output is a non-empty string describing the failure

---

### FT-16-3: filesystem_io_message_is_non_empty

- **Given:** An `Error::FilesystemIo` variant wrapping an OS error
- **When:** `to_string()` is called
- **Then:** The output is a non-empty string; the I/O context is represented

---

### FT-16-4: all_variants_implement_display

- **Given:** One instance of each error variant (MissingParameters, RenderFailure, FilesystemIo, InvalidTemplate)
- **When:** `to_string()` is called on each
- **Then:** All return non-empty strings without panicking
