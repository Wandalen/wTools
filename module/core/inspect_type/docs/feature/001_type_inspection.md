# Feature: Type Inspection

### Scope

- **Purpose**: Enable runtime inspection of any value's type name and memory size without consuming the value.
- **Responsibility**: Documents the type inspection feature — its two inspection modes, output contract, and all implementing artifacts.
- **In Scope**: Type name reporting, size-in-bytes reporting, string-mode and print-mode inspection, single-value inspection.
- **Out of Scope**: Compile-time type inspection, type comparison, deep inspection of value contents, multi-value batch inspection.

### Design

Type inspection provides two modes that operate on any expression passed by the caller:

- **String mode** (`inspect_to_str_type_of`): inspects the value, formats the result as a string, and returns it without side effects.
- **Print mode** (`inspect_type_of`): delegates to string mode, prints the result to standard output, and returns the same string.

Both modes take the expression by reference internally to avoid consuming it. When the caller passes a reference, a reference-to-reference is formed — this is intentional and correctly reports the referenced type rather than the reference wrapper.

The output format is fixed across both modes: `sizeof( {expression} : {type_name} ) = {size_in_bytes}`. See invariant/002_fixed_output_format.md for the formal contract.

The crate carries zero runtime dependencies by design — both macros rely exclusively on stable standard library facilities available since Rust 1.76. See invariant/001_zero_dependencies.md.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Implements both inspection macros |
| test | `tests/corner_cases_test.rs` | Comprehensive type coverage across 16 categories |
| test | `tests/example_produces_output_test.rs` | Verifies examples produce expected stdout output |
| doc | `docs/api/001_inspect_to_str_type_of.md` | String-mode inspection macro — operations and compatibility |
| doc | `docs/api/002_inspect_type_of.md` | Print-mode inspection macro — operations and compatibility |
| doc | `docs/invariant/001_zero_dependencies.md` | Zero runtime dependency constraint |
| doc | `docs/invariant/002_fixed_output_format.md` | Fixed output format invariant |
