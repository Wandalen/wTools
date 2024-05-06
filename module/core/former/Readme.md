<!-- {{# generate.module_header{} #}} -->

# Module :: former
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)[![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_former_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_former_push.yml)[![docs.rs](https://img.shields.io/docsrs/former?color=e3e8f0&logo=docs.rs)](https://docs.rs/former)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module/core/former/examples/former_trivial.rs,RUN_POSTFIX=--example%20/home/sakapoi/Документи/wTools_fork/module/core/former/examples/former_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

A flexible and extensible implementation of the builder pattern.

It offers specialized subformers for common Rust collections like `Vec`, `HashMap`, and `HashSet`, enabling the construction of complex data structures in a fluent and intuitive manner.

## How Former Works

- **Trait Derivation** : By deriving `Former` on a struct, you automatically generate builder methods for each field.
- **Fluent Interface** : Each field's builder method allows for setting the value of that field and returns a mutable reference to the builder,
  enabling method chaining.
- **Optional Fields** : Optional fields can be easily handled without needing to explicitly set them to `None`.
- **Finalization** : The `.form()` method finalizes the building process and returns the constructed struct instance.

This approach abstracts away the need for manually implementing a builder for each struct, making code more readable and maintainable.

## Basic use-case

The provided code snippet illustrates a basic use-case of the Former crate in Rust, which is used to apply the builder pattern for structured and flexible object creation. Below is a detailed explanation of each part of the markdown chapter, aimed at clarifying how the Former trait simplifies struct instantiation.

```rust
#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{
  use former::Former;

  // Use attribute debug to print expanded code.
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ]
  pub struct UserProfile
  {
    age : i32,
    username : String,
    bio_optional : Option< String >, // Fields could be optional
  }

  let profile = UserProfile::former()
  .age( 30 )
  .username( "JohnDoe".to_string() )
  .bio_optional( "Software Developer".to_string() ) // Optionally provide a bio
  .form();

  dbg!( &profile );
  // Expected output:
  // &profile = UserProfile {
  //   age: 30,
  //   username: "JohnDoe",
  //   bio_optional: Some("Software Developer"),
  // }

 }
 ```

<details>
<summary>The code above will be expanded to this</summary>

```rust
#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{

  // Use attribute debug to print expanded code.
  #[ derive( Debug, PartialEq ) ]
  pub struct UserProfile
  {
    age : i32,
    username : String,
    bio_optional : Option< String >, // Fields could be optional
  }

  impl< > UserProfile< >
  where
  {
    #[ inline( always ) ]
    pub fn former() -> UserProfileFormer<
      UserProfileFormerDefinition< (), UserProfile< >, former::ReturnPreformed >
    >
    {
      UserProfileFormer::< UserProfileFormerDefinition< (), UserProfile< >, former::ReturnPreformed > >::
      new_coercing(former::ReturnPreformed)
    }
  }

  // = entity to

  impl< Definition > former::EntityToFormer< Definition > for UserProfile< >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage< > >,
  {
    type Former = UserProfileFormer< Definition >;
  }

  impl< > former::EntityToStorage for UserProfile< >
  where
  {
    type Storage = UserProfileFormerStorage< >;
  }

  impl< Context, Formed, End > former::EntityToDefinition< Context, Formed, End > for UserProfile< >
  where
    End : former::FormingEnd< UserProfileFormerDefinitionTypes< Context, Formed > >,
  {
    type Definition = UserProfileFormerDefinition< Context, Formed, End >;
  }

  // = definition

  #[derive(Debug)]
  pub struct UserProfileFormerDefinitionTypes< Context = (), Formed = UserProfile< >, >
  where
  {
    _phantom : core::marker::PhantomData< (*const Context, *const Formed) >,
  }

  impl< Context, Formed, > ::core::default::Default for UserProfileFormerDefinitionTypes< Context, Formed, >
  where
  {
    fn default() -> Self
    {
      Self
      {
        _phantom : core::marker::PhantomData,
      }
    }
  }

  impl< Context, Formed, > former::FormerDefinitionTypes for UserProfileFormerDefinitionTypes< Context, Formed, >
  where
  {
    type Storage = UserProfileFormerStorage< >;
    type Formed = Formed;
    type Context = Context;
  }

  #[derive(Debug)]
  pub struct UserProfileFormerDefinition< Context = (), Formed = UserProfile< >, End = former::ReturnPreformed, >
  where
  {
    _phantom : core::marker::PhantomData< (*const Context, *const Formed, *const End) >,
  }

  impl< Context, Formed, End, > ::core::default::Default for UserProfileFormerDefinition< Context, Formed, End, >
  where
  {
    fn default() -> Self
    {
      Self
      {
        _phantom : core::marker::PhantomData,
      }
    }
  }

  impl< Context, Formed, End, > former::FormerDefinition for UserProfileFormerDefinition< Context, Formed, End, >
  where
    End : former::FormingEnd< UserProfileFormerDefinitionTypes< Context, Formed, > >,
  {
    type Types = UserProfileFormerDefinitionTypes< Context, Formed, >;
    type End = End;
    type Storage = UserProfileFormerStorage< >;
    type Formed = Formed;
    type Context = Context;
  }

  impl< Context, Formed, > former::FormerMutator for UserProfileFormerDefinitionTypes< Context, Formed, >
  where
  {}

  // = storage

  pub struct UserProfileFormerStorage< >
  where
  {
    pub age : ::core::option::Option< i32 >,
    pub username : ::core::option::Option< String >,
    pub bio_optional : Option< String >,
  }

  impl< > ::core::default::Default for UserProfileFormerStorage< >
  where
  {
    #[ inline( always ) ]
    fn default() -> Self
    {
      Self
      {
        age : ::core::option::Option::None,
        username : ::core::option::Option::None,
        bio_optional : ::core::option::Option::None,
      }
    }
  }

  impl< > former::Storage for UserProfileFormerStorage< >
  where
  {
    type Formed = UserProfile< >;
  }

  impl< > former::StoragePreform for UserProfileFormerStorage< >
  where
  {
    type Preformed = UserProfile< >;
    fn preform(mut self) -> Self::Preformed
    {
      let age = if self.age.is_some()
      {
        self.age.take().unwrap()
      }
      else
      {
        {
          trait MaybeDefault< T >
          {
            fn maybe_default(self : &Self) -> T
            {
              panic!("Field 'age' isn't initialized")
            }
          }
          impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T >
          {}
          impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
          where T : ::core::default::Default,
          {
            fn maybe_default(self : &Self) -> T
            {
              T::default()
            }
          }
          (&::core::marker::PhantomData::< i32 >).maybe_default()
        }
      };
      let username = if self.username.is_some()
      {
        self.username.take().unwrap()
      }
      else
      {
        {
          trait MaybeDefault< T >
          {
            fn maybe_default(self : &Self) -> T
            {
              panic!("Field 'username' isn't initialized")
            }
          }
          impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T >
          {}
          impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
          where T : ::core::default::Default,
          {
            fn maybe_default(self : &Self) -> T
            {
              T::default()
            }
          }
          (&::core::marker::PhantomData::< String >).maybe_default()
        }
      };
      let bio_optional = if self.bio_optional.is_some()
      {
        ::core::option::Option::Some(self.bio_optional.take().unwrap())
      }
      else
      {
        ::core::option::Option::None
      };
      let result = UserProfile::<>
      {
        age,
        username,
        bio_optional,
      };
      return result;
    }
  }

  pub struct UserProfileFormer< Definition = UserProfileFormerDefinition< (), UserProfile< >, former::ReturnPreformed >, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage< > >,
  {
    pub storage : Definition::Storage,
    pub context : core::option::Option< Definition::Context >,
    pub on_end : core::option::Option< Definition::End >,
  }

  impl< Definition, > UserProfileFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage< > >, Definition::Types : former::FormerDefinitionTypes< Storage = UserProfileFormerStorage< > >,
  {
    #[ inline( always ) ]
    pub fn new(on_end : Definition::End) -> Self
    {
      Self::begin_coercing(None, None, on_end)
    }

    #[ inline( always ) ]
    pub fn new_coercing< IntoEnd >(end : IntoEnd) -> Self
    where IntoEnd : Into< Definition::End >,
    {
      Self::begin_coercing(None, None, end,)
    }

    #[ inline( always ) ]
    pub fn begin(mut storage : core::option::Option< Definition::Storage >, context : core::option::Option< Definition::Context >, on_end : <Definition as former::FormerDefinition>::End,) -> Self
    {
      if storage.is_none()
      {
        storage = Some(::core::default::Default::default());
      }
      Self
      {
        storage : storage.unwrap(),
        context : context,
        on_end : ::core::option::Option::Some(on_end),
      }
    }

    #[ inline( always ) ]
    pub fn begin_coercing< IntoEnd >(mut storage : core::option::Option< Definition::Storage >, context : core::option::Option< Definition::Context >, on_end : IntoEnd,) -> Self
    where IntoEnd : ::core::convert::Into< <Definition as former::FormerDefinition>::End >,
    {
      if storage.is_none()
      {
        storage = Some(::core::default::Default::default());
      }
      Self
      {
        storage : storage.unwrap(),
        context : context,
        on_end : ::core::option::Option::Some(::core::convert::Into::into(on_end)),
      }
    }

    #[ inline( always ) ]
    pub fn form(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
    {
      self.end()
    }

    #[ inline( always ) ]
    pub fn end(mut self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
    {
      let on_end = self.on_end.take().unwrap();
      let mut context = self.context.take();
      <Definition::Types as former::FormerMutator>::form_mutation(&mut self.storage, &mut context);
      former::FormingEnd::<Definition::Types>::call(&on_end, self.storage, context)
    }

    #[ inline( always ) ]
    pub fn age< Src >(mut self, src : Src) -> Self
    where Src : ::core::convert::Into< i32 >,
    {
      debug_assert!(self.storage.age.is_none());
      self.storage.age = ::core::option::Option::Some(::core::convert::Into::into(src));
      self
    }

    #[ inline( always ) ]
    pub fn username< Src >(mut self, src : Src) -> Self
    where Src : ::core::convert::Into< String >,
    {
      debug_assert!(self.storage.username.is_none());
      self.storage.username = ::core::option::Option::Some(::core::convert::Into::into(src));
      self
    }

    #[ inline( always ) ]
    pub fn bio_optional< Src >(mut self, src : Src) -> Self
    where Src : ::core::convert::Into< String >,
    {
      debug_assert!(self.storage.bio_optional.is_none());
      self.storage.bio_optional = ::core::option::Option::Some(::core::convert::Into::into(src));
      self
    }
  }

  impl< Definition, > UserProfileFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage< >, Formed = UserProfile< > >,
  {
    pub fn preform(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
    {
      former::StoragePreform::preform(self.storage)
    }
  }

  impl< Definition, > UserProfileFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage< >, Formed = UserProfile< >, >,
  {
    #[ inline( always ) ]
    pub fn perform(self) -> Definition::Formed
    {
      let result = self.form();
      return result;
    }
  }

  impl< Definition > former::FormerBegin< Definition > for UserProfileFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage< > >,
  {
    #[ inline( always ) ]
    fn former_begin(storage : core::option::Option< Definition::Storage >, context : core::option::Option< Definition::Context >, on_end : Definition::End,) -> Self
    {
      debug_assert!(storage.is_none());
      Self::begin(None, context, on_end)
    }
  }

  // = as subformer

  pub type UserProfileAsSubformer< Superformer, End > =
  UserProfileFormer< UserProfileFormerDefinition< Superformer, Superformer, End, >, >;

  pub trait UserProfileAsSubformerEnd< SuperFormer >
  where
    Self : former::FormingEnd< UserProfileFormerDefinitionTypes< SuperFormer, SuperFormer >, >, {}

  impl< SuperFormer, T > UserProfileAsSubformerEnd< SuperFormer > for T
  where
    Self : former::FormingEnd< UserProfileFormerDefinitionTypes< SuperFormer, SuperFormer >, >,
  {}

  // = end

  let profile = UserProfile::former()
  .age( 30 )
  .username( "JohnDoe".to_string() )
  .bio_optional( "Software Developer".to_string() ) // Optionally provide a bio
  .form();
  dbg!( &profile );

  // Expected output:
  //
  // &profile = UserProfile {
  //   age: 30,
  //   username: "JohnDoe",
  //   bio_optional: Some("Software Developer"),
  // }

}
```

</details>

## Custom and Alternative Setters

With help of `Former`, it is possible to define multiple versions of a setter for a single field, providing the flexibility to include custom logic within the setter methods. This feature is particularly useful when you need to preprocess data or enforce specific constraints before assigning values to fields. Custom setters should have unique names to differentiate them from the default setters generated by `Former`, allowing for specialized behavior while maintaining clarity in your code.

```rust
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# {

use former::Former;

/// Structure with a custom setter.
#[ derive( Debug, Former ) ]
pub struct StructWithCustomSetters
{
  word : String,
}

impl StructWithCustomSettersFormer
{

  // Custom alternative setter for `word`
  pub fn word_exclaimed( mut self, value : impl Into< String > ) -> Self
  {
    debug_assert!( self.storage.word.is_none() );
    self.storage.word = Some( format!( "{}!", value.into() ) );
    self
  }

}

let example = StructWithCustomSetters::former()
.word( "Hello" )
.form();
assert_eq!( example.word, "Hello".to_string() );

let example = StructWithCustomSetters::former()
.word_exclaimed( "Hello" )
.form();
assert_eq!( example.word, "Hello!".to_string() );

# }
```

In the example above showcases a custom alternative setter, `word_exclaimed`, which appends an exclamation mark to the input string before storing it. This approach allows for additional processing or validation of the input data without compromising the simplicity of the builder pattern.

## Custom Setter Overriding

But it's also possible to completely override setter and write its own from scratch. For that use attribe `[ setter( false ) ]` to disable setter.

```rust
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# {

use former::Former;

/// Structure with a custom setter.
#[ derive( Debug, Former ) ]
pub struct StructWithCustomSetters
{
  #[ scalar( setter = false ) ]
  word : String,
}

impl StructWithCustomSettersFormer
{

  // Custom alternative setter for `word`
  pub fn word( mut self, value : impl Into< String > ) -> Self
  {
    debug_assert!( self.storage.word.is_none() );
    self.storage.word = Some( format!( "{}!", value.into() ) );
    self
  }

}

let example = StructWithCustomSetters::former()
.word( "Hello" )
.form();
assert_eq!( example.word, "Hello!".to_string() );
# }
```

In the example above, the default setter for `word` is disabled, and a custom setter is defined to automatically append an exclamation mark to the string. This method allows for complete control over the data assignment process, enabling the inclusion of any necessary logic or validation steps.

## Custom Default

The `Former` crate enhances struct initialization in Rust by allowing the specification of custom default values for fields through the `default` attribute. This feature not only provides a way to set initial values for struct fields without relying on the `Default` trait but also adds flexibility in handling cases where a field's type does not implement `Default`, or a non-standard default value is desired.

```rust
# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# {

use former::Former;

/// Structure with default attributes.
#[ derive(  Debug, PartialEq, Former ) ]
pub struct ExampleStruct
{
  #[ former( default = 5 ) ]
  number : i32,
  #[ former( default = "Hello, Former!".to_string() ) ]
  greeting : String,
  #[ former( default = vec![ 10, 20, 30 ] ) ]
  numbers : Vec< i32 >,
}

let instance = ExampleStruct::former().form();
let expected = ExampleStruct
{
  number : 5,
  greeting : "Hello, Former!".to_string(),
  numbers : vec![ 10, 20, 30 ],
};
assert_eq!( instance, expected );
dbg!( &instance );
// > &instance = ExampleStruct {
// >    number: 5,
// >    greeting: "Hello, Former!",
// >    numbers: [
// >        10,
// >        20,
// >        30,
// >    ],
// > }
# }
```

The above code snippet showcases the `Former` crate's ability to initialize struct fields with custom default values:
- The `number` field is initialized to `5`.
- The `greeting` field defaults to a greeting message, "Hello, Former!".
- The `numbers` field starts with a vector containing the integers `10`, `20`, and `30`.

This approach significantly simplifies struct construction, particularly for complex types or where defaults beyond the `Default` trait's capability are required. By utilizing the `default` attribute, developers can ensure their structs are initialized safely and predictably, enhancing code clarity and maintainability.

## Concept of Storage and Former

Storage is temporary storage structure holds the intermediate state of an object during its construction.

Purpose of Storage:

- **Intermediate State Holding**: Storage serves as a temporary repository for all the partially set properties and data of the object being formed. This functionality is essential in situations where the object's completion depends on multiple, potentially complex stages of configuration.
- **Decoupling Configuration from Instantiation**: Storage separates the accumulation of configuration states from the actual creation of the final object. This separation fosters cleaner, more maintainable code, allowing developers to apply configurations in any order and manage interim states more efficiently, without compromising the integrity of the final object.

Storage is not just a passive container; it is an active part of a larger ecosystem that includes the former itself, a context, and a callback (often referred to as `FormingEnd`):

- **Former as an Active Manager**: The former is responsible for managing the storage, utilizing it to keep track of the object's evolving configuration. It orchestrates the formation process by handling intermediate states and preparing the object for its final form.
- **Contextual Flexibility**: The context associated with the former adds an additional layer of flexibility, allowing the former to adjust its behavior based on the broader circumstances of the object's formation. This is particularly useful when the forming process involves conditions or states external to the object itself.
- **FormingEnd Callback**: The `FormingEnd` callback is a dynamic component that defines the final steps of the forming process. It can modify the storage based on final adjustments, validate the object's readiness, or integrate the object into a larger structure, such as embedding it as a subformer within another structure.

These elements work in concert to ensure that the forming process is not only about building an object step-by-step but also about integrating it seamlessly into larger, more complex structures or systems. The `Former` framework, with its sophisticated management of storage, context, and callbacks, enables a highly flexible and reusable approach to object formation, making it ideal for scenarios where objects are part of nested or interdependent systems.

## Concept of Definitions

Definitions are utilized to encapsulate and manage generic parameters efficiently and avoid passing each parameter individually.

Two key definition Traits:

1. **`FormerDefinitionTypes`**:
   - This trait outlines the essential components involved in the formation process, including the types of storage, the form being created, and the context used. It focuses on the types involved rather than the termination of the formation process.
2. **`FormerDefinition`**:
   - Building upon `FormerDefinitionTypes`, this trait incorporates the `FormingEnd` callback, linking the formation types with a definitive ending. It specifies how the formation process should conclude, which may involve validations, transformations, or integrations into larger structures.
   - The inclusion of the `End` type parameter specifies the end conditions of the formation process, effectively connecting the temporary state held in storage to its ultimate form.

## Overview of Formation Traits

The formation process in our framework utilizes several core traits, each serving a specific purpose in the lifecycle of entity creation. These traits ensure that entities are constructed methodically, adhering to a structured pattern that enhances maintainability and scalability. Below is a summary of these key traits:

- `EntityToDefinition`: Links entities to their respective formation definitions which dictate their construction process.
- `EntityToFormer`: Connects entities with formers that are responsible for their step-by-step construction.
- `EntityToStorage`: Specifies the storage structures that temporarily hold the state of an entity during its formation.
- `FormerDefinition`, `FormerDefinitionTypes`: Define the essential properties and ending conditions of the formation process, ensuring entities are formed according to predetermined rules and logic.
- `Storage`: Establishes the fundamental interface for storage types used in the formation process, ensuring each can initialize to a default state.
- `StoragePreform`: Describes the transformation of storage from a mutable, intermediate state into the final, immutable state of the entity, crucial for accurately concluding the formation process.
- `FormerMutator`: Allows for custom mutation logic on the storage and context immediately before the formation process completes, ensuring last-minute adjustments are possible.
- `FormingEnd`: Specifies the closure action at the end of the formation process, which can transform or validate the final state of the entity.
- `FormingEndClosure`: Provides a flexible mechanism for dynamically handling the end of the formation process using closures, useful for complex scenarios.
- `FormerBegin`: Initiates a subforming process, managing how entities begin their formation in terms of storage and context setup.

These traits collectively facilitate a robust and flexible builder pattern that supports complex object creation and configuration scenarios.

## Concept of subformer

Subformers are specialized builders used within the `Former` framework to construct nested or collection-based data structures like vectors, hash maps, and hash sets. They simplify the process of adding elements to these structures by providing a fluent interface that can be seamlessly integrated into the overall builder pattern of a parent struct. This approach allows for clean and intuitive initialization of complex data structures, enhancing code readability and maintainability.

## Types of Setters / Subformers

It's crucial to understand the differences among subform setters, container setters, and scalar setters:

- **Scalar Setter**: Directly sets scalar values or simple fields within the forming entity. Unlike subform or container setters that manage complex objects or collections, scalar setters handle basic data types or individual fields. These are typically straightforward setter methods that do not involve nested formers or additional structuring.

- **Container Setter**: Returns a former of the container itself, offering an interface to manage the container as a whole rather than its individual elements. This type of setter is useful for applying configurations or validations to the entire collection, such as a `HashMap` of children.

- **Subform Setter**: Returns a former of an element within a container, providing an interface to individually form each element. For example, the `child` method acts as a subform setter, allowing for the addition and configuration of individual `Child` entities within the `Parent`'s `HashMap`.

Each type of setter is designed to address different needs in the formation process, ensuring that users can build complex, nested structures or simply set individual field values as required.

## Subformer example: Building a Vector

The following example illustrates how to use a `VectorSubformer` to construct a `Vec` field within a struct. The subformer enables adding elements to the vector with a fluent interface, streamlining the process of populating collection fields within structs.

```rust
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithVec
  {
    #[ container ]
    vec : Vec< &'static str >,
  }

  let instance = StructWithVec::former()
  .vec()
    .add( "apple" )
    .add( "banana" )
    .end()
  .form();

  assert_eq!( instance, StructWithVec { vec: vec![ "apple", "banana" ] } );

}
```

## Subformer example: Building a Hashmap

This example demonstrates the use of a `HashMapSubformer` to build a hash map within a struct. The subformer provides a concise way to insert key-value pairs into the map, making it easier to manage and construct hash map fields.

```rust
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use test_tools::exposed::*;

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithMap
  {
    #[ container ]
    map : collection_tools::HashMap< &'static str, &'static str >,
  }

  let struct1 = StructWithMap::former()
  .map()
    .add( ( "a", "b" ) )
    .add( ( "c", "d" ) )
    .end()
  .form()
  ;
  assert_eq!( struct1, StructWithMap { map : hmap!{ "a" => "b", "c" => "d" } } );
}
```

## Subformer example: Building a Hashset

In the following example, a `HashSetSubformer` is utilized to construct a hash set within a struct. This illustrates the convenience of adding elements to a set using the builder pattern facilitated by subformers.

```rust
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use test_tools::exposed::*;

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithSet
  {
    #[ container ]
    set : collection_tools::HashSet< &'static str >,
  }

  let instance = StructWithSet::former()
  .set()
    .add( "apple" )
    .add( "banana" )
    .end()
  .form();

  assert_eq!(instance, StructWithSet { set : hset![ "apple", "banana" ] });

}
```

## Custom Scalar Setter

This example demonstrates the implementation of a scalar setter using the `Former` trait in Rust. Unlike the more complex subform and container setters shown in previous examples, this example focuses on a straightforward approach to directly set a scalar value within a parent entity. The `Parent` struct manages a `HashMap` of `Child` entities, and the scalar setter is used to set the entire `HashMap` directly.

The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a container—each child entity in this case.

```rust
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use collection_tools::HashMap;
  use former::Former;

  // Child struct with Former derived for builder pattern support
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ]
  pub struct Child
  {
    name : String,
    description : String,
  }

  // Parent struct to hold children
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ]
  pub struct Parent
  {
    // Use `hint = true` to gennerate sketch of setter.
    #[ scalar( setter = false, hint = false ) ]
    children : HashMap< String, Child >,
  }

  impl< Definition > ParentFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = ParentFormerStorage >,
  {
    #[ inline ]
    pub fn children< Src >( mut self, src : Src ) -> Self
    where
      Src : ::core::convert::Into< HashMap< String, Child > >,
    {
      debug_assert!( self.storage.children.is_none() );
      self.storage.children = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
      self
    }
  }

  let echo = Child { name : "echo".to_string(), description : "prints all subjects and properties".to_string() };
  let exit = Child { name : "exit".to_string(), description : "just exit".to_string() };
  let mut children = HashMap::new();
  children.insert( echo.name.clone(), echo );
  children.insert( exit.name.clone(), exit );
  let ca = Parent::former()
  .children( children )
  .form();

  dbg!( &ca );
  // > &ca = Parent {
  // >     child: {
  // >          "echo": Child {
  // >              name: "echo",
  // >              description: "prints all subjects and properties",
  // >          },
  // >          "exit": Child {
  // >              name: "exit",
  // >              description: "just exit",
  // >          },
  // >     },
  // > }
}
```

In this example, the `Parent` struct functions as a container for multiple `Child` structs, each identified by a unique child name. The `ParentFormer` implements a custom method `child`, which serves as a subformer for adding `Child` instances into the `Parent`.

- **Child Definition**: Each `Child` consists of a `name` and a `description`, and we derive `Former` to enable easy setting of these properties using a builder pattern.
- **Parent Definition**: It holds a collection of `Child` objects in a `HashMap`. The `#[setter(false)]` attribute is used to disable the default setter, and a custom method `child` is defined to facilitate the addition of children with specific attributes.
- **Custom Subformer Integration**: The `child` method in the `ParentFormer` initializes a `ChildFormer` with a closure that integrates the `Child` into the `Parent`'s `child` map upon completion.

This pattern of using a structure's former as a subformer within another facilitates the creation of deeply nested or complex data structures through a coherent and fluent interface, showcasing the powerful capabilities of the `Former` framework for Rust applications.

## Custom Container Setter

This example demonstrates the use of container setters to manage complex nested data structures with the `Former` trait, focusing on a parent-child relationship structured around a container `HashMap`. Unlike typical builder patterns that add individual elements using subform setters, this example uses a container setter to manage the entire collection of children.

The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a container—each child entity in this case.

```rust
// Ensure the example only compiles when the appropriate features are enabled.
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use collection_tools::HashMap;
  use former::Former;

  // Child struct with Former derived for builder pattern support
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ]
  pub struct Child
  {
    name : String,
    description : String,
  }

  // Parent struct to hold children
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ]
  pub struct Parent
  {
    // Use `hint = true` to gennerate sketch of setter.
    #[ scalar( setter = false, hint = false ) ]
    children : HashMap< String, Child >,
  }

  impl< Definition > ParentFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = ParentFormerStorage >,
  {
    #[ inline ]
    pub fn children< Src >( mut self, src : Src ) -> Self
    where
      Src : ::core::convert::Into< HashMap< String, Child > >,
    {
      debug_assert!( self.storage.children.is_none() );
      self.storage.children = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
      self
    }
  }

  let echo = Child { name : "echo".to_string(), description : "prints all subjects and properties".to_string() };
  let exit = Child { name : "exit".to_string(), description : "just exit".to_string() };
  let mut children = HashMap::new();
  children.insert( echo.name.clone(), echo );
  children.insert( exit.name.clone(), exit );
  let ca = Parent::former()
  .children( children )
  .form();

  dbg!( &ca );
  // > &ca = Parent {
  // >     child: {
  // >          "echo": Child {
  // >              name: "echo",
  // >              description: "prints all subjects and properties",
  // >          },
  // >          "exit": Child {
  // >              name: "exit",
  // >              description: "just exit",
  // >          },
  // >     },
  // > }
}
```

## Custom Subform Setter

This example illustrates the implementation of nested builder patterns in Rust using the `Former` trait, emphasizing a parent-child relationship. Here, the `Parent` struct utilizes `ChildFormer` as a custom subformer to dynamically manage its `child` field—a `HashMap`. Each child in the `HashMap` is uniquely identified and configured via the `ChildFormer`.

The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a container—each child entity in this case.

```rust

// Ensure the example only compiles when the appropriate features are enabled.
#[ cfg( all( feature = "enabled", feature = "derive_former", not( feature = "no_std" ) ) ) ]
fn main()
{
  use collection_tools::HashMap;
  use former::Former;

  // Child struct with Former derived for builder pattern support
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ]
  pub struct Child
  {
    name : String,
    description : String,
  }

  // Parent struct to hold children
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ]
  pub struct Parent
  {
    // Use `hint = true` to gennerate sketch of setter.
    #[ subform( setter = false, hint = false ) ]
    child : HashMap< String, Child >,
  }

  /// Initializes and configures a subformer for adding named child entities. This method leverages an internal function
  /// to create and return a configured subformer instance. It allows for the dynamic addition of children with specific names,
  /// integrating them into the formation process of the parent entity.
  ///
  impl< Definition > ParentFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
  {

    #[ inline( always ) ]
    pub fn child( self, name : &str ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
    {
      self._child_add::< ChildFormer< _ >, _, >()
      .name( name )
    }

  }

  // Required to define how `value` is converted into pair `( key, value )`
  impl former::ValToEntry< HashMap< String, Child > > for Child
  {
    type Entry = ( String, Child );
    #[ inline( always ) ]
    fn val_to_entry( self ) -> Self::Entry
    {
      ( self.name.clone(), self )
    }
  }

  let ca = Parent::former()
  .child( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .child( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  dbg!( &ca );
  // > &ca = Parent {
  // >     child: {
  // >          "echo": Child {
  // >              name: "echo",
  // >              description: "prints all subjects and properties",
  // >          },
  // >          "exit": Child {
  // >              name: "exit",
  // >              description: "just exit",
  // >          },
  // >     },
  // > }
}
```

## Concept of Mutator

Provides a mechanism for mutating the context and storage just before the forming process is completed.

The `FormerMutator` trait allows for the implementation of custom mutation logic on the internal state
of an entity (context and storage) just before the final forming operation is completed. This mutation
occurs immediately before the `FormingEnd` callback is invoked.

Use cases of Mutator

- Applying last-minute changes to the data being formed.
- Setting or modifying properties that depend on the final state of the storage or context.
- Storage-specific fields which are not present in formed structure.

## Storage-Specific Fields

Storage-specific fields are intermediate fields that exist only in the storage structure during
the forming process. These fields are not present in the final formed structure but are instrumental
in complex forming operations, such as conditional mutations, temporary state tracking, or accumulations.

These fields are used to manage intermediate data or state that aids in the construction
of the final object but does not necessarily have a direct representation in the object's schema. For
instance, counters, flags, or temporary computation results that determine the final state of the object.

The `FormerMutator` trait facilitates the implementation of custom mutation logic. It acts on the internal
state (context and storage) just before the final forming operation is completed, right before the `FormingEnd`
callback is invoked. This trait is crucial for making last-minute adjustments or computations based on the
accumulated state in the storage.

## Mutator vs `FormingEnd`

Unlike `FormingEnd`, which is responsible for integrating and finalizing the formation process of a field within
a parent former, `form_mutation` directly pertains to the entity itself. This method is designed to be independent
of whether the forming process is occurring within the context of a superformer or if the structure is a standalone
or nested field. This makes `form_mutation` suitable for entity-specific transformations that should not interfere
with the hierarchical forming logic managed by `FormingEnd`.

## Example: Mutator

This example illustrates how to use the `FormerMutator` trait for implementing custom mutations
and demonstrates the concept of storage-specific fields in the forming process.

In this example, the fields `a` and `b` are defined only within the storage and used
within the custom mutator to enrich or modify the field `c` of the formed entity. This approach
allows for a richer and more flexible formation logic that can adapt based on the intermediate state
held within the storage.

```rust
#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{
  use former::Former;

  #[ derive( Debug, PartialEq, Former ) ]
  #[ storage_fields( a : i32, b : Option< String > ) ]
  #[ mutator( custom = true ) ]
  pub struct Struct1
  {
    c : String,
  }

  // = former mutator

  impl< Context, Formed > former::FormerMutator
  for Struct1FormerDefinitionTypes< Context, Formed >
  {
    Mutates the context and storage of the entity just before the formation process completes.
    #[ inline ]
    fn form_mutation( storage : &mut Self::Storage, _context : &mut ::core::option::Option< Self::Context > )
    {
      storage.a.get_or_insert_with( Default::default );
      storage.b.get_or_insert_with( Default::default );
      storage.c = Some( format!( "{:?} - {}", storage.a.unwrap(), storage.b.as_ref().unwrap() ) );
    }
  }

  let got = Struct1::former().a( 13 ).b( "abc" ).c( "def" ).form();
  let exp = Struct1
  {
    c : "13 - abc".to_string(),
  };
  assert_eq!( got, exp );
  dbg!( got );
  // > got = Struct1 {
  // >  c: "13 - abc",
  // > }

}
```

## To add to your project

```sh
cargo add former
```

## Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/former_trivial
cargo run
```
