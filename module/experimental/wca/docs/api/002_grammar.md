# API: Grammar

The grammar subsystem defines the type model for command registration: command definitions, the command dictionary, and the type-value system for argument validation.

### Scope

- **Purpose**: Provides the vocabulary for declaring CLI command structure and argument types.
- **Responsibility**: Documents Command, Dictionary, Type, Value, and value descriptor interfaces.
- **In Scope**: Command fields, Dictionary registration and lookup, type and value variants, command ordering.
- **Out of Scope**: Type casting logic (see feature/003), verification process (see api/003).

### Abstract

The grammar module provides three core abstractions: Command defines a single CLI command with its phrase, hints, subject list, property map, property aliases, and routine. Dictionary holds registered Commands in an ordered map and provides lookup by exact name and prefix search. Type and Value form the type system used for argument declaration and runtime representation.

### Operations

Command construction uses the former builder: phrase sets the command name, brief hint and extended hint provide help text, subject and property define typed parameters with optionality, and routine attaches the execution closure.

Dictionary provides registration for adding commands, exact lookup, prefix search (used by dot commands), and ordered iteration in either registration order or alphabetical order.

A value descriptor pairs a hint string with a type and an optional required-flag, used for both subjects and properties. A property descriptor extends this with alias support.

### Error Handling

Dictionary lookup returns an absent result for missing commands. Type casting returns either a typed value or an error indicating the expected type and the actual string that failed conversion.

### Compatibility Guarantees

Command, Dictionary, Type, and Value are public types. The Type enum variants and Value enum variants are part of the stable API surface.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/ca/grammar/command.rs` | Command, ValueDescription, PropertyDescription |
| source | `src/ca/grammar/dictionary.rs` | Dictionary, Order enum |
| source | `src/ca/grammar/types.rs` | Type, Value, TryCast trait |
| test | `tests/inc/grammar/types.rs` | Type casting and conversion tests |
| test | `tests/inc/grammar/from_command.rs` | Single command grammar tests |
| doc | [feature/003_type_system.md](../feature/003_type_system.md) | Type system feature overview |
| doc | [invariant/003_bool_accepted_values.md](../invariant/003_bool_accepted_values.md) | Bool type strictness contract |
| doc | [invariant/005_routine_required.md](../invariant/005_routine_required.md) | Routine presence requirement for commands |
