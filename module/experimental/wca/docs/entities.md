# Doc Entities

## Master Doc Entities Table

| Type | Purpose | Master File | Instances |
|------|---------|-------------|----------:|
| `api/` | Describes operations, error handling, and compatibility for each API surface. | [api/readme.md](api/readme.md) | 5 |
| `feature/` | Indexes source files, test files, and related documentation per feature without duplicating content. | [feature/readme.md](feature/readme.md) | 7 |
| `invariant/` | Documents invariant statements, enforcement mechanisms, and violation consequences. | [invariant/readme.md](invariant/readme.md) | 5 |

## Master Doc Instances Table

| Entity    | ID  | Name                  | File                                                                             |
|-----------|-----|-----------------------|----------------------------------------------------------------------------------|
| api       | 001 | Commands Aggregator   | [api/001_commands_aggregator.md](api/001_commands_aggregator.md)                 |
| api       | 002 | Grammar               | [api/002_grammar.md](api/002_grammar.md)                                         |
| api       | 003 | Verifier              | [api/003_verifier.md](api/003_verifier.md)                                       |
| api       | 004 | Executor              | [api/004_executor.md](api/004_executor.md)                                       |
| api       | 005 | Input                 | [api/005_input.md](api/005_input.md)                                             |
| feature   | 001 | Command Pipeline      | [feature/001_command_pipeline.md](feature/001_command_pipeline.md)               |
| feature   | 002 | Fluent Builder        | [feature/002_fluent_builder.md](feature/002_fluent_builder.md)                   |
| feature   | 003 | Type System           | [feature/003_type_system.md](feature/003_type_system.md)                         |
| feature   | 004 | Help System           | [feature/004_help_system.md](feature/004_help_system.md)                         |
| feature   | 005 | Command Routing       | [feature/005_command_routing.md](feature/005_command_routing.md)                 |
| feature   | 006 | Context Sharing       | [feature/006_context_sharing.md](feature/006_context_sharing.md)                 |
| feature   | 007 | Fuzzy Suggest         | [feature/007_fuzzy_suggest.md](feature/007_fuzzy_suggest.md)                     |
| invariant | 001 | Dot Prefix Required   | [invariant/001_dot_prefix_required.md](invariant/001_dot_prefix_required.md)     |
| invariant | 002 | Colon Property Syntax | [invariant/002_colon_property_syntax.md](invariant/002_colon_property_syntax.md) |
| invariant | 003 | Bool Accepted Values  | [invariant/003_bool_accepted_values.md](invariant/003_bool_accepted_values.md)   |
| invariant | 004 | Help No Execute       | [invariant/004_help_no_execute.md](invariant/004_help_no_execute.md)             |
| invariant | 005 | Routine Required      | [invariant/005_routine_required.md](invariant/005_routine_required.md)           |
