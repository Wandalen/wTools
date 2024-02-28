

/// Handler which is called on end of subforming to return origina context.
pub trait OnEnd< T, Context >
{
  /// Function to call.
  fn call( &self, container : T, context : Context ) -> Context;
}

impl< T, Context, F > OnEnd< T, Context > for F
where
  F : Fn( T, Context ) -> Context,
{
  #[ inline( always ) ]
  fn call( &self, container : T, context : Context ) -> Context
  {
    self( container, context )
  }
}

//
