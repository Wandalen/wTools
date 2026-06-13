# Test Surface Specs

### Scope

- **Purpose**: House test surface specification documents for doc entity test coverage.
- **Responsibility**: Root directory for test-aligned documentation specs.
- **In Scope**: All 12 doc entity type surfaces: algorithm (006), api (004), builder (002), data_structure (001), feature (007), formatter (010), input_model (002), input_type (002), invariant (004), pattern (004), trait (003), variant (033) — covering all instances from corresponding `docs/` directories.
- **Out of Scope**: Test code organization (see `tests/readme.md`), manual testing procedures (see `tests/manual/readme.md`).

### Responsibility Table

| Directory | Responsibility |
|-----------|----------------|
| `algorithm/` | Algorithm correctness spec files; AC-N cases, min 4 per spec |
| `api/` | API contract verification spec files; AP-N cases, min 4 per spec |
| `builder/` | Builder API contract spec files; BL-N cases, min 4 per spec |
| `data_structure/` | Data structure contract spec files; DS-N cases, min 4 per spec |
| `feature/` | Feature behavioral spec files; FT-N cases, min 4 per spec |
| `formatter/` | Formatter output contract spec files; FM-N cases, min 4 per spec |
| `input_model/` | Input model contract spec files; IM-N cases, min 4 per spec |
| `input_type/` | Input type contract spec files; IV-N cases, min 4 per spec |
| `invariant/` | Invariant enforcement spec files; IN-N cases, min 2 per spec |
| `pattern/` | Design pattern verification spec files; PT-N cases, min 3 per spec |
| `trait/` | Trait contract spec files; TR-N cases, min 4 per spec |
| `variant/` | Variant output contract spec files; VT-N cases, min 4 per spec |
