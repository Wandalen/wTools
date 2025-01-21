use super::*;

#[ test ]
fn exposed_main_namespace()
{

  the_module::error::debug_assert_id!( 1, 1 );
  the_module::exposed::error::debug_assert_id!( 1, 1 );
  use the_module::exposed::*;
  error::debug_assert_id!( 1, 1 );

}