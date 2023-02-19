pub( crate ) mod private
{
  use crate::Type;

  use wtools::{ HashMap, Former };

  /// Command subject description
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub struct ValueDescription
  {
    /// subject hint
    pub hint : String,
    /// subject type
    pub kind : Type,
  }

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
    /// Command subjects hints and types.
    pub subjects : Vec< ValueDescription >,
    /// Hints and types for command options.
    pub properties : HashMap< String, ValueDescription >,
    /// TODO: Map of aliases.
    pub properties_aliases : HashMap< String, Vec< String > >,
  }

  impl CommandFormer
  {
    /// Setter for separate properties.
    pub fn subject< S : Into< String > >( mut self, hint : S, kind : Type ) -> Self
    {
      let hint = hint.into();
      let subject = ValueDescription { hint, kind };

      if self.subjects.is_none()
      {
        self.subjects = Some( vec![ subject ] );
      }
      else
      {
        let hints = self.subjects.as_mut().unwrap();
        hints.push( subject );
      }
      self
    }

    /// Setter for separate properties.
    pub fn property< S : AsRef< str >, H : Into< String > >( mut self, key : S, hint : H, kind : Type ) -> Self
    {
      let key = key.as_ref();
      let hint = hint.into();
      let property = ValueDescription { hint, kind };

      if self.properties.is_none()
      {
        self.properties = Some( HashMap::from([ ( key.into(), property ) ]) );
      }
      else
      {
        let hmap = self.properties.as_mut().unwrap();
        hmap.insert( key.into(), property );
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
  }
}

//

crate::mod_interface!
{
  prelude use Command;
  protected use ValueDescription;
}
