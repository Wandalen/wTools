/// Private namespace of the module.
pub( crate ) mod private
{
  ///
  /// Expand field to method of instance.
  ///

  #[ macro_export ]
  macro_rules! field_str
  {
    ( $name:ident ) =>
    {
      #[ allow( missing_docs ) ] // rrr : for Dmytro : make proper solution, maybe use Former
      pub fn $name< Str : AsRef< str > >( &mut self, src : Str ) -> &mut Self
      where
        String : From<Str>
      {
        self.ins.$name = src.into();
        self
      }
    };
    ( $name1:ident, $name2:ident ) =>
    {
      #[ allow( missing_docs ) ] // rrr : for Dmytro : : make proper solution, maybe use Former
      pub fn $name2< Str : AsRef< str > >( &mut self, src : Str ) -> &mut Self
      where
        String : From<Str>
      {
        self.ins.$name1 = src.into();
        self
      }
    };
  }

  ///
  /// Expand field to alias method of instance.
  ///

  #[ macro_export ]
  macro_rules! field_map_str_str
  {
    ( $name1:ident, $name2:ident ) =>
    {
      #[ allow( missing_docs ) ] // rrr : for Dmytro : : make proper solution, maybe use Former
      pub fn $name2< Str : AsRef< str > >( &mut self, property : Str, hint : Str ) -> &mut Self
      where
        String : From<Str>
      {
        self.ins.$name1.insert( property.into(), hint.into() );
        self
      }
    };
  }

  ///
  /// Expand field to method that works with a vector of strings.
  ///

  #[ macro_export ]
  macro_rules! field_map_str_vec_str
  {
    ( $name1:ident, $name2:ident ) =>
    {
      #[ allow( missing_docs ) ] // rrr : for Dmytro : : make proper solution, maybe use Former
      pub fn $name2< Str : AsRef< str > >( &mut self, property : Str, alias : Str ) -> &mut Self
      where
        String : From<Str>
      {
        let entry = self.ins.$name1.entry( property.into() ).or_insert_with( || -> Vec< String > { vec![] } );
        entry.push( alias.into() );
        self
      }
    };
  }

  ///
  /// Expand field to method of instance to setup routine.
  ///

  #[ macro_export ]
  macro_rules! field_routine
  {
    ( $name:ident ) =>
    {
      #[ allow( missing_docs ) ] // rrr : for Dmytro : : make proper solution, maybe use Former
      pub fn $name( &mut self, routine : &'static dyn Fn( &crate::instruction::Instruction ) -> Result< () > ) -> &mut Self
      {
        self.ins.$name = routine.into();
        self
      }
    };
    ( $name1:ident, $name2:ident ) =>
    {
      #[ allow( missing_docs ) ] // rrr : for Dmytro : : make proper solution, maybe use Former
      pub fn $name2( &mut self, routine : &'static dyn Fn( &crate::instruction::Instruction ) -> Result< () > ) -> &mut Self
      {
        self.ins.$name1 = routine.into();
        self
      }
    };
  }

  pub( crate ) use field_str;
  pub( crate ) use field_map_str_str;
  pub( crate ) use field_map_str_vec_str;
  pub( crate ) use field_routine;

}

//

crate::mod_interface!
{
  // qqq : for Dima : bad : list all elements, don't use * for private /* aaa : Dmytro : expanded */
  prelude( crate ) use field_str;
  prelude( crate ) use field_map_str_str;
  prelude( crate ) use field_map_str_vec_str;
  prelude( crate ) use field_routine;
}
