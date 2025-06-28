# Specification Addendum: Unilang Framework

### Purpose
This document is a companion to the main `specification.md`. It is intended to be completed by the **Developer** during the implementation phase. While the main specification defines the "what" and "why" of the project architecture, this addendum captures the "how" of the final implementation.

### Instructions for the Developer
As you build the system, please fill out the sections below with the relevant details. This creates a crucial record for future maintenance, debugging, and onboarding.

---

### Implementation Notes
*A space for any key decisions, trade-offs, or discoveries made during development that are not captured elsewhere. For example: "Chose `indexmap` over `std::collections::HashMap` for the command registry to preserve insertion order for help generation."*

-   **Decision on Parser Integration:** The legacy `unilang::parsing` module will be completely removed. The `unilang::SemanticAnalyzer` will be refactored to directly consume `Vec<unilang_instruction_parser::GenericInstruction>`. This is a breaking change for the internal API but necessary for architectural consistency.
-   **Data Model Enhancement:** The `CommandDefinition` and `ArgumentDefinition` structs in `unilang/src/data.rs` will be updated to include all fields from spec v1.3 (e.g., `aliases`, `sensitive`, `is_default_arg`). This will require careful updates to the `former` derive macros and associated tests.

### Environment Variables
*List all environment variables required to run the application's tests or examples. Note that the `unilang` framework itself has no runtime environment variables, but an `Integrator`'s `utility1` might.*

| Variable | Description | Example |
| :--- | :--- | :--- |
| `RUST_LOG` | Controls the log level for tests and examples using the `env_logger` crate. | `unilang=debug` |
| `UTILITY1_CONFIG_PATH` | (Example for an Integrator) A path to a configuration file for a `utility1` application. | `/etc/utility1/config.toml` |

### Finalized Library & Tool Versions
*List the critical libraries, frameworks, or tools used and their exact locked versions from `Cargo.lock` upon release.*

-   `rustc`: `1.78.0`
-   `cargo`: `1.78.0`
-   `serde`: `1.0.203`
-   `serde_yaml`: `0.9.34`
-   `serde_json`: `1.0.117`
-   `thiserror`: `1.0.61`
-   `indexmap`: `2.2.6`
-   `chrono`: `0.4.38`
-   `url`: `2.5.0`
-   `regex`: `1.10.4`

### Publication Checklist
*A step-by-step guide for publishing the `unilang` crates to `crates.io`. This replaces a typical deployment checklist.*

1.  Ensure all tests pass for all workspace crates: `cargo test --workspace`.
2.  Ensure all clippy lints pass for all workspace crates: `cargo clippy --workspace -- -D warnings`.
3.  Increment version numbers in `Cargo.toml` for all crates being published, following SemVer.
4.  Update `changelog.md` with details of the new version.
5.  Run `cargo publish -p unilang_instruction_parser --dry-run` to verify.
6.  Run `cargo publish -p unilang_instruction_parser`.
7.  Run `cargo publish -p unilang --dry-run` to verify.
8.  Run `cargo publish -p unilang`.
9.  Run `cargo publish -p unilang_meta --dry-run` to verify.
10. Run `cargo publish -p unilang_meta`.
11. Create a new git tag for the release version (e.g., `v0.2.0`).
12. Push the tag to the remote repository: `git push --tags`.
