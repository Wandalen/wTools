#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::*;
// xxx: temporarily disabled due to macro resolution issues
// use test_tools::impls_index::tests_impls;
// use test_tools::impls_index::tests_index;

#[cfg(any(feature = "either", feature = "dt_either"))]
mod either_test;

// #[ cfg( any( feature = "type_constructor", feature = "dt_type_constructor" ) ) ]
// #[ path = "../../../../core/type_constructor/tests/inc/mod.rs" ]
// mod type_constructor;

#[cfg(feature = "dt_interval")]
#[path = "../../../../core/interval_adapter/tests/inc/mod.rs"]
mod interval_test;
