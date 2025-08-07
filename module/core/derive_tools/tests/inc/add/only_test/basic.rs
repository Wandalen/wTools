// T1.1: Named struct
#[ test ]
fn test_named_struct_add_sub() 
{
  let a = NamedStruct { x : 1, y : 2 };
  let b = NamedStruct { x : 3, y : 4 };
  let sum = a + b.clone();
  let diff = sum.clone() - b;
  assert_eq!( sum.x, 4 );
  assert_eq!( sum.y, 6 );
  assert_eq!( diff.x, 1 );
  assert_eq!( diff.y, 2 );
}

// T1.2: Tuple struct
#[ test ]
fn test_tuple_struct_add_sub()
{
  let a = TupleStruct( 5 );
  let b = TupleStruct( 7 );
  let sum = a + b.clone();
  let diff = sum.clone() - b;
  assert_eq!( sum.0, 12 );
  assert_eq!( diff.0, 5 );
}

// T1.6: Generic struct T: Add/Sub
#[ test ]
fn test_generic_struct_add_sub()
{
  let a = GenericStruct { x : 10u32 };
  let b = GenericStruct { x : 20u32 };
  let sum = a + b.clone();
  let diff = sum.clone() - b;
  assert_eq!( sum.x, 30 );
  assert_eq!( diff.x, 10 );
}

// T1.7: Enum - same variant
#[ test ]
fn test_enum_e_add_sub()
{
  let a = E::One( 2 );
  let b = E::One( 3 );
  let sum = ( a + b.clone() ).expect( "Failed to add `E` enum variants" );
  let diff = ( sum.clone() - b ).expect( "Failed to sub `E` enum variants" );
  match sum 
  {
    E::One( val ) => assert_eq!( val, 5 ),
    #[ allow ( unreachable_patterns ) ]
    _ => {}
  }
  match diff 
  {
    E::One( val ) => assert_eq!( val, 2 ),
    #[ allow ( unreachable_patterns ) ]
    _ => {}
  }
}

// T1.9: Enum with #[error_type] attribute - same variant
#[ test ]
fn test_e2_add_same_variant() 
{
  let a = E2::One( 10 );
  let b = E2::One( 20 );
  let sum = ( a.clone() + b.clone() ).expect( "Should add same variants" );
  match sum 
  {
    E2::One( val ) => assert_eq!( val, 30 ),
    _ => panic!( "Unexpected variant" ),
  }  
  let a = E2::Two( 5 );
  let b = E2::Two( 7 );
  let sum = ( a.clone() + b.clone() ).expect( "Should add same variants" );
  match sum 
  {
    E2::Two( val ) => assert_eq!( val, 12 ),
    _ => panic!( "Unexpected variant" ),
  }
}

// T1.9: Enum with #[error_type] attribute - same variant
#[ test ]
fn test_e2_sub_same_variant() 
{
  let a = E2::One( 15 );
  let b = E2::One( 5 );
  let diff = ( a.clone() - b.clone() ).expect( "Should subtract same variants" );
  match diff 
  {
    E2::One( val ) => assert_eq!( val, 10 ),
    _ => panic!( "Unexpected variant" ),
  }  
  let a = E2::Two( 20 );
  let b = E2::Two( 8 );
  let diff = ( a.clone() - b.clone() ).expect( "Should subtract same variants" );
  match diff 
  {
    E2::Two( val ) => assert_eq!( val, 12 ),
    _ => panic!( "Unexpected variant" ),
  }
}

// T1.8: Enum - different variants (returns Err)
#[ test ]
fn test_e2_add_different_variant()
{
  let a = E2::One( 10 );
  let b = E2::Two( 20 );
  let result = a + b;
  assert!( result.is_err() );  
  let a = E2::Two( 5 );
  let b = E2::One( 7 );
  let result = a + b;
  assert!( result.is_err() );
}

// T1.8: Enum - different variants (returns Err)
#[ test ]
fn test_e2_sub_different_variant() 
{
  let a = E2::One( 15 );
  let b = E2::Two( 5 );
  let result = a - b;
  assert!( result.is_err() );  
  let a = E2::Two( 20 );
  let b = E2::One( 8 );
  let result = a - b;
  assert!( result.is_err() );
}

