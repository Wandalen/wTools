//!
//! ## Example : Custom Setter Overriding
//!
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
    // Use `debug` to gennerate sketch of setter.
    #[ scalar( setter = false ) ]
    word : String,
  }

  impl< Definition > StructWithCustomSettersFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = StructWithCustomSettersFormerStorage >,
  {
    // Custom alternative setter for `word`
    #[ inline ]
    pub fn word< Src >( mut self, src : Src ) -> Self
    where
      Src : ::core::convert::Into< String >,
    {
      debug_assert!( self.storage.word.is_none() );
      self.storage.word = Some( format!( "{}!", src.into() ) );
      self
    }
  }

  let example = StructWithCustomSetters::former()
  .word( "Hello" )
  .form();
  assert_eq!( example.word, "Hello!".to_string() );
  dbg!( example );
  //> StructWithCustomSetters {
  //>     word: "Hello!",
  //> }

}
