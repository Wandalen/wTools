# Unilang Crate/Framework Implementation Roadmap

### Current Status (as of 2025-07-31)
The project has successfully completed its foundational phases (1-3), culminating in a critical architectural refactoring that unified the parsing pipeline and data models. The framework is now stable and robust. The next phase will focus on implementing the mandatory performance requirement for a zero-overhead static command registry, which is the cornerstone for building large-scale, high-performance utilities.

**Legend:**
*   ⚫ : Not Started
*   ⏳ : In Progress
*   ✅ : Done
*   🏁 : Phase Complete / Major Milestone

---

### Phase 1: Core `unilang` Language Engine & CLI Foundations 🏁
*   **Goal:** Establish the `unilang` parsing pipeline, core data structures, command registration, basic type handling, execution flow, initial help capabilities, and error reporting to enable a functional CLI.
*   **Outcome:** A working, foundational `unilang` crate capable of handling basic CLI commands from parsing to execution.
*   **Status:** All milestones are complete.

### Phase 2: Enhanced Type System, Runtime Commands & CLI Maturity 🏁
*   **Goal:** Expand the `unilang` crate's type system, provide APIs for runtime command management, and mature CLI support.
*   **Outcome:** A feature-rich framework capable of handling complex data types, dynamic command loading, and advanced CLI interactions.
*   **Status:** All milestones are complete.

### Phase 3: Architectural Unification & Enhancement 🏁
*   **Goal:** Correct the project's architecture by removing legacy components, integrating `unilang_parser` as the single source of truth, and fully aligning data models with the specification.
*   **Outcome:** A stable, maintainable codebase with a unified architecture, ready for the implementation of core functional requirements.
*   **Status:** All milestones are complete.

### Phase 4: Zero-Overhead Static Command Registry
*   **Goal:** To implement the mandatory performance NFR for a zero-overhead static command system, enabling utilities with thousands of commands to start instantly.
*   **Outcome:** A framework with a hybrid command registry where all compile-time commands are stored in a Perfect Hash Function (PHF), eliminating runtime registration costs and ensuring sub-millisecond command resolution.

*   [⚫] **M4.1: registry_design_hybrid_architecture:**
    *   **Spec Reference:** FR-PERF-1, NFR-Performance
    *   **Deliverable:** A detailed task plan for implementing a zero-overhead static command registry.
    *   **Description:** Design a build-time mechanism (using `build.rs` and the `phf` crate) to generate a Perfect Hash Function (PHF) map from a command manifest. This plan will outline the steps to refactor the `CommandRegistry` into a hybrid model.
*   [⚫] **M4.2: phf_implement_build_time_generation:**
    *   **Prerequisites:** M4.1
    *   **Deliverable:** A `build.rs` script that generates a `.rs` file containing the static PHF map from `unilang.commands.yaml`.
    *   **Description:** Implement the build script that parses the YAML manifest and uses `phf_codegen` to construct the perfect hash map.
*   [⚫] **M4.3: registry_refactor_to_hybrid_model:**
    *   **Prerequisites:** M4.2
    *   **Deliverable:** An updated `CommandRegistry` that uses the generated PHF for static commands and a `HashMap` for dynamic commands.
    *   **Description:** Refactor all lookup methods to query the static PHF first before falling back to the dynamic `HashMap`.
*   [⚫] **M4.4: test_implement_performance_stress_harness:**
    *   **Prerequisites:** M4.3
    *   **Spec Reference:** FR-PERF-1
    *   **Deliverable:** A new integration test that generates a large YAML manifest (1000+ commands) and a test binary that proves the performance NFRs are met.
    *   **Description:** The test will generate the manifest, compile a test binary against it, and then execute the binary to measure and assert that startup time is negligible and p99 command resolution latency is under 1ms.

### Phase 5: Core API Enhancements & Modality Support
*   **Goal:** To implement the remaining mandatory functional requirements from Spec v2.2.0, ensuring the framework fully supports REPL, interactive CLI, and WebAssembly (WASM) modalities.
*   **Outcome:** A functionally complete and validated API for building sophisticated, user-friendly command-line applications that can run in native and web environments.

