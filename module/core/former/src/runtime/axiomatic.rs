

/// Handler which is called on end of subforming to return origina context.
pub trait ToSuperFormer< T, Context >
{
  /// Function to call.
  fn call( &self, container : T, context : core::option::Option< Context > ) -> Context;
}

impl< T, Context, F > ToSuperFormer< T, Context > for F
where
  F : Fn( T, core::option::Option< Context > ) -> Context,
{
  #[ inline( always ) ]
  fn call( &self, container : T, context : core::option::Option< Context > ) -> Context
  {
    self( container, context )
  }
}

/// Don't do any processing, but return context as is.
#[ derive( Debug, Default ) ]
pub struct NoEnd;

impl< T, Context > ToSuperFormer< T, Context >
for NoEnd
{
  #[ inline( always ) ]
  fn call( &self, _container : T, context : core::option::Option< Context > ) -> Context
  {
    context.unwrap()
  }
}

/// Don't do any processing, but return container instrad of context.
#[ derive( Debug, Default ) ]
pub struct ReturnContainer;

impl< T > ToSuperFormer< T, T >
for ReturnContainer
{
  #[ inline( always ) ]
  fn call( &self, container : T, _context : core::option::Option< T > ) -> T
  {
    container
  }
}

//
