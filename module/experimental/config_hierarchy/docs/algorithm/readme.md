# Algorithm Doc Entity

### Scope

- **Purpose**: Specify detection and resolution algorithms used internally by config_hierarchy.
- **Responsibility**: Internal algorithm specifications for type detection and hierarchical resolution.
- **In Scope**: Type detection heuristics and resolution waterfall logic.
- **Out of Scope**: User-facing trait contracts (→ api/)

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Type Detection](001_type_detection.md) | Heuristic inference of config value types | ✅ |
| 002 | [Resolution Waterfall](002_resolution_waterfall.md) | Hierarchical config source precedence | ✅ |