*   [⚫] **M5.1: pipeline_refactor_for_reusability:**
    *   **Spec Reference:** FR-REPL-1
    *   **Deliverable:** An audited and confirmed stateless core pipeline and a new example file (`repl_example.rs`).
    *   **Description:** Audit the core pipeline components (`Parser`, `SemanticAnalyzer`, `Interpreter`) to ensure they are stateless and can be reused in a REPL loop.
*   [⚫] **M5.2: argument_implement_interactive_signaling:**
    *   **Spec Reference:** FR-INTERACTIVE-1
    *   **Deliverable:** The `SemanticAnalyzer` correctly returns the `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` error for missing interactive arguments.
    *   **Description:** Modify the `bind_arguments` logic to check for the `interactive: true` attribute on missing mandatory arguments and return the specific error code.
*   [⚫] **M5.3: test_create_interactive_prompting_verification:**
    *   **Prerequisites:** M5.2
    *   **Deliverable:** A new unit test for the `SemanticAnalyzer` and an updated CLI binary demonstrating how to catch the interactive signal.
*   [⚫] **M5.4: example_create_wasm_repl:**
    *   **Prerequisites:** M5.1
    *   **Spec Reference:** NFR-PLATFORM-1
    *   **Deliverable:** A working, browser-based REPL example compiled to WebAssembly.
    *   **Description:** Create a minimal web application that uses the `unilang` WASM package to provide a fully client-side REPL, proving the WASM compatibility NFR.

### Phase 6: Performance Hardening & SIMD Optimization
*   **Goal:** To meet the stringent performance NFRs by systematically eliminating bottlenecks identified in the performance analysis, with a focus on reducing string allocations and leveraging SIMD instructions.
*   **Outcome:** A framework with throughput competitive with minimalist parsers like `pico-args`, achieved through zero-copy techniques, string interning, and SIMD-accelerated operations.

*   [⚫] **M6.1: optimization_implement_string_interning:**
    *   **Spec Reference:** `performance.md` (Task 001)
    *   **Deliverable:** A string interning system integrated into the `SemanticAnalyzer` to cache command names and other common strings.
*   [⚫] **M6.2: token_refactor_to_zero_copy:**
    *   **Prerequisites:** M6.1
    *   **Spec Reference:** `performance.md` (Task 002)
    *   **Deliverable:** The `unilang_parser` crate updated to use `&str` tokens, and the `unilang` crate updated to consume them, eliminating major allocation overhead.
*   [⚫] **M6.3: parser_integrate_simd_json:**
    *   **Prerequisites:** M6.2
    *   **Spec Reference:** `performance.md` (Task 009)
    *   **Deliverable:** The type system's JSON parsing logic updated to use the `simd-json` crate for a 4-25x performance improvement on JSON-heavy workloads.
*   [⚫] **M6.4: benchmark_audit_performance_final:**
    *   **Prerequisites:** M6.3
    *   **Deliverable:** An updated `performance.md` with final benchmark results proving all performance NFRs are met.

### Phase 7: Modularity & Lightweight Core Refactoring
*   **Goal:** To fulfill the modularity NFRs by refactoring the crate to use granular feature flags for all non-essential functionality, creating a minimal core profile that is as lightweight as `pico-args`.
*   **Outcome:** A highly modular framework where users can opt-in to features, ensuring minimal binary size and dependency footprint for simple use cases.

*   [⚫] **M7.1: dependency_audit_features:**
    *   **Spec Reference:** NFR-MODULARITY-1, NFR-MODULARITY-2
    *   **Deliverable:** A dependency graph mapping features to the libraries they introduce.
    *   **Description:** Analyze `Cargo.toml` and the codebase to identify all dependencies that can be made optional.
