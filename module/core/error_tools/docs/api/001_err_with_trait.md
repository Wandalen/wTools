# API: Error-With-Report Trait

### Scope

**Purpose:** Provide a mechanism to pair an error with a pre-built report value at the point of failure.

**Responsibility:** Define the error-with-report trait and the paired-result type alias that enable callers to produce a report-and-error pair from a result, useful when the report must be constructed regardless of whether the operation succeeds or fails.

**In Scope:**
- The error-with-report trait with two conversion methods
- The paired-result type alias used as the return type of both methods
- The error-trait alias re-exporting the standard library base error trait

**Out of Scope:**
- Error formatting or display
- Integration with the untyped or typed error components
- Async variants of the conversion methods

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| Feature | feature/001_error_facade.md | Facade that exposes this API surface |

### Abstract

**Error-With-Report Trait**

A trait implemented for all result types. Converts a failing result into a paired report-and-error result, where the report carries context that was generated regardless of success or failure.

**err_with** — accepts a closure that receives the result's output and builds the report value. On success returns the report-wrapped success; on failure returns a pair of the built report and the original error.

**err_with_report** — accepts a pre-built report value by reference. On failure clones the report and pairs it with the original error. Requires the report type to implement the clone contract.

**Paired-Result Alias**

A type alias for a result whose error variant is a two-element tuple of report and error. Used as the return type of all error-with-report methods to make the pairing explicit in function signatures.

**Error Trait Alias**

A type alias for the standard library's base error trait, re-exported for uniform access without a direct standard library import.

### Implementations

| Target | Notes |
|--------|-------|
| All result types | Blanket implementation — no consumer action required |

### Compatibility Guarantees

The trait and type alias are stable across minor versions. The closure-based `err_with` method is the primary form; `err_with_report` is provided for callers who build the report before the operation returns.
