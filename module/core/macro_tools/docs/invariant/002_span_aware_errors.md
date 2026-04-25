# Invariant: Span-Aware Errors

### Scope
- **Purpose**: Ensure proc-macro error messages point to the exact user-code token that caused the failure.
- **Responsibility**: Document the invariant that every error produced by macro_tools operations carries source span information.
- **In Scope**: All fallible operations during attribute parsing, type analysis, and code generation.
- **Out of Scope**: Internal panics and logic errors; runtime errors outside the proc-macro expansion context.

### Invariant Statement
Every error value produced by any macro_tools operation that may fail carries a source span
corresponding to the token or tokens in the user's source code that caused the failure.
No error is constructed with a call-site span, a synthesized fake span, or no span at all,
unless no better span is available from the original source tokens.

### Enforcement Mechanism
The Result type alias fixes the error type to the syn error type, which always carries a
span. The primary error creation macro takes a token reference alongside the message format,
ensuring the span is specified at error construction time. The early-return error macro
enforces that the same convention is used at propagation sites. Operations that parse tokens
from user input always thread the token's span through to any error they produce, rather
than substituting a generic location.

### Violation Consequences
A span-free or wrong-span error message causes the Rust compiler's error caret to point to
the wrong location in user source — typically the macro invocation site or an internal
implementation location rather than the specific attribute token, type expression, or field
name that is actually malformed. The user sees a correct error message text but the pointing
indicator is misleading, making the error appear unrelated to its cause and substantially
increasing debugging time.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/diag.rs` | Span-aware error macro implementations |
| doc | `docs/feature/005_error_diagnostics.md` | End-to-end error diagnostics capability |
