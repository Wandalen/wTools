# Test Spec: Template Value Trait

- **Source**: `docs/feature/001_template_value_trait.md`
- **Prefix**: `FT-01`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-01-1 | trait_to_template_string_returns_string | ⏳ |
| FT-01-2 | trait_from_str_constructs_value | ⏳ |
| FT-01-3 | trait_is_empty_returns_true_for_empty | ⏳ |
| FT-01-4 | custom_type_satisfies_trait_bound | ⏳ |

---

### FT-01-1: trait_to_template_string_returns_string

- **Given:** A built-in value implementing the template value trait
- **When:** `to_template_string` is called
- **Then:** A non-panicking `String` is returned representing the value

---

### FT-01-2: trait_from_str_constructs_value

- **Given:** A raw string representation `"42"`
- **When:** `from_str` is called on the built-in value type
- **Then:** A value is constructed without error

---

### FT-01-3: trait_is_empty_returns_true_for_empty

- **Given:** A value constructed to represent an empty state (empty string or equivalent)
- **When:** `is_empty` is called
- **Then:** It returns `true`; for a non-empty value it returns `false`

---

### FT-01-4: custom_type_satisfies_trait_bound

- **Given:** A user-defined struct that implements `to_template_string`, `from_str`, and `is_empty`
- **When:** The struct is used as the generic value type in a `Values<MyType>` container
- **Then:** The code compiles and the struct's conversion logic is invoked during serialization