*   [⚫] **M7.2: feature_gate_implement_granular:**
    *   **Prerequisites:** M7.1
    *   **Deliverable:** An updated `Cargo.toml` and codebase where all non-essential functionality is gated by feature flags (e.g., `declarative_loading`, `chrono_types`).
*   [⚫] **M7.3: profile_create_minimal_core:**
    *   **Prerequisites:** M7.2
    *   **Deliverable:** A working `unilang` crate when compiled with `--no-default-features`.
*   [⚫] **M7.4: footprint_verify_lightweight:**
    *   **Prerequisites:** M7.3
    *   **Deliverable:** Benchmark results comparing the compile time and dependency count of the minimal `unilang` profile against `pico-args`.

### Phase 8: Advanced Features - Web Modality
*   **Goal:** To implement a full Web API modality, building on the now stable, performant, and modular architecture.
*   **Outcome:** A versatile, multi-modal framework that can serve its command registry as a RESTful API.

*   [⚫] **M8.1: modality_design_web_api:**
    *   **Deliverable:** A plan for mapping `unilang` commands to HTTP endpoints.
*   [⚫] **M8.2: generator_implement_openapi:**
    *   **Prerequisites:** M8.1
    *   **Deliverable:** A function that generates an OpenAPI v3+ specification from the `CommandRegistry`.
*   [⚫] **M8.3: mapper_implement_http_to_command:**
    *   **Prerequisites:** M8.1
    *   **Deliverable:** A utility/adapter that converts an incoming HTTP request into a `unilang` command invocation.
*   [⚫] **M8.4: example_create_web_api:**
    *   **Prerequisites:** M8.3
    *   **Deliverable:** An example application that serves a `unilang` registry as a REST API.

### Phase 9: Advanced Features - Developer Experience
*   **Goal:** To significantly improve the developer experience by providing procedural macros that reduce boilerplate code.
*   **Outcome:** A framework that is not only powerful but also ergonomic for developers to use.

*   [⚫] **M9.1: macro_design_procedural:**
    *   **Deliverable:** An API design for the `#[command]` procedural macro in the `unilang_meta` crate.
*   [⚫] **M9.2: macro_implement_command:**
    *   **Prerequisites:** M9.1
    *   **Deliverable:** A working `#[command]` macro that generates `CommandDefinition` structs from Rust functions.

### Phase 10: Release Candidate Preparation
*   **Goal:** Focus on stability, developer experience, and documentation to prepare for a v1.0 release.
*   **Outcome:** A polished, production-ready v1.0.0-rc.1 release of the `unilang` framework.

*   [⚫] **M10.1: guide_write_core_concepts:**
    *   **Deliverable:** A comprehensive guide in the documentation explaining the core architecture and philosophy of `unilang`.
*   [⚫] **M10.2: tutorial_write_modality:**
    *   **Prerequisites:** M8.4
    *   **Deliverable:** Tutorials for building a CLI, REPL, and a Web API with `unilang`.
*   [⚫] **M10.3: api_conduct_final_review:**
    *   **Deliverable:** A final review of the public API, with any necessary breaking changes made before the 1.0 release.
*   [⚫] **M10.4: release_publish_v1_candidate:**
    *   **Prerequisites:** M10.3
    *   **Deliverable:** `unilang` v1.0.0-rc.1 published to crates.io.

### Phase 11: Post-v1.0 Ecosystem & Advanced Features
*   **Goal:** Expand the `unilang` ecosystem with new modalities, improved tooling, and advanced integration capabilities.
*   **Outcome:** A mature and extensible framework that solidifies its position as a universal command-line tool.

*   [⚫] **M11.1: modality_implement_tui_framework:**
    *   **Deliverable:** Utilities and an example for building interactive Textual User Interfaces.
*   [⚫] **M11.2: routine_implement_dynamic_loading:**
    *   **Deliverable:** A robust implementation for `routine_link` that can load routines from dynamic libraries.
*   [⚫] **M11.3: system_design_plugin:**
    *   **Deliverable:** A formal specification for a plugin system, allowing third-party crates to provide `unilang` commands to a host application.
