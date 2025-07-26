# Task Plan: Improve `error_tools` Readme and Examples

### Goal
*   Refactor `error_tools` to provide a clear, unified API that wraps `anyhow` and `thiserror`, while maintaining its existing `mod_interface` structure.
*   Create a user-friendly `Readme.md` that explains this unified approach with runnable examples, making the crate easy to adopt.

### Ubiquitous Language (Vocabulary)
*   **`error_tools`:** The crate to be documented and refactored.
*   **`untyped` module:** The facade for `anyhow` for flexible, untyped error handling.
*   **`typed` module:** The facade for `thiserror` for structured, typed error handling.
*   **Unified Interface:** The concept that `error_tools` provides a single, consistent entry point to the functionality of `anyhow` and `thiserror`.

### Progress
*   **Roadmap Milestone:** M2: Improved Documentation and Usability
*   **Primary Editable Crate:** `module/core/error_tools`
*   **Overall Progress:** 2/6 increments complete
*   **Increment Status:**
    *   ✅ Increment 1: Fix Build Issues and Add Core Documentation
    *   ✅ Increment 2: Create `untyped` (anyhow) Usage Example
    *   ⚫ Increment 3: Create `typed` (thiserror) Usage Example
    *   ⚫ Increment 4: Update `Readme.md` with New Content and Examples
    *   ⚫ Increment 5: Clean up `error_tools_trivial.rs` Example
    *   ⚫ Increment 6: Finalization

### Permissions & Boundaries
*   **Mode:** code
*   **Run workspace-wise commands:** false
*   **Add transient comments:** true
*   **Additional Editable Crates:** N/A

### Relevant Context
*   Files to Include:
    *   `module/core/error_tools/src/lib.rs`
    *   `module/core/error_tools/src/error/mod.rs`
    *   `module/core/error_tools/src/error/untyped.rs`
    *   `module/core/error_tools/src/error/typed.rs`
    *   `module/core/error_tools/Readme.md`
    *   `module/core/error_tools/examples/error_tools_trivial.rs`
    *   `module/alias/unilang_instruction_parser/Cargo.toml` (for build fix)
    *   `module/core/test_tools/src/lib.rs` (for build fix)

### Expected Behavior Rules / Specifications
*   Rule 1: The `Readme.md` must clearly explain the unified interface concept for `anyhow` and `thiserror`.
*   Rule 2: The `Readme.md` must show simple, correct `use` statements (e.g., `use error_tools::prelude::*;`) that enable all documented features, including macros.
*   Rule 3: All code examples in the `Readme.md` must correspond to a runnable example file in the `examples/` directory.
*   Rule 4: The crate's public API must maintain its existing `mod_interface` structure, ensuring `private` namespaces and `own`/`orphan`/`exposed` modules are present and correctly configured.

### Tests
| Test ID | Status | Notes |
|---|---|---|
| Build Failure | Fixed (Monitored) | Package collision resolved by correcting path in `unilang_instruction_parser/Cargo.toml`. |
| `test_tools::E0432` | Fixed (Monitored) | Unresolved imports in `test_tools` fixed by removing references to `orphan` and `exposed` modules. |
| `test_tools::E0308` | Fixed (Monitored) | Mismatched error types in `test_tools` resolved by re-adding `error_tools` prelude import. |
| `error_tools::missing_docs` | Fixed (Monitored) | Missing documentation for `ErrWith` trait, its methods, and `ResultWithReport` type alias added. |

### Crate Conformance Check Procedure
*   **Step 1: Run build and tests.** Execute `timeout 90 cargo test -p error_tools`.
*   **Step 2: Run Linter (Conditional).** Only if Step 1 passes, execute `timeout 120 cargo clippy -p error_tools -- -D warnings`.
*   **Step 3: Run Codestyle Check (Conditional).** Only if Step 2 passes, execute `timeout 90 cargo fmt --check`.
*   **Step 4: Check examples (if they exist).** This step will be populated as examples are created.

