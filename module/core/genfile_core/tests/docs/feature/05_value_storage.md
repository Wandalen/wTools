# Test Spec: Value Storage

- **Source**: `docs/feature/005_value_storage.md`
- **Prefix**: `FT-05`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-05-1 | insert_stores_value_by_key | ⏳ |
| FT-05-2 | insert_if_empty_does_not_overwrite_existing | ⏳ |
| FT-05-3 | insert_if_empty_sets_value_when_absent | ⏳ |
| FT-05-4 | serialize_converts_all_values_to_string_map | ⏳ |

---

### FT-05-1: insert_stores_value_by_key

- **Given:** An empty value storage
- **When:** A value is inserted under key `"name"` with content `"genfile"`
- **Then:** The key `"name"` maps to `"genfile"` in the storage

---

### FT-05-2: insert_if_empty_does_not_overwrite_existing

- **Given:** A value storage that already has `"name"` = `"original"`
- **When:** `insert_if_empty("name", "replacement")` is called
- **Then:** The stored value for `"name"` remains `"original"`

---

### FT-05-3: insert_if_empty_sets_value_when_absent

- **Given:** An empty value storage
- **When:** `insert_if_empty("version", "1.0.0")` is called
- **Then:** The stored value for `"version"` is `"1.0.0"`

---

### FT-05-4: serialize_converts_all_values_to_string_map

- **Given:** A value storage with three entries of different built-in types (String, Number, Bool)
- **When:** The serialization method is called to produce a string map
- **Then:** All three entries appear in the result as string-keyed string values using their template string conversions
