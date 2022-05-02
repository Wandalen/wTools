// #![cfg_attr(docsrs, feature(doc_cfg))]
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]
// #![ feature( concat_idents ) ]

mod implements_test
{
  use typing_tools as TheModule;
  include!( "./common/implements_test.rs" );
}
mod is_slice_test
{
  use typing_tools as TheModule;
  include!( "./common/is_slice_test.rs" );
}
mod inspect_type_test
{
  #[ allow( unused_imports ) ]
  use typing_tools as TheModule;
  include!( "./common/inspect_type_test.rs" );
}
