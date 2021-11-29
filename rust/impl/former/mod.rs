// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

// pub extern crate former_derive;
// pub extern crate former_runtime;

// #[macro_use]
// pub extern crate maplit;

// pub mod runtime;
// pub mod derive;
// pub mod former;

// pub extern crate former_runtime;
// pub extern crate former_derive;

pub use former_runtime as runtime;
pub use former_derive as derive;
pub use derive::Former as Former;
