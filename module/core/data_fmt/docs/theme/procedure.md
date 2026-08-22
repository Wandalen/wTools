# Theme Instance Procedure

### Scope

- **Trigger**: A new predefined color theme, or the custom-theme mechanism, needs documentation.
- **Audience**: Contributors adding, updating, or deprecating theme doc instances.

### Procedure: Create Instance

1. Confirm the theme belongs here — verify it fits **In Scope** in `readme.md` (per-theme color role specification). Application behavior goes in `feature/004_color_themes.md`; attribute schema reference is in `data_structure/002_theme_attributes.md`; API signatures are in `api/005_theme_types.md`.
2. Assign the next unused NNN from the **Overview Table** in `readme.md` (extends the current maximum, respecting the construction-path groupings in `### Organization`).
3. Create `docs/theme/NNN_name.md` with all required sections per **Required instance sections** in `readme.md` (Scope, typed cross-reference sections, all 21 attributes grouped per `data_structure/002_theme_attributes.md`, Example Output).
4. Add a row to the **Overview Table** in `readme.md` in the correct position:
   `| NNN | [Name](NNN_name.md) | One-line purpose | 🔄 |`
5. Update `### Organization` in `readme.md` if the new instance introduces a new construction path.
6. Add a row to `docs/entity.md` **Master Doc Instances Table**:
   `| theme | NNN | Name | [theme/NNN_name.md](theme/NNN_name.md) |`
7. Update the **Instances** count in `docs/entity.md` **Master Doc Entities Table** for the `theme/` row.
8. Add a node to `docs/doc_graph.yml` under `nodes`:
   `- id: theme/NNN, file: theme/NNN_name.md, label: Name, entity: theme`
9. Add the new node to the component's `nodes` list; update `size`.
10. Update `node_count` in `doc_graph.yml` `meta`.
11. Add edges for every file listed in the new instance's cross-reference sections (typically `data_structure/002`, `api/005`, `feature/004`); update `edge_count` in `meta`.

### Procedure: Deprecate Instance

1. Do **not** delete `NNN_name.md` or remove its Overview Table row — ID permanence is required.
2. Change the row status in **Overview Table** to `❌`.
3. Update `### Organization` if the deprecated theme was the sole member of its construction path.
4. Update `docs/entity.md` Master Doc Instances Table: append `(deprecated)` to the Name column.
5. Remove the node from `doc_graph.yml`'s component `nodes` list; update `size`.
6. Remove all edges referencing this node; update `edge_count` in `meta`; update `node_count` in `meta`.
