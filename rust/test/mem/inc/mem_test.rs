use super::*;

//

tests_impls!
{

  fn same_ptr()
  {

    let src1 = "abc";
    let src2 = "abc";
    a_true!( TheModule::same_ptr( src1, src2 ) );

    let src1 = ( 1 );
    let src2 = ( 1 );
    a_false!( TheModule::same_ptr( &src1, &src2 ) );

    let src1 = ( 1 );
    let src2 = "abcde";
    a_false!( TheModule::same_ptr( &src1, src2 ) );

  }

  //

  fn same_size()
  {

    let src1 = "abc";
    let src2 = "abc";
    a_true!( TheModule::same_size( src1, src2 ) );

    let src1 = ( 1 );
    let src2 = ( 1 );
    a_true!( TheModule::same_size( &src1, &src2 ) );

    let src1 = ( 1 );
    let src2 = "abcde";
    a_false!( TheModule::same_size( &src1, src2 ) );

  }

  //

  fn same_region()
  {

    let src1 = "abc";
    let src2 = "abc";
    a_true!( TheModule::same_region( src1, src2 ) );

    let src1 = ( 1 );
    let src2 = ( 1 );
    a_false!( TheModule::same_region( &src1, &src2 ) );

    let src1 = ( 1 );
    let src2 = "abcde";
    a_false!( TheModule::same_region( &src1, src2 ) );

  }

}

//

tests_index!
{
  same_ptr,
  same_size,
  same_region,
}
