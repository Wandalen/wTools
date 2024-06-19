// #[ allow( dead_code ) ]
// struct GenericsTypesDefault< T = i32 >( T );

// impl< T > From< GenericsTypesDefault< T > > for T
// {
//   fn from( other : GenericsTypesDefault< T > ) -> Self
//   {
//     other.0
//   }
// }

// include!( "./only_test/generics_types_default.rs" );

// TODO: violates orphan rules. to be fixed
