
mod integration
{
  include!( "./integration_test.rs" );
}


mod dt;
mod error;

// xxx : remove include
mod former
{
  include!( "./former/front_test.rs" );
}

mod derive;
mod meta;
mod options;
mod string;
mod test;
mod time;

mod typing
{
  // #![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
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
