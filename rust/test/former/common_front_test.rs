#[ allow( unused_imports ) ]
use super::*;

#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use test_tools::dependency::*;

#[ path = "./all/alias.rs" ]
mod alias;
#[ path = "./all/basic.rs" ]
mod basic;
#[ path = "./all/conflict.rs" ]
mod conflict;
#[ path = "./all/string_slice_runtime.rs" ]
mod string_slice_runtime;
#[ path = "./all/string_slice.rs" ]
mod string_slice;

#[ path = "./all/default_user_type.rs" ]
mod default_user_type;
#[ path = "./all/user_type_no_default.rs" ]
mod user_type_no_default;
#[ path = "./all/user_type_no_debug.rs" ]
mod user_type_no_debug;
#[ path = "./all/default_primitive.rs" ]
mod default_primitive;
#[ path = "./all/default_primitive.rs" ]
mod unsigned_primitive_types;
#[ path = "./all/unsigned_primitive_types.rs" ]
mod default_container;
#[ path = "./all/perform.rs" ]
mod perform;
