# Test Spec: Parameter API

- **Source**: `docs/api/002_parameter_api.md`
- **Prefix**: `AP-02`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| AP-02-1 | descriptor_stores_name | ⏳ |
| AP-02-2 | descriptor_mandatory_flag_defaults_false | ⏳ |
| AP-02-3 | descriptor_optional_default_value | ⏳ |
| AP-02-4 | collection_lists_only_mandatory_names | ⏳ |
| AP-02-5 | collection_empty_when_no_mandatory_params | ⏳ |

---

### AP-02-1: descriptor_stores_name

- **Given:** A parameter descriptor constructed with name `"output_dir"`
- **When:** The name attribute is accessed
- **Then:** The returned value is `"output_dir"`

---

### AP-02-2: descriptor_mandatory_flag_defaults_false

- **Given:** A parameter descriptor constructed with only a name (no mandatory flag specified)
- **When:** The mandatory attribute is accessed
- **Then:** It returns `false` (optional by default)

---

### AP-02-3: descriptor_optional_default_value

- **Given:** A parameter descriptor constructed with a default value `"src/"`
- **When:** The default value attribute is accessed
- **Then:** It returns `Some("src/")` for a descriptor with a default, and `None` for one without

---

### AP-02-4: collection_lists_only_mandatory_names

- **Given:** A parameter collection with one mandatory parameter `"name"` and one optional parameter `"desc"`
- **When:** The mandatory-listing method is called on the collection
- **Then:** The result contains `"name"` and does not contain `"desc"`

---

### AP-02-5: collection_empty_when_no_mandatory_params

- **Given:** A parameter collection where all parameters have `mandatory = false`
- **When:** The mandatory-listing method is called
- **Then:** The returned list is empty
