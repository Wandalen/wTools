# Variant Instance Procedure

### Scope

- **Trigger**: A new output format variant supported by a formatter needs documentation.
- **Audience**: Contributors adding, updating, or deprecating variant doc instances.

### Procedure: Create Instance

1. Confirm the variant belongs here — verify it fits **In Scope** in `readme.md` (per-variant output specification). Formatter implementation goes in `feature/`; attribute schema reference is in `data_structure/001_variant_attributes.md`.
2. Assign the next unused NNN from the **Overview Table** in `readme.md` (extends the current maximum, respecting the category groupings in `### Organization`).
3. Create `docs/variant/NNN_name.md` with all 14 required sections per **Type-Specific Requirements** in `readme.md` (Scope, Cross-References, Identity & Classification, Build & Dependencies, Character Set & Encoding, Visual Structure, Data Representation, Output Characteristics, Usage Context, Technical Details, API & Construction, Performance & Size, Compatibility, Example Output).
4. Add a row to the **Overview Table** in `readme.md` in the correct category position:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`
5. Update `### Organization` in `readme.md` to include the new NNN in the correct category range.
6. Add a row to `docs/entities.md` **Master Doc Instances Table**:
   `| variant | NNN | Name | [variant/NNN_name.md](variant/NNN_name.md) |`
7. Update the **Instances** count in `docs/entities.md` **Master Doc Entities Table** for the `variant/` row.
8. Add a node to `docs/doc_graph.yml` under `nodes`:
   `- id: variant/NNN, file: variant/NNN_name.md, label: Name, entity: variant, status: planned`
9. Add the new node to the `variant-catalog` component's `nodes` list; update `size`.
10. Update `node_count` in `doc_graph.yml` `meta`.
11. Add edges for every doc listed in the new instance's `### Cross-References`; update `edge_count` in `meta`.

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Update `### Organization` if the deprecated variant was the sole member of its category range.
4. Update `docs/entities.md` Master Doc Instances Table: append `(deprecated)` to the Name column.
5. Remove the node from `doc_graph.yml` `variant-catalog` component `nodes` list; update `size`.
6. Remove all edges referencing this node; update `edge_count` in `meta`; update `node_count` in `meta`.
