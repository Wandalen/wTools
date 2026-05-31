# Test Spec: Default Value Type

- **Source**: `docs/feature/002_default_value_type.md`
- **Prefix**: `FT-02`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-02-1 | string_variant_to_template_string | ⏳ |
| FT-02-2 | number_variant_to_template_string | ⏳ |
| FT-02-3 | bool_variant_to_template_string | ⏳ |
| FT-02-4 | list_variant_to_template_string | ⏳ |

---

### FT-02-1: string_variant_to_template_string

- **Given:** A built-in value of the String variant holding `"hello world"`
- **When:** `to_template_string` is called
- **Then:** The result is `"hello world"` unchanged

---

### FT-02-2: number_variant_to_template_string

- **Given:** A built-in value of the Number variant holding `1234`
- **When:** `to_template_string` is called
- **Then:** The result is the string `"1234"`

---

### FT-02-3: bool_variant_to_template_string

- **Given:** A built-in value of the Bool variant holding `true` and another holding `false`
- **When:** `to_template_string` is called on each
- **Then:** Results are `"true"` and `"false"` respectively

---

### FT-02-4: list_variant_to_template_string

- **Given:** A built-in value of the List variant holding `["x", "y", "z"]`
- **When:** `to_template_string` is called
- **Then:** The result is a non-empty string containing all three elements
