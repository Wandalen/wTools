wtools::mod_interface!
{
  /// Init aggregator commands.
  prelude mod init;

  /// Information about package
  prelude mod info;

  /// Publish package
  prelude mod publish;

  /// Works with package dependencies
  prelude mod dep;

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
  #[ derive( Debug, Default ) ]
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
  #[ derive( Debug, Default ) ]
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
