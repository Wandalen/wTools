
# Task Plan: Fix `FormerBegin` Trait Lifetime

### Goal
*   To resolve the `E0726: implicit elided lifetime not allowed here` compilation error by adding a lifetime parameter to the `FormerBegin` trait in `former_types`. This change is critical to unblock the compilation of dependent crates (like `wca`) that use `#[derive(Former)]` on structs with explicit lifetimes.

### Ubiquitous Language (Vocabulary)
*   **MRE:** Minimum Reproducible Example. A small, self-contained test case that demonstrates a bug.
*   **Lifetime Elision:** Rust's feature of inferring lifetimes in function signatures, which has rules that can be violated, leading to errors like E0726.

### Progress
*   **Roadmap Milestone:** N/A
*   **Primary Editable Crate:** `module/core/former_types`
*   **Overall Progress:** 0/5 increments complete
*   **Increment Status:**
    *   ⚫ Increment 1: Create MRE Test for Lifetime Error
    *   ⚫ Increment 2: Add Lifetime Parameter to `FormerBegin` Trait and Function
    *   ⚫ Increment 3: Update `CollectionFormer` Implementation of `FormerBegin`
    *   ⚫ Increment 4: Verify the Fix with MRE and Regression Tests
    *   ⚫ Increment 5: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:**
    *   None

### Relevant Context
*   **Files to Modify:**
    *   `module/core/former_types/src/forming.rs` (Primary target for the fix)
    *   `module/core/former_types/src/collection.rs` (Will require updates due to the trait change)
    *   `module/core/former_types/tests/inc/mod.rs` (To add the new test module)
*   **File to Create:**
    *   `module/core/former_types/tests/inc/lifetime_mre_test.rs`
*   **Driving Change Proposal:** `module/core/former_types/task/task.md`

### Relevant Rules & Principles
*   **Strict TDD:** All code changes must be driven by a failing test. We will first create a test that fails to compile (the MRE), then write the code to make it compile and pass.
*   **Preserve MRE Tests:** The MRE test created in Increment 1 must be marked with `// test_kind: bug_reproducer(...)` and preserved to prevent future regressions.
*   **Codestyle for Traits/Impls:** All trait and `impl` definitions must follow the project's codestyle, with `where` clauses on a new line and each bound on its own line for readability.

### Expected Behavior Rules / Specifications
*   The `FormerBegin` trait must be generic over a lifetime parameter (`'a`).
*   The change must resolve the `E0726` error when `#[derive(Former)]` is used on a struct with a lifetime.
*   Existing tests in `former_types` must continue to pass, ensuring no regressions are introduced.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| `lifetime_mre_test::reproduces_error_and_passes_after_fix` | Failing (New) | Expected to fail compilation initially, then pass after the fix. |

### Crate Conformance Check Procedure
*   **Step 1: Run Build.** Execute `timeout 300 cargo build -p former_types`. If this fails, fix all compilation errors before proceeding.
*   **Step 2: Run Tests (Conditional).** Only if Step 1 passes, execute `timeout 300 cargo test -p former_types`.
*   **Step 3: Run Linter (Conditional).** Only if Step 2 passes, execute `timeout 300 cargo clippy -p former_types -- -D warnings`.

