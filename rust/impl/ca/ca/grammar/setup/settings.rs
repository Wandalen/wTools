pub( crate ) mod private
{
  use crate::{ Routine, Args, Props, Context };
  use wtools::
  {
    HashMap,
    Former,
    Result,
  };

  ///
  /// Command descriptor.
  ///

  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  #[ derive( Former ) ]
  pub struct Command
  {
    /// Command common hint.
    #[ alias( h ) ]
    pub hint : String,
    /// Command full hint.
    #[ alias( lh ) ]
    pub long_hint : String,
    /// Phrase descriptor for command.
    pub phrase : String,
    /// Command subjects hints.
    pub subjects_hint : Vec< String >,
    /// Hints for command options.
    pub properties_hints : HashMap< String, String >,

    // TODO:
    // /// Map of aliases.
    // pub properties_aliases : HashMap< String, Vec< String > >,

    /// Command routine.
    #[ setter( false ) ]
    pub routine : Routine,
  }

  impl CommandFormer
  {
    /// Setter for separate properties.
    pub fn subject_hint< S : Into< String > >( mut self, hint : S ) -> Self
    {
      let hint = hint.into();

      if self.subjects_hint.is_none()
      {
        self.subjects_hint = Some( vec![ hint ] );
      }
      else
      {
        let hints = self.subjects_hint.as_mut().unwrap();
        hints.push( hint );
      }
      self
    }

    /// Setter for separate properties.
    pub fn property_hint< S : AsRef< str > >( mut self, key : S, hint : S ) -> Self
    {
      let key = key.as_ref();
      let hint = hint.as_ref();

      if self.properties_hints.is_none()
      {
        self.properties_hints = Some( HashMap::from([ ( key.into(), hint.into() ) ]) );
      }
      else
      {
        let hmap = self.properties_hints.as_mut().unwrap();
        hmap.insert( key.into(), hint.into() );
      }
      self
    }

    // // Setter for separate properties aliases.
    // pub fn property_alias< S : AsRef< str > >( mut self, key : S, alias : S ) -> Self
    // {
    //   let key = key.as_ref();
    //   let alias = alias.as_ref();

    //   if self.properties_aliases.is_none()
    //   {
    //     self.properties_aliases = Some( HashMap::from([ ( key.into(), vec![ alias.into() ] ) ]) );
    //   }
    //   else
    //   {
    //     let hmap = self.properties_aliases.as_mut().unwrap();
    //     if hmap.get( key ).is_some()
    //     {
    //       let vec_aliases = hmap.get_mut( key ).unwrap();
    //       vec_aliases.push( alias.into() );
    //     }
    //     else
    //     {
    //       hmap.insert( key.into(), vec![ alias.into() ] );
    //     }
    //   }
    //   self
    // }

    ///
    /// Routine setter.
    ///

    pub fn routine< F >( mut self, callback: F ) -> Self
    where
      F : Fn(( &Args, &Props )) -> Result< () > + 'static,
    {
      self.routine = Some( Routine::new( callback ) );
      self
    }

    ///
    /// Routine setter with context.
    ///

    pub fn routine_with_ctx< F >( mut self, callback: F ) -> Self
    where
      F : Fn( ( &Args, &Props ), Context ) -> Result< () > + 'static,
    {
      self.routine = Some( Routine::new_with_ctx( callback ) );
      self
    }
  }
}

//

crate::mod_interface!
{
  prelude use Command;
}
