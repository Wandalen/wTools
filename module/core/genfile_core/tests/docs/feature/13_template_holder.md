# Test Spec: Template Holder

- **Source**: `docs/feature/013_template_holder.md`
- **Prefix**: `FT-13`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-13-1 | holder_composes_all_components | ⏳ |
| FT-13-2 | value_type_is_generic | ⏳ |
| FT-13-3 | renderer_is_generic | ⏳ |
| FT-13-4 | filesystem_is_generic | ⏳ |

---

### FT-13-1: holder_composes_all_components

- **Given:** A `Template` holder constructed with file descriptors, a parameter collection, a value map, a renderer, and a file system
- **When:** The holder is inspected
- **Then:** It holds references to all five components without error

---

### FT-13-2: value_type_is_generic

- **Given:** Two `Template` holders — one using the built-in value type, one using a custom value type
- **When:** Each holder is constructed and `generate` is called
- **Then:** Both compile and produce correct output using their respective value types

---

### FT-13-3: renderer_is_generic

- **Given:** A `Template` holder parameterized with the `HandlebarsRenderer`
- **When:** `generate` is called with a template `"{{key}}"` and value `{key: "val"}`
- **Then:** The output file contains `"val"`

---

### FT-13-4: filesystem_is_generic

- **Given:** A `Template` holder parameterized with `MemoryFileSystem`
- **When:** `generate` is called
- **Then:** All output files are stored in memory with no disk side effects
