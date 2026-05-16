# Builder Instance Procedure

### Scope

- **Trigger**: A new construction helper that produces an input type from user data needs documentation.
- **Audience**: Contributors adding, updating, or deprecating builder doc instances.

### Procedure: Create Instance

1. Confirm the builder belongs here — verify it fits **In Scope** in `readme.md` (construction helper producing an input type). Input type internals go in `input_type/`; formatter behavior goes in `feature/`.
2. Assign the next unused NNN from the **Overview Table** in `readme.md`.
3. Create `docs/builder/NNN_name.md` with the following sections in order per **Type-Specific Requirements** in `readme.md`: `### Scope`, typed cross-reference H3 sections (see existing instances), `### Construction API`, `### Invariants`.
4. Add a row to the **Overview Table** in `readme.md`:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`
5. Add a row to `docs/entities.md` **Master Doc Instances Table**:
   `| builder | NNN | Name | [builder/NNN_name.md](builder/NNN_name.md) |`
6. Update the **Instances** count in `docs/entities.md` **Master Doc Entities Table** for the `builder/` row.
7. Add a node to `docs/doc_graph.yml` under `nodes`:
   `- id: builder/NNN, file: builder/NNN_name.md, label: Name, entity: builder, status: planned`
8. Update `node_count` in `doc_graph.yml` `meta`.
9. Add edges for every file listed in the new instance's cross-reference sections; update `edge_count` in `meta`.
10. Update or add the new node to the appropriate `components` entry; update `size`.

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Update `docs/entities.md` Master Doc Instances Table: append `(deprecated)` to the Name column.
4. Remove the node from `doc_graph.yml` `nodes`; remove all edges referencing this node.
5. Update `node_count` and `edge_count` in `meta`; update the relevant component `nodes` list and `size`.
