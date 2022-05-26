
#[ allow( unused_imports ) ]
use super::TheModule;

include!( "./_conditional/wtools.rs" );

mod dt;
mod error;
mod former;
mod derive;
mod meta;
mod options;
mod iter;
mod string;
mod time;
mod typing;
mod diagnostics;

// #[ cfg( feature = "proc_macro" ) ]
// mod proc_macro;
// mod vector;
