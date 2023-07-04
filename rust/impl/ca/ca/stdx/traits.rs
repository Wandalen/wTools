pub( crate ) mod private
{
  /// An extension trait for commands.
  ///
  /// This trait provides additional methods for enhancing commands, such as
  /// adding arguments and properties.
  pub trait CommandExt< T >: Sized
  {
    /// Adds an argument to the command.
    fn arg( self, hint : &str, tag : wca::Type ) -> Builder< Self > 
    {
      Builder::new(self).arg(hint, tag)
    }

    /// Adds properties to the command.
    fn properties< const N: usize >( self, properties: [ Property; N ] ) -> Builder< Self >
    {
      Builder::new( self ).properties( properties )
    }
  }

  impl< F: Fn( T, wca::Args, wca::Props ) -> Result, T > CommandExt<T> for F {}

  /// A trait for converting a type into a `Builder`.
  pub trait IntoBuilder< F, T >: Sized 
  {
    /// Converts the type into a `Builder` instance.
    fn into_builder( self ) -> Builder< F >;
  }

  impl< F, T > IntoBuilder< F, T > for Builder< F > 
  {
    fn into_builder(self) -> Self 
    {
      self
    }
  }

  impl< F: Fn( T, wca::Args, wca::Props ) -> Result, T > IntoBuilder< F, T > for F 
  {
    fn into_builder( self ) -> Builder< F > 
    {
      Builder::new( self )
    }
  }
}

crate::mod_interface!
{

}
