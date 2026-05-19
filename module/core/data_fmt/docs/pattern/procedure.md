# Pattern Instance Procedure

### Scope

- **Trigger**: A new architectural pattern or design decision governing the library needs documentation.
- **Audience**: Contributors adding, updating, or deprecating pattern doc instances.

### Procedure: Create Instance

1. Confirm the pattern belongs here — verify it fits **In Scope** in `readme.md`. API signatures go in `api/`; behavioral contracts go in `invariant/`; algorithm pseudocode goes in `algorithm/`.
2. Assign the next unused NNN from the **Overview Table** in `readme.md` (extends the current maximum, respecting category groupings in `### Organization`).
3. Create `docs/pattern/NNN_name.md` with all required sections per **Type-Specific Requirements** in `readme.md` (Scope, typed cross-reference sections, Description, Structure, Rationale).
4. Add a row to the **Overview Table** in `readme.md` in the correct category position:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`
5. Update `### Organization` in `readme.md` to include the new NNN in the correct category range.
6. Add a row to `docs/entities.md` **Master Doc Instances Table**:
   `| pattern | NNN | Name | [pattern/NNN_name.md](pattern/NNN_name.md) |`
7. Update the **Instances** count in `docs/entities.md` **Master Doc Entities Table** for the `pattern/` row.
8. Add a node to `docs/doc_graph.yml` under `nodes`:
   `- id: pattern/NNN, file: pattern/NNN_name.md, label: Name, entity: pattern, status: active`
9. Add the new node to the `pattern-catalog` component's `nodes` list; update `size`.
10. Update `node_count` in `doc_graph.yml` `meta`.
11. Add edges for every file listed in the new instance's cross-reference sections; update `edge_count` in `meta`.

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Update `### Organization` if the deprecated pattern was the sole member of its category range.
4. Update `docs/entities.md` Master Doc Instances Table: append `(deprecated)` to the Name column.
5. Remove the node from `doc_graph.yml` `pattern-catalog` component `nodes` list; update `size`.
6. Remove all edges referencing this node; update `edge_count` in `meta`; update `node_count` in `meta`.
