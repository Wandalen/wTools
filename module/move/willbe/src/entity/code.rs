mod private
{
  use crate::*;

  use std::
  {
    borrow::Cow,
  };

  /// A trait for converting an object to its code representation.
  ///
  /// The `AsCode` trait defines a method for converting an object into a code representation,
  /// typically as a string. This can be useful for generating code from various data structures
  /// or objects.
  ///
  /// ```
  pub trait AsCode
  {
    /// Converts the object to its code representation.
    fn as_code< 'a >( &'a self ) -> std::io::Result< Cow< 'a, str > >;
  }

  /// A trait for retrieving an iterator over items of a source file.
  ///
  /// The `CodeItems` trait is used to represent objects that can provide an iterator over their
  /// contained source files. This is useful in scenarios where you need to access or process
  /// all source files associated with an object, such as a workspace or a package.
  pub trait CodeItems
  {
    /// Returns an iterator over the source files.
    fn items( &self ) -> impl IterTrait< '_, syn::Item >;
  }
}

//

crate::mod_interface!
{

  exposed use AsCode;
  exposed use CodeItems;

}
