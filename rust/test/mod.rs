
mod dt;
mod error;

mod former
{
  include!( "./former/wtools_front_test.rs" );
}

mod meta;
mod options;
mod string;
mod test;
mod time;

mod typing
{
  #[ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
  mod implements_test
  {
    #[ allow( unused_imports ) ]
    use wtools::typing as TheModule;
    include!( "./typing/common/implements_test.rs" );
  }
  mod is_slice_test
  {
    #[ allow( unused_imports ) ]
    use wtools::typing as TheModule;
    include!( "./typing/common/is_slice_test.rs" );
  }
  mod inspect_type_test
  {
    #[ allow( unused_imports ) ]
    use wtools::typing as TheModule;
    include!( "./typing/common/inspect_type_test.rs" );
  }
}
// #[ cfg( feature = "proc_macro" ) ]
// mod proc_macro;
// mod vector;
