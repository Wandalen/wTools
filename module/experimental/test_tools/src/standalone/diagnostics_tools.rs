// Re-export pretty_assertions if available
#[cfg(feature = "diagnostics_runtime_assertions")]
#[allow(unused_imports)]
pub use pretty_assertions::*;

// Placeholder macros for diagnostics tools compatibility
// NOTE: Real implementations in `diagnostics_tools` use `pretty_assertions` for better diff output
// Standalone versions use standard Rust assertions without fancy formatting

/// Placeholder macro for `a_true` - asserts boolean expression is true at runtime
/// NOTE: Real implementation in `diagnostics_tools` wraps `assert`!() with same semantics
#[macro_export]
macro_rules! a_true {
  () => {};
  ( $($Rest:tt)* ) => {
    assert!( $($Rest)* );
  };
}

/// Placeholder macro for `a_false` - asserts boolean expression is false at runtime
/// NOTE: Real implementation in `diagnostics_tools` wraps `assert`!(!...) with same semantics
#[macro_export]
macro_rules! a_false {
  () => {};
  ( $($Rest:tt)* ) => {
    assert!( ! $($Rest)* );
  };
}

/// Placeholder macro for `a_id` - asserts two expressions are identical (equal)
/// NOTE: Real implementation in `diagnostics_tools` uses `pretty_assertions::assert_eq`!() for better diff
/// Standalone version uses `pretty_assertions` when available, standard `assert_eq` otherwise
#[macro_export]
macro_rules! a_id {
  ( $left:expr , $right:expr $(,)? ) => {
    #[ cfg( feature = "diagnostics_runtime_assertions" ) ]
    {
      $crate::diagnostics_tools::assert_eq!( $left, $right );
    }
    #[ cfg( not( feature = "diagnostics_runtime_assertions" ) ) ]
    {
      assert_eq!( $left, $right );
    }
  };
  ( $left:expr, $right:expr, $($arg:tt)* ) => {
    #[ cfg( feature = "diagnostics_runtime_assertions" ) ]
    {
      $crate::diagnostics_tools::assert_eq!( $left, $right, $($arg)* );
    }
    #[ cfg( not( feature = "diagnostics_runtime_assertions" ) ) ]
    {
      assert_eq!( $left, $right, $($arg)* );
    }
  };
}

/// Placeholder macro for `a_not_id` - asserts two expressions are not identical (not equal)
/// NOTE: Real implementation in `diagnostics_tools` uses `pretty_assertions::assert_ne`!() for better diff
/// Standalone version uses `pretty_assertions` when available, standard `assert_ne` otherwise
#[macro_export]
macro_rules! a_not_id {
  ( $left:expr , $right:expr $(,)? ) => {
    #[ cfg( feature = "diagnostics_runtime_assertions" ) ]
    {
      $crate::diagnostics_tools::assert_ne!( $left, $right );
    }
    #[ cfg( not( feature = "diagnostics_runtime_assertions" ) ) ]
    {
      assert_ne!( $left, $right );
    }
  };
  ( $left:expr, $right:expr, $($arg:tt)* ) => {
    #[ cfg( feature = "diagnostics_runtime_assertions" ) ]
    {
      $crate::diagnostics_tools::assert_ne!( $left, $right, $($arg)* );
    }
    #[ cfg( not( feature = "diagnostics_runtime_assertions" ) ) ]
    {
      assert_ne!( $left, $right, $($arg)* );
    }
  };
}

/// Placeholder macro for `a_dbg_true` - asserts boolean expression is true in debug builds
/// NOTE: Real implementation in `diagnostics_tools` wraps `debug_assert`!() with same semantics
#[macro_export]
macro_rules! a_dbg_true {
  () => {};
  ( $($Rest:tt)* ) => {
    debug_assert!( $($Rest)* );
  };
}

/// Placeholder macro for `a_dbg_false` - asserts boolean expression is false in debug builds
/// NOTE: Real implementation in `diagnostics_tools` wraps `debug_assert`!(!...) with same semantics
#[macro_export]
macro_rules! a_dbg_false {
  () => {};
  ( $($Rest:tt)* ) => {
    debug_assert!( ! $($Rest)* );
  };
}

