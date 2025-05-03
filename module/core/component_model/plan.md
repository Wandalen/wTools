# Formatting Plan for `component_model`, `component_model_meta`, and `component_model_types`

This plan outlines the steps to manually format the specified Rust crates according to a defined set of guidelines, as `cargo fmt` is not to be used.

**Goal:** Ensure consistent and readable code formatting across the three crates.

**Files to Format:**

The following `.rs` files will be formatted:

*   `module/core/component_model/examples/component_model_trivial.rs`
*   `module/core/component_model/src/lib.rs`
*   `module/core/component_model/tests/experimental.rs`
*   `module/core/component_model/tests/smoke_test.rs`
*   `module/core/component_model/tests/tests.rs`
*   `module/core/component_model/tests/inc/components_tests/component_assign_manual.rs`
*   `module/core/component_model/tests/inc/components_tests/component_assign_tuple_manual.rs`
*   `module/core/component_model/tests/inc/components_tests/component_assign_tuple.rs`
*   `module/core/component_model/tests/inc/components_tests/component_assign.rs`
*   `module/core/component_model/tests/inc/components_tests/component_from_manual.rs`
*   `module/core/component_model/tests/inc/components_tests/component_from_tuple_manual.rs`
*   `module/core/component_model/tests/inc/components_tests/component_from_tuple.rs`
*   `module/core/component_model/tests/inc/components_tests/component_from.rs`
*   `module/core/component_model/tests/inc/components_tests/components_assign_manual.rs`
*   `module/core/component_model/tests/inc/components_tests/components_assign_tuple_manual.rs`
*   `module/core/component_model/tests/inc/components_tests/components_assign_tuple.rs`
*   `module/core/component_model/tests/inc/components_tests/components_assign.rs`
*   `module/core/component_model/tests/inc/components_tests/composite_manual.rs`
*   `module/core/component_model/tests/inc/components_tests/composite.rs`
*   `module/core/component_model/tests/inc/components_tests/from_components_manual.rs`
*   `module/core/component_model/tests/inc/components_tests/from_components_tuple_manual.rs`
*   `module/core/component_model/tests/inc/components_tests/from_components_tuple.rs`
*   `module/core/component_model/tests/inc/components_tests/from_components.rs`
*   `module/core/component_model/tests/inc/components_tests/compiletime/components_component_from_debug.rs`
*   `module/core/component_model/tests/inc/components_tests/only_test/component_assign_tuple.rs`
*   `module/core/component_model/tests/inc/components_tests/only_test/component_assign.rs`
*   `module/core/component_model/tests/inc/components_tests/only_test/component_from_tuple.rs`
*   `module/core/component_model/tests/inc/components_tests/only_test/component_from.rs`
*   `module/core/component_model/tests/inc/components_tests/only_test/components_assign_tuple.rs`
*   `module/core/component_model/tests/inc/components_tests/only_test/components_assign.rs`
*   `module/core/component_model/tests/inc/components_tests/only_test/composite.rs`
*   `module/core/component_model/tests/inc/components_tests/only_test/from_components_tuple.rs`
*   `module/core/component_model/tests/inc/components_tests/only_test/from_components.rs`
*   `module/core/component_model_meta/src/lib.rs`
*   `module/core/component_model_meta/src/component/component_assign.rs`
*   `module/core/component_model_meta/src/component/component_from.rs`
*   `module/core/component_model_meta/src/component/components_assign.rs`
*   `module/core/component_model_meta/src/component/from_components.rs`
*   `module/core/component_model_meta/tests/smoke_test.rs`
*   `module/core/component_model_types/examples/component_model_types_trivial.rs`
*   `module/core/component_model_types/src/component.rs`
*   `module/core/component_model_types/src/lib.rs`
*   `module/core/component_model_types/tests/smoke_test.rs`
*   `module/core/component_model_types/tests/tests.rs`

**Steps:**

1.  **Define Formatting Guidelines:**
    *   Establish a clear set of formatting rules to be applied. These rules should cover aspects such as:
        *   **Indentation:** Use 4 spaces for indentation.
        *   **Line Length:** Aim for a maximum line length of 100 characters. Break lines logically if they exceed this limit.
        *   **Spacing:**
            *   Single space around operators (`=`, `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`, etc.).
            *   Single space after commas and colons in lists, tuples, function arguments, etc.
            *   No space before commas, colons, or semicolons.
            *   Single space after keywords like `if`, `for`, `while`, `match`, `fn`, `let`, `use`, etc.
            *   Space between function name and parentheses (`fn my_func()`).
            *   No space between macro name and exclamation mark/parentheses (`my_macro!()`).
        *   **Brace Placement:** Use K&R style (opening brace on the same line as the control structure, closing brace on its own line).
        *   **Imports (`use` statements):**
            *   Group imports by category (e.g., standard library, external crates, local modules).
            *   Sort imports alphabetically within each group.
            *   Use glob imports (`*`) sparingly, preferably only in test modules or prelude-like modules.
        *   **Comments:**
            *   Use `//` for line comments and `/* ... */` for block comments.
            *   Ensure comments are properly indented and aligned with the code they refer to.
            *   Add a space after `//` for readability.
        *   **Blank Lines:** Use blank lines to separate logical blocks of code, such as between function definitions, struct definitions, or within functions to separate distinct operations.
        *   **Attributes:** Place attributes on the line immediately preceding the item they apply to.
2.  **Apply Formatting to Each File:**
    *   Go through each `.rs` file listed above.
    *   Manually apply the defined formatting guidelines to the code in each file.
    *   Pay close attention to consistency across all files and crates.
3.  **Review and Verify:**
    *   After applying the formatting, review the changes in each file.
    *   Consider using a diff tool to compare the formatted code with the original to easily spot any unintended changes or inconsistencies.
    *   Occasionally, if a linter is available and permitted, run it to catch any formatting issues that might have been missed.
    *   Ensure that the formatting changes have not altered the code's logic or functionality.

This plan requires careful manual work but ensures that the formatting adheres to the specified guidelines without using `cargo fmt`.