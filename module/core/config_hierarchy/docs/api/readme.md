# API Doc Entity

### Scope

- **Purpose**: Provide design documentation for the three public traits of config_hierarchy.
- **Responsibility**: Documents each trait's operations, error handling, and compatibility guarantees.
- **In Scope**: ConfigPaths, ConfigDefaults, and ConfigValidator trait design specifications.
- **Out of Scope**: Implementation algorithms (→ algorithm/) and resolution order invariants (→ invariant/).

### Overview Table

| ID  | Name                                                    | Purpose                                                        | Status |
|-----|---------------------------------------------------------|----------------------------------------------------------------|--------|
| 001 | [ConfigPaths Trait](001_config_paths_trait.md)          | Path and naming configuration contract                         | ✅     |
| 002 | [ConfigDefaults Trait](002_config_defaults_trait.md)    | Default configuration values contract                          | ✅     |
| 003 | [ConfigValidator Trait](003_config_validator_trait.md)  | Optional per-parameter and cross-parameter validation contract | ✅     |
