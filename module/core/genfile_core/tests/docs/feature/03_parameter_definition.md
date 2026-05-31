# Test Spec: Parameter Definition

- **Source**: `docs/feature/003_parameter_definition.md`
- **Prefix**: `FT-03`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-03-1 | descriptor_name_is_stored | ⏳ |
| FT-03-2 | descriptor_mandatory_flag_is_stored | ⏳ |
| FT-03-3 | descriptor_default_value_is_optional | ⏳ |
| FT-03-4 | descriptor_description_is_optional | ⏳ |

---

### FT-03-1: descriptor_name_is_stored

- **Given:** A parameter descriptor built with name `"crate_name"`
- **When:** The name field is accessed
- **Then:** It returns `"crate_name"`

---

### FT-03-2: descriptor_mandatory_flag_is_stored

- **Given:** A parameter descriptor built with `mandatory = true`
- **When:** The mandatory field is accessed
- **Then:** It returns `true`; a descriptor built with `mandatory = false` returns `false`

---

### FT-03-3: descriptor_default_value_is_optional

- **Given:** One descriptor built with a default value `"default_val"` and one without
- **When:** The default value field is accessed on each
- **Then:** The first returns `Some("default_val")`; the second returns `None`

---

### FT-03-4: descriptor_description_is_optional

- **Given:** One descriptor built with description `"The crate name"` and one without
- **When:** The description field is accessed on each
- **Then:** The first returns `Some("The crate name")`; the second returns `None`
