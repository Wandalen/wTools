// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

// extern crate proc_macro;
// extern crate proc_macro_error;

// #[macro_use]
// mod wt;
// #[macro_use]
// mod meta_tools;
// mod former;

// #[proc_macro_derive( Former )]
// #[proc_macro_error::proc_macro_error]
// pub fn former( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
// {
//   former::former( input )
// }

mod runtime;
pub mod test_framework;
pub use runtime::*;
