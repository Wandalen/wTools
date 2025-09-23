# Implement StaticCommandRegistry

## Description

Implement the `StaticCommandRegistry` struct that provides hybrid command lookup with PHF-based static commands and HashMap-based dynamic commands. This is the core performance component that enables zero-overhead static command resolution while maintaining backward compatibility with runtime command registration.

Links to related tasks: Depends on task 060 (tests), leads to task 062 (integration with existing registry).

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Must implement hybrid lookup that checks static PHF map first, then dynamic HashMap
-   Must provide `from_phf(&'static StaticCommandMap)` constructor
-   Must maintain API compatibility with existing `CommandRegistry` methods
-   Must implement `lookup_static()` method for direct PHF access
-   Static command lookup must achieve O(1) performance with <1ms p99 latency
-   Must use 2-space indentation following codestyle rules
-   All tests from task 060 must pass after implementation
-   Must integrate seamlessly with existing `Pipeline` infrastructure
-   No clippy warnings when running `cargo clippy --all-targets --all-features -- -D warnings`