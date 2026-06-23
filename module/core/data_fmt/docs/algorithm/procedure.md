# Algorithm Instance Procedure

### Scope

- **Trigger**: A non-trivial algorithm used by the library needs documentation.
- **Audience**: Contributors adding, updating, or deprecating algorithm doc instances.

### Procedure: Create Instance

1. Confirm the algorithm belongs here — verify it fits **In Scope** in `readme.md` (non-trivial, used by formatters, has measurable complexity). Simple one-liners or delegated library calls do not qualify.
2. Assign the next unused NNN from the **Overview Table** in `readme.md` (e.g., if 005 is the last row, assign 006).
3. Create `docs/algorithm/NNN_name.md` with the following sections in order: `### Scope`, `### Sources`, `### Tests`, `### Trigger Condition`, `### Algorithm` (pseudocode), `### Complexity`.
4. Add a row to the **Overview Table** in `readme.md`:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`
5. Add a row to `docs/entity.md` **Master Doc Instances Table**:
   `| algorithm | NNN | Name | [algorithm/NNN_name.md](algorithm/NNN_name.md) |`
6. Update the **Instances** count in `docs/entity.md` **Master Doc Entities Table** for the `algorithm/` row.
7. Add a node to `docs/doc_graph.yml` under `nodes`:
   `- id: algorithm/NNN, file: algorithm/NNN_name.md, label: Name, entity: algorithm, status: planned`
8. Update `node_count` in `doc_graph.yml` `meta`.
9. Add edges for every file listed in the new instance's `### Sources` and `### Tests`; update `edge_count` in `meta`.
10. Update or add the new node to the appropriate `components` entry; update `size`.

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Update `docs/entity.md` Master Doc Instances Table: append `(deprecated)` to the Name column.
4. Remove the node from `doc_graph.yml` `nodes`; remove all edges referencing this node.
5. Update `node_count` and `edge_count` in `meta`; update the relevant component `nodes` list and `size`.
