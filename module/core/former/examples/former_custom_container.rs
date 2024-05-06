/// Example former_custom_container.rs
///
/// Container interface is defined in the crate and implemented for containers like vectors, hash maps, etc, but if you want to use non-standard container you can implement container interface for the container. This example demonstrate how to do that.


#[ cfg( not( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ) ]
fn main() {}
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use collection_tools::HashSet;
  use std::fmt;

  // = define custom container

  // Custom container that logs additions
  #[derive(Default)]
  pub struct LoggingSet<T>
  {
    set: HashSet<T>,
  }

//   // Implementing the container traits for LoggingSet
//   impl<T: Eq + std::hash::Hash + fmt::Debug> former::Container for LoggingSet<T>
//   {
//     type Entry = T;
//     type Val = T;
//
//     fn entry_to_val(e: Self::Entry) -> Self::Val
//     {
//       e // In this simple case, entry and value are the same.
//     }
//   }

  // This trait allows adding entries to the LoggingSet
  impl<T: Eq + std::hash::Hash + fmt::Debug> former::ContainerAdd for LoggingSet<T>
  {
    fn add(&mut self, e: Self::Entry) -> bool
    {
      let result = self.set.insert(e);
      if result {
        println!("{:?} was added to the set", e);
      }
      result
    }
  }

  // = use custom container

  // Define a struct to use with Former
  #[derive(Debug, PartialEq, former::Former)]
  pub struct CollectionContainer
  {
    #[container]
    data: LoggingSet<i32>,
  }

  // Using the builder pattern provided by Former to manipulate CollectionContainer
  let mut container = CollectionContainer::former().data();

  container.add(10);
  container.add(20);
  container.add(10); // This will not be added again, and "add" will log the attempt.

  let final_container = container.end().form();

  println!("Final container: {:?}", final_container);
}