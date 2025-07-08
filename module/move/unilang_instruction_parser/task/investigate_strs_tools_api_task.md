# Task Plan: Investigate `strs_tools::string::split::SplitOptionsFormer` API

### Goal
*   To thoroughly investigate the `strs_tools` crate's `SplitOptionsFormer` API, specifically its methods for setting delimiters and its lifetime requirements. The primary goal is to understand why passing a `Vec<&str>` (derived from `Vec<String>`) to `SplitOptionsFormer::new()` results in `E0716: temporary value dropped while borrowed` and `E0507: cannot move out of *former which is behind a mutable reference` errors. A robust solution for correctly passing dynamic delimiters to `SplitOptionsFormer` without lifetime or ownership errors must be identified and documented.

### Ubiquitous Language (Vocabulary)
*   **`strs_tools`:** An external Rust crate used for string manipulation, particularly splitting.
*   **`SplitOptionsFormer`:** A builder struct within `strs_tools` used to configure string splitting options.
*   **`SplitOptions`:** The final configuration struct produced by `SplitOptionsFormer`'s `perform()` method, used to create a split iterator.
*   **`E0716` (Temporary value dropped while borrowed):** A Rust compiler error indicating that a temporary value (e.g., a `Vec<&str>`) is being dropped before a reference to its contents is no longer in use.
*   **`E0507` (Cannot move out of `*former`):** A Rust compiler error indicating an attempt to move a value out of a mutable reference when the type does not implement `Copy`. This suggests the builder methods return `&mut Self` rather than `Self`.
*   **`OpType`:** An internal type within `strs_tools` used to abstract over different delimiter types (single string, vector of strings, etc.).

### Progress
*   **Roadmap Milestone:** N/A (This is an investigative task to unblock a feature task)
*   **Primary Editable Crate:** `module/move/unilang_instruction_parser` (This task is to resolve a dependency issue for this crate)
*   **Overall Progress:** 0/1 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Investigate `strs_tools` API and propose solution

### Permissions & Boundaries
*   **Mode:** architect
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   `module/core/strs_tools` (Reason: To read source code and documentation for investigation)

### Relevant Context
*   Control Files to Reference (if they exist):
    *   `module/move/unilang_instruction_parser/task_plan.md` (The blocked task)
*   Files to Include (for AI's reference, if `read_file` is planned):
    *   `module/move/unilang_instruction_parser/src/config.rs`
    *   `module/move/unilang_instruction_parser/src/parser_engine.rs`
    *   `module/core/strs_tools/src/string/split.rs` (Primary file for `SplitOptionsFormer` and `SplitOptions`)
    *   `module/core/strs_tools/src/string/split/options.rs` (Possible location for `SplitOptions` if re-exported)
    *   `module/core/strs_tools/src/string/split/op_type.rs` (For `OpType` definition)
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `strs_tools`

### Expected Behavior Rules / Specifications
*   The solution must allow `unilang_instruction_parser` to dynamically configure delimiters for `strs_tools` without compilation errors related to lifetimes or ownership.
*   The solution should be idiomatic Rust and align with the intended usage of the `strs_tools` API.
*   The solution should not introduce unnecessary allocations or performance overhead.

### Crate Conformance Check Procedure
*   N/A (This is an investigation task, not a code implementation task for `unilang_instruction_parser`. Verification will be manual review of findings and proposed solution.)

### Increments
##### Increment 1: Investigate `strs_tools` API and propose solution
*   **Goal:** Understand the `strs_tools::string::split::SplitOptionsFormer` API's requirements for delimiters and propose a concrete, working solution for `unilang_instruction_parser`.
*   **Specification Reference:** N/A
*   **Steps:**
    *   Step 1: Read `module/core/strs_tools/src/string/split.rs` and `module/core/strs_tools/src/string/split/op_type.rs` to understand the definitions of `SplitOptionsFormer`, `SplitOptions`, and `OpType`, paying close attention to their constructors, methods, and generic parameters, especially those related to lifetimes and `Into<OpType<T>>` bounds.
    *   Step 2: Analyze the `new` method of `SplitOptionsFormer` and any methods for setting delimiters (e.g., `delimeter`, `delimiters`) to determine the expected type and ownership of the delimiter arguments.
    *   Step 3: Formulate a hypothesis about the correct way to pass dynamic `Vec<String>` delimiters to `SplitOptionsFormer` without triggering `E0716` or `E0507`. Consider options like `Cow<'a, str>`, `Arc<String>`, or if `strs_tools` has a method that takes `Vec<String>` directly.
    *   Step 4: Propose a concrete code snippet for `module/move/unilang_instruction_parser/src/config.rs` and `module/move/unilang_instruction_parser/src/parser_engine.rs` that implements the identified solution.
    *   Step 5: Document the findings and the proposed solution clearly, explaining the `strs_tools` API behavior and why the proposed solution works.
    *   Step 6: Perform Increment Verification.
*   **Increment Verification:**
    *   Step 1: Review the proposed solution and documentation for clarity, correctness, and adherence to the goal.
    *   Step 2: Ensure the proposed code snippets are syntactically correct and address the identified compilation errors.
*   **Commit Message:** "feat(unilang_instruction_parser): Propose solution for strs_tools API lifetime issue"

### Task Requirements
*   The solution must directly address the `E0716` and `E0507` errors encountered when using `strs_tools::string::split::SplitOptionsFormer` with dynamic delimiters.
*   The proposed solution must be implementable within the `unilang_instruction_parser` crate without requiring changes to `strs_tools` itself (unless a formal change proposal for `strs_tools` is deemed absolutely necessary and approved).

### Project Requirements
*   All code must strictly adhere to the `codestyle` rulebook provided by the user at the start of the task.
*   Must use Rust 2021 edition.
*   All new APIs must be async.

### Assumptions
*   `strs_tools` is a stable and actively maintained library.
*   There is an idiomatic way to use `SplitOptionsFormer` with dynamic delimiters that does not involve the observed lifetime errors.

### Out of Scope
*   Implementing the proposed solution in `unilang_instruction_parser` (this task is only for investigation and proposal).
*   Full refactoring of `strs_tools` (unless a minimal, targeted change proposal is explicitly approved).

### External System Dependencies (Optional)
*   None

### Notes & Insights
*   The `strs_tools` API for `SplitOptionsFormer` seems to have changed, leading to confusion regarding its builder pattern and delimiter handling.

### Changelog
*   [User Feedback | 2025-07-06 06:16 UTC] Denied `new_task` operation, requested creation of a task file first.