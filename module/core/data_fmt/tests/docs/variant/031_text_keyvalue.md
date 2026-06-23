# Variant: Text KeyValue

### Scope

- **Purpose**: Drive test coverage for the key-value text output variant.
- **Responsibility**: Documents test cases for the KeyValue text variant in `docs/variant/031_text_keyvalue.md`.
- **In Scope**: Colon-separated key-value pairs, one pair per field, no borders or alignment characters.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | fields rendered as key: value pairs | ⏳ |
| VT-2 | colon separator between key and value | ⏳ |
| VT-3 | multiple rows produce separate record blocks | ⏳ |
| VT-4 | empty table produces no key-value output | ⏳ |

---

### VT-1: fields rendered as key: value pairs

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row `["Alice", "30"]`.
- **When:** Formatted with `TextFormatter` using `TextVariant::KeyValue`.
- **Then:** Output contains `Name: Alice` and `Age: 30` (or similar colon-separated pairs); header names become keys.

---

### VT-2: colon separator between key and value

- **Given:** A `TableView` with headers `["key", "val"]` and one row `["a", "b"]`.
- **When:** Formatted with `TextVariant::KeyValue`.
- **Then:** Each field is separated by a colon character; no pipe, tab, or equals sign is used as separator.

---

### VT-3: multiple rows produce separate record blocks

- **Given:** A `TableView` with headers `["Name"]` and rows `[["Alice"], ["Bob"]]`.
- **When:** Formatted with `TextVariant::KeyValue`.
- **Then:** Two record blocks appear; each contains key-value pairs for one row; blocks are visually separated.

---

### VT-4: empty table produces no key-value output

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TextVariant::KeyValue`.
- **Then:** Output is empty; no key-value pairs appear.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/031_text_keyvalue.md`](../../../docs/variant/031_text_keyvalue.md) | Source variant doc — Text KeyValue attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/text.rs`](../../text.rs) | Text formatter test implementation |