/// Placeholder macro for `a_dbg_id` - asserts two expressions are identical in debug builds
/// NOTE: Real implementation in `diagnostics_tools` calls `a_id`!() only if `debug_assertions` enabled
#[macro_export]
macro_rules! a_dbg_id {
  ( $($arg:tt)* ) => {
    if cfg!( debug_assertions ) {
      $crate::a_id!( $($arg)* );
    }
  };
}

/// Placeholder macro for `a_dbg_not_id` - asserts two expressions are not identical in debug builds
/// NOTE: Real implementation in `diagnostics_tools` calls `a_not_id`!() only if `debug_assertions` enabled
#[macro_export]
macro_rules! a_dbg_not_id {
  ( $($arg:tt)* ) => {
    if cfg!( debug_assertions ) {
      $crate::a_not_id!( $($arg)* );
    }
  };
}

/// Placeholder macro for `cta_true` (compile-time assertion compatibility)
/// NOTE: Real implementation in `diagnostics_tools` does compile-time boolean checking
/// Standalone version returns true without compile-time validation
#[macro_export]
macro_rules! cta_true {
  ( $($tokens:tt)* ) => { true };
}

/// Placeholder macro for `cta_type_same_size` (compile-time assertion compatibility)
/// NOTE: Real implementation in `diagnostics_tools` does compile-time size checking
/// Standalone version returns true without compile-time validation
#[macro_export]
macro_rules! cta_type_same_size {
  ( $($tokens:tt)* ) => { true };
}

/// Placeholder macro for `cta_type_same_align` (compile-time assertion compatibility)
/// NOTE: Real implementation in `diagnostics_tools` does compile-time alignment checking
/// Standalone version returns true without compile-time validation
#[macro_export]
macro_rules! cta_type_same_align {
  ( $($tokens:tt)* ) => { true };
}

/// Placeholder macro for `cta_ptr_same_size` (compile-time assertion compatibility)
/// NOTE: Real implementation in `diagnostics_tools` does compile-time pointer size checking
/// Standalone version returns true without compile-time validation
#[macro_export]
macro_rules! cta_ptr_same_size {
  ( $($tokens:tt)* ) => { true };
}

/// Placeholder macro for `cta_mem_same_size` (compile-time assertion compatibility)
/// NOTE: Real implementation in `diagnostics_tools` does compile-time memory size checking
/// Standalone version returns true without compile-time validation
#[macro_export]
macro_rules! cta_mem_same_size {
  ( $($tokens:tt)* ) => { true };
}

pub use a_true;
pub use a_id;
pub use a_false;
pub use cta_true;
pub use a_not_id;
pub use a_dbg_true;
pub use a_dbg_id;
pub use a_dbg_not_id;
pub use cta_type_same_size;
pub use cta_type_same_align;
pub use cta_ptr_same_size;
pub use cta_mem_same_size;

/// Orphan module for compatibility
#[allow(unused_imports)]
pub mod orphan {
  #[cfg(feature = "diagnostics_runtime_assertions")]
  pub use pretty_assertions::*;

  #[cfg(feature = "standalone_diagnostics_tools")]
  pub use super::{a_true, a_id, a_false, cta_true, a_not_id, a_dbg_true, a_dbg_id, a_dbg_not_id,
                  cta_type_same_size, cta_type_same_align, cta_ptr_same_size, cta_mem_same_size};
}

/// Exposed module for compatibility
#[allow(unused_imports)]
pub mod exposed {
  #[cfg(feature = "diagnostics_runtime_assertions")]
  pub use pretty_assertions::*;

  #[cfg(feature = "standalone_diagnostics_tools")]
  pub use super::{a_true, a_id, a_false, cta_true, a_not_id, a_dbg_true, a_dbg_id, a_dbg_not_id,
                  cta_type_same_size, cta_type_same_align, cta_ptr_same_size, cta_mem_same_size};
}

/// Prelude module for compatibility
#[allow(unused_imports)]
pub mod prelude {
  #[cfg(feature = "diagnostics_runtime_assertions")]
  pub use pretty_assertions::*;

  #[cfg(feature = "standalone_diagnostics_tools")]
  pub use super::{a_true, a_id, a_false, cta_true, a_not_id, a_dbg_true, a_dbg_id, a_dbg_not_id,
                  cta_type_same_size, cta_type_same_align, cta_ptr_same_size, cta_mem_same_size};
}
