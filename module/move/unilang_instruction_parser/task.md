# Change Proposal for `unilang_instruction_parser`

### Task ID
*   `TASK-20250524-STRS-TOOLS-COMPAT`

### Requesting Context
*   **Requesting Crate/Project:** `module/core/strs_tools`
*   **Driving Feature/Task:** Compatibility update after `strs_tools` fixed a typo in `SplitType` enum.
*   **Link to Requester's Plan:** `../core/strs_tools/plan.md`
*   **Date Proposed:** 2025-05-24

### Overall Goal of Proposed Change
*   Update `unilang_instruction_parser` to be compatible with the latest `strs_tools` API, specifically the `SplitType` enum.

### Problem Statement / Justification
*   The `strs_tools` crate, a dependency of `unilang_instruction_parser`, recently fixed a typo in its `SplitType` enum, changing `SplitType::Delimeter` to `SplitType::Delimiter`. This change was necessary to resolve clippy warnings and ensure correct behavior within `strs_tools`.
*   As a result, `unilang_instruction_parser` now fails to compile because it still references the old `SplitType::Delimeter` variant, which no longer exists. This blocks `unilang_instruction_parser`'s development and testing.

### Proposed Solution / Specific Changes
*   **File:** `src/parser_engine.rs`
*   **Changes:**
    *   Change all occurrences of `SplitType::Delimeter` to `SplitType::Delimiter`.
    *   Specifically, at line 40: `split_item.typ == SplitType::Delimeter` should become `split_item.typ == SplitType::Delimiter`.
    *   And at line 62: `split_item.typ == SplitType::Delimeter` should become `split_item.typ == SplitType::Delimiter`.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   After these changes, `cargo build -p unilang_instruction_parser` and `cargo test -p unilang_instruction_parser` should compile and run successfully without errors related to `SplitType`.
*   The functionality of `unilang_instruction_parser` should remain unchanged.

### Acceptance Criteria (for this proposed change)
*   `cargo build -p unilang_instruction_parser` exits with code 0.
*   `cargo test -p unilang_instruction_parser` exits with code 0.
*   The `unilang_instruction_parser` crate successfully compiles and passes its tests.

### Potential Impact & Considerations
*   **Breaking Changes:** No breaking changes are anticipated for `unilang_instruction_parser`'s public API, only internal adjustments for compatibility.
*   **Dependencies:** No new dependencies. This is a compatibility fix for an existing dependency.
*   **Performance:** No performance impact expected.
*   **Security:** No security implications.
*   **Testing:** Existing tests for `unilang_instruction_parser` should pass after this change.

### Alternatives Considered (Optional)
*   None. This change is a direct consequence of a necessary fix in a dependency.

### Notes & Open Questions
*   This change is critical for `unilang_instruction_parser` to function correctly with the updated `strs_tools` crate.