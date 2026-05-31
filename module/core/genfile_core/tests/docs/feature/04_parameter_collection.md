# Test Spec: Parameter Collection

- **Source**: `docs/feature/004_parameter_collection.md`
- **Prefix**: `FT-04`
- **Min cases**: 4

## Cases

| ID | Name | Status |
|----|------|--------|
| FT-04-1 | collection_holds_multiple_descriptors | ⏳ |
| FT-04-2 | mandatory_listing_returns_only_mandatory_names | ⏳ |
| FT-04-3 | mandatory_listing_empty_when_all_optional | ⏳ |
| FT-04-4 | collection_preserves_insertion_order | ⏳ |

---

### FT-04-1: collection_holds_multiple_descriptors

- **Given:** A parameter collection built with three descriptors: `"a"`, `"b"`, `"c"`
- **When:** The collection is inspected
- **Then:** All three descriptor names are accessible

---

### FT-04-2: mandatory_listing_returns_only_mandatory_names

- **Given:** A collection with mandatory parameters `"name"` and `"version"` and optional parameter `"description"`
- **When:** The mandatory-listing method is called
- **Then:** The result is a list containing `"name"` and `"version"` and not `"description"`

---

### FT-04-3: mandatory_listing_empty_when_all_optional

- **Given:** A collection where every parameter has `mandatory = false`
- **When:** The mandatory-listing method is called
- **Then:** The returned collection is empty

---

### FT-04-4: collection_preserves_insertion_order

- **Given:** A collection built by inserting parameters in order `["z", "a", "m"]`
- **When:** The parameters are iterated
- **Then:** They appear in the same order as insertion
