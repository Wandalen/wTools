// Define a simple bound for testing generics
pub trait Bound : core::fmt::Debug + Default + Clone + PartialEq {}

// Define a concrete type satisfying the bound
#[ derive( Debug, Default, Clone, PartialEq ) ]
pub struct MyType( String );
impl Bound for MyType {}

// Define an inner generic struct to be used within the enum variants
#[ derive( Debug, Clone, PartialEq, Default ) ] // Removed former::Former derive
pub struct InnerScalar< T : Bound >
{
  pub data : T,
}
// Implement Into manually for testing the constructor signature
impl< T : Bound > From< T > for InnerScalar< T >
{
  fn from( data : T ) -> Self { Self { data } }
}