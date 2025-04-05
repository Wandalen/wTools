use super::*;

mod impls_index_test;
mod mem_test;
mod try_build_test;

/// Error tools.
#[ path = "../../../../core/error_tools/tests/inc/mod.rs" ]
pub mod error_tests;

/// Collection tools.
#[ path = "../../../../core/collection_tools/tests/inc/mod.rs" ]
pub mod collection_tests;

/// impl and index macros.
#[ path = "../../../../core/impls_index/tests/inc/mod.rs" ]
pub mod impls_index_tests;

/// Memory tools.
#[ path = "../../../../core/mem_tools/tests/inc/mod.rs" ]
pub mod mem_tools_tests;

/// Typing tools.
#[ path = "../../../../core/typing_tools/tests/inc/mod.rs" ]
pub mod typing_tools_tests;

/// Diagnostics tools.
#[ path = "../../../../core/diagnostics_tools/tests/inc/mod.rs" ]
pub mod diagnostics_tools_tests;
