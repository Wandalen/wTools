//!
//! ## Basic use-case
//!
//! Demonstrates the usage of `clone_dyn` to enable cloning for trait objects.
//!
//! By default, Rust does not support cloning for trait objects due to the `Clone` trait
//! requiring compile-time knowledge of the type's size. The `clone_dyn` crate addresses
//! this limitation through procedural macros, allowing for cloning collections of trait objects
//! and crate `clone_dyn_types` contains implementation of all types.
//!
//! ##### Overview
//!
//! This example shows how to use the `clone_dyn` crate to enable cloning for trait objects,
//! specifically for iterators. It defines a custom trait, `IterTrait`, that encapsulates
//! an iterator with specific characteristics and demonstrates how to use `CloneDyn` to
//! overcome the object safety constraints of the `Clone` trait.
//!
//! ##### The `IterTrait` Trait
//!
//! The `IterTrait` trait is designed to represent iterators that yield references to items (`&'a T`).
//! These iterators must also implement the `ExactSizeIterator` and `DoubleEndedIterator` traits.
//! Additionally, the iterator must implement the `CloneDyn` trait, which allows cloning of trait objects.
//!
//! The trait is implemented for any type that meets the specified requirements.
//!
//! ##### Cloning Trait Objects
//!
//! Rust's type system does not allow trait objects to implement the `Clone` trait directly due to object safety constraints.
//! Specifically, the `Clone` trait requires knowledge of the concrete type at compile time, which is not available for trait objects.
//!
//! The `CloneDyn` trait from the `clone_dyn_types` crate provides a workaround for this limitation by allowing trait objects to be cloned.
//!
//! The example demonstrates how to implement `Clone` for boxed `IterTrait` trait objects.
//!
//! ##### `get_iter` Function
//!
//! The `get_iter` function returns a boxed iterator that implements the `IterTrait` trait.
//! If the input is `Some`, it returns an iterator over the vector.
//! If the input is `None`, it returns an empty iterator.
//!
//! It's not possible to use `impl Iterator` here because the code returns iterators of two different types:
//! - `std::slice::Iter` when the input is `Some`.
//! - `std::iter::Empty` when the input is `None`.
//!
//! To handle this, the function returns a trait object ( `Box< dyn IterTrait >` ).
//! However, Rust's `Clone` trait cannot be implemented for trait objects due to object safety constraints.
//! The `CloneDyn` trait addresses this problem by enabling cloning of trait objects.
//!
//! ##### `use_iter` Function
//!
//! The `use_iter` function demonstrates the use of the `CloneDyn` trait by cloning the iterator.
//! It then iterates over the cloned iterator and prints each element.
//!
//! ##### Main Function
//!
//! The main function demonstrates the overall usage by creating a vector, obtaining an iterator, and using the iterator to print elements.
//!

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}
#[ cfg( feature = "enabled" ) ]
fn main()
{
  use clone_dyn_types::CloneDyn;

  /// Trait that encapsulates an iterator with specific characteristics, tailored for your needs.
  pub trait IterTrait< 'a, T >
  where
    T : 'a,
    Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
    Self : CloneDyn,
  {
  }

  impl< 'a, T, I > IterTrait< 'a, T > for I
  where
    T : 'a,
    Self : Iterator< Item = T > + ExactSizeIterator< Item = T > + DoubleEndedIterator,
    Self : CloneDyn,
  {
  }

  // Implement `Clone` for boxed `IterTrait` trait objects.
  #[ allow( non_local_definitions ) ]
  impl< 'c, T > Clone for Box< dyn IterTrait< 'c, T > + 'c >
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      clone_dyn_types::clone_into_box( &**self )
    }
  }

  ///
  /// Function to get an iterator over a vector of integers.
  ///
  /// This function returns a boxed iterator that implements the `IterTrait` trait.
  /// If the input is `Some`, it returns an iterator over the vector.
  /// If the input is `None`, it returns an empty iterator.
  ///
  /// Rust's type system does not allow trait objects to implement the `Clone` trait directly due to object safety constraints.
  /// Specifically, the `Clone` trait requires knowledge of the concrete type at compile time, which is not available for trait objects.
  ///
  /// In this example, we need to return an iterator that can be cloned. Since we are returning a trait object ( `Box< dyn IterTrait >` ),
  /// we cannot directly implement `Clone` for this trait object. This is where the `CloneDyn` trait from the `clone_dyn_types` crate comes in handy.
  ///
  /// The `CloneDyn` trait provides a workaround for this limitation by allowing trait objects to be cloned.
  ///
  /// It's not possible to use `impl Iterator` here because the code returns iterators of two different types:
  /// - `std::slice::Iter` when the input is `Some`.
  /// - `std::iter::Empty` when the input is `None`.
  ///
  /// To handle this, the function returns a trait object (`Box<dyn IterTrait>`).
  /// However, Rust's `Clone` trait cannot be implemented for trait objects due to object safety constraints.
  /// The `CloneDyn` trait addresses this problem by enabling cloning of trait objects.

  pub fn get_iter< 'a >( src : Option< &'a Vec< i32 > > ) -> Box< dyn IterTrait< 'a, &'a i32 > + 'a >
  {
    match &src
    {
      Some( src ) => Box::new( src.iter() ),
      _ => Box::new( core::iter::empty() ),
    }
  }

  /// Function to use an iterator and print its elements.
  ///
  /// This function demonstrates the use of the `CloneDyn` trait by cloning the iterator.
  /// It then iterates over the cloned iterator and prints each element.
  pub fn use_iter< 'a >( iter : Box< dyn IterTrait< 'a, &'a i32 > + 'a > )
  {
    // Clone would not be available if CloneDyn is not implemented for the iterator.
    // And being an object-safe trait, it can't implement Clone.
    // Nevertheless, thanks to CloneDyn, the object is clonable.
    //
    // This line demonstrates cloning the iterator and iterating over the cloned iterator.
    iter.clone().for_each( | e | println!( "{e}" ) );

    // Iterate over the original iterator and print each element.
    iter.for_each( | e | println!( "{e}" ) );
  }

  // Create a vector of integers.
  let data = vec![ 1, 2, 3 ];
  // Get an iterator over the vector.
  let iter = get_iter( Some( &data ) );
  // Use the iterator to print its elements.
  use_iter( iter );

}
