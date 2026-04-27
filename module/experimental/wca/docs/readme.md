# Docs

### Scope

Design, API, and invariant documentation for the wca crate.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `entities.md` | Master index of all doc entity types and instances |
| `doc_graph.yml` | Cross-reference graph of all doc instances |
| `feature/` | Behavioral feature documentation |
| `api/` | Public interface documentation |
| `invariant/` | Correctness property documentation |

### System Actors

| Actor | Role |
|-------|------|
| Application developer | Builds CLI apps using CommandsAggregator builder |
| Pipeline consumer | Uses raw Parser, Verifier, Executor components directly |
| End user | Invokes dot-prefixed commands and help queries at runtime |
| willbe | Primary workspace consumer using the fluent builder API |

### Vocabulary

| Term | Definition |
|------|------------|
| Command | A registered CLI operation with phrase, subjects, properties, and routine |
| Dictionary | Ordered map of registered Command definitions |
| Pipeline | Three-stage processing: parse, verify, execute |
| Routine | Execution closure attached to a command (with or without context) |
| Subject | Positional argument to a command, typed via ValueDescription |
| Property | Named argument using colon syntax, typed via PropertyDescription |
| Internal command | Dot-suffix or question-mark-suffix command routed to help display |
| Former | Builder pattern for constructing aggregator and command definitions |
| Context | Type-erased shared state passed to routines via Arc |
| Phrase | The dot-prefixed name that identifies a command |
