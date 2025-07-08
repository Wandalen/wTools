# Change Proposal for strs_tools

### Task ID
*   TASK-20250708-STRSTOOLS-ITERATOR-FIX

### Requesting Context
*   **Requesting Crate/Project:** `unilang_instruction_parser`
*   **Driving Feature/Task:** Fixing parsing logic and re-enabling tests in `unilang_instruction_parser` (specifically, `Increment 3: Fix Unescaping and Re-enable Tests`).
*   **Link to Requester's Plan:** `module/move/unilang_instruction_parser/task/task_plan.md`
*   **Date Proposed:** 2025-07-08

### Overall Goal of Proposed Change
*   To ensure that `strs_tools::split::SplitOptions` correctly implements the `Iterator` trait when the delimiter type `D` is `Vec<&str>`, allowing it to be consumed by methods like `collect()` or iterated over directly without compilation errors related to unsatisfied trait bounds.

### Problem Statement / Justification
*   The `unilang_instruction_parser` crate relies on `strs_tools` for robust string splitting and tokenization. Currently, when `strs_tools::split()...form()` is used with a `Vec<&str>` as the delimiter type (e.g., `delimeter(vec!["...", "..."])`), the resulting `split::private::SplitOptions` struct fails to satisfy the `Iterator` trait bounds, leading to compilation errors like `error[E0599]: the method `into_iter` exists for struct ..., but its trait bounds were not satisfied`. This prevents the `unilang_instruction_parser` from compiling and utilizing `strs_tools` as intended. A Minimal Reproducible Example (MRE) demonstrating this issue has been created at `module/move/unilang_instruction_parser/tests/strs_tools_mre.rs`.

### Proposed Solution / Specific Changes
*   **API Changes:** No public API changes are expected for `strs_tools`. The change is internal to ensure existing `Iterator` trait implementations are correctly satisfied for all valid `D` types, specifically `Vec<&str>`.
*   **Behavioral Changes:** `strs_tools::split::SplitOptions` should behave as a standard iterator when `Vec<&str>` is used as the delimiter type, allowing direct iteration and collection into `Vec<Split<'_>>`.
*   **Internal Changes:** The internal implementation of `SplitOptions` or its `Iterator` trait bounds may need adjustment to correctly handle the `Vec<&str>` delimiter type. This might involve ensuring lifetimes are correctly propagated or that `OpType<T>` correctly implements `From<Vec<T>>` in all necessary contexts for iteration.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   The `unilang_instruction_parser` expects to be able to use `strs_tools::split()...form().iter().collect()` or `for s in strs_tools::split()...form()` without compilation errors.
*   Example from `unilang_instruction_parser`:
    ```rust
    use strs_tools::string::split::{ Split, SplitType };
    let input = "test string";
    let delimiters = vec![ " " ];
    let splits : Vec< Split<'_> > = strs_tools::split()
    .src( input )
    .delimeter( delimiters )
    .form()
    .iter() // This line currently causes the error
    .collect();
    // Expected: `splits` contains the correctly parsed `Split` items.
    ```

### Acceptance Criteria (for this proposed change)
*   The `module/move/unilang_instruction_parser/tests/strs_tools_mre.rs` test compiles and passes (or is ignored if the fix makes it unnecessary to run).
*   The `unilang_instruction_parser` crate compiles successfully when using `strs_tools::split()...form().iter().collect()` with `Vec<&str>` delimiters.

### Potential Impact & Considerations
*   **Breaking Changes:** No breaking changes are anticipated, as this aims to fix an existing compilation issue and ensure expected `Iterator` behavior.
*   **Dependencies:** No new dependencies.
*   **Performance:** No significant performance impact is expected.
*   **Testing:** The `strs_tools` crate's test suite should be updated to include a test case similar to the provided MRE to prevent regressions.

### Notes & Open Questions
*   The exact cause of the unsatisfied trait bounds for `SplitOptions<'_, Vec<&str>>: Iterator` needs to be investigated within the `strs_tools` crate.