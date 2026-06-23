# Data Structure: Variant Attributes

### Scope

- **Purpose**: Drive test coverage for the 46-attribute variant schema.
- **Responsibility**: Documents test cases for the variant attributes schema in `docs/data_structure/001_variant_attributes.md`.
- **In Scope**: Attribute group completeness, attribute count, attribute structure (name/purpose/example), variant instance compliance with schema.
- **Out of Scope**: Per-variant attribute values (see `../variant/`), formatter implementation (see `../feature/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| DS-1 | all 10 attribute groups present | ⏳ |
| DS-2 | schema defines exactly 46 attributes | ⏳ |
| DS-3 | every attribute has name, purpose, and example values | ⏳ |
| DS-4 | variant doc instances fill all 46 attributes | ⏳ |

---

### DS-1: all 10 attribute groups present

- **Given:** The variant attributes schema document.
- **When:** Enumerating the attribute group headings.
- **Then:** Exactly 10 groups exist: Identity & Classification, Build & Dependencies, Character Set & Encoding, Visual Structure, Data Representation, Output Characteristics, Usage Context, Technical Details, API & Construction, Performance & Size, Compatibility.

---

### DS-2: schema defines exactly 46 attributes

- **Given:** The variant attributes schema document.
- **When:** Counting all numbered attributes across all 10 groups.
- **Then:** The total is exactly 46; attributes are numbered 1 through 46 without gaps or duplicates.

---

### DS-3: every attribute has name, purpose, and example values

- **Given:** Any single attribute row in the schema (e.g., attribute #1 `formatter`).
- **When:** Inspecting the row columns.
- **Then:** The row contains a numeric ID, an attribute name in backtick notation, a purpose description, and one or more example values; no column is empty.

---

### DS-4: variant doc instances fill all 46 attributes

- **Given:** Any variant doc instance (e.g., `docs/variant/001_table_plain.md`).
- **When:** Counting the attribute rows in its attribute table.
- **Then:** The variant fills exactly 46 attribute rows; attribute names match those defined in the schema; no attribute is omitted or invented.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/data_structure/001_variant_attributes.md`](../../../docs/data_structure/001_variant_attributes.md) | Source data structure doc — 46-attribute schema, 10 groups, attribute definitions |
