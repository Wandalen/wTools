# API: Error-With-Report Operation Set

### Scope

- **Purpose**: Provide a mechanism to pair an error with a pre-built report value at the point of failure.
- **Responsibility**: Documents the error-with-report operation set — its operations, error conditions, and compatibility guarantees.
- **In Scope**: The error-with-report operation set with two conversion operations, the paired-result return type shorthand, and the base error interface re-export.
- **Out of Scope**: Error formatting or display, integration with typed or untyped error components, and asynchronous forms of the conversion operations.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_error_facade.md](../feature/001_error_facade.md) | Facade that exposes this API surface |

### Abstract

An operation set available on all fallible return types that converts a failing result into a paired report-and-error result, where the report carries context generated regardless of success or failure. Accompanied by a paired-result return type shorthand and a base error interface re-export.

### Operations

**err_with** — accepts a zero-argument callback that builds the report value. On success returns the report-wrapped success; on failure returns a paired value containing the built report and the original error.

**err_with_report** — accepts a pre-built report value. On failure copies the report and pairs it with the original error. Requires the report type to support copying. Use when the report is built before the operation returns.

**Paired-Result Shorthand** — shorthand return type for a fallible operation whose failure value is a paired report-and-error. Used as the return type of both operations to make the pairing explicit.

**Error Interface Re-export** — re-export of the base error interface for uniform access without a direct standard library import.

### Error Handling

Both conversion operations are infallible — they never return an error from the conversion itself. When the underlying result is an error, the operation pairs the report with the original error and returns it; the original error is preserved unchanged. The operation set covers all fallible return types; no error condition arises from applying it to any well-formed fallible type.

### Compatibility Guarantees

The operation set and return type shorthand are stable across minor versions. The callback-based `err_with` operation is the primary form; `err_with_report` is the secondary form for callers who build the report before the operation returns. Both are permanent — neither is deprecated in favor of the other.
