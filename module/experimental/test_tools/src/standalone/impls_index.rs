// NOTE: impls_index_meta intentionally removed to break circular dependency
// See Cargo.toml:85,94,155-166 - standalone_impls_index feature is empty by design
// Circular dependency chain: macro_tools → clone_dyn_types → test_tools → impls_index_meta → macro_tools

// Import placeholder macros at module level
#[allow(unused_imports)]
pub use crate::{fn_name, fn_rename, fns};

// Always provide these modules even if impls_index_meta is not available
/// Implementation traits module
#[allow(unused_imports)]
pub mod impls {
  // Placeholder - no impls_index_meta to avoid circular dependency
}


/// Test implementations module
#[allow(unused_imports)]
pub mod tests_impls {
  // Placeholder - no impls_index_meta to avoid circular dependency
}

/// Optional test implementations module
#[allow(unused_imports)]
pub mod tests_impls_optional {
  // Placeholder - no impls_index_meta to avoid circular dependency
}

/// Test index module
#[allow(unused_imports)]
pub mod tests_index {
  // Placeholder - no impls_index_meta to avoid circular dependency
}

/// Orphan module for compatibility
#[allow(unused_imports)]
pub mod orphan {
  // Placeholder - no impls_index_meta to avoid circular dependency
}

/// Exposed module for compatibility
#[allow(unused_imports)]
pub mod exposed {
  // Placeholder - no impls_index_meta to avoid circular dependency

  // Import placeholder macros at module level
  pub use crate::{fn_name, fn_rename, fns, index};
}
