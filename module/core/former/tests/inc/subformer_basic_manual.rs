// xxx : finish
use super::*;

// let ca = wca::CommandsAggregator::former()
// .command( "echo" )
//   .hint( "prints all subjects and properties" )
//   .subject( "Subject", wca::Type::String, true )
//   .property( "property", "simple property", wca::Type::String, true )
//   .routine( f1 )
//   .perform()
// .command( "exit" )
//   .hint( "just exit" )
//   .routine( || exit() )
//   .perform()
// .perform()
// ;
// ca.execute( input ).unwrap();

#[ derive( Debug, PartialEq ) ]
pub struct Command< K, E >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  pub hint : String,
  pub subject : String,
  pub properties : std::collections::HashMap< K, E >,
}

// impl< K, E > Default
// for Command< K, E >
// where
//   Routine : Fn( Context ) -> RoutineResult,
// {
//   #[ inline( always ) ]
//   fn default() -> Self
//   {
//     Self { container : Default::default() }
//   }
// }

pub trait End< T, Context >
{
  fn call( &self, container : T, context : Context ) -> Context;
}

impl< T, Context, F > End< T, Context > for F
where
  F : Fn( T, Context ) -> Context,
{
  #[ inline( always ) ]
  fn call( &self, container : T, context : Context ) -> Context
  {
    self( container, context )
  }
}

pub struct NoEnd;

impl< T, Context > End< T, Context >
for NoEnd
{
  #[ inline( always ) ]
  fn call( &self, _container : T, context : Context ) -> Context
  {
    context
  }
}

// // generated by new
// impl< K, E > Command< K, E >
// where
//   K : core::hash::Hash + std::cmp::Eq,
// {
//
//   #[ inline( always ) ]
//   pub fn new( container : std::collections::HashMap< K, E > ) -> Self
//   {
//     Self { container }
//   }
//
// }

// generated by former
impl< K, E > Command< K, E >
where
  K : core::hash::Hash + std::cmp::Eq,
{

  #[ inline( always ) ]
  pub fn former() -> CommandFormer< K, E, (), impl End< std::collections::HashMap< K, E >, () > >
  {
    CommandFormer::< K, E, (), NoEnd >::new
    (
      (),
      NoEnd,
    )
  }

  #[ inline( always ) ]
  pub fn perform( self ) -> Self
  {
    self
  }

}

// generated by former
// #[ derive( Debug, Default ) ]
pub struct CommandFormer< K, E, Context = (), End = NoEnd >
where
  K : core::hash::Hash + std::cmp::Eq,
{
  hint : core::option::Option< String >,
  subject : core::option::Option< String >,
  properties : core::option::Option< std::collections::HashMap< K, E > >,
  context : core::option::Option< Context >,
  on_end : core::option::Option< End >,
  // _e_phantom : core::marker::PhantomData< E >,
  // _k_phantom : core::marker::PhantomData< K >,
}

// generated by former
impl< K, E, Context, P >
CommandFormer< K, E, Context, P >
where
  K : core::hash::Hash + std::cmp::Eq,
  P : End< Command< K, E >, Context >,
{

  #[ inline( always ) ]
  fn form( mut self ) -> Command< K, E >
  {

    let hint = if self.hint.is_some()
    {
      self.hint.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };

    let subject = if self.subject.is_some()
    {
      self.subject.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };

    let properties = if self.properties.is_some()
    {
      self.properties.take().unwrap()
    }
    else
    {
      let val = Default::default();
      val
    };

    Command
    {
      hint,
      subject,
      properties,
    }.perform()
  }

  #[ inline( always ) ]
  pub fn new
  (
    context : Context,
    on_end : P,
  ) -> Self
  {

    Self
    {
      hint : None,
      subject : None,
      properties : None,
      context : Some( context ),
      on_end : Some( on_end ),
    }
  }

  // hint : core::option::Option< String >,
  // subject : core::option::Option< String >,
  // properties : core::option::Option< std::collections::HashMap< K, E > >,

  /// Return former of your struct moving container there. Should be called after configuring the container.
  #[ inline( always ) ]
  pub fn end( mut self ) -> Context
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take().unwrap();
    let container = self.form();
    on_end.call( container, context )
  }

  pub fn hint< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< String >,
  {
    debug_assert!( self.hint.is_none() );
    self.hint = Some( src.into() );
    self
  }

  pub fn subject< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< String >,
  {
    debug_assert!( self.subject.is_none() );
    self.subject = Some( src.into() );
    self
  }

  // pub fn properties( mut self ) -> former::runtime::HashMapFormer
  // <
  //   K,
  //   E,
  //   std::collections::HashMap< String, String >,
  //   CommandFormer< K, E, Context, P >,
  //   impl Fn( &mut Struct1Former, core::option::Option< std::collections::HashMap< K, E > > )
  // >
  // {
  //   let container = self.hashmap_strings_1.take();
  //   let on_end = | former : &mut Struct1Former, container : core::option::Option< std::collections::HashMap< String, String > > |
  //   {
  //     former.hashmap_strings_1 = container;
  //   };
  //   former::runtime::HashMapFormer::new( self, container, on_end )
  // }

}

// impl< K, E, Context, P >
// CommandFormer< K, E, Context, P >
// where
//   K : core::hash::Hash + std::cmp::Eq,
// {
//
//   /// Inserts a key-value pair into the map. Make a new container if it was not made so far.
//   #[ inline( always ) ]
//   pub fn insert< K2, E2 >( mut self, k : K2, e : E2 ) -> Self
//   where
//     K2 : core::convert::Into< K >,
//     E2 : core::convert::Into< E >,
//   {
//     if self.container.is_none()
//     {
//       self.container = core::option::Option::Some( Default::default() );
//     }
//     if let core::option::Option::Some( ref mut container ) = self.container
//     {
//       container.insert( k.into(), e.into() );
//     }
//     self
//   }
//
// }

//

// include!( "only_test/subformer_basic.rs" );
