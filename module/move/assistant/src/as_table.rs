
use super::*;
use core::ops::{ Deref, DerefMut };
use core::marker::PhantomData;
use core::fmt;

/// Transparent wrapper for table-like structures.
// #[ derive( Debug ) ]
#[ repr( transparent ) ]
pub struct AsTable< 'a, T, Row, Key, Cell, Title >( T, ::core::marker::PhantomData< fn () -> ( Row, Key, Cell, Title, &'a () ) > )
where
  T : TableRows< 'a, Row, Key, Cell >,
  T : TableHeader< 'a, Key, Title >,
  T : TableSize,
  Row : 'a + Clone + Cells< 'a, Key, Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'static,
;

impl< 'a, T, Row, Key, Cell, Title > AsRef< T > for AsTable< 'a, T, Row, Key, Cell, Title >
where
  T : TableRows< 'a, Row, Key, Cell > + TableHeader< 'a, Key, Title > + TableSize,
  Row : 'a + Clone + Cells< 'a, Key, Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'static,
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

impl< 'a, T, Row, Key, Cell, Title > AsMut< T > for AsTable< 'a, T, Row, Key, Cell, Title >
where
  T : TableRows< 'a, Row, Key, Cell > + TableHeader< 'a, Key, Title > + TableSize,
  Row : 'a + Clone + Cells< 'a, Key, Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'static,
{
  fn as_mut( &mut self ) -> &mut T
  {
    &mut self.0
  }
}

impl< 'a, T, Row, Key, Cell, Title > Deref for AsTable< 'a, T, Row, Key, Cell, Title >
where
  T : TableRows< 'a, Row, Key, Cell > + TableHeader< 'a, Key, Title > + TableSize,
  Row : 'a + Clone + Cells< 'a, Key, Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'static,
{
  type Target = T;

  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< 'a, T, Row, Key, Cell, Title > DerefMut for AsTable< 'a, T, Row, Key, Cell, Title >
where
  T : TableRows< 'a, Row, Key, Cell > + TableHeader< 'a, Key, Title > + TableSize,
  Row : 'a + Clone + Cells< 'a, Key, Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'static,
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

impl< 'a, T, Row, Key, Cell, Title > From< T > for AsTable< 'a, T, Row, Key, Cell, Title >
where
  T : TableRows< 'a, Row, Key, Cell > + TableHeader< 'a, Key, Title > + TableSize,
  Row : 'a + Clone + Cells< 'a, Key, Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'static,
{
  fn from( table : T ) -> Self
  {
    AsTable( table, PhantomData )
  }
}

// impl< 'a, T, Row, Key, Cell, Title > From< AsTable< 'a, T, Row, Key, Cell, Title > > for T
// where
//   T : TableRows< 'a, Row, Key, Cell > + TableHeader< 'a, Key, Title > + TableSize,
//   Row : 'a + Clone + Cells< 'a, Key, Cell >,
//   Title : fmt::Debug,
//   Cell : fmt::Debug + Clone + 'static,
// {
//   fn from( as_table : AsTable< 'a, T, Row, Key, Cell, Title > ) -> Self
//   {
//     as_table.0
//   }
// }

impl< 'a, T, Row, Key, Cell, Title > Default for AsTable< 'a, T, Row, Key, Cell, Title >
where
  T : Default + TableRows< 'a, Row, Key, Cell > + TableHeader< 'a, Key, Title > + TableSize,
  Row : 'a + Clone + Cells< 'a, Key, Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'static,
{
  fn default() -> Self
  {
    AsTable( T::default(), PhantomData )
  }
}

impl< 'a, T, Row, Key, Cell, Title > fmt::Debug for AsTable< 'a, T, Row, Key, Cell, Title >
where
  T : TableRows< 'a, Row, Key, Cell > + TableHeader< 'a, Key, Title > + TableSize + fmt::Debug,
  Row : 'a + Clone + Cells< 'a, Key, Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'static,
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "AsTable" )
    .field( "0", &self.0 )
    .finish()
  }
}
