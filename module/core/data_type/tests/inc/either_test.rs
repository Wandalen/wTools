use super::*;

//

tests_impls!
{

  fn basic_test()
  {
    let left : the_module::Either< _, () > = the_module::Either::Left( 13 );
    a_id!( left.flip(), the_module::Either::Right( 13 ) );
  }

}

//

tests_index!
{
  basic_test,
}
