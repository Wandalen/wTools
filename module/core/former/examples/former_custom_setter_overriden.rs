//! It's also possible to completely override setter and write its own from scratch.
//!
//! For that use attribe `[ setter( false ) ]` to disable setter. In the example, the default setter for `word` is disabled, and a custom setter is defined to automatically append an exclamation mark to the string. This method allows for complete control over the data assignment process, enabling the inclusion of any necessary logic or validation steps.
//!

#[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
fn main() {}

#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{
  use former::Former;

  /// Structure with a custom setter.
  #[ derive( Debug, Former ) ]
  pub struct StructWithCustomSetters
  {
    #[ setter( false ) ]
    word : String,
  }

  impl StructWithCustomSettersFormer
  {

    // Custom alternative setter for `word`
    pub fn word( mut self, value : impl Into< String > ) -> Self
    {
      debug_assert!( self.container.word.is_none() );
      self.container.word = Some( format!( "{}!", value.into() ) );
      self
    }

  }

  let example = StructWithCustomSetters::former()
  .word( "Hello" )
  .form();
  assert_eq!( example.word, "Hello!".to_string() );
}
