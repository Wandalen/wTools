# API Instance Procedure

### Scope

- **Trigger**: A new public interface group needs documentation, or an existing API instance requires updating.
- **Audience**: Contributors adding, updating, or deprecating API doc instances.

### Procedure: Create Instance

1. Confirm the API belongs here — verify it is a public interface description, not a feature (→ `feature/`) or invariant (→ `invariant/`). Apply the YAGNI gate: only create when a committed need exists.
2. Assign the next unused NNN from the **Overview Table** in `readme.md`.
3. Create `docs/api/NNN_name.md` at Progressive Documentation Level 1 minimum (H1 title + Abstract). Elaborate to Level 2 before release.
4. Add a row to the **Overview Table** in `readme.md`:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`
5. Add a row to `docs/entities.md` **Master Doc Instances Table**:
   `| api | NNN | Name | [api/NNN_name.md](api/NNN_name.md) |`
6. Update the **Instances** count in `docs/entities.md` **Master Doc Entities Table** for the `api/` row.
7. Add a node to `docs/doc_graph.yml` under `nodes`:
   `- id: api/NNN, file: api/NNN_name.md, label: Name, entity: api`
8. Update `node_count` in `doc_graph.yml` `meta`.
9. Add edges for every doc listed in the new instance's `### Cross-References`; update `edge_count` in `meta`.
10. Update or add the new node to the appropriate `components` entry; update `size`.

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Remove the row for this instance from `docs/entities.md` Master Doc Instances Table in the same session.
4. Remove the node from `doc_graph.yml` `nodes`; remove all edges referencing this node.
5. Update `node_count` and `edge_count` in `meta`; update the relevant component `nodes` list and `size`.
