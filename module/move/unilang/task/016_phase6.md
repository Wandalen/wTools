
# Task Plan: Phase 6 - Performance Hardening & SIMD Optimization (Elaborated)

### Goal
*   To execute Phase 6 of the `unilang` roadmap by implementing the stringent performance non-functional requirements. This will be achieved by systematically eliminating bottlenecks identified in `performance.md`, with a focus on reducing string allocations and leveraging SIMD instructions for critical parsing operations.

### Ubiquitous Language (Vocabulary)
*   **SIMD (Single Instruction, Multiple Data):** A class of parallel computers in Flynn's taxonomy. It describes computers with multiple processing elements that perform the same operation on multiple data points simultaneously.
*   **`simd-json`:** A high-performance Rust library for parsing JSON that leverages SIMD instructions.
*   **String Interning:** A method of storing only one copy of each distinct string value, which must be immutable. This reduces memory usage and improves performance on string comparisons.
*   **Performance Baseline:** A set of performance metrics captured before optimizations are applied, used as a benchmark to measure improvement.

### Progress
*   **Roadmap Milestone:** Phase 6: Performance Hardening & SIMD Optimization
*   **Primary Editable Crate:** `module/move/unilang`
*   **Overall Progress:** 0/5 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Establish Performance Baseline and Add Dependencies
    *   ⚫ Increment 2: Implement SIMD JSON Parsing (M6.3)
    *   ⚫ Increment 3: Implement String Interning System (M6.1)
    *   ⚫ Increment 4: Final Benchmark Audit & Documentation Update (M6.4)
    *   ⚫ Increment 5: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:** None

### Relevant Context
*   **Prerequisites:** This plan assumes that upstream performance optimizations in dependency crates (like `unilang_parser` for zero-copy tokens) are either complete or will be handled separately. This plan focuses exclusively on the optimizations that can be implemented within the `unilang` crate itself.
*   Control Files to Reference:
    *   `module/move/unilang/roadmap.md`
    *   `module/move/unilang/spec.md`
    *   `module/move/unilang/performance.md`
