# Doc Entities

## Master Doc Entities Table

| Type          | Purpose                                                              | Master File                                   | Instances |
|---------------|----------------------------------------------------------------------|-----------------------------------------------|----------:|
| `algorithm/`  | Documents each algorithm's inputs, computational logic, and outputs  | [algorithm/readme.md](algorithm/readme.md)    |         1 |
| `api/`        | Documents each trait's operations, error handling, and compatibility guarantees | [api/readme.md](api/readme.md)                |         3 |
| `feature/`    | Cross-references all source, test, and doc artifacts for each feature | [feature/readme.md](feature/readme.md)       |         1 |
| `format/`     | Documents each format's data model, encoding structure, and compatibility | [format/readme.md](format/readme.md)      |         1 |
| `invariant/`  | Documents each invariant's statement, enforcement mechanism, and consequences | [invariant/readme.md](invariant/readme.md) |     1 |

## Master Doc Instances Table

| Entity    | ID  | Name                  | File                                                                           |
|-----------|-----|-----------------------|--------------------------------------------------------------------------------|
| algorithm | 001 | Type Detection        | [algorithm/001_type_detection.md](algorithm/001_type_detection.md)             |
| api       | 001 | ConfigPaths Trait     | [api/001_config_paths_trait.md](api/001_config_paths_trait.md)                 |
| api       | 002 | ConfigDefaults Trait  | [api/002_config_defaults_trait.md](api/002_config_defaults_trait.md)           |
| api       | 003 | ConfigValidator Trait | [api/003_config_validator_trait.md](api/003_config_validator_trait.md)         |
| feature   | 001 | Config Hierarchy      | [feature/001_config_hierarchy.md](feature/001_config_hierarchy.md)             |
| format    | 001 | Config File Format    | [format/001_config_file_format.md](format/001_config_file_format.md)           |
| invariant | 001 | Resolution Hierarchy  | [invariant/001_resolution_hierarchy.md](invariant/001_resolution_hierarchy.md) |
