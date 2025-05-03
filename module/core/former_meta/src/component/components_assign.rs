#[ allow( clippy::wildcard_imports ) ]
use super::*;
use macro_tools::{ attr, diag, Result, format_ident };
use iter_tools::Itertools;

///
/// Generate `ComponentsAssign` trait implementation for the type, providing `components_assign` function
///
/// Output example can be found in in the root of the module
///