### Increments
##### Increment 1: Fix Build Issues and Add Core Documentation
*   **Goal:** Resolve the package collision build issue and add missing documentation to core error handling traits and types, ensuring the crate compiles and tests cleanly.
*   **Specification Reference:** N/A (build fix), `error_tools::missing_docs` (documentation)
*   **Steps:**
    *   **Step 1.1: Correct conflicting path in `unilang_instruction_parser/Cargo.toml`.** Use `search_and_replace` to change `unilang_parser = { path = "/home/user1/pro/lib/wTools/module/move/unilang_parser" }` to `unilang_parser = { path = "../../move/unilang_parser" }`.
    *   **Step 1.2: Remove problematic imports from `test_tools/src/lib.rs`.** Use `search_and_replace` to remove references to `error_tools::orphan`, `error_tools::exposed`, and `error_tools::prelude` from `module/core/test_tools/src/lib.rs`.
        *   Replace `error_tools::orphan::*, collection_tools::orphan::*, impls_index::orphan::*, mem_tools::orphan::*, typing_tools::orphan::*, diagnostics_tools::orphan::*,` with `collection_tools::orphan::*, impls_index::orphan::*, mem_tools::orphan::*, typing_tools::orphan::*, diagnostics_tools::orphan::*,`
        *   Replace `error_tools::exposed::*, collection_tools::exposed::*, impls_index::exposed::*, mem_tools::exposed::*, typing_tools::exposed::*, diagnostics_tools::exposed::*,` with `collection_tools::exposed::*, impls_index::exposed::*, mem_tools::exposed::*, typing_tools::exposed::*, diagnostics_tools::exposed::*,`
        *   Replace `error_tools::prelude::*, collection_tools::prelude::*, impls_index::prelude::*, mem_tools::prelude::*, typing_tools::prelude::*, diagnostics_tools::prelude::*,` with `collection_tools::prelude::*, impls_index::prelude::*, mem_tools::prelude::*, typing_tools::prelude::*, diagnostics_tools::prelude::*,`
    *   **Step 1.3: Add documentation to `error/mod.rs`.**
        *   Add `/// Trait to add extra context or information to an error.` above `pub trait ErrWith< ReportErr, ReportOk, E >`.
        *   Add `/// Wraps an error with additional context generated by a closure.` above `fn err_with< F >( self, f : F ) -> core::result::Result< ReportOk, ( ReportErr, E ) >`.
        *   Add `/// Wraps an error with additional context provided by a reference.` above `fn err_with_report( self, report : &ReportErr ) -> core::result::Result< ReportOk, ( ReportErr, E ) >`.
        *   Add `/// A type alias for a `Result` that contains an error which is a tuple of a report and an original error.` above `pub type ResultWithReport< Report, Error > = Result< Report, ( Report, Error ) >;`.
    *   **Step 1.4: Clean and update Cargo.** Execute `cargo clean && cargo update`.
    *   **Step 1.5: Perform Increment Verification.**
*   **Increment Verification:**
    *   Execute `timeout 90 cargo test -p error_tools`. The command must pass without any errors or warnings.
*   **Commit Message:** `fix(build): Resolve package collision and add core documentation`

##### Increment 2: Create `untyped` (anyhow) Usage Example
*   **Goal:** Create a clear, runnable example demonstrating how to use the `untyped` module as a facade for `anyhow`.
*   **Specification Reference:** Rule 3
*   **Steps:**
    *   **Step 2.1: Create new example file.** Use `write_to_file` to create `module/core/error_tools/examples/replace_anyhow.rs` with the following content:
        ```rust
        //! A runnable example demonstrating how to use `error_tools::untyped`
        //! as a replacement for `anyhow`.

        use error_tools::untyped::{ Result, Context, format_err };

        fn read_and_process_file( path : &str ) -> Result< String >
        {
          let content = std::fs::read_to_string( path )
            .context( format_err!( "Failed to read file at '{}'", path ) )?;

          if content.is_empty()
          {
            return Err( format_err!( "File is empty!" ) );
          }

          Ok( content.to_uppercase() )
        }

        fn main()
        {
          // Create a dummy file for the example
          _ = std::fs::write( "temp.txt", "hello world" );

          match read_and_process_file( "temp.txt" )
          {
            Ok( processed ) => println!( "Processed content: {}", processed ),
            Err( e ) => println!( "An error occurred: {:?}", e ),
          }

          match read_and_process_file( "non_existent.txt" )
          {
            Ok( _ ) => (),
            Err( e ) => println!( "Correctly handled error for non-existent file: {:?}", e ),
          }

          // Clean up the dummy file
          _ = std::fs::remove_file( "temp.txt" );
        }
        ```
    *   **Step 2.2: Perform Increment Verification.**
*   **Increment Verification:**
    *   Execute `timeout 90 cargo run --example replace_anyhow`.
*   **Commit Message:** `feat(examples): Add untyped (anyhow) usage example`

