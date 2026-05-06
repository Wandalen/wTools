# API Doc Entity

### Scope

- **Purpose**: Define contracts for all public traits and their required methods.
- **Responsibility**: Public API contracts for ConfigPaths, ConfigDefaults, and ConfigValidator.
- **In Scope**: Method signatures, required behaviors, and invariants of each trait.
- **Out of Scope**: Implementation algorithms (→ algorithm/), resolution order (→ invariant/)

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [ConfigPaths Trait](001_config_paths_trait.md) | Path resolution customization contract | ✅ |
| 002 | [ConfigDefaults Trait](002_config_defaults_trait.md) | Default values provision contract | ✅ |
| 003 | [ConfigValidator Trait](003_config_validator_trait.md) | Config validation hook contract | ✅ |
| 004 | [ConfigManager Type](004_config_manager.md) | Central config manager public API | ✅ |
