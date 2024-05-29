//! ## Example : Custom Definition
//!
//! Define a custom former definition and custom forming logic, and apply them to a collection.
//!
//! The example showcases how to accumulate elements into a collection and then transform them into a single result
//! using a custom `FormingEnd` implementation. This pattern is useful for scenarios where the formation process
//! involves aggregation or transformation of input elements into a different type or form.

#[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
fn main() {}

#[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
fn main()
{
  // Define a struct `Sum` that will act as a custom former definition.
  struct Sum;

  // Implement `FormerDefinitionTypes` for `Sum`.
  // This trait defines the types used during the forming process.
  impl former::FormerDefinitionTypes for Sum
  {
    type Storage = Vec<i32>; // Collection for the integers.
    type Formed = i32;       // The final type after forming, which is a single integer.
    type Context = ();       // No additional context is used in this example.
  }

  // Implement `FormerMutator` for `Sum`.
  // This trait could include custom mutation logic applied during the forming process, but it's empty in this example.
  impl former::FormerMutator for Sum
  {
  }

  // Implement `FormerDefinition` for `Sum`.
  // This trait links the custom types to the former.
  impl former::FormerDefinition for Sum
  {
    type Types = Sum;        // Associate the `FormerDefinitionTypes` with `Sum`.
    type End = Sum;          // Use `Sum` itself as the end handler.
    type Storage = Vec<i32>; // Specify the storage type.
    type Formed = i32;       // Specify the final formed type.
    type Context = ();       // Specify the context type, not used here.
  }

  // Implement `FormingEnd` for `Sum`.
  // This trait handles the final step of the forming process.
  impl former::FormingEnd<Sum> for Sum
  {
    fn call
    (
      &self,
      storage: < Sum as former::FormerDefinitionTypes >::Storage,
      _context: Option< < Sum as former::FormerDefinitionTypes >::Context>
    )
    -> < Sum as former::FormerDefinitionTypes >::Formed
    {
      // Sum all integers in the storage vector.
      storage.iter().sum()
    }
  }

  // Use the custom `Former` to sum a list of integers.
  let got = former::CollectionFormer::<i32, Sum>::new(Sum)
  .add( 1 )  // Add an integer to the storage.
  .add( 2 )  // Add another integer.
  .add( 10 ) // Add another integer.
  .form(); // Perform the form operation, which triggers the summing logic.
  let exp = 13; // Expected result after summing 1, 2, and 10.
  assert_eq!(got, exp); // Assert the result is as expected.

  dbg!(got); // Debug print the result to verify the output.
  // > got = 13
}
