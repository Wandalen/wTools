/// Internal namespace.
pub( crate ) mod private
{

  pub trait TransitiveTryFrom< T > : Sized
  {
    type Error;

    // Required method
    fn transitive_try_from( value : T ) -> Result< Self, Self::Error >;
  }

  impl< T, U > TransitiveTryInto< U > for T
  where
    U : TransitiveTryFrom< T >,
  {
    type Error = U::Error;

    #[ inline ]
    fn transitive_try_into( self ) -> Result< U, U::Error >
    {
      U::transitive_try_from( self )
    }
  }

  pub trait TransitiveTryInto< T > : Sized
  {
    type Error;

    // Required method
    fn transitive_try_into( self ) -> Result< T, Self::Error >;
  }

}

crate::mod_interface!
{
  exposed use TransitiveTryFrom;
  exposed use TransitiveTryInto;
}
