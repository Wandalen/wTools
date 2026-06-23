# Invariant Instance Procedure

### Scope

- **Trigger**: A behavioral contract or guarantee maintained across all operations needs formal documentation.
- **Audience**: Contributors adding, updating, or deprecating invariant doc instances.

### Procedure: Create Instance

1. Confirm the invariant belongs here — verify it fits **In Scope** in `readme.md` (behavioral contract, measurable guarantee). API details go in `api/`; formatting features go in `feature/`. Every invariant must have a measurable threshold or a verifiable condition — vague statements ("must be fast") are invalid.
2. Assign the next unused NNN from the **Overview Table** in `readme.md`.
3. Create `docs/invariant/NNN_name.md` with the following sections in order: `### Scope`, typed cross-reference H3 sections (see existing instances), `### Statement` (measurable form), `### Enforcement` (how it is monitored or tested), `### Violation Consequences`.
4. Add a row to the **Overview Table** in `readme.md`:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`
5. Add a row to `docs/entity.md` **Master Doc Instances Table**:
   `| invariant | NNN | Name | [invariant/NNN_name.md](invariant/NNN_name.md) |`
6. Update the **Instances** count in `docs/entity.md` **Master Doc Entities Table** for the `invariant/` row.
7. Add a node to `docs/doc_graph.yml` under `nodes`:
   `- id: invariant/NNN, file: invariant/NNN_name.md, label: Name, entity: invariant, status: planned`
8. Update `node_count` in `doc_graph.yml` `meta`.
9. Add edges for every file listed in the new instance's cross-reference sections; update `edge_count` in `meta`.
10. Update or add the new node to the appropriate `components` entry; update `size`.

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Update `docs/entity.md` Master Doc Instances Table: append `(deprecated)` to the Name column.
4. Remove the node from `doc_graph.yml` `nodes`; remove all edges referencing this node.
5. Update `node_count` and `edge_count` in `meta`; update the relevant component `nodes` list and `size`.
