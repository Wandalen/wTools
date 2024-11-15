use super::*;

//

#[ allow( dead_code ) ]
#[ test ]
fn same_data()
{
  let buf = [ 0u8; 128 ];
  assert!( the_module::mem::same_data( &buf, &buf ) );

  let x = [ 0u8; 1 ];
  let y = 0u8;

  assert!( the_module::mem::same_data( &x, &y ) );

  assert!( !the_module::mem::same_data( &buf, &x ) );
  assert!( !the_module::mem::same_data( &buf, &y ) );

  struct H1( &'static str );
  struct H2( &'static str );

  assert!( the_module::mem::same_data( &H1( "hello" ), &H2( "hello" ) ) );
  assert!( !the_module::mem::same_data( &H1( "qwerty" ), &H2( "hello" ) ) );

}