### Increments
##### Increment 1: Create MRE Test for Lifetime Error
*   **Goal:** Create a new test case that reliably reproduces the `E0726` lifetime compilation error. This test will initially fail to compile, which is the expected outcome and serves as the verification for the subsequent fix.
*   **Specification Reference:** `task.md` - "Problem Statement / Justification"
*   **Steps:**
    1.  Create a new file: `module/core/former_types/tests/inc/lifetime_mre_test.rs`.
    2.  In `module/core/former_types/tests/inc/mod.rs`, add `mod lifetime_mre_test;`.
    3.  In the new test file, add the following MRE code. This code manually simulates what the `former` derive macro would do for a struct with a lifetime, exposing the flaw in the `FormerBegin` trait.
        ```rust
        // test_kind: bug_reproducer(E0726)
        use super::*;

        // A simple struct with a lifetime.
        #[derive(Debug, PartialEq)]
        pub struct Sample<'a> { field: &'a str }

        // Manually define the Storage, Definition, and Former for the struct.
        pub struct SampleFormerStorage<'a> { pub field: Option<&'a str> }
        impl<'a> Default for SampleFormerStorage<'a> { fn default() -> Self { Self { field: None } } }
        impl<'a> Storage for SampleFormerStorage<'a> { type Preformed = Sample<'a>; }
        impl<'a> StoragePreform for SampleFormerStorage<'a> {
            fn preform(mut self) -> Self::Preformed { Sample { field: self.field.take().unwrap_or("") } }
        }

        pub struct SampleFormerDefinitionTypes< 'a, C = (), F = Sample< 'a > >
        { _p: core::marker::PhantomData<(&'a(), C, F)> }
        impl< 'a, C, F > FormerDefinitionTypes for SampleFormerDefinitionTypes< 'a, C, F >
        {
            type Storage = SampleFormerStorage<'a>;
            type Context = C;
            type Formed = F;
        }
        impl< 'a, C, F > FormerMutator for SampleFormerDefinitionTypes< 'a, C, F > {}

        pub struct SampleFormerDefinition< 'a, C = (), F = Sample< 'a >, E = ReturnPreformed >
        { _p: core::marker::PhantomData<(&'a(), C, F, E)> }
        impl< 'a, C, F, E > FormerDefinition for SampleFormerDefinition< 'a, C, F, E >
        where E: FormingEnd<SampleFormerDefinitionTypes<'a, C, F>>
        {
            type Storage = SampleFormerStorage<'a>;
            type Context = C;
            type Formed = F;
            type Types = SampleFormerDefinitionTypes<'a, C, F>;
            type End = E;
        }

        pub struct SampleFormer< 'a, D = SampleFormerDefinition< 'a > >
        where D: FormerDefinition<Storage = SampleFormerStorage<'a>>
        {
            storage: D::Storage,
            context: Option<D::Context>,
            on_end: Option<D::End>,
        }

        // This impl block is what will fail to compile.
        // The `FormerBegin` trait needs a lifetime parameter to handle `Definition`
        // which now carries the lifetime `'a`.
        impl< 'a, D > FormerBegin<D> for SampleFormer< 'a, D >
        where
          D: FormerDefinition<Storage = SampleFormerStorage<'a>>,
        {
          fn former_begin( storage: Option<D::Storage>, context: Option<D::Context>, on_end: D::End ) -> Self
          {
            Self { storage: storage.unwrap_or_default(), context, on_end: Some(on_end) }
          }
        }

        #[test]
        fn reproduces_error_and_passes_after_fix()
        {
            // This test will not be reached until the compilation error is fixed.
            // After the fix, it will serve as a regression test.
            // We will add assertions in Increment 4.
        }
        ```
    4.  Execute `cargo test -p former_types --test tests`.
    5.  **Critically analyze the output.** Confirm that the command fails with a compilation error containing `E0726` or a similar lifetime-related message pointing to the `impl FormerBegin` block. This failure is the success condition for this increment.
*   **Increment Verification:**
    *   The `cargo test` command fails with the expected lifetime compilation error.
*   **Commit Message:** "test(former_types): Add MRE for lifetime elision error in FormerBegin"

##### Increment 2: Add Lifetime Parameter to `FormerBegin` Trait and Function
*   **Goal:** Modify the `FormerBegin` trait and its `former_begin` function in `forming.rs` to be generic over a lifetime. This is the core of the required fix.
*   **Specification Reference:** `task.md` - "Proposed Solution / Specific Changes"
*   **Steps:**
    1.  Read the file `module/core/former_types/src/forming.rs`.
    2.  Use `search_and_replace` to change the trait definition and its function signature.
        *   **Search for:**
            ```rust
            pub trait FormerBegin<Definition>
            where
              Definition: crate::FormerDefinition,
            {
              /// Launches the subforming process with an initial storage and context, setting up an `on_end` completion handler.
              ///
              /// This method initializes the formation process by providing the foundational elements necessary for
              /// building the entity. It allows for the configuration of initial states and contextual parameters, which
              /// are critical for accurately reflecting the intended final state of the entity.
              ///
              /// # Parameters
              ///
              /// * `storage` - An optional initial state for the intermediary storage structure. This parameter allows
              ///   for the pre-configuration of storage, which can be crucial for entities requiring specific initial states.
              /// * `context` - An optional initial setting providing contextual information for the subforming process.
              ///   This context can influence how the formation process progresses, especially in complex forming scenarios.
              /// * `on_end` - A completion handler responsible for transforming the accumulated `Storage` into the final `Formed` structure.
              ///   This parameter is vital for ensuring that the transition from `Storage` to `Formed` is handled correctly,
              ///   incorporating any last-minute adjustments or validations necessary for the entity's integrity.
              ///
              /// # Returns
              ///
              /// Returns an instance of Former.
              ///
              fn former_begin(
                storage: core::option::Option<Definition::Storage>,
                context: core::option::Option<Definition::Context>,
                on_end: Definition::End,
              ) -> Self;
            }
            ```
        *   **Replace with:**
            ```rust
            pub trait FormerBegin< 'a, Definition >
            where
              Definition: crate::FormerDefinition,
              Definition::Storage : 'a,
              Definition::Context : 'a,
              Definition::End : 'a,
            {
              /// Launches the subforming process with an initial storage and context, setting up an `on_end` completion handler.
              ///
              /// This method initializes the formation process by providing the foundational elements necessary for
              /// building the entity. It allows for the configuration of initial states and contextual parameters, which
              /// are critical for accurately reflecting the intended final state of the entity.
              ///
              /// # Parameters
              ///
              /// * `storage` - An optional initial state for the intermediary storage structure. This parameter allows
              ///   for the pre-configuration of storage, which can be crucial for entities requiring specific initial states.
              /// * `context` - An optional initial setting providing contextual information for the subforming process.
              ///   This context can influence how the formation process progresses, especially in complex forming scenarios.
              /// * `on_end` - A completion handler responsible for transforming the accumulated `Storage` into the final `Formed` structure.
              ///   This parameter is vital for ensuring that the transition from `Storage` to `Formed` is handled correctly,
              ///   incorporating any last-minute adjustments or validations necessary for the entity's integrity.
              ///
              /// # Returns
              ///
              /// Returns an instance of Former.
              ///
              fn former_begin
              (
                storage: core::option::Option< Definition::Storage >,
                context: core::option::Option< Definition::Context >,
                on_end: Definition::End,
              ) -> Self;
            }
            ```        *   **Rationale for change:** The lifetime `'a` is added to the trait. The `where` clause now correctly bounds the associated types `Storage`, `Context`, and `End` to the lifetime `'a`. This ensures that any of these types that contain references (like our MRE's `Storage` and `End` types implicitly do) are correctly handled by the compiler.
    3.  Execute `cargo build -p former_types`. Expect compilation errors in `collection.rs` and `lifetime_mre_test.rs`, which will be fixed in the next increments.
*   **Increment Verification:**
    *   The trait definition and function signature in `forming.rs` are updated.
*   **Commit Message:** "fix(former_types): Add lifetime parameter to FormerBegin trait and function"

##### Increment 3: Update `CollectionFormer` Implementation of `FormerBegin`
*   **Goal:** Update the `impl FormerBegin` block for `CollectionFormer` to align with the new lifetime parameter on the trait.
*   **Specification Reference:** `task.md` - "Potential Impact & Considerations"
*   **Steps:**
    1.  Read the file `module/core/former_types/src/collection.rs`.
    2.  Use `search_and_replace` to change the implementation signature.
        *   **Search for:**
            ```rust
            impl<E, Definition> FormerBegin<Definition> for CollectionFormer<E, Definition>
            where
              Definition: FormerDefinition,
              Definition::Storage: CollectionAdd<Entry = E>,
            {
              #[inline(always)]
              fn former_begin(
                storage: core::option::Option<Definition::Storage>,
                context: core::option::Option<Definition::Context>,
                on_end: Definition::End,
              ) -> Self {
                Self::begin(storage, context, on_end)
              }
            }
            ```
        *   **Replace with:**
            ```rust
            impl< 'a, E, Definition > FormerBegin< 'a, Definition > for CollectionFormer< E, Definition >
            where
              Definition: FormerDefinition,
              Definition::Storage: CollectionAdd<Entry = E> + 'a,
              Definition::Context: 'a,
              Definition::End : 'a,
            {
              #[inline(always)]
              fn former_begin
              (
                storage: core::option::Option< Definition::Storage >,
                context: core::option::Option< Definition::Context >,
                on_end: Definition::End,
              ) -> Self
              {
                Self::begin( storage, context, on_end )
              }
            }
            ```
        *   **Rationale for change:** The `impl` now correctly matches the new trait definition, including the lifetime `'a` and the necessary bounds on the `Definition`'s associated types.
    3.  Execute `cargo build -p former_types`. The error in `collection.rs` should be resolved. The MRE test will still fail to compile.
