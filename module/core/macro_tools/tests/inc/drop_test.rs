
use super::*;

#[ test ]
fn test_needs_drop()
{
  struct NeedsDrop;

  impl Drop for NeedsDrop
  {
    fn drop( &mut self ) {}
  }

  assert!( std::mem::needs_drop::< NeedsDrop >() );

  // Test each of the types with a handwritten TrivialDrop impl above.
  assert!( !std::mem::needs_drop::< std::iter::Empty< NeedsDrop > >() );
  assert!( !std::mem::needs_drop::< std::slice::Iter< '_, NeedsDrop > >() );
  assert!( !std::mem::needs_drop::< std::slice::IterMut< '_, NeedsDrop > >() );
  assert!( !std::mem::needs_drop::< std::option::IntoIter< &NeedsDrop > >() );
  assert!( !std::mem::needs_drop::< std::option::IntoIter< &mut NeedsDrop > >() );

}
