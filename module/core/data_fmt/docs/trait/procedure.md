# Trait Instance Procedure

### Scope

- **Trigger**: A new Rust trait defining an interface contract between input types and formatters needs documentation.
- **Audience**: Contributors adding, updating, or deprecating trait doc instances.

### Procedure: Create Instance

1. Confirm the trait belongs here — verify it fits **In Scope** in `readme.md` (interface contract connecting input to formatters). Formatter implementation details go in `feature/`; variant output goes in `variant/`.
2. Assign the next unused NNN from the **Overview Table** in `readme.md`.
3. Create `docs/trait/NNN_name.md` with the following sections in order per **Type-Specific Requirements** in `readme.md`: `### Scope`, typed cross-reference H3 sections (see existing instances), `### Signature`, `### Implementors`, `### Coverage Gaps`.
4. Add a row to the **Overview Table** in `readme.md`:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`
5. Add a row to `docs/entity.md` **Master Doc Instances Table**:
   `| trait | NNN | Name | [trait/NNN_name.md](trait/NNN_name.md) |`
6. Update the **Instances** count in `docs/entity.md` **Master Doc Entities Table** for the `trait/` row.
7. Add a node to `docs/doc_graph.yml` under `nodes`:
   `- id: trait/NNN, file: trait/NNN_name.md, label: Name, entity: trait, status: planned`
8. Update `node_count` in `doc_graph.yml` `meta`.
9. Add edges for every file listed in the new instance's cross-reference sections; update `edge_count` in `meta`.
10. Update or add the new node to the appropriate `components` entry; update `size`.

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Update `docs/entity.md` Master Doc Instances Table: append `(deprecated)` to the Name column.
4. Remove the node from `doc_graph.yml` `nodes`; remove all edges referencing this node.
5. Update `node_count` and `edge_count` in `meta`; update the relevant component `nodes` list and `size`.
