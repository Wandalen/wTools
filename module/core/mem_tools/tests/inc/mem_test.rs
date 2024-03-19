use super::*;

//

tests_impls!
{

  fn same_data()
  {
    let buf = [ 0u8; 128 ];
    a_true!( the_module::same_data( &buf, &buf ) );

    let x = [ 0u8; 1 ];
    let y = 0u8;

    a_true!( the_module::same_data( &x, &y ) );

    a_false!( the_module::same_data( &buf, &x ) );
    a_false!( the_module::same_data( &buf, &y ) );

    struct H1( &'static str );
    struct H2( &'static str );

    a_true!( the_module::same_data( &H1( "hello" ), &H2( "hello" ) ) );
    a_false!( the_module::same_data( &H1( "qwerty" ), &H2( "hello" ) ) );

  }

  fn same_ptr()
  {

    let src1 = "abc";
    let src2 = "abc";
    a_true!( the_module::same_ptr( src1, src2 ) );

    let src1 = ( 1, );
    let src2 = ( 1, );
    a_false!( the_module::same_ptr( &src1, &src2 ) );

    let src1 = ( 1 );
    let src2 = "abcde";
    a_false!( the_module::same_ptr( &src1, src2 ) );

  }

  //

  fn same_size()
  {

    let src1 = "abc";
    let src2 = "cba";
    a_true!( the_module::same_size( src1, src2 ) );

    let src1 = ( 1, );
    let src2 = ( 3, );
    a_true!( the_module::same_size( &src1, &src2 ) );

    let src1 = ( 1 );
    let src2 = "abcde";
    a_false!( the_module::same_size( &src1, src2 ) );

  }

  //

  fn same_region()
  {

    let src1 = "abc";
    let src2 = "abc";
    a_true!( the_module::same_region( src1, src2 ) );

    let src1 = ( 1, );
    let src2 = ( 1, );
    a_false!( the_module::same_region( &src1, &src2 ) );

    let src1 = ( 1 );
    let src2 = "abcde";
    a_false!( the_module::same_region( &src1, src2 ) );

  }

  //

  fn samples()
  {
    use the_module as mem;

    // Are two pointers are the same, not taking into accoint type.
    // Unlike `std::ptr::eq()` does not require arguments to have the same type.
    let src1 = ( 1, );
    let src2 = ( 1, );
    assert!( !mem::same_ptr( &src1, &src2 ) );

    // Are two pointers points on data of the same size.
    let src1 = "abc";
    let src2 = "cba";
    assert!( mem::same_size( src1, src2 ) );

    // Are two pointers points on the same region, ie same size and same pointer.
    // Does not require arguments to have the same type.
    let src1 = "abc";
    let src2 = "abc";
    assert!( mem::same_region( src1, src2 ) );

  }

}

//

tests_index!
{
  same_data,
  same_ptr,
  same_size,
  same_region,
  samples,
}
