#![allow(dead_code)]

//

// pub trait Options
// {
//   type For : Clone;

//   fn form( &self ) -> Self::For
//   {
//     self.ins.clone()
//   }

// }

//

#[macro_export]
macro_rules! field_str
{
  ( $name:ident ) =>
  {
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
    pub fn $name2< Str : AsRef< str > >( &mut self, src : Str ) -> &mut Self
    where
      String : From<Str>
    {
      self.ins.$name1 = src.into();
      self
    }
  };
}

#[macro_export]
macro_rules! field_map_str_str
{
  ( $name1:ident, $name2:ident ) =>
  {
    pub fn $name2< Str : AsRef< str > >( &mut self, property : Str, hint : Str ) -> &mut Self
    where
      String : From<Str>
    {
      self.ins.$name1.insert( property.into(), hint.into() );
      self
    }
  };
}

#[macro_export]
macro_rules! field_map_str_vec_str
{
  ( $name1:ident, $name2:ident ) =>
  {
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

#[macro_export]
macro_rules! field_routine
{
  ( $name:ident ) =>
  {
    pub fn $name( &mut self, routine : &'static dyn Fn() ) -> &mut Self
    {
      self.ins.$name = routine.into();
      self
    }
  };
  ( $name1:ident, $name2:ident ) =>
  {
    pub fn $name2( &mut self, routine : &'static dyn Fn() ) -> &mut Self
    {
      self.ins.$name1 = routine.into();
      self
    }
  };
}
