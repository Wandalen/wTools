# Test Spec: Missing Mandatory Detection

- **Source**: `docs/feature/015_missing_mandatory_detection.md`
- **Prefix**: `FT-15`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-15-1 | check_returns_empty_when_all_mandatory_filled | ⏳ |
| FT-15-2 | check_returns_names_of_missing_mandatory_params | ⏳ |
| FT-15-3 | check_ignores_optional_params_with_no_value | ⏳ |
| FT-15-4 | check_reports_multiple_missing_params | ⏳ |

---

### FT-15-1: check_returns_empty_when_all_mandatory_filled

- **Given:** A parameter collection with mandatory `"name"` and a value map containing `"name"` = `"foo"`
- **When:** The missing-mandatory check is performed
- **Then:** The result is an empty list

---

### FT-15-2: check_returns_names_of_missing_mandatory_params

- **Given:** A parameter collection with mandatory `"name"` and a value map that does not contain `"name"`
- **When:** The missing-mandatory check is performed
- **Then:** The result is a list containing `"name"`

---

### FT-15-3: check_ignores_optional_params_with_no_value

- **Given:** A parameter collection with optional `"desc"` (no value provided in the map)
- **When:** The missing-mandatory check is performed
- **Then:** `"desc"` does not appear in the result; the list is empty

---

### FT-15-4: check_reports_multiple_missing_params

- **Given:** A parameter collection with mandatory params `"host"`, `"port"`, `"token"` and a value map with none of them
- **When:** The missing-mandatory check is performed
- **Then:** All three names appear in the returned list
