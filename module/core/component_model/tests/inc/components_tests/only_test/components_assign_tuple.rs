#[ test ]
fn components_assign()
{
  // Create an instance of the larger struct
  let t1 = TupleStruct1( 42, "Hello".to_string(), 13.1 );

  // Create a default instance of the smaller struct
  let mut t2 = TupleStruct2::default();

  // Call the generated assign method (assuming snake_case name)
  // TupleStruct2ComponentsAssign::tuple_struct_2_assign( &mut t2, &t1 );
  t2.tuple_struct_2_assign( &t1 ); // Use the method directly

  // Define the expected result
  let exp = TupleStruct2( 42, "Hello".to_string() );

  // Assert equality
  assert_eq!( t2, exp );
}

// Optional: Test assigning to self if types match exactly
#[ derive( Debug, Default, PartialEq, component_model::Assign, component_model::ComponentsAssign ) ]
struct SelfTuple(bool, char);

impl From<&SelfTuple> for bool
{
    fn from( src: &SelfTuple ) -> Self
    {
        src.0
    }
}
impl From<&SelfTuple> for char
{
    fn from( src: &SelfTuple ) -> Self
    {
        src.1
    }
}

#[ test ]
fn components_assign_self()
{
    let t1 = SelfTuple(true, 'a');
    let mut t2 = SelfTuple::default();
    t2.self_tuple_assign(&t1);
    assert_eq!(t2, t1);
}