// T1.10: Enum with #[error_expr] attribute - same variant
#[ test ]
fn test_e3_add_same_variant() 
{
  let a = E3::One( 10 );
  let b = E3::One( 20 );
  let sum = ( a.clone() + b.clone() ).expect( "Should add same variants" );
  match sum 
  {
    E3::One( val ) => assert_eq!( val, 30 ),
    _ => panic!( "Unexpected variant" ),
  }  
  let a = E3::Two( 5 );
  let b = E3::Two( 7 );
  let sum = ( a.clone() + b.clone() ).expect( "Should add same variants" );
  match sum 
  {
    E3::Two( val ) => assert_eq!( val, 12 ),
    _ => panic!( "Unexpected variant" ),
  }
}

// T1.10: Enum with #[error_expr] attribute - same variant
#[ test ]
fn test_e3_sub_same_variant() 
{
  let a = E3::One( 15 );
  let b = E3::One( 5 );
  let diff = ( a.clone() - b.clone() ).expect( "Should subtract same variants" );
  match diff 
  {
    E3::One( val ) => assert_eq!( val, 10 ),
    _ => panic!( "Unexpected variant" ),
  }  
  let a = E3::Two( 20 );
  let b = E3::Two( 8 );
  let diff = ( a.clone() - b.clone() ).expect( "Should subtract same variants" );
  match diff 
  {
    E3::Two( val ) => assert_eq!( val, 12 ),
    _ => panic!( "Unexpected variant" ),
  }
}

// T1.10: Enum with #[error_expr] attribute - different variants (returns Err(ErrorExpr))
#[ test ]
fn test_e3_add_different_variant_returns_error_expression() 
{
  let a = E3::One( 10 );
  let b = E3::Two( 20 );
  let result = a + b;
  assert_eq!( result, Err( ErrorExpr::DifferentVariants ) );  
  let a = E3::Two( 5 );
  let b = E3::One( 7 );
  let result = a + b;
  assert_eq!( result, Err( ErrorExpr::DifferentVariants ) );
}

// T1.10: Enum with #[error_expr] attribute - different variants (returns Err(ErrorExpr))
#[ test ]
fn test_e3_sub_different_variant_returns_error_expression()
{
  let a = E3::One( 15 );
  let b = E3::Two( 5 );
  let result = a - b;
  assert_eq!( result, Err( ErrorExpr::DifferentVariants ) );  
  let a = E3::Two( 20 );
  let b = E3::One( 8 );
  let result = a - b;
  assert_eq!( result, Err( ErrorExpr::DifferentVariants ) );
}

// T1.11: Enum with #[derive_ops(error_type)] - different variants
#[ test ]
fn test_e4_sub_different_variant_error_type() 
{
  let a = E4::Two( 30 );
  let b = E4::One( 15 );
  let result = a - b;
  assert!( result.is_err(), "Expected error when subtracting different variants for E4" );
}

// T1.12: Enum with #[derive_ops(error_expr)] - different variants (returns Err(ErrorExpr))
#[ test ]
fn test_e5_add_different_variant_returns_error_expression() 
{
  let a = E5::One( 10 );
  let b = E5::Two( 20 );
  let result = a + b;
  assert_eq!( result, Err( ErrorExpr::DifferentVariants ) );  
  let a = E5::Two( 5 );
  let b = E5::One( 7 );
  let result = a + b;
  assert_eq!( result, Err( ErrorExpr::DifferentVariants ) );
}

// T1.12: Enum with #[derive_ops(error_expr)] - different variants (returns Err(ErrorExpr))
#[ test ]
fn test_e5_sub_different_variant_returns_error_expression()
{
  let a = E5::One( 15 );
  let b = E5::Two( 5 );
  let result = a - b;
  assert_eq!( result, Err( ErrorExpr::DifferentVariants ) );  
  let a = E5::Two( 20 );
  let b = E5::One( 8 );
  let result = a - b;
  assert_eq!( result, Err( ErrorExpr::DifferentVariants ) );
}