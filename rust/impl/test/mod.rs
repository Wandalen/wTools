pub extern crate paste;

#[ macro_export ]
macro_rules! test_suite
{
  ( $( $Name : ident ),* $(,)? ) =>
  {
    $( #[test] fn $Name() { $crate::paste::paste!([< _ $Name >])() } )*
    // $( #[test] fn $Name() { concat_idents!( _, $Name )() } )*
  }
  // ( $( $Name : ident ),* $(,)? ) =>
  // {
  //   // $( #[test] fn concat_idents!( $Name, _test )() { $Name() } )*
  //   $( #[test] fn paste!([< $Name _test >])() { $Name() } )*
  // }
}
