
#[ allow( unused_imports ) ]
use super::TheModule;

include!( "./_conditional/wtools.rs" );

mod dt;
mod error;
mod derive;
mod meta;
mod iter;
mod string;
mod time;
mod typing;
mod diagnostics;

mod former;
mod options;
// xxx : meta should include former and options

// #[ cfg( feature = "proc_macro" ) ]
// mod proc_macro;
// mod vector;
