# Feature: Trait Implementation Check

### Scope

- **Purpose**: Enable callers to determine at the call site whether the type of a given expression satisfies one or more trait bounds, obtaining a bool result without consuming or altering the expression.
- **Responsibility**: Documents the trait implementation check feature — its runtime evaluation model, non-consuming contract, and all implementing artifacts.
- **In Scope**: Checking a single expression against one or more trait bounds at the point of call, returning a bool.
- **Out of Scope**: Compile-time type branching, type comparison between two values, batch checking across multiple expressions, checking trait bounds that require associated type constraints.

### Design

The check is performed using autoref specialization: a temporary phantom value is constructed from a non-consuming reference to the expression, and method resolution produces either true or false depending on whether the type satisfies the given trait bounds. No runtime dispatch occurs — the compiler resolves the outcome entirely at compile time; the resulting bool is a compile-time constant wrapped in a runtime expression.

Two names are provided for the same behaviour: `implements` and `instance_of`. Both accept the same input form and produce identical output. The alias exists to serve callers who prefer the phrasing "is this value an instance of this trait" over "does this value implement this trait".

The macro accepts compound trait bounds using the same syntax as Rust's where-clause bounds. Checking against multiple traits at once is supported by expressing them in conjunction.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Public macro exports — implements and instance_of |
| source | `src/implements_impl.rs` | Internal autoref specialization mechanism |
| test | `tests/tests.rs` | Test root — delegates to test cases |
| test | `tests/inc/test_cases.rs` | Comprehensive cases: basic types, closures, function traits |
| doc | `docs/api/001_implements.md` | implements macro — accepted inputs and return contract |
| doc | `docs/api/002_instance_of.md` | instance_of macro — alias for implements |
| doc | `docs/invariant/001_value_not_consumed.md` | Non-consuming evaluation guarantee |
| doc | `docs/invariant/002_zero_runtime_dependencies.md` | Zero runtime dependency constraint |
