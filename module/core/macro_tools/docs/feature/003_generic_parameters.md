# Feature: Generic Parameters

### Scope
- **Purpose**: Provide decomposition and synthesis of generic parameter lists for proc-macro code generation.
- **Responsibility**: Navigate all artifacts for generic parameter manipulation.
- **In Scope**: Parameter decomposition, merging, filtering, classification, argument access, impl-block token generation.
- **Out of Scope**: Type-level analysis → feature/002; item and struct parsing → feature/004.

### Design
Generic parameter handling operates in two directions. Decomposition breaks a generics
node into lifetime, type, and const parameters for independent processing. Classification
separates parameters by kind without destroying the original structure. Synthesis
recombines or merges parameters from multiple sources — for example, merging generics from
a struct definition with additional bounds for a specific impl block. Filtering removes
parameters matching a predicate, supporting cases where only a subset of parameters apply
to a generated impl. Generic argument access addresses the arguments on applied types
(as opposed to the parameters on declarations). All operations preserve span information
from the original source tokens.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/generic_params.rs` | Generic parameter module router |
| source | `src/generic_params/classification.rs` | Parameter classification by kind |
| source | `src/generic_params/combine.rs` | Multi-source generic parameter merge |
| source | `src/generic_params/filter.rs` | Predicate-based parameter filtering |
| source | `src/generic_args.rs` | Generic argument access on applied types |
| test | `tests/inc/generic_params_test.rs` | Core generic parameter operations |
| test | `tests/inc/generic_params_ref_test.rs` | Reference-based parameter access |
| test | `tests/inc/generic_params_ref_refined_test.rs` | Refined reference access edge cases |
| test | `tests/inc/generic_args_test.rs` | Generic argument access correctness |
| test | `tests/test_decompose_full_coverage.rs` | Full decomposition path coverage |
| test | `tests/test_generic_params_no_trailing_commas.rs` | Trailing comma edge cases |
| test | `tests/test_generic_param_utilities.rs` | Utility function correctness |
| test | `tests/test_trailing_comma_issue.rs` | Trailing comma regression |
