mod private
{
  use crate::*;

  use std::
  {
    borrow::Cow,
  };

  // // use crates_tools::CrateArchive;
  // // use workspace::Workspace;
  // use error::
  // {
  //   // untyped::Result,
  //   // typed::Error,
  //   untyped::format_err,
  // };

  pub trait AsCode
  {
    fn as_code< 'a >( &'a self ) -> std::io::Result< Cow< 'a, str > >;
  }

  /// A trait that defines a method for retrieving an iterator over items of a source file.
  ///
  /// The `Sources` trait is used to represent objects that can provide an iterator over their
  /// contained source files. This can be useful in scenarios where you need to access or process
  /// all source files associated with an object.
  pub trait CodeItems
  {
    /// Returns an iterator over the source files.
    fn items( &self ) -> impl Iterator< Item = syn::Item > + ExactSizeIterator< Item = syn::Item > + DoubleEndedIterator + Clone;
  }

}

//

crate::mod_interface!
{

  exposed use AsCode;
  exposed use CodeItems;

}