##### Increment 3: Create `typed` (thiserror) Usage Example
*   **Goal:** Create a clear, runnable example demonstrating how to use the `typed` module as a facade for `thiserror`.
*   **Specification Reference:** Rule 3
*   **Steps:**
    *   **Step 3.1: Create new example file.** Use `write_to_file` to create `module/core/error_tools/examples/replace_thiserror.rs` with the following content:
        ```rust
        //! A runnable example demonstrating how to use `error_tools::typed`
        //! as a replacement for `thiserror`.

        use error_tools::typed::Error;
        use std::path::PathBuf;

        // Define a custom error type using the derive macro from error_tools.
        #[ derive( Debug, Error ) ]
        pub enum DataError
        {
          #[ error( "I/O error for file: {0}" ) ]
          Io( #[ from ] std::io::Error, PathBuf ),
          #[ error( "Parsing error: {0}" ) ]
          Parse( String ),
        }

        // A function that can return our custom error.
        fn process_data( path : &PathBuf ) -> Result< i32, DataError >
        {
          let content = std::fs::read_to_string( path )
            .map_err( | e | DataError::Io( e, path.clone() ) )?;

          content.trim().parse::< i32 >()
            .map_err( | _ | DataError::Parse( "Could not parse content as integer".into() ) )
        }

        fn main()
        {
          // Create dummy files for the example
          _ = std::fs::write( "data.txt", "123" );
          _ = std::fs::write( "invalid_data.txt", "abc" );

          let path1 = PathBuf::from( "data.txt" );
          match process_data( &path1 )
          {
            Ok( num ) => println!( "Processed data: {}", num ),
            Err( e ) => println!( "An error occurred: {}", e ),
          }

          let path2 = PathBuf::from( "invalid_data.txt" );
          match process_data( &path2 )
          {
            Ok( _ ) => (),
            Err( e ) => println!( "Correctly handled parsing error: {}", e ),
          }

          // Clean up dummy files
          _ = std::fs::remove_file( "data.txt" );
          _ = std::fs::remove_file( "invalid_data.txt" );
        }
        ```
    *   **Step 3.2: Perform Increment Verification.**
*   **Increment Verification:**
    *   Execute `timeout 90 cargo run --example replace_thiserror`.
*   **Commit Message:** `feat(examples): Add typed (thiserror) usage example`

