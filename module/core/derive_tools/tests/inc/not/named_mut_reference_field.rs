use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct NamedMutReferenceField< 'a >
{
  a : &'a mut bool,
  b : u8,
}

include!( "only_test/named_mut_reference_field.rs" );
