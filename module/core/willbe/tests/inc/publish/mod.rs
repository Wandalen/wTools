use super :: *;

/// Tests for data structures (`PublishReason`, `StaleDependency`)
mod data_structures_test;

/// Bug documentation for dependency staleness issue
mod bug_dependency_staleness_test;

/// Integration tests for staleness detection and transitive closure
mod integration_staleness_test;

/// Bug documentation for PathBuf cast panic with malformed properties
mod bug_pathbuf_cast_panic_test;
