
#[ cfg( feature = "options" ) ]
use super::TheModule::options as TheModule;
#[ cfg( all( feature = "options", feature = "former" ) ) ]
use super::TheModule::former::Former;

#[ cfg( all( feature = "options", feature = "former" ) ) ]
mod runtime_test;
#[ cfg( feature = "options" ) ]
mod front_test;
