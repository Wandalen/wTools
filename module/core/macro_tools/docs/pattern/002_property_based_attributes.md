# Pattern: Property-Based Attributes

### Scope
- **Purpose**: Replace ad-hoc attribute token parsing with typed, composable, self-describing properties.
- **Responsibility**: Document the design pattern for structured attribute parsing across macro_tools consumers.
- **In Scope**: Property typing, composable parsing, type-dispatch assignment, span-aware error generation.
- **Out of Scope**: Concrete property implementations → feature/001; trait contracts → api/001 and api/002.

### Problem
Traditional proc-macro attribute parsing requires custom hand-written parsers for every
attribute. Each parser handles token iteration, keyword matching, value extraction, and error
construction manually. This produces non-uniform parsing code, inconsistent error messages,
attributes that are difficult to extend when new parameters are needed, and no shared testing
of the parsing infrastructure across derive macros.

### Solution
Each attribute parameter is represented as a distinct named type carrying a keyword constant
and a typed value. An attribute struct is composed of these property types. Parsing iterates
tokens and dispatches each recognised keyword to the corresponding property type, which
parses its own value. Assignment to the struct field occurs by type dispatch via the Assign
trait — no string matching at assignment time. Error messages come from property parsers
with the exact span of the offending token.

### Applicability
Apply when an attribute has multiple named parameters; when different call sites enable
different subsets of parameters; when consistent error quality across all parameters is
required; when the attribute structure may grow with new parameters over time; when the
same property kind — such as a boolean flag or optional type — appears in multiple attributes
across different macros.

### Consequences
**Positive**: Each property type is independently testable; new parameters are additive
changes to the property set; error messages are uniform in format and quality; the type
system prevents assigning a value to the wrong field at compile time.

**Negative**: More types per attribute than manual parsing approaches; per-property type
boilerplate; a learning curve for contributors unfamiliar with the trait system.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/attr.rs` | Trait definitions this pattern is built on |
| source | `src/attr_prop.rs` | Concrete property types implementing the pattern |
| source | `src/components.rs` | Assign integration for the pattern |
| doc | `docs/api/001_attribute_component_api.md` | Trait contract for this pattern |
| doc | `docs/api/002_assign_api.md` | Assignment contract for this pattern |
| doc | `docs/feature/001_attribute_parsing.md` | End-to-end feature using this pattern |
