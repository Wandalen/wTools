# Project Plan: Audit, Improve, and Run Clippy Lints for `former` Crate

### Goal
*   Audit, improve, and run `module/core/former/task_clippy_lints.md` to ensure it follows codestyle rules, has concise documentation, and avoids breaking the working crate. **Additionally, ensure `cargo test` passes for the `former` crate without any warnings.**

### Progress
*   🚀 Increment 1 Complete
*   🚀 Increment 2 Complete
*   🚀 Increment 3 Complete
*   🚀 Increment 4 Complete
*   🚀 Increment 5 Complete

### Target Crate
*   `module/core/former`

### Relevant Context
*   Files to Include:
    *   `module/core/former/task_clippy_lints.md`
    *   `module/core/former/Cargo.toml`
    *   `module/core/former/src/lib.rs`
    *   `Cargo.toml` (workspace root)
    *   `module/core/former/tests/inc/enum_unit_tests/generic_enum_simple_unit_derive.rs`
    *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_zero_fields_derive.rs`
    *   `module/core/former/tests/inc/enum_unnamed_tests/tuple_zero_fields_manual.rs`
*   Crates for Documentation:
    *   `former`

### Expected Behavior Rules / Specifications (for Target Crate)
*   The `module/core/former/task_clippy_lints.md` file should be well-formatted, concise, and adhere to the codestyle rules.
*   The `module/core/former` crate should compile without warnings when `cargo clippy -p former` is run with the recommended lints.
*   `cargo test -p former` should pass without errors **and without any warnings.**
*   No existing knowledge or functionality should be lost or broken.

### Increments

*   ✅ Increment 1: Read and analyze `module/core/former/task_clippy_lints.md` and `module/core/former/Cargo.toml`.
    *   Detailed Plan Step 1: Read `module/core/former/task_clippy_lints.md`.
    *   Detailed Plan Step 2: Read `module/core/former/Cargo.toml`.
    *   Pre-Analysis: Understand the current content and identify areas for improvement based on codestyle and documentation rules.
    *   Crucial Design Rules: [Code Style: Do Not Reformat Arbitrarily], [Comments and Documentation], [Lints and warnings], [Prefer workspace lints over entry file lints].
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: Analyze the content of the files.
    *   Commit Message: `docs(former): Analyze clippy lints task file and Cargo.toml`

*   ✅ Increment 2: Improve `module/core/former/task_clippy_lints.md` content.
    *   Detailed Plan Step 1: Apply conservative changes to `module/core/former/task_clippy_lints.md` to improve formatting, conciseness, and adherence to codestyle rules.
    *   Pre-Analysis: Based on the analysis from Increment 1, identify specific sections to rephrase, reformat, or add/remove details.
    *   Crucial Design Rules: [Code Style: Do Not Reformat Arbitrarily], [Comments and Documentation].
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: Visually inspect the updated Markdown file.
    *   Commit Message: `docs(former): Improve clippy lints task file content`

*   ✅ Increment 3: Verify `former` crate lints and apply necessary `Cargo.toml` changes.
    *   Detailed Plan Step 1: Run `cargo clippy -p former` to check current lint status for the `former` crate. (Previously blocked by OpenSSL when running `--workspace`, but now runs successfully when targeted at `-p former`).
    *   Detailed Plan Step 2: Based on clippy output and lint rules, propose and apply necessary changes to `module/core/former/Cargo.toml` to ensure lints are correctly configured and inherited from the workspace, and that the crate compiles without warnings. (No changes needed as `former` is clean).
    *   Pre-Analysis: The `former` crate now passes `cargo clippy -p former` without warnings.
    *   Crucial Design Rules: [Lints and warnings], [Prefer workspace lints over entry file lints].
    *   Relevant Behavior Rules: The `former` crate should pass `cargo clippy` without warnings.
    *   Verification Strategy: Execute `cargo clippy -p former` via `execute_command` and analyze output.
    *   Commit Message: `fix(former): Configure clippy lints for former crate`

*   ✅ Increment 4: Address failing `cargo test` for `former` crate.
    *   Detailed Plan Step 1: Run `cargo test -p former` to identify test failures.
    *   Detailed Plan Step 2: Analyze test output and identify root cause of failures.
    *   Detailed Plan Step 3: Apply conservative fixes to resolve test failures, ensuring no new lints or regressions are introduced.
    *   Pre-Analysis: The `former` crate now passes its tests.
    *   Crucial Design Rules: [Testing: Avoid Writing Automated Tests Unless Asked], [Testing: Standard Directory for All Tests], [Testing: Use Integration Tests only if Asked], [Testing: Plan with a Test Matrix When Writing Tests].
    *   Relevant Behavior Rules: `cargo test -p former` should pass.
    *   Verification Strategy: Execute `cargo test -p former` via `execute_command` and analyze output.
    *   Commit Message: `fix(former): Resolve failing tests`

*   ✅ Increment 5: Address `cargo test` warnings for `former` crate.
    *   Detailed Plan Step 1: Read `module/core/former/tests/inc/enum_unit_tests/generic_enum_simple_unit_derive.rs` to address `EnumOuter` warning.
    *   Detailed Plan Step 2: Read `module/core/former/tests/inc/enum_unnamed_tests/tuple_zero_fields_derive.rs` to address `InnerForSubform` warning.
    *   Detailed Plan Step 3: Read `module/core/former/tests/inc/enum_unnamed_tests/tuple_zero_fields_manual.rs` to address `InnerForSubform` warning.
    *   Detailed Plan Step 4: Apply conservative changes (e.g., `#[allow(dead_code)]` or using the items if appropriate) to resolve the warnings.
    *   Pre-Analysis: The `former` crate now passes its tests without warnings.
    *   Crucial Design Rules: [Comments and Documentation], [Enhancements: Only Implement What’s Requested].
    *   Relevant Behavior Rules: `cargo test -p former` should pass without warnings.
    *   Verification Strategy: Execute `cargo test -p former` via `execute_command` and analyze output for warnings.
    *   Commit Message: `fix(former): Resolve cargo test warnings`

### Task Requirements
*   Do only conservative changes.
*   Avoid breaking working crate.
*   Avoid deleting, losing knowledge from repo.
*   Make sure code edited follows codestyle rules and has concise documentation.
*   Never run `cargo clippy` for the entire workspace.

### Project Requirements
*   (To be populated from existing `plan.md` or `Cargo.toml` if found)

### Notes & Insights
*   The task is primarily about a Markdown file, but also implies ensuring the associated Rust crate (`former`) adheres to clippy lints.
*   I will prioritize using `apply_diff` for small changes to the Markdown file and `Cargo.toml`.
*   **Resolved Issue:** The `openssl-sys` blocking issue was only present when running `cargo clippy --workspace`. When targeted specifically at the `former` crate (`cargo clippy -p former`), it compiles and passes without OpenSSL errors.