*   **Increment Verification:**
    *   The `impl` block in `collection.rs` is updated and compiles.
*   **Commit Message:** "refactor(former_types): Update CollectionFormer to use lifetime in FormerBegin"

##### Increment 4: Verify the Fix with MRE and Regression Tests
*   **Goal:** Update the MRE test to use the corrected trait and confirm that it now compiles and passes a meaningful assertion. Then, run all tests to ensure no regressions were introduced.
*   **Specification Reference:** `task.md` - "Acceptance Criteria"
*   **Steps:**
    1.  Read the file `module/core/former_types/tests/inc/lifetime_mre_test.rs`.
    2.  Use `search_and_replace` to update the failing `impl` block to use the new trait signature.
        *   **Search for:**
            ```rust
            impl< 'a, D > FormerBegin<D> for SampleFormer< 'a, D >
            where
              D: FormerDefinition<Storage = SampleFormerStorage<'a>>,
            {
              fn former_begin( storage: Option<D::Storage>, context: Option<D::Context>, on_end: D::End ) -> Self
              {
                Self { storage: storage.unwrap_or_default(), context, on_end: Some(on_end) }
              }
            }
            ```
        *   **Replace with:**
            ```rust
            impl< 'a, D > FormerBegin< 'a, D > for SampleFormer< 'a, D >
            where
              D: FormerDefinition<Storage = SampleFormerStorage<'a>>,
              D::Storage: 'a,
              D::Context: 'a,
              D::End: 'a,
            {
              fn former_begin( storage: Option<D::Storage>, context: Option<D::Context>, on_end: D::End ) -> Self
              {
                Self { storage: storage.unwrap_or_default(), context, on_end: Some(on_end) }
              }
            }
            ```
    3.  Use `search_and_replace` to update the test function to perform a meaningful check.
        *   **Search for:**
            ```rust
            #[test]
            fn reproduces_error_and_passes_after_fix()
            {
                // This test will not be reached until the compilation error is fixed.
                // After the fix, it will serve as a regression test.
                // We will add assertions in Increment 4.
            }
            ```
        *   **Replace with:**
            ```rust
            // Add a former impl for SampleFormer to add a setter
            impl< 'a, D > SampleFormer< 'a, D >
            where D: FormerDefinition<Storage = SampleFormerStorage<'a>>
            {
                pub fn field(mut self, value: &'a str) -> Self
                {
                    self.storage.field = Some(value);
                    self
                }
                pub fn form(mut self) -> D::Formed
                {
                    let on_end = self.on_end.take().unwrap();
                    on_end.call(self.storage, self.context.take())
                }
            }

            #[test]
            fn reproduces_error_and_passes_after_fix()
            {
                // Now that it compiles, we can create and use the former.
                let former = SampleFormer::begin(None, None, ReturnPreformed);
                let instance = former.field("hello").form();
                assert_eq!(instance, Sample { field: "hello" });
            }
            ```
    4.  Execute `cargo test -p former_types --test tests`.
    5.  **Critically analyze the output.** All tests, including `lifetime_mre_test::reproduces_error_and_passes_after_fix`, should now compile and pass.
    6.  Update the `### Tests` table to mark the MRE test as `Fixed (Monitored)`.
*   **Increment Verification:**
    *   The full test suite for `former_types` passes without any compilation errors or test failures.
*   **Commit Message:** "test(former_types): Verify lifetime fix and ensure no regressions"

##### Increment 5: Finalization
*   **Goal:** Perform a final verification of the crate and prepare for task completion.
*   **Specification Reference:** N/A
*   **Steps:**
    1.  Perform a final Crate Conformance Check on `former_types`.
    2.  Self-critique against all requirements and rules defined in the plan, ensuring the MRE test is correctly marked and all changes are consistent with the project's style.
*   **Increment Verification:**
    *   All crate conformance checks pass.
*   **Commit Message:** "chore(former_types): Finalize FormerBegin lifetime fix"