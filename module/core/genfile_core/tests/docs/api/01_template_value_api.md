# Test Spec: Template Value API

- **Source**: `docs/api/001_template_value_api.md`
- **Prefix**: `AP-01`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| AP-01-1 | custom_type_implements_trait | ⏳ |
| AP-01-2 | builtin_string_converts_to_template_string | ⏳ |
| AP-01-3 | builtin_number_converts_to_decimal_string | ⏳ |
| AP-01-4 | builtin_bool_converts_to_string | ⏳ |
| AP-01-5 | builtin_list_converts_to_joined_string | ⏳ |
| AP-01-6 | from_string_roundtrip_preserves_value | ⏳ |

---

### AP-01-1: custom_type_implements_trait

- **Given:** A custom struct that implements the template value trait with its own `to_template_string`, `from_str`, and `is_empty` methods
- **When:** The struct is used as the value type in a value storage map
- **Then:** The storage accepts it without compilation error and `to_template_string` returns the custom conversion result

---

### AP-01-2: builtin_string_converts_to_template_string

- **Given:** A built-in value constructed as the String variant with content `"hello"`
- **When:** `to_template_string` is called
- **Then:** The result is the string `"hello"` with no modification

---

### AP-01-3: builtin_number_converts_to_decimal_string

- **Given:** A built-in value constructed as the Number variant with value `42`
- **When:** `to_template_string` is called
- **Then:** The result is the string `"42"`

---

### AP-01-4: builtin_bool_converts_to_string

- **Given:** A built-in value constructed as the Bool variant with value `true`
- **When:** `to_template_string` is called
- **Then:** The result is `"true"`; for `false` the result is `"false"`

---

### AP-01-5: builtin_list_converts_to_joined_string

- **Given:** A built-in value constructed as the List variant with elements `["a", "b", "c"]`
- **When:** `to_template_string` is called
- **Then:** The result is a comma-separated string of all elements

---

### AP-01-6: from_string_roundtrip_preserves_value

- **Given:** A built-in String variant value `"test"`
- **When:** `to_template_string` is called and the result is passed to `from_str`
- **Then:** The reconstructed value compares equal to the original
