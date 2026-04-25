# Algorithm Doc Entity

### Scope

- **Purpose**: Preserve algorithm design knowledge for contributors modifying internal resolution or detection logic.
- **Responsibility**: Documents each algorithm's inputs, computational logic, and outputs.
- **In Scope**: Type detection and path resolution algorithms used internally by config_hierarchy.
- **Out of Scope**: User-facing trait contracts (→ api/) and resolution order rules (→ invariant/).

### Overview Table

| ID  | Name                                            | Purpose                                                      | Status |
|-----|-------------------------------------------------|--------------------------------------------------------------|--------|
| 001 | [Type Detection](001_type_detection.md)         | String-to-typed-value conversion for env var and file values | ✅     |
