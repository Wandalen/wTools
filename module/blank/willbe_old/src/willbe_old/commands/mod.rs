crate::mod_interface!
{
  /// Init aggregator commands.
  prelude mod init;

  /// Works with crates
  prelude mod package;

  /// Iterate over subject
  prelude mod each;

  /// End of loop/program
  prelude mod end;

  protected use super::init::protected::*;

  protected use super::private::StartPointStack;
  protected use super::private::EndPointStack;
}

mod private
{
  /// Allow to go back to the iterator
  #[ derive( Debug, Default, Clone ) ]
  pub struct StartPointStack( Vec< usize > );

  impl std::ops::Deref for StartPointStack
  {
    type Target = Vec< usize >;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl std::ops::DerefMut for StartPointStack
  {
    fn deref_mut( &mut self ) -> &mut Self::Target
    {
      &mut self.0
    }
  }

  /// Allow to go back to the end
  #[ derive( Debug, Default, Clone ) ]
  pub struct EndPointStack( Vec< usize > );

  impl std::ops::Deref for EndPointStack
  {
    type Target = Vec< usize >;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl std::ops::DerefMut for EndPointStack
  {
    fn deref_mut( &mut self ) -> &mut Self::Target
    {
      &mut self.0
    }
  }
}
