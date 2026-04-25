# src/attr_prop/

Property-based attribute component implementations. Type-safe wrappers for parsing different property value types in procedural macro attributes.

## Organization

Paired modules for each property type: base type handles required properties, `_optional` variant handles optional properties.

## Property Types

Each property type provides trait implementations for parsing, conversion, and assignment following the component model pattern.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `boolean.rs` | Parse and store required boolean property values |
| `boolean_optional.rs` | Parse and store optional boolean property values |
| `singletone.rs` | Parse and store required single-value properties |
| `singletone_optional.rs` | Parse and store optional single-value properties |
| `syn.rs` | Parse and store required syn-parsed property values |
| `syn_optional.rs` | Parse and store optional syn-parsed property values |