##### Increment 4: Update `Readme.md` with New Content and Examples
*   **Goal:** Rewrite the `Readme.md` to be user-friendly, explaining the unified interface and linking to the new examples.
*   **Specification Reference:** Rule 1, Rule 2
*   **Steps:**
    *   **Step 4.1: Rewrite `Readme.md`.** Use `write_to_file` on `module/core/error_tools/Readme.md` with the following content:
        ```markdown
        <!-- {{# generate.module_header{} #}} -->

        # Module :: `error_tools`
        <!--{ generate.module_header.start() }-->
         [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_error_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_error_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/error_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/error_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Ferror_tools%2Fexamples%2Ferror_tools_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Ferror_tools%2Fexamples%2Ferror_tools_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
        <!--{ generate.module_header.end }-->

        `error_tools` is a foundational library for error handling in Rust, providing a unified interface over the popular `anyhow` and `thiserror` crates. It simplifies error management by offering clear, consistent patterns for both untyped and typed errors, without requiring you to choose between them at the crate level.

        ### Key Features

        -   **Unified Error Handling:** Use `anyhow`'s flexibility and `thiserror`'s structure through a single, consistent API.
        -   **Simple Prelude:** A comprehensive `prelude` makes it easy to import everything you need.
        -   **Contextual Errors:** Easily add context to your errors with the `ErrWith` trait.

        ### How It Works

        `error_tools` acts as a facade, re-exporting the core functionalities of `anyhow` and `thiserror` under its `untyped` and `typed` modules, respectively. This allows you to leverage the power of these crates with simplified imports and a consistent feel across your project.

        ---

        ### Untyped Errors (like `anyhow`)

        For functions where you need flexible, dynamic error handling without defining custom error types for every possible failure, use the `untyped` module. It's a direct pass-through to `anyhow`.

        #### Example

        This example shows a function that reads a file and can fail in multiple ways, all handled by `error_tools::untyped::Result`.

        ```rust
        // In your code:
        use error_tools::untyped::{ Result, Context, format_err };

        fn read_and_process_file( path : &str ) -> Result< String >
        {
          let content = std::fs::read_to_string( path )
            .context( format_err!( "Failed to read file at '{}'", path ) )?;

          if content.is_empty()
          {
            return Err( format_err!( "File is empty!" ) );
          }

          Ok( content.to_uppercase() )
        }
        ```
        > See the full runnable example in [`examples/replace_anyhow.rs`](./examples/replace_anyhow.rs).

        ---

        ### Typed Errors (like `thiserror`)

        For library code or situations where you want to define a clear, structured contract for possible errors, use the `typed` module. It re-exports `thiserror`'s `Error` derive macro.

        #### Example

        Here, we define a custom `DataError` enum. The `#[derive(Error)]` macro comes directly from `error_tools`.

        ```rust
        // In your code:
        use error_tools::typed::Error;
        use std::path::PathBuf;

        // The derive macro is re-exported for convenience.
        #[ derive( Debug, Error ) ]
        pub enum DataError
        {
          #[ error( "I/O error for file: {0}" ) ]
          Io( #[ from ] std::io::Error, PathBuf ),
          #[ error( "Parsing error: {0}" ) ]
          Parse( String ),
        }

        fn process_data( path : &PathBuf ) -> Result< i32, DataError >
        {
          let content = std::fs::read_to_string( path )
            .map_err( | e | DataError::Io( e, path.clone() ) )?;

          content.trim().parse::< i32 >()
            .map_err( | _ | DataError::Parse( "Could not parse content as integer".into() ) )
        }
        ```
        > See the full runnable example in [`examples/replace_thiserror.rs`](./examples/replace_thiserror.rs).

        ---

        ### To add to your project

        ```sh
        cargo add error_tools
        ```

        ### Try out from the repository

        ```sh
        git clone https://github.com/Wandalen/wTools
        cd wTools
        cargo run --example error_tools_trivial
        # Or try the specific examples
        cargo run --example replace_anyhow
        cargo run --example replace_thiserror
        ```
        ```
    *   **Step 4.2: Perform Increment Verification.**
*   **Increment Verification:**
    *   Manually review the `Readme.md` for clarity, correctness, and fulfillment of all requirements.
*   **Commit Message:** `docs(readme): Rewrite to explain unified error handling patterns`

##### Increment 5: Clean up `error_tools_trivial.rs` Example
*   **Goal:** Refactor the existing `error_tools_trivial.rs` to be a simple, clear "hello world" for the crate.
*   **Specification Reference:** N/A
*   **Steps:**
    *   **Step 5.1: Simplify the example.** Use `write_to_file` on `module/core/error_tools/examples/error_tools_trivial.rs` with the following content:
        ```rust
        //! A trivial example for `error_tools`.

        use error_tools::untyped::{ Result, format_err };

        fn get_message() -> Result< &'static str >
        {
          Ok( "Hello, world!" )
          // Err( format_err!( "An unexpected error!" ) )
        }

        fn main()
        {
          match get_message()
          {
            Ok( msg ) => println!( "Success: {}", msg ),
            Err( e ) => println!( "Error: {:?}", e ),
          }
        }
        ```
    *   **Step 5.2: Perform Increment Verification.**
*   **Increment Verification:**
    *   Execute `timeout 90 cargo run --example error_tools_trivial`.
*   **Commit Message:** `refactor(examples): Simplify trivial example`

##### Increment 6: Finalization
*   **Goal:** Perform a final, holistic review and verification of the entire task's output.
*   **Specification Reference:** N/A
*   **Steps:**
    *   **Step 6.1: Self-Critique.** Review all changes against the `Goal` and `Expected Behavior Rules`.
    *   **Step 6.2: Full Conformance Check.** Run the full, updated `Crate Conformance Check Procedure`.
*   **Increment Verification:**
    *   All steps in the `Crate Conformance Check Procedure` must pass.
*   **Commit Message:** `chore(task): Finalize readme and examples improvements`

### Task Requirements
*   The `Readme.md` must be the primary focus and deliverable.
*   All examples must be runnable and reflect the documentation.
*   Code must adhere to existing style.

### Project Requirements
*   (Inherited from workspace `Cargo.toml`)

### Assumptions
*   A simpler, more direct API will be more user-friendly than the current module system.

### Out of Scope
*   `no_std` compatibility.
*   Adding new features beyond what is needed for the examples.

### External System Dependencies
*   N/A

### Notes & Insights
*   This task will significantly improve the crate's approachability for new users by providing clear documentation and a more conventional API.
*   **Root Cause of Build Failure:** The package collision for `clone_dyn_types` was caused by an absolute path reference in `module/alias/unilang_instruction_parser/Cargo.toml` pointing to the old `wTools` directory.
*   **Solution:** Replaced the absolute path with a relative path: `unilang_parser = { path = "../../move/unilang_parser" }`. This resolved the conflict and allowed the build to proceed.

### Changelog
*   [Increment 1 | 2025-07-26 21:27 UTC] Resolved package collision in `unilang_instruction_parser/Cargo.toml`. Removed problematic imports from `test_tools/src/lib.rs`. Added missing documentation to `error/mod.rs`.