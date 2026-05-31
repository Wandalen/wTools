# Test Spec: Error Contract

- **Source**: `docs/api/004_error_contract.md`
- **Prefix**: `AP-04`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| AP-04-1 | missing_parameters_variant_is_user_fixable | ⏳ |
| AP-04-2 | render_failure_variant_surfaces_engine_error | ⏳ |
| AP-04-3 | filesystem_io_variant_wraps_os_error | ⏳ |
| AP-04-4 | invalid_template_variant_is_user_fixable | ⏳ |
| AP-04-5 | all_variants_satisfy_standard_error_contract | ⏳ |

---

### AP-04-1: missing_parameters_variant_is_user_fixable

- **Given:** A generation call where mandatory parameters are absent from the value map
- **When:** The returned error is matched
- **Then:** It is the `MissingParameters` variant containing the names of the unfilled parameters; the error message includes at least one parameter name

---

### AP-04-2: render_failure_variant_surfaces_engine_error

- **Given:** A template containing invalid Handlebars syntax (e.g., `"{{#if}}"` without closing block)
- **When:** Generation is attempted
- **Then:** The returned error is the `RenderFailure` variant with a non-empty description of the engine error

---

### AP-04-3: filesystem_io_variant_wraps_os_error

- **Given:** A real file system write call targeting a path where the parent directory cannot be created (e.g., a path under a read-only directory)
- **When:** The write fails
- **Then:** The error is the `FilesystemIo` variant wrapping the underlying OS error

---

### AP-04-4: invalid_template_variant_is_user_fixable

- **Given:** A template string with malformed syntax that causes a compilation error before rendering
- **When:** The renderer processes the template
- **Then:** The returned error is the `InvalidTemplate` variant (or equivalent); the error message identifies the template syntax problem

---

### AP-04-5: all_variants_satisfy_standard_error_contract

- **Given:** Any error variant from the typed error enum
- **When:** The standard `Display` or `Error` trait methods are called
- **Then:** They return non-empty, human-readable strings without panicking
