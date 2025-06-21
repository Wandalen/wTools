// We don't want to run doctest of aggregate

/// Error tools.
#[ path = "../../../core/error_tools/src/error/mod.rs" ]
pub mod error_tools;
pub use error_tools as error;

/// Collection tools.
#[ path = "../../../core/collection_tools/src/collection/mod.rs" ]
pub mod collection_tools;
pub use collection_tools as collection;

/// impl and index macros.
#[ path = "../../../core/impls_index/src/impls_index/mod.rs" ]
pub mod impls_index;

/// Memory tools.
#[ path = "../../../core/mem_tools/src/mem.rs" ]
pub mod mem_tools;
pub use mem_tools as mem;

/// Typing tools.
#[ path = "../../../core/typing_tools/src/typing.rs" ]
pub mod typing_tools;
pub use typing_tools as typing;

/// Dagnostics tools.
#[ path = "../../../core/diagnostics_tools/src/diag/mod.rs" ]
pub mod diagnostics_tools;
pub use diagnostics_tools as diag;
