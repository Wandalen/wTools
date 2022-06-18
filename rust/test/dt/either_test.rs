use super::*;

//

tests_impls!
{

  fn basic_test()
  {
    let left : TheModule::Either< _, () > = TheModule::Either::Left( 13 );
    a_id!( left.flip(), TheModule::Either::Right( 13 ) );
  }

}

//

tests_index!
{
  basic_test,
}
