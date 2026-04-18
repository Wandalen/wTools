# Input Model Instance Procedure

### Scope

- **Trigger**: A new conceptual data shape that the library accepts as input needs documentation.
- **Audience**: Contributors adding, updating, or deprecating input_model doc instances.

### Procedure: Create Instance

1. Confirm the model belongs here — verify it fits **In Scope** in `readme.md` (conceptual data shape, not a Rust type). Rust type details go in `input_type/`; construction APIs go in `builder/`.
2. Assign the next unused NNN from the **Overview Table** in `readme.md`.
3. Create `docs/input_model/NNN_name.md` with the following sections in order per **Type-Specific Requirements** in `readme.md`: `### Scope`, `### Cross-References`, `### Data Shape`, `### Downstream Connections`.
4. Add a row to the **Overview Table** in `readme.md`:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`
5. Add a row to `docs/entities.md` **Master Doc Instances Table**:
   `| input_model | NNN | Name | [input_model/NNN_name.md](input_model/NNN_name.md) |`
6. Update the **Instances** count in `docs/entities.md` **Master Doc Entities Table** for the `input_model/` row.
7. Add a node to `docs/doc_graph.yml` under `nodes`:
   `- id: input_model/NNN, file: input_model/NNN_name.md, label: Name, entity: input_model, status: planned`
8. Update `node_count` in `doc_graph.yml` `meta`.
9. Add edges for every doc listed in the new instance's `### Cross-References`; update `edge_count` in `meta`.
10. Update or add the new node to the appropriate `components` entry; update `size`.

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Update `docs/entities.md` Master Doc Instances Table: append `(deprecated)` to the Name column.
4. Remove the node from `doc_graph.yml` `nodes`; remove all edges referencing this node.
5. Update `node_count` and `edge_count` in `meta`; update the relevant component `nodes` list and `size`.
