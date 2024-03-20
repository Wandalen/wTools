//! # Builder Pattern Implementation with Former
//!
//! This module demonstrates the use of the `Former` trait to apply the builder pattern for Rust structs.
//! The `Former` trait simplifies the instantiation of structs by enabling a fluent, method-chaining approach
//! to set fields before finalizing the instance with `.form()`. It is particularly useful for structs with optional fields
//! or when a clear and concise way to instantiate complex data structures is needed.
//!
//! ## How Former Works
//!
//! - **Trait Derivation** : By deriving `Former` on a struct, you automatically generate builder methods for each field.
//! - **Fluent Interface** : Each field's builder method allows for setting the value of that field and returns a mutable reference to the builder,
//!   enabling method chaining.
//! - **Optional Fields** : Optional fields can be easily handled without needing to explicitly set them to `None`.
//! - **Finalization** : The `.form()` method finalizes the building process and returns the constructed struct instance.
//!
//! This approach abstracts away the need for manually implementing a builder for each struct, making code more readable and maintainable.
//!

// xxx : regenerate

#![ allow( dead_code ) ]

#[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
fn main(){}

#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{

  #[ derive( Debug, PartialEq ) ]
  pub struct UserProfile
  {
    age : i32,
    username : String,
    bio_optional : Option< String >, // Fields could be optional
  }

  impl UserProfile
  {
    #[ inline( always ) ]
    pub fn former() -> UserProfileFormer< UserProfile, former::ReturnFormed >
    {
      UserProfileFormer::< UserProfile, former::ReturnFormed >::new()
    }
  }

  #[ derive( Debug, Default ) ]
  pub struct UserProfileFormerStorage
  {
    age : Option< i32 >,
    username : Option< String >,
    bio_optional : Option< String >,
  }

  pub struct UserProfileFormer
  <
    FormerContext = UserProfile,
    FormerEnd = former::ReturnFormed,
  >
  where
    FormerEnd : former::ToSuperFormer< UserProfile, FormerContext >,
  {
    storage : UserProfileFormerStorage,
    context : Option< FormerContext >,
    on_end : Option< FormerEnd >,
  }

  impl< FormerContext, FormerEnd > UserProfileFormer< FormerContext, FormerEnd >
  where
    FormerEnd : former::ToSuperFormer< UserProfile, FormerContext >,
  {
    #[ inline( always ) ]
    pub fn form( mut self ) -> UserProfile
    {
      let age = if self.storage.age.is_some()
      {
        self.storage.age.take().unwrap()
      }
      else
      {
        let val : i32 =
        {
          trait NotDefault< T >
          {
            fn maybe_default( self : &Self ) -> T { panic!( "Field 'age' isn't initialized" ) }
          }
          trait WithDefault< T >
          {
            fn maybe_default( self : &Self ) -> T;
          }
          impl< T > NotDefault< T > for &::core::marker::PhantomData< T > {}
          impl< T > WithDefault< T > for ::core::marker::PhantomData< T >
          where
            T : ::core::default::Default,
          {
            fn maybe_default( self : &Self ) -> T
            {
              T::default()
            }
          }
          ( &::core::marker::PhantomData::< i32 > ).maybe_default()
        };
        val
      };
      let username = if self.storage.username.is_some()
      {
        self.storage.username.take().unwrap()
      }
      else
      {
        let val : String =
        {
          trait NotDefault< T >
          {
            fn maybe_default( self : &Self ) -> T { panic!( "Field 'username' isn't initialized" ) }
          }
          trait WithDefault< T >
          {
            fn maybe_default( self : &Self ) -> T;
          }
          impl< T > NotDefault< T > for &::core::marker::PhantomData< T > {}
          impl< T > WithDefault< T > for ::core::marker::PhantomData< T >
          where
            T : ::core::default::Default,
          {
            fn maybe_default( self : &Self ) -> T
            {
              T::default()
            }
          }
          ( &::core::marker::PhantomData::< String > ).maybe_default()
        };
        val
      };
      let bio_optional = if self.storage.bio_optional.is_some()
      {
        Option::Some( self.storage.bio_optional.take().unwrap() )
      }
      else
      {
        Option::None
      };
      let result = UserProfile
      {
        age,
        username,
        bio_optional,
      };
      return result;
    }

    #[ inline( always ) ]
    pub fn perform( self ) -> UserProfile
    {
      let result = self.form();
      return result;
    }

    #[ inline( always ) ]
    pub fn new() -> UserProfileFormer< UserProfile, former::ReturnFormed >
    {
      UserProfileFormer::< UserProfile, former::ReturnFormed >::begin( None, former::ReturnFormed )
    }

    #[ inline( always ) ]
    pub fn begin
    (
      context : Option< FormerContext >,
      on_end : FormerEnd,
    ) -> Self
    {
      Self
      {
        storage : core::default::Default::default(),
        context : context,
        on_end : Option::Some( on_end ),
      }
    }

    #[ inline( always ) ]
    pub fn end( mut self ) -> FormerContext
    {
      let on_end = self.on_end.take().unwrap();
      let context = self.context.take();
      let formed = self.form();
      on_end.call( formed, context )
    }

    #[ inline ]
    pub fn age< Src >( mut self, src : Src ) -> Self
    where
      Src : Into< i32 >,
    {
      debug_assert!( self.storage.age.is_none() );
      self.storage.age = Option::Some( src.into() );
      self
    }

    #[ inline ]
    pub fn username< Src >( mut self, src : Src ) -> Self
    where
      Src : Into< String >,
    {
      debug_assert!( self.storage.username.is_none() );
      self.storage.username = Option::Some( src.into() );
      self
    }

    #[ inline ]
    pub fn bio_optional< Src >( mut self, src : Src ) -> Self
    where
      Src : Into< String >,
    {
      debug_assert!( self.storage.bio_optional.is_none() );
      self.storage.bio_optional = Option::Some( src.into() );
      self
    }
  }

  let profile = UserProfile::former()
  .age( 30 )
  .username( "JohnDoe".to_string() )
  .bio_optional( "Software Developer".to_string() )
  .form();

  dbg!( &profile );
  // Expected output:
  // &profile = UserProfile {
  //   age: 30,
  //   username: "JohnDoe",
  //   bio_optional: Some("Software Developer"),
  // }

}
