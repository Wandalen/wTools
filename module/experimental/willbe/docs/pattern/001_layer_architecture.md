# Pattern: Layer Architecture

### Scope

- **Purpose**: Document the five-layer decomposition that organizes all willbe source modules and establishes dependency direction rules.
- **Responsibility**: Authoritative description of the layer boundaries and the rationale for the separation.
- **In Scope**: Layer definitions (CLI, Command, Action, Entity, Tool), module file assignment per layer, inter-layer dependency rules.
- **Out of Scope**: Per-command implementation details (see `../../src/`), API signatures (see `../api/`), feature behavior (see `../feature/`).

### Problem

A workspace management tool must parse CLI arguments, implement business logic, manage domain data structures, and invoke external utilities (cargo, git, HTTP). Without explicit layer boundaries, these concerns mix, making the codebase hard to test and maintain.

### Solution

Five layers with strict downward dependency flow:

```
┌────────────────────────────────────────────┐
│  CLI Layer  (src/bin/)                     │
│  Binary entry points: will, willbe         │
├────────────────────────────────────────────┤
│  Command Layer  (src/command/)             │
│  CLI argument parsing via wca              │
├────────────────────────────────────────────┤
│  Action Layer  (src/action/)               │
│  Business logic implementation             │
├────────────────────────────────────────────┤
│  Entity Layer  (src/entity/)               │
│  Domain data structures and models         │
├────────────────────────────────────────────┤
│  Tool Layer  (src/tool/)                   │
│  Low-level utilities: cargo, git, graph    │
└────────────────────────────────────────────┘
```

Each layer may only depend on layers below it. The CLI layer calls Command; Command calls Action; Action calls Entity and Tool; Entity calls Tool. No upward dependencies.

### Applicability

- New functionality belongs in Action layer if it orchestrates business logic
- New data structures belong in Entity layer
- New external-process invocations belong in Tool layer
- Command layer contains only argument parsing and result formatting — no business logic

### Consequences

- **Benefit**: Actions can be tested without CLI (invoke action functions directly in tests)
- **Benefit**: Entities and tools are reusable across commands
- **Benefit**: Clear change isolation — CLI changes don't affect business logic
- **Limitation**: Adding a new CLI parameter requires changes in three layers (Command, Action, and whatever downstream function uses it)

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/lib.rs` | Crate root: mod_interface layer wiring |
| source | `../../src/bin/` | CLI layer: binary entry points |
| source | `../../src/command/` | Command layer: wca argument parsing |
| source | `../../src/action/` | Action layer: business logic |
| source | `../../src/entity/` | Entity layer: domain models |
| source | `../../src/tool/` | Tool layer: cargo, git, graph utilities |
