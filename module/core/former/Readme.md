<!-- {{# generate.module_header{} #}} -->

# Module :: former
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_former_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_former_push.yml) [![docs.rs](https://img.shields.io/docsrs/former?color=e3e8f0&logo=docs.rs)](https://docs.rs/former) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fformer%2Fexamples%2Fformer_trivial.rs,RUN_POSTFIX=--example%20former_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

A flexible implementation of the Builder pattern supporting nested builders and collection-specific subformers.

The Builder pattern allows you to construct objects step by step, using only the steps you need. Any fields not explicitly set will receive default values. By implementing this pattern, you can avoid passing numerous parameters into your constructors.

This crate offers specialized subformers for common Rust collections, enabling the construction of complex data structures in a fluent and intuitive manner. Additionally, it provides the ability to define and reuse formers as subformers within other formers.

## How Former Works

- **Derivation**: By deriving `Former` on a struct, you automatically generate builder methods for each field.
- **Fluent Interface**: Each field's builder method allows for setting the value of that field and returns a mutable reference to the builder, enabling method chaining.
- **Optional Fields**: Optional fields can be easily handled without needing to explicitly set them to `None`.
- **Finalization**: The `.form()` method finalizes the building process and returns the constructed struct instance.
- **Subforming**: If a field has its own former defined or is a container of items for which a former is defined, it can be used as a subformer.

This approach abstracts away the need for manually implementing a builder for each struct, making the code more readable and maintainable.

## Comparison

The Former crate and the abstract Builder pattern concept share a common goal: to construct complex objects step-by-step, ensuring they are always in a valid state and hiding internal structures. Both use a fluent interface for setting fields and support default values for fields that aren't explicitly set. They also have a finalization method to return the constructed object (.form() in Former, build() in [traditional Builder](https://refactoring.guru/design-patterns/builder)).

However, the Former crate extends the traditional Builder pattern by automating the generation of builder methods using macros. This eliminates the need for manual implementation, which is often required in the abstract concept. Additionally, Former supports nested builders and subformers for complex data structures, allowing for more sophisticated object construction.

Advanced features such as custom setters, subformer reuse, storage-specific fields, mutators, and context management further differentiate Former from the [traditional approach](https://refactoring.guru/design-patterns/builder), which generally focuses on simpler use-cases without these capabilities. Moreover, while the traditional Builder pattern often includes a director class to manage the construction process, Former is not responsible for that aspect.

## Example : Trivial

<!-- qqq : for Petro : implement command `will .generators.list` show rules of applying generators -->
<!-- qqq : for Petro : implement command `will .generators.applied.list` show actual files to which applied which generator -->

<!-- qqq : for Petro : make this generator working -->
<!--{ example.use{ code : "former_trivial", hidden_code : "former_trivial_expanded", link : true, try_out : true } }-->
<!--{ example.use.end }-->

The provided code snippet illustrates a basic use-case of the Former, which is used to apply the builder pattern for to construct complex objects step-by-step, ensuring they are always in a valid state and hiding internal structures.

```rust
# #[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
# fn main() {}

# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# fn main()
# {

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

# }
```

<details>
<summary>The code above will be expanded to this</summary>

```rust
# #[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
# fn main() {}

# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# fn main()
# {

  // Use attribute debug to print expanded code.
  #[ derive( Debug, PartialEq ) ]
  pub struct UserProfile
  {
    age : i32,
    username : String,
    bio_optional : Option< String >, // Fields could be optional
  }

  impl UserProfile
  where
  {
    #[ inline( always ) ]
    pub fn former() -> UserProfileFormer<
      UserProfileFormerDefinition< (), UserProfile, former::ReturnPreformed >
    >
    {
      UserProfileFormer::< UserProfileFormerDefinition< (), UserProfile, former::ReturnPreformed > >::
      new_coercing(former::ReturnPreformed)
    }
  }

  // = entity to

  impl< Definition > former::EntityToFormer< Definition > for UserProfile
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage >,
  {
    type Former = UserProfileFormer< Definition >;
  }

  impl former::EntityToStorage for UserProfile
  where
  {
    type Storage = UserProfileFormerStorage;
  }

  impl< Context, Formed, End > former::EntityToDefinition< Context, Formed, End > for UserProfile
  where
    End : former::FormingEnd< UserProfileFormerDefinitionTypes< Context, Formed > >,
  {
    type Definition = UserProfileFormerDefinition< Context, Formed, End >;
    type Types = UserProfileFormerDefinitionTypes< Context, Formed >;
  }

  // = definition

  #[derive(Debug)]
  pub struct UserProfileFormerDefinitionTypes< Context = (), Formed = UserProfile, >
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
    type Storage = UserProfileFormerStorage;
    type Formed = Formed;
    type Context = Context;
  }

  #[derive(Debug)]
  pub struct UserProfileFormerDefinition< Context = (), Formed = UserProfile, End = former::ReturnPreformed, >
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
    type Storage = UserProfileFormerStorage;
    type Formed = Formed;
    type Context = Context;
  }

  impl< Context, Formed, > former::FormerMutator for UserProfileFormerDefinitionTypes< Context, Formed, >
  where
  {}

  // = storage

  pub struct UserProfileFormerStorage
  where
  {
    pub age : ::core::option::Option< i32 >,
    pub username : ::core::option::Option< String >,
    pub bio_optional : Option< String >,
  }

  impl ::core::default::Default for UserProfileFormerStorage
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

  impl former::Storage for UserProfileFormerStorage
  where
  {
    type Preformed = UserProfile;
  }

  impl former::StoragePreform for UserProfileFormerStorage
  where
  {
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

  pub struct UserProfileFormer< Definition = UserProfileFormerDefinition< (), UserProfile, former::ReturnPreformed >, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage >,
  {
    pub storage : Definition::Storage,
    pub context : core::option::Option< Definition::Context >,
    pub on_end : core::option::Option< Definition::End >,
  }

  impl< Definition, > UserProfileFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage >, Definition::Types : former::FormerDefinitionTypes< Storage = UserProfileFormerStorage >,
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
      self.storage.age = ::core::option::Option::Some(::core::convert::Into::into( src ));
      self
    }

    #[ inline( always ) ]
    pub fn username< Src >(mut self, src : Src) -> Self
    where Src : ::core::convert::Into< String >,
    {
      debug_assert!(self.storage.username.is_none());
      self.storage.username = ::core::option::Option::Some(::core::convert::Into::into( src ));
      self
    }

    #[ inline( always ) ]
    pub fn bio_optional< Src >(mut self, src : Src) -> Self
    where Src : ::core::convert::Into< String >,
    {
      debug_assert!(self.storage.bio_optional.is_none());
      self.storage.bio_optional = ::core::option::Option::Some(::core::convert::Into::into( src ));
      self
    }
  }

  impl< Definition, > UserProfileFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage, Formed = UserProfile >,
  {
    pub fn preform(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
    {
      former::StoragePreform::preform(self.storage)
    }
  }

  impl< Definition, > UserProfileFormer< Definition, >
  where
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage, Formed = UserProfile, >,
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
    Definition : former::FormerDefinition< Storage = UserProfileFormerStorage >,
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

# }
```

</details>

Try out `cargo run --example former_trivial`.
<br/>
[See code](./examples/former_trivial.rs).

## Example : Custom and Alternative Setters

With help of `Former`, it is possible to define multiple versions of a setter for a single field, providing the flexibility to include custom logic within the setter methods. This feature is particularly useful when you need to preprocess data or enforce specific constraints before assigning values to fields. Custom setters should have unique names to differentiate them from the default setters generated by `Former`, allowing for specialized behavior while maintaining clarity in your code.

```rust
# #[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
# fn main() {}

# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# fn main()
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

Try out `cargo run --example former_custom_setter`.
<br/>
[See code](./examples/former_custom_setter.rs).

## Example : Custom Setter Overriding

But it's also possible to completely override setter and write its own from scratch. For that use attribe `[ setter( false ) ]` to disable setter.

```rust
# #[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
# fn main() {}

# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# fn main()
# {

  use former::Former;

  /// Structure with a custom setter.
  #[ derive( Debug, Former ) ]
  pub struct StructWithCustomSetters
  {
    // Use `debug` to gennerate sketch of setter.
    #[ scalar( setter = false ) ]
    word : String,
  }

  impl< Definition > StructWithCustomSettersFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = StructWithCustomSettersFormerStorage >,
  {
    // Custom alternative setter for `word`
    #[ inline ]
    pub fn word< Src >( mut self, src : Src ) -> Self
    where
      Src : ::core::convert::Into< String >,
    {
      debug_assert!( self.storage.word.is_none() );
      self.storage.word = Some( format!( "{}!", src.into() ) );
      self
    }
  }

  let example = StructWithCustomSetters::former()
  .word( "Hello" )
  .form();
  assert_eq!( example.word, "Hello!".to_string() );
  dbg!( example );
  //> StructWithCustomSetters {
  //>     word: "Hello!",
  //> }

# }
```

In the example above, the default setter for `word` is disabled, and a custom setter is defined to automatically append an exclamation mark to the string. This method allows for complete control over the data assignment process, enabling the inclusion of any necessary logic or validation steps.

Try out `cargo run --example former_custom_setter_overriden`.
<br/>
[See code](./examples/former_custom_setter_overriden.rs).

## Example : Custom Defaults

The `Former` crate enhances struct initialization by allowing the specification of custom default values for fields through the `default` attribute. This feature not only provides a way to set initial values for struct fields without relying on the `Default` trait but also adds flexibility in handling cases where a field's type does not implement `Default`, or a non-standard default value is desired.

```rust
# #[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
# fn main() {}

# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# fn main()
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

Try out `cargo run --example former_custom_defaults`.
<br/>
[See code](./examples/former_custom_defaults.rs).

## Concept of Storage and Former

Storage is temporary storage structure holds the intermediate state of an object during its construction.

Purpose of Storage:

- **Intermediate State Holding**: Storage serves as a temporary repository for all the partially set properties and data of the object being formed. This functionality is essential in situations where the object's completion depends on multiple, potentially complex stages of configuration.
- **Decoupling Configuration from Instantiation**: Storage separates the accumulation of configuration states from the actual creation of the final object. This separation fosters cleaner, more maintainable code, allowing developers to apply configurations in any order and manage interim states more efficiently, without compromising the integrity of the final object.

Storage is not just a passive collection; it is an active part of a larger ecosystem that includes the former itself, a context, and a callback (often referred to as `FormingEnd`):

- **Former as an Active Manager**: The former is responsible for managing the storage, utilizing it to keep track of the object's evolving configuration. It orchestrates the formation process by handling intermediate states and preparing the object for its final form.
- **Contextual Flexibility**: The context associated with the former adds an additional layer of flexibility, allowing the former to adjust its behavior based on the broader circumstances of the object's formation. This is particularly useful when the forming process involves conditions or states external to the object itself.
- **FormingEnd Callback**: The `FormingEnd` callback is a dynamic component that defines the final steps of the forming process. It can modify the storage based on final adjustments, validate the object's readiness, or integrate the object into a larger structure, such as embedding it as a subformer within another structure.

These elements work in concert to ensure that the forming process is not only about building an object step-by-step but also about integrating it seamlessly into larger, more complex structures or systems.

## Concept of subformer

Subformers are specialized builders used within the former to construct nested or collection-based data structures like vectors, hash maps, and hash sets. They simplify the process of adding elements to these structures by providing a fluent interface that can be seamlessly integrated into the overall builder pattern of a parent struct. This approach allows for clean and intuitive initialization of complex data structures, enhancing code readability and maintainability.

## Types of Setters / Subformers

Understanding the distinctions among the types of setters or subformers is essential for effectively employing the builder pattern in object construction. Each type of setter is designed to meet specific needs in building complex, structured data entities:

- **Scalar Setter**: Handles the direct assignment of scalar values or simple fields within an entity. These setters manage basic data types or individual fields and do not involve nested formers or complex structuring.

- **Subform Collection Setter**: Facilitates the management of a collection as a whole by returning a former that provides an interface to configure the entire collection. This setter is beneficial for applying uniform configurations or validations to all elements in a collection, such as a `HashMap` of children.

- **Subform Entry Setter**: This setter allows for the individual formation of elements within a collection. It returns a former for each element, enabling detailed configuration and addition of complex elements within collections, exemplified by managing `Child` entities within a `Parent`'s `HashMap`.

- **Subform Scalar Setter**: Similar to the subform entry setter but designed for scalar fields that have a former implementation. This setter does not collect instances into a collection because there is no collection involved, only a scalar field. It is used when the scalar field itself needs to be configured or modified through its dedicated former.

These setters ensure that developers can precisely and efficiently set properties, manage collections, and configure complex structures within their applications.

## Example : Collection Setter for a Vector

This example demonstrates how to employ the `Former` to configure a `Vec` using a collection setter in a structured manner.

```rust
# #[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
# fn main() {}

# #[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
# fn main()
# {

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithVec
  {
    #[ subform_collection ]
    vec : Vec< &'static str >,
  }

  let instance = StructWithVec::former()
  .vec()
    .add( "apple" )
    .add( "banana" )
    .end()
  .form();

  assert_eq!( instance, StructWithVec { vec: vec![ "apple", "banana" ] } );
  dbg!( instance );

# }
```

Try out `cargo run --example former_collection_vector`.
<br/>
[See code](./examples/former_collection_vector.rs).

## Example : Collection Setter for a Hashmap

This example demonstrates how to effectively employ the `Former` to configure a `HashMap` using a collection setter.

```rust
# #[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
# fn main() {}

# #[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
# fn main()
# {
  use collection_tools::{ HashMap, hmap };

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithMap
  {
    #[ subform_collection ]
    map : HashMap< &'static str, &'static str >,
  }

  let instance = StructWithMap::former()
  .map()
    .add( ( "a", "b" ) )
    .add( ( "c", "d" ) )
    .end()
  .form()
  ;
  assert_eq!( instance, StructWithMap { map : hmap!{ "a" => "b", "c" => "d" } } );
  dbg!( instance );

# }
```

Try out `cargo run --example former_collection_hashmap`.
<br/>
[See code](./examples/former_collection_hashmap.rs).

## Example : Collection Setter for a Hashset

This example demonstrates the use of the `Former` to build a `collection_tools::HashSet` through subforming.

```rust
# #[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
# fn main() {}

# #[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
# fn main()
{
  use collection_tools::{ HashSet, hset };

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithSet
  {
    #[ subform_collection ]
    set : HashSet< &'static str >,
  }

  let instance = StructWithSet::former()
  .set()
    .add( "apple" )
    .add( "banana" )
    .end()
  .form();

  assert_eq!(instance, StructWithSet { set : hset![ "apple", "banana" ] });
  dbg!( instance );

# }
```

Try out `cargo run --example former_collection_hashset`.
<br/>
[See code](./examples/former_collection_hashset.rs).

## Example : Custom Scalar Setter

This example demonstrates the implementation of a scalar setter using the `Former`. Unlike the more complex subform and collection setters shown in previous examples, this example focuses on a straightforward approach to directly set a scalar value within a parent entity. The `Parent` struct manages a `HashMap` of `Child` entities, and the scalar setter is used to set the entire `HashMap` directly.

The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a collection—each child entity in this case.

```rust
# #[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
# fn main() {}

# #[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
# fn main()
# {
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
    // Use `debug` to gennerate sketch of setter.
    #[ scalar( setter = false ) ]
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

# }
```

In this example, the `Parent` struct functions as a collection for multiple `Child` structs, each identified by a unique child name. The `ParentFormer` implements a custom method `child`, which serves as a subformer for adding `Child` instances into the `Parent`.

- **Child Definition**: Each `Child` consists of a `name` and a `description`, and we derive `Former` to enable easy setting of these properties using a builder pattern.
- **Parent Definition**: It holds a collection of `Child` objects in a `HashMap`. The `#[setter(false)]` attribute is used to disable the default setter, and a custom method `child` is defined to facilitate the addition of children with specific attributes.
- **Custom Subformer Integration**: The `child` method in the `ParentFormer` initializes a `ChildFormer` with a closure that integrates the `Child` into the `Parent`'s `child` map upon completion.

Try out `cargo run --example former_custom_scalar_setter`.
<br/>
[See code](./examples/former_custom_scalar_setter.rs).

## Example : Custom Subform Scalar Setter

Implementation of a custom subform scalar setter using the `Former`.

This example focuses on the usage of a subform scalar setter to manage complex scalar types within a parent structure.
Unlike more general subform setters that handle collections, this setter specifically configures scalar fields that have
their own formers, allowing for detailed configuration within a nested builder pattern.

```rust

# #[ cfg( not( all( feature = "enabled", feature = "derive_former" ) ) ) ]
# fn main()
# {}
#
# // Ensures the example only compiles when the appropriate features are enabled.
# #[ cfg( all( feature = "enabled", feature = "derive_former" ) ) ]
# fn main()
# {

  use former::Former;

  // Child struct with Former derived for builder pattern support
  #[ derive( Debug, PartialEq, Former ) ]
  // Optional: Use `#[debug]` to expand and debug generated code.
  // #[debug]
  pub struct Child
  {
    name : String,
    description : String,
  }

  // Parent struct designed to hold a single Child instance using subform scalar
  #[ derive( Debug, PartialEq, Former ) ]
  // Optional: Use `#[debug]` to expand and debug generated code.
  // #[debug]
  pub struct Parent
  {
    // The `subform_scalar` attribute is used to specify that the 'child' field has its own former
    // and can be individually configured via a subform setter. This is not a collection but a single scalar entity.
    #[ subform_scalar( setter = false ) ]
    child : Child,
  }

  /// Extends `ParentFormer` to include a method that initializes and configures a subformer for the 'child' field.
  /// This function demonstrates the dynamic addition of a named child, leveraging a subformer to specify detailed properties.
  impl< Definition > ParentFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = < Parent as former::EntityToStorage >::Storage >,
  {
    #[ inline( always ) ]
    pub fn child( self, name : &str ) -> ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
    {
      self._child_subform_scalar::< ChildFormer< _ >, _, >().name( name )
    }
  }

  // Creating an instance of `Parent` using the builder pattern to configure `Child`
  let ca = Parent::former()
    .child( "echo" ) // starts the configuration of the `child` subformer
    .description( "prints all subjects and properties" ) // sets additional properties for the `Child`
    .end() // finalize the child configuration
    .form(); // finalize the Parent configuration

  dbg!( &ca ); // Outputs the structured data for review
  // Expected output:
  //> Parent {
  //>   child: Child {
  //>       name: "echo",
  //>       description: "prints all subjects and properties",
  //>   },
  //> }

# }
```

## Example : Custom Subform Collection Setter

This example demonstrates the use of collection setters to manage complex nested data structures with the `Former`, focusing on a parent-child relationship structured around a collection `HashMap`. Unlike typical builder patterns that add individual elements using subform setters, this example uses a collection setter to manage the entire collection of children.

The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a collection—each child entity in this case.

```rust
# #[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
# fn main() {}

// Ensure the example only compiles when the appropriate features are enabled.
# #[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
# fn main()
# {
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
    // Use `debug` to gennerate sketch of setter.
    #[ scalar( setter = false ) ]
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

# }
```

Try out `cargo run --example former_custom_subform_collection`.
<br/>
[See code](./examples/former_custom_subform_collection.rs).

## Example : Custom Subform Entry Setter

This example illustrates the implementation of nested builder patterns using the `Former`, emphasizing a parent-child relationship. Here, the `Parent` struct utilizes `ChildFormer` as a custom subformer to dynamically manage its `child` field—a `HashMap`. Each child in the `HashMap` is uniquely identified and configured via the `ChildFormer`.

The `child` function within `ParentFormer` is a custom subform setter that plays a crucial role. It uniquely employs the `ChildFormer` to add and configure children by their names within the parent's builder pattern. This method demonstrates a powerful technique for integrating subformers that manage specific elements of a collection—each child entity in this case.

```rust
# #[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
# fn main() {}

# // Ensure the example only compiles when the appropriate features are enabled.
# #[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
# fn main()
# {
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
    // Use `debug` to gennerate sketch of setter.
    #[ subform_entry( setter = false ) ]
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
      self._child_subform_entry::< ChildFormer< _ >, _, >()
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
# }
```

Try out `cargo run --example former_custom_subform_entry`.
<br/>
[See code](./examples/former_custom_subform_entry.rs).

## General Collection Interface

There are suite of traits designed to abstract and enhance the functionality of collection data structures within the forming process. These traits are integral to managing the complexity of collection operations, such as adding, modifying, and converting between different representations within collections like vectors, hash maps, etc. They are especially useful when used in conjunction with the `collection` attribute in the `former` macro, which automates the implementation of these traits to create robust and flexible builder patterns for complex data structures.

- [`Collection`] - Defines basic functionalities for collections, managing entries and values, establishing the fundamental operations required for any custom collection implementation in forming processes.
- [`EntryToVal`] - Facilitates the conversion of collection entries to their value representations, crucial for operations that treat collection elements more abstractly as values.
- [`ValToEntry`] - Provides the reverse functionality of `EntryToVal`, converting values back into entries, which is essential for operations that require adding or modifying entries in the collection based on value data.
- [`CollectionAdd`] - Adds functionality for inserting entries into a collection, considering collection-specific rules such as duplication handling and order preservation, enhancing the usability of collections in forming scenarios.
- [`CollectionAssign`] - Extends the collection functionality to replace all existing entries with new ones, enabling bulk updates or complete resets of collection contents, which is particularly useful in dynamic data environments.

## Custom Collection Former

Collection interface is defined in the crate and implemented for collections like vectors, hash maps, etc, but if you want to use non-standard collection you can implement collection interface for the collection. This example demonstrate how to do that.

Try out `cargo run --example former_custom_collection`.
<br/>
[See code](./examples/former_custom_collection.rs).

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

## Example : Mutator and Storage Fields

This example illustrates how to use the `FormerMutator` trait for implementing custom mutations
and demonstrates the concept of storage-specific fields in the forming process.

In this example, the fields `a` and `b` are defined only within the storage and used
within the custom mutator to enrich or modify the field `c` of the formed entity. This approach
allows for a richer and more flexible formation logic that can adapt based on the intermediate state
held within the storage.

```rust
# #[ cfg( not( all( feature = "enabled", feature = "derive_former" ) ) ) ]
# fn main() {}

# #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
# fn main()
# {

  use former::Former;

  #[ derive( Debug, PartialEq, Former ) ]
  #[ storage_fields( a : i32, b : Option< String > ) ]
  #[ mutator( custom ) ]
  pub struct Struct1
  {
    c : String,
  }

  // = former mutator

  impl< Context, Formed > former::FormerMutator
  for Struct1FormerDefinitionTypes< Context, Formed >
  {
    /// Mutates the context and storage of the entity just before the formation process completes.
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
  // >  c : "13 - abc",
  // > }

# }
```

Try out `cargo run --example former_custom_mutator`.
<br/>
[See code](./examples/former_custom_mutator.rs).

## Concept of Definitions

Definitions are utilized to encapsulate and manage generic parameters efficiently and avoid passing each parameter individually.

Two key definition Traits:

1. **`FormerDefinitionTypes`**:
   - This trait outlines the essential components involved in the formation process, including the types of storage, the form being created, and the context used. It focuses on the types involved rather than the termination of the formation process.
2. **`FormerDefinition`**:
   - Building upon `FormerDefinitionTypes`, this trait incorporates the `FormingEnd` callback, linking the formation types with a definitive ending. It specifies how the formation process should conclude, which may involve validations, transformations, or integrations into larger structures.
   - The inclusion of the `End` type parameter specifies the end conditions of the formation process, effectively connecting the temporary state held in storage to its ultimate form.

## Overview of Formation Traits

The formation process utilizes several core traits, each serving a specific purpose in the lifecycle of entity creation. These traits ensure that entities are constructed methodically, adhering to a structured pattern that enhances maintainability and scalability. Below is a summary of these key traits:

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

## Example : Custom Definition

Define a custom former definition and custom forming logic, and apply them to a collection.

The example showcases how to accumulate elements into a collection and then transform them into a single result using a custom `FormingEnd` implementation. This pattern is useful for scenarios where the formation process involves aggregation or transformation of input elements into a different type or form.

```rust
# #[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
# fn main() {}

# #[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
# fn main()
# {

  // Define a struct `Sum` that will act as a custom former definition.
  struct Sum;

  // Implement `FormerDefinitionTypes` for `Sum`.
  // This trait defines the types used during the forming process.
  impl former::FormerDefinitionTypes for Sum
  {
    type Storage = Vec<i32>; // Collection for the integers.
    type Formed = i32;       // The final type after forming, which is a single integer.
    type Context = ();       // No additional context is used in this example.
  }

  // Implement `FormerMutator` for `Sum`.
  // This trait could include custom mutation logic applied during the forming process, but it's empty in this example.
  impl former::FormerMutator for Sum
  {
  }

  // Implement `FormerDefinition` for `Sum`.
  // This trait links the custom types to the former.
  impl former::FormerDefinition for Sum
  {
    type Types = Sum;        // Associate the `FormerDefinitionTypes` with `Sum`.
    type End = Sum;          // Use `Sum` itself as the end handler.
    type Storage = Vec<i32>; // Specify the storage type.
    type Formed = i32;       // Specify the final formed type.
    type Context = ();       // Specify the context type, not used here.
  }

  // Implement `FormingEnd` for `Sum`.
  // This trait handles the final step of the forming process.
  impl former::FormingEnd<Sum> for Sum
  {
    fn call
    (
      &self,
      storage: < Sum as former::FormerDefinitionTypes >::Storage,
      _context: Option< < Sum as former::FormerDefinitionTypes >::Context>
    )
    -> < Sum as former::FormerDefinitionTypes >::Formed
    {
      // Sum all integers in the storage vector.
      storage.iter().sum()
    }
  }

  // Use the custom `Former` to sum a list of integers.
  let got = former::CollectionFormer::<i32, Sum>::new(Sum)
  .add( 1 )  // Add an integer to the storage.
  .add( 2 )  // Add another integer.
  .add( 10 ) // Add another integer.
  .form(); // Perform the form operation, which triggers the summing logic.
  let exp = 13; // Expected result after summing 1, 2, and 10.
  assert_eq!(got, exp); // Assert the result is as expected.

  dbg!(got); // Debug print the result to verify the output.
  // > got = 13

# }
```

## Index of Examples

<!-- qqq : for Petro : make this generator working -->
<!--{ examples.index{ description : "short" } }-->
<!--{ examples.index.end }-->

<!-- qqq : for Petro : first write -->

- [Custom Defaults](./examples/former_custom_defaults.rs) - Former allows the specification of custom default values for fields through the `former( default )` attribute.
- [Custom Definition](./examples/former_custom_definition.rs) - Define a custom former definition and custom forming logic, and apply them to a collection.

<!-- qqq : for Petro : implement command `will .mdbook.from.readme` -->

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
