# Test Spec: Error Message Quality

- **Source**: `docs/invariant/006_error_message_quality.md`
- **Prefix**: `IN-06`
- **Min cases**: 2

## Cases

| ID | Name | Status |
|----|------|--------|
| IN-06-1 | missing_params_message_includes_param_name | ⏳ |
| IN-06-2 | all_error_messages_include_diagnostic_context | ⏳ |

---

### IN-06-1: missing_params_message_includes_param_name

- **Given:** `Error::MissingParameters(vec!["output_dir"])`
- **When:** `to_string()` is called
- **Then:** The message contains the string `"output_dir"` so the caller can diagnose which parameter is missing

---

### IN-06-2: all_error_messages_include_diagnostic_context

- **Given:** One instance of each error variant: MissingParameters, RenderFailure, FilesystemIo, InvalidTemplate
- **When:** `to_string()` is called on each
- **Then:** Each message is non-empty and contains context beyond a generic label — parameter names for MissingParameters, path or syntax info for others
