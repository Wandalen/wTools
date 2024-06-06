
use super::*;
use core::ops::{ Deref };
use core::marker::PhantomData;
use core::fmt;

/// Transparent wrapper for table-like structures.
#[ repr( transparent ) ]
#[ derive( Clone, Copy ) ]
pub struct AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
(
  &'a T,
  ::core::marker::PhantomData< ( &'a (), fn () -> ( RowKey, Row, CellKey, Cell, Title ) ) >,
)
where
  // T : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  // T : TableHeader< 'a, CellKey, Title >,
  // T : TableSize< 'a >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
;
// xxx : use maybe ref inside

impl< 'a, T, RowKey, Row, CellKey, Cell, Title > AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
where
  // T : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  // T : TableHeader< 'a, CellKey, Title >,
  // T : TableSize< 'a >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{
  /// Just a constructor.
  pub fn new( src : &'a T ) -> Self
  {
    Self( src, Default::default() )
  }
}

impl< 'a, T, RowKey, Row, CellKey, Cell, Title > AsRef< T > for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
where
  // T : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  // T : TableHeader< 'a, CellKey, Title >,
  // T : TableSize< 'a >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

// impl< 'a, T, RowKey, Row, CellKey, Cell, Title > AsMut< T >
// for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
// where
//   // T : TableRows< 'a, RowKey, Row, CellKey, Cell >,
//   // T : TableHeader< 'a, CellKey, Title >,
//   // T : TableSize< 'a >,
//   Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
//   Title : fmt::Debug,
//   Cell : fmt::Debug + Clone + 'a,
//   CellKey : fmt::Debug + Clone,
// {
//   fn as_mut( &mut self ) -> &mut T
//   {
//     &mut self.0
//   }
// }

impl< 'a, T, RowKey, Row, CellKey, Cell, Title > Deref for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
where
  // T : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  // T : TableHeader< 'a, CellKey, Title >,
  // T : TableSize< 'a >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{
  type Target = T;

  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

// impl< 'a, T, RowKey, Row, CellKey, Cell, Title > DerefMut
// for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
// where
//   // T : TableRows< 'a, RowKey, Row, CellKey, Cell >,
//   // T : TableHeader< 'a, CellKey, Title >,
//   // T : TableSize< 'a >,
//   Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
//   Title : fmt::Debug,
//   Cell : fmt::Debug + Clone + 'a,
//   CellKey : fmt::Debug + Clone,
// {
//   fn deref_mut( &mut self ) -> &mut Self::Target
//   {
//     &mut self.0
//   }
// }

impl< 'a, T, RowKey, Row, CellKey, Cell, Title > From< &'a T >
for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
where
  // T : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  // T : TableHeader< 'a, CellKey, Title >,
  // T : TableSize< 'a >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{
  fn from( table : &'a T ) -> Self
  {
    AsTable( table, PhantomData )
  }
}

// impl< 'a, T, RowKey, Row, CellKey, Cell, Title > Default for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
// where
//   T : Default,
//   // T : TableRows< 'a, RowKey, Row, CellKey, Cell >,
//   // T : TableHeader< 'a, CellKey, Title >,
//   // T : TableSize< 'a >,
//   Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
//   Title : fmt::Debug,
//   Cell : fmt::Debug + Clone + 'a,
//   CellKey : fmt::Debug + Clone,
// {
//   fn default() -> Self
//   {
//     AsTable( T::default(), PhantomData )
//   }
// }

impl< 'a, T, RowKey, Row, CellKey, Cell, Title > fmt::Debug for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
where
  T : fmt::Debug,
  // T : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  // T : TableHeader< 'a, CellKey, Title >,
  // T : TableSize< 'a >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "AsTable" )
    .field( "0", &self.0 )
    .finish()
  }
}