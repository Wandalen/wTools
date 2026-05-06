# src/commands/

Command registration — loads YAML definitions and wires handler functions into the registry.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| mod.rs | Registry creation and coordination of all command modules |
| archive.rs | Archive command registration and YAML loading |
| file.rs | File command registration and YAML loading |
| value.rs | Value command registration and YAML loading |
| parameter.rs | Parameter command registration and YAML loading |
| content.rs | Content command registration and YAML loading |
| materialize.rs | Materialize command registration and YAML loading |
| pack.rs | Pack command registration and YAML loading |
| info.rs | Info and status command registration and YAML loading |
