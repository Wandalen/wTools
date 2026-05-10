# src/commands/

Command registration — builds `CommandDefinition` structs and wires handler functions into the registry.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| mod.rs | Registry creation and coordination of all command modules |
| archive.rs | Archive command registration (.archive.*) |
| file.rs | File command registration (.file.*) |
| value.rs | Value command registration (.value.*) |
| parameter.rs | Parameter command registration (.parameter.*) |
| content.rs | Content command registration (.content.*) |
| materialize.rs | Materialize command registration (.materialize, .unpack) |
| pack.rs | Pack command registration (.pack) |
| info.rs | Info and status command registration (.info, .status, .analyze, .discover.*) |