*   Files to Include (for AI's reference):
    *   `module/move/unilang/src/types.rs`
    *   `module/move/unilang/src/semantic.rs`
    *   `module/move/unilang/Cargo.toml`
    *   `module/move/unilang/benchmarks/throughput_benchmark.rs`

### Expected Behavior Rules / Specifications
*   **`performance.md` Targets:** The implementation should aim to address the critical bottlenecks identified in `performance.md`, specifically targeting a 5-10x improvement from string interning and a 4-25x improvement for JSON-heavy workloads.
*   **`spec.md` NFRs:** The final result must meet or exceed the performance NFRs outlined in the specification.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `simd_json_test` | Not Started | Will verify the correctness and performance of the SIMD JSON parser integration. |
| `string_interning_test` | Not Started | Will verify the correctness and performance of the string interning system. |

### Crate Conformance Check Procedure
*   **Context:** This procedure is defined in the `design.md` rulebook and is executed after every increment to ensure no regressions.
*   **Procedure:**
    *   Step 1: Execute `timeout 180 cargo test -p unilang --all-targets`. Analyze the output to ensure all tests pass and there are no compiler warnings.
    *   Step 2: If tests pass, execute `timeout 180 cargo clippy -p unilang -- -D warnings -A clippy::too-many-lines`. Analyze the output to ensure there are no linter errors.

### Increments

##### Increment 1: Establish Performance Baseline and Add Dependencies
*   **Goal:** To capture the current performance metrics of the framework before any optimizations are applied and to add the necessary dependencies for subsequent increments.
*   **Steps:**
    1.  **Run Benchmarks:** Execute the `throughput_benchmark` to get the current performance numbers.
        *   `execute_command`: `cargo bench --bench throughput_benchmark --features benchmarks`
    2.  **Log Baseline:** From the benchmark output, find the throughput for 1000 commands for `unilang-simd`. Use `insert_content` to save this baseline to the `### Notes & Insights` section of this plan file.
    3.  **Add Dependencies:** Use `insert_content` to add the `simd-json` dependency and the `lazy_static` utility to `module/move/unilang/Cargo.toml`.
        *   **Context:** `simd-json` is the high-performance parser. `lazy_static` is a standard crate for creating thread-safe global statics, which is the ideal pattern for our string interner.
        *   **Action:** Add the following lines to the `[dependencies]` section.
            ```toml
            simd-json = { version = "0.13", optional = true }
            lazy_static = "1.4.0"
            ```
    4.  **Update `simd` Feature:** Use `search_and_replace` on `module/move/unilang/Cargo.toml` to add `simd-json` to the `simd` feature gate.
        *   **Search For:** `simd = [ "unilang_parser/simd" ]`
        *   **Replace With:** `simd = [ "simd-json", "unilang_parser/simd" ]`
*   **Increment Verification:**
    1.  The benchmark results must be logged in the `### Notes & Insights` section.
    2.  Execute `cargo check -p unilang --features simd`. The command must pass, confirming the new dependencies are resolved.
*   **Commit Message:** "chore(perf): Establish performance baseline and add SIMD dependencies"

##### Increment 2: Implement SIMD JSON Parsing (M6.3)
*   **Goal:** To replace the standard `serde_json` parsing for `Kind::Object` and `Kind::JsonString` with the high-performance `simd-json` library.
*   **Specification Reference:** `roadmap.md` M6.3
*   **Steps:**
    1.  **Plan Test (TDD):**
        *   **Rule Reference:** `Testing: Mandatory for All Code Changes` from `design.md`. All production code changes must be accompanied by automated tests.
        *   **Action 1: Create Test File:** Use `write_to_file` to create `module/move/unilang/tests/inc/phase6/simd_json_test.rs` with the following content.
            ```rust
            //! Tests for SIMD JSON parsing integration.
            use unilang::prelude::*;

            #[test]
            #[cfg(feature = "simd")]
            fn test_simd_json_parsing_valid_object() {
                let json_input = r#"{"key": "value", "number": 123, "nested": {"a": true}}"#;
                let result = unilang::types::parse_value(json_input, &Kind::Object);
                assert!(result.is_ok(), "Parsing valid JSON object should succeed");
                if let Ok(Value::Object(obj)) = result {
                    assert_eq!(obj.get("key").unwrap().as_str().unwrap(), "value");
                    assert_eq!(obj.get("number").unwrap().as_i64().unwrap(), 123);
                    assert_eq!(obj.get("nested").unwrap().get("a").unwrap().as_bool().unwrap(), true);
                } else {
                    panic!("Expected a valid JSON object");
                }
            }

            #[test]
            #[cfg(feature = "simd")]
            fn test_simd_json_parsing_invalid_json() {
                let json_input = r#"{"key": "value", "#; // Invalid JSON
                let result = unilang::types::parse_value(json_input, &Kind::Object);
                assert!(result.is_err(), "Parsing invalid JSON should fail");
            }
            ```
        *   **Action 2: Add Test Target:** Use `insert_content` to add the new test target to `module/move/unilang/Cargo.toml`.
            ```toml

            [[test]]
            name = "simd_json_test"
            path = "tests/inc/phase6/simd_json_test.rs"
            ```
    2.  **Implement SIMD Parser Module:**
        *   **Action 1: Create Module File:** Use `write_to_file` to create `module/move/unilang/src/simd_json_parser.rs`.
            ```rust
            //! SIMD-accelerated JSON parser with a fallback to serde_json.
            use serde_json::Value as SerdeValue;

            /// Parses a string into a `serde_json::Value` using `simd-json` with a `serde_json` fallback.
            pub fn parse_to_serde_value(input: &str) -> Result<SerdeValue, String> {
                let mut bytes = input.as_bytes().to_vec();
                match simd_json::to_owned_value(&mut bytes) {
                    Ok(value) => Ok(simd_to_serde(value)),
                    Err(e) => serde_json::from_str(input).map_err(|se| format!("SIMD-JSON failed ({}), and serde_json also failed ({})", e, se)),
                }
            }

            /// Converts a `simd_json::OwnedValue` to a `serde_json::Value`.
            fn simd_to_serde(simd_value: simd_json::OwnedValue) -> SerdeValue {
                match simd_value {
                    simd_json::OwnedValue::Null => SerdeValue::Null,
                    simd_json::OwnedValue::Bool(b) => SerdeValue::Bool(b),
                    simd_json::OwnedValue::Number(n) => n.into(),
                    simd_json::OwnedValue::String(s) => SerdeValue::String(s),
                    simd_json::OwnedValue::Array(arr) => SerdeValue::Array(arr.into_iter().map(simd_to_serde).collect()),
                    simd_json::OwnedValue::Object(obj) => SerdeValue::Object(obj.into_iter().map(|(k, v)| (k, simd_to_serde(v))).collect()),
                }
            }
            ```
        *   **Action 2: Declare Module:** Use `insert_content` in `module/move/unilang/src/lib.rs` to add `#[cfg(feature = "simd")] pub mod simd_json_parser;`.
    3.  **Refactor `types.rs` to Use SIMD Parser:**
        *   **Action 1: Read `types.rs`:** Use `read_file` to get the current content of `module/move/unilang/src/types.rs`.
        *   **Action 2: Write Updated `types.rs`:** Use `write_to_file` to overwrite `module/move/unilang/src/types.rs` with the complete refactored content, including the conditional compilation logic.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo test -p unilang --test simd_json_test --features simd`. The new test must pass.
    2.  Perform the Crate Conformance Check.
*   **Commit Message:** "feat(perf): Integrate simd-json for high-performance JSON parsing"

##### Increment 3: Implement String Interning System (M6.1)
*   **Goal:** To significantly reduce string allocations during semantic analysis by implementing a string interning system for command names.
*   **Specification Reference:** `roadmap.md` M6.1
*   **Steps:**
    1.  **Plan Test (TDD):**
        *   **Action 1: Create Test File:** Use `write_to_file` to create `module/move/unilang/tests/inc/phase6/string_interning_test.rs`.
            ```rust
            //! Tests for the string interning system.
            use unilang::interner::INTERNER;

            #[test]
            fn test_interning_returns_same_static_ref() {
                let s1 = "a_unique_string_for_testing";
                let s2 = String::from("a_unique_string_for_testing");

                let interned1 = INTERNER.intern(s1);
                let interned2 = INTERNER.intern(&s2);

                // Check that both inputs result in the same static string reference by comparing pointers.
                assert_eq!(interned1.as_ptr(), interned2.as_ptr(), "Interned strings should have the same memory address");
            }
            ```
        *   **Action 2: Add Test Target:** Use `insert_content` to add the new test target to `module/move/unilang/Cargo.toml`.
            ```toml

            [[test]]
            name = "string_interning_test"
            path = "tests/inc/phase6/string_interning_test.rs"
            ```
    2.  **Implement Interner Module:**
        *   **Context:** Using a `lazy_static` global instance is an idiomatic Rust pattern for shared, thread-safe services like an interner. It avoids the need to pass an interner instance through the entire call stack.
        *   **Action 1: Create Module File:** Use `write_to_file` to create `module/move/unilang/src/interner.rs`.
            ```rust
            //! A simple, thread-safe string interning system to reduce allocations.
            use std::collections::HashSet;
            use std::sync::Mutex;
            use lazy_static::lazy_static;

            lazy_static! {
                pub static ref INTERNER: StringInterner = StringInterner::new();
            }

            pub struct StringInterner {
                strings: Mutex<HashSet<&'static str>>,
            }

            impl StringInterner {
                fn new() -> Self {
                    Self { strings: Mutex::new(HashSet::new()) }
                }

                pub fn intern(&self, s: &str) -> &'static str {
                    let mut strings = self.strings.lock().unwrap();
                    if let Some(interned) = strings.get(s) {
                        return interned;
                    }
                    let interned = Box::leak(s.to_string().into_boxed_str());
                    strings.insert(interned);
                    interned
                }
            }
            ```
        *   **Action 2: Declare Module:** Use `insert_content` in `module/move/unilang/src/lib.rs` to add `pub mod interner;`.
    3.  **Integrate into `SemanticAnalyzer`:**
        *   **Action 1: Read `semantic.rs`:** Use `read_file` to get the current content of `module/move/unilang/src/semantic.rs`.
        *   **Action 2: Write Updated `semantic.rs`:** Use `write_to_file` to overwrite the file with the refactored version that uses the global interner.
*   **Increment Verification:**
    1.  Execute `timeout 180 cargo test -p unilang --test string_interning_test`. The new test must pass.
    2.  Perform the Crate Conformance Check.
*   **Commit Message:** "feat(perf): Implement string interning to reduce allocations"

##### Increment 4: Final Benchmark Audit & Documentation Update (M6.4)
*   **Goal:** To run the full benchmark suite again, compare the results against the baseline to quantify the improvements, and update all relevant performance documentation.
*   **Specification Reference:** `roadmap.md` M6.4
*   **Steps:**
    1.  **Run Final Benchmarks:** Execute the `throughput_benchmark` with the `simd` feature enabled.
        *   `execute_command`: `cargo bench --bench throughput_benchmark --features "benchmarks simd"`
    2.  **Analyze Results:** Compare the new throughput for 1000 commands against the baseline captured in Increment 1.
    3.  **Update `performance.md`:** Use `write_to_file` to overwrite `module/move/unilang/performance.md` with an updated analysis reflecting the outcomes of the Phase 6 optimizations.
    4.  **Verify `benchmarks/readme.md` Update:** The benchmark script should automatically update the tables in `benchmarks/readme.md`. Use `read_file` to load its content and verify the tables reflect the new, higher performance numbers.
    5.  **Update `roadmap.md`:** Use `search_and_replace` to mark all Phase 6 milestones as complete (`✅`).
*   **Increment Verification:**
    1.  The benchmark command must complete successfully.
    2.  The `performance.md` and `roadmap.md` files must be updated with the new information.
*   **Commit Message:** "docs(perf): Update performance documentation after Phase 6 optimizations"

##### Increment 5: Finalization
*   **Goal:** To perform a final, holistic review and verification of the entire task's output.
*   **Steps:**
    1.  **Rule Reference:** `Finalization Increment Verification` procedure from `design.md`.
    2.  Perform a final self-critique of all changes against the plan's `Goal`.
    3.  Execute the full Crate Conformance Check procedure one last time.
    4.  Run `git status` to ensure the working directory is clean.
*   **Increment Verification:**
    1.  All steps of the Crate Conformance Check must pass.
*   **Commit Message:** "chore(task): Complete and finalize Phase 6 performance hardening"

### Notes & Insights
*   **Baseline is Crucial:** Capturing a clear performance baseline in Increment 1 is essential to objectively measure the success of the optimizations.
*   **Feature Gating:** All performance optimizations that introduce new dependencies (like `simd-json`) must be gated behind the `simd` feature flag to maintain a lightweight core profile.

### Changelog
*   [Initial] Created a new development plan for Phase 6, synthesizing goals from the roadmap and existing task files.
*   [Revised] Elaborated the plan with full context, code snippets, and a more robust API design for the string interner, ensuring the Executor has a complete and unambiguous guide.
