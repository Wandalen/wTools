wtools::mod_interface!
{
  /// Init aggregator commands.
  prelude mod init;

  /// Information about package
  prelude mod info;

  /// Publish package
  prelude mod publish;

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
  pub struct StartPointStack( pub Vec< usize > );

  /// Allow to go back to the iterator
  #[ derive( Debug, Default ) ]
  pub struct EndPointStack( pub Vec< usize > );
}
