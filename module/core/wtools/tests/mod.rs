
#[ allow( unused_imports ) ]
use super::*;

include!( "./_conditional/wtools.rs" );

#[ cfg( feature = "dt" ) ]
mod dt;
#[ cfg( feature = "error" ) ]
mod error;
#[ cfg( feature = "derive" ) ]
mod derive;
#[ cfg( feature = "meta" ) ]
mod meta;
#[ cfg( feature = "iter" ) ]
mod iter;
#[ cfg( feature = "string" ) ]
mod string;
#[ cfg( feature = "time" ) ]
mod time;
#[ cfg( feature = "typing" ) ]
mod typing;
#[ cfg( feature = "diagnostics" ) ]
mod diagnostics;
#[ cfg( feature = "mem_tools" ) ]
mod mem;

#[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
mod former;
#[ cfg( any( feature = "options", feature = "meta_options" ) ) ]
mod options;

// zzz : meta should include former and options
