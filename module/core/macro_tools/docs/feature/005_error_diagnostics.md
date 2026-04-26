# Feature: Error Diagnostics

### Scope
- **Purpose**: Provide span-aware error creation and formatting for proc-macro user messages.
- **Responsibility**: Navigate all artifacts for error creation, formatting, and equation-based diagnostics.
- **In Scope**: Span-attached error macros, diagnostic formatting, equation parsing for structured error context.
- **Out of Scope**: Compile-time string formatting → feature/006; test diagnostics; runtime error handling.

### Design
Errors in proc-macro contexts must carry the source span of the offending token so the
compiler can point to the exact location in user code. The primary error macro creates a
span-attached error at any token's position with a formatted message. A complementary
early-return macro provides ergonomic error propagation for the common pattern of
validating and returning in one step. Equation parsing supports structured diagnostics
that express expected-vs-actual relationships, producing clearer user messages when
attribute values do not match their expected forms.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/diag.rs` | Diagnostic formatting and span-aware error utilities |
| source | `src/equation.rs` | Equation parsing for structured error context |
| test | `tests/inc/diag_test.rs` | Diagnostic correctness |
| test | `tests/inc/equation_test.rs` | Equation parsing correctness |
| doc | `docs/invariant/002_span_aware_errors.md` | Invariant that all errors must carry spans |
| doc | `docs/feature/001_attribute_parsing.md` | Primary consumer: attribute parsing produces span-aware errors |
| doc | `docs/feature/006_code_generation_support.md` | Compile-time string formatting — out of scope for error diagnostics |
