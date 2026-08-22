# Data Structure: Theme Attributes

### Scope

- **Purpose**: Drive test coverage for the 21-attribute theme schema.
- **Responsibility**: Documents test cases for the theme attributes schema in `docs/data_structure/002_theme_attributes.md`.
- **In Scope**: Attribute group completeness, attribute count, attribute structure (name/purpose/example), theme doc instance compliance with schema, the schema's scope relative to the 46-attribute variant schema.
- **Out of Scope**: Per-theme attribute values (see `docs/theme/`), theme application behavior (see `tests/docs/feature/004_color_themes.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| DS-1 | all 7 attribute groups present | ⏳ |
| DS-2 | schema defines exactly 21 attributes | ⏳ |
| DS-3 | every attribute has name, purpose, and example values | ⏳ |
| DS-4 | theme doc instances fill all 21 attributes across 7 grouped bullet sections | ⏳ |
| DS-5 | the 21-attribute theme schema omits groups that apply only to output-format encodings | ⏳ |
| DS-6 | no_color_aware is No for every theme, contrasting with QuantityStyle::resolve | ⏳ |

---

### DS-1: all 7 attribute groups present

- **Given:** The theme attributes schema document.
- **When:** Enumerating the `####` attribute group headings under `### Structure`.
- **Then:** Exactly 7 groups exist: Identity & Classification, Build & Dependencies, Color Role Assignments, Visual Characteristics, Application Targets, API & Construction, Compatibility.

---

### DS-2: schema defines exactly 21 attributes

- **Given:** The theme attributes schema document.
- **When:** Counting all numbered attribute rows across all 7 groups.
- **Then:** The total is exactly 21; attributes are numbered 1 through 21 without gaps or duplicates.

---

### DS-3: every attribute has name, purpose, and example values

- **Given:** Any single attribute row in the schema (e.g., attribute #7 `header_color`).
- **When:** Inspecting the row columns.
- **Then:** The row contains a numeric ID, an attribute name in backtick notation, a purpose description, and one or more example values; no column is empty.

---

### DS-4: theme doc instances fill all 21 attributes across 7 grouped bullet sections

- **Given:** Any theme doc instance (e.g., `docs/theme/001_dark.md`).
- **When:** Counting the `- **attribute**: value` bullet lines under its 7 `###` group headings (Identity & Classification, Build & Dependencies, Color Role Assignments, Visual Characteristics, Application Targets, API & Construction, Compatibility).
- **Then:** The instance fills exactly 21 attribute bullets; attribute names match those defined in the schema; no attribute is omitted or invented; the instance presents its values as grouped `###` headings with a bullet list, not a table — unlike the schema document itself, which defines each attribute as a table row.

---

### DS-5: the 21-attribute theme schema omits groups that apply only to output-format encodings

- **Given:** The theme attributes schema (21 attributes, 7 groups) and the variant attributes schema (46 attributes, 10 groups, per `docs/data_structure/001_variant_attributes.md`).
- **When:** Comparing the two schemas' group lists.
- **Then:** The theme schema has no `Character Set & Encoding`, `Data Representation`, `Usage Context`, `Technical Details`, or `Performance & Size` groups — `ColorTheme` is a 5-field color overlay, not an output-format encoding, so those groups do not apply; the theme schema is deliberately smaller by design, not by omission.

---

### DS-6: no_color_aware is No for every theme, contrasting with QuantityStyle::resolve

- **Given:** The `no_color_aware` attribute (#20) and its accompanying Operations note.
- **When:** Checking every theme doc instance's `no_color_aware` value (all 8: dark, light, monokai, solarized, nord, dracula, none, custom).
- **Then:** Every instance declares `no_color_aware: No` — `ColorTheme` construction never inspects `NO_COLOR`/TTY state itself, unlike `QuantityStyle::resolve` (`docs/api/006_quantity_formatting.md`), which does fold `NO_COLOR` automatically; the two environment-awareness policies coexist deliberately within the same crate.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/data_structure/002_theme_attributes.md`](../../../docs/data_structure/002_theme_attributes.md) | Source data structure doc — 21-attribute schema, 7 groups, attribute definitions |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/data_structure_test.rs`](../../data_structure_test.rs) (extend) | Spec tests for DS-1..DS-6 — theme attributes data structure |
