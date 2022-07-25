#[ allow( unused_imports ) ]
use super::*;

#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use test_tools::dependency::*;

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
#[ path = "./all/default_primitive.rs" ]
mod default_primitive;
#[ path = "./all/default_container.rs" ]
mod default_container;

#[ path = "./all/user_setter.rs" ]
mod user_setter;

#[ path = "./all/perform.rs" ]
mod perform;
