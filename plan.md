# Project Plan: Audit, Improve, and Run Clippy Lints for `former` Crate

### Goal
*   Audit, improve, and run `module/core/former/task_clippy_lints.md` to ensure it follows codestyle rules, has concise documentation, and avoids breaking the working crate.

### Progress
*   🚀 Increment 1 Complete
*   🚀 Increment 2 Complete
*   ❌ Increment 3 Blocked (OpenSSL dependency unresolvable, proceeding with audit/improve only)

### Target Crate
*   `module/core/former`

### Relevant Context
*   Files to Include:
    *   `module/core/former/task_clippy_lints.md`
    *   `module/core/former/Cargo.toml`
    *   `module/core/former/src/lib.rs`
    *   `Cargo.toml` (workspace root)
*   Crates for Documentation:
    *   `former`

### Expected Behavior Rules / Specifications (for Target Crate)
*   The `module/core/former/task_clippy_lints.md` file should be well-formatted, concise, and adhere to the codestyle rules.
*   The `module/core/former` crate should compile without warnings when `cargo clippy -p former` is run with the recommended lints. (Note: This part is currently unachievable due to environment issue).
*   No existing knowledge or functionality should be lost or broken.

### Target File Structure (If Applicable, within Target Crate)
*   No changes to file structure are anticipated for `module/core/former`.

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

*   ❌ Increment 3: Verify `former` crate lints and apply necessary `Cargo.toml` changes.
    *   Detailed Plan Step 1: Attempted to run `cargo clippy -p former` but it consistently failed due to missing OpenSSL development libraries, which is an environment issue.
    *   Detailed Plan Step 2: All attempts to temporarily disable the dependency or isolate the `former` crate have failed to resolve the OpenSSL issue.
    *   Pre-Analysis: The "run" aspect of the task (running `cargo clippy -p former`) cannot be completed due to the unresolvable `openssl-sys` environment issue.
    *   Crucial Design Rules: N/A
    *   Relevant Behavior Rules: N/A
    *   Verification Strategy: N/A
    *   Commit Message: `chore(former): Acknowledge unresolvable clippy lint issue`

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
*   **Blocking Issue:** `cargo clippy` failed due to missing OpenSSL development libraries. This issue is unresolvable through code modifications within the project. The "run" part of the task cannot be completed.