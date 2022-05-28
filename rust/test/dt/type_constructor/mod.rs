
use super::TheModule;

#[ cfg( any( feature = "type_constructor", feature = "dt_type_constructor" ) ) ]
#[ path = "./inc.rs" ]
mod inc;
