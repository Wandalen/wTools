# Feature Instance Procedure

### Scope

- **Trigger**: A committed task exists for a new library capability (status 🔄 Planned or higher).
- **Audience**: Contributors adding, updating, or deprecating feature doc instances.

### Procedure: Create Instance

1. Confirm the feature belongs here — verify it fits **In Scope** in `readme.md`. API signatures go in `api/`; output examples go in `variant/`. Apply the YAGNI gate: only create when a committed task exists.
2. Assign the next unused NNN from the **Overview Table** in `readme.md`.
3. Create `docs/feature/NNN_name.md` at Progressive Documentation Level 1 minimum (H1 title + one-paragraph Statement). Elaborate to Level 2 before implementation begins.
4. Add a row to the **Overview Table** in `readme.md`:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`
5. Add a row to `docs/entities.md` **Master Doc Instances Table**:
   `| feature | NNN | Name | [feature/NNN_name.md](feature/NNN_name.md) |`
6. Update the **Instances** count in `docs/entities.md` **Master Doc Entities Table** for the `feature/` row.
7. Add a node to `docs/doc_graph.yml` under `nodes`:
   `- id: feature/NNN, file: feature/NNN_name.md, label: Name, entity: feature, status: planned`
8. Update `node_count` in `doc_graph.yml` `meta`.
9. Add edges for every file listed in the new instance's cross-reference sections; update `edge_count` in `meta`.
10. Update or add the new node to the appropriate `components` entry; update `size`.

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Update `docs/entities.md` Master Doc Instances Table: append `(deprecated)` to the Name column.
4. Remove the node from `doc_graph.yml` `nodes`; remove all edges referencing this node.
5. Update `node_count` and `edge_count` in `meta`; update the relevant component `nodes` list and `size`.
