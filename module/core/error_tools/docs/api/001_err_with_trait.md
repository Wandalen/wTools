# API: Error-With-Report Trait

### Scope

- **Purpose**: Provide a mechanism to pair an error with a pre-built report value at the point of failure.
- **Responsibility**: Documents the error-with-report trait API — its operations, error conditions, and compatibility guarantees.
- **In Scope**: The error-with-report trait with two conversion methods, the paired-result type alias, and the error-trait alias.
- **Out of Scope**: Error formatting or display, integration with typed or untyped error components, and async variants of the conversion methods.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_error_facade.md](../feature/001_error_facade.md) | Facade that exposes this API surface |

### Abstract

A trait implemented for all result types that converts a failing result into a paired report-and-error result, where the report carries context generated regardless of success or failure. Accompanied by a paired-result type alias and an error-trait re-export.

### Operations

**err_with** — accepts a closure that receives the result's output and builds the report value. On success returns the report-wrapped success; on failure returns a two-element tuple of the built report and the original error.

**err_with_report** — accepts a pre-built report value by reference. On failure clones the report and pairs it with the original error. Requires the report type to implement the clone contract. Use when the report is built before the operation returns.

**Paired-Result Alias** — type alias for a result whose error variant is a two-element tuple of report and error. Used as the return type of both methods to make the pairing explicit in function signatures.

**Error Trait Alias** — re-export of the standard library's base error trait for uniform access without a direct standard library import.

### Error Handling

Both conversion methods are infallible — they never return an error from the conversion itself. When the underlying result is an error, the method pairs the report with the original error and returns it; the original error is preserved unchanged. The blanket implementation covers all result types; no error condition arises from applying the trait to an incompatible type (the trait bound is always satisfied for well-formed result types).

### Compatibility Guarantees

The trait and type alias are stable across minor versions. The closure-based `err_with` method is the primary form; `err_with_report` is the secondary form for callers who build the report before the operation returns. Both are permanent — neither is deprecated in favor of the other.
