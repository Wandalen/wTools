use super::*;


pub trait FilterCol : fmt::Debug
{
  fn filter_col( &self, key : &str ) -> bool;
}

#[ derive( Debug, Default, PartialEq, Clone, Copy ) ]
pub struct All;

impl All
{
  pub fn instance() -> & 'static dyn FilterCol
  {
    static INSTANCE : All = All;
    &INSTANCE
  }
}

impl Default for &'static dyn FilterCol
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    All::instance()
  }
}

impl FilterCol for All
{
  #[ inline( always ) ]
  fn filter_col( &self, _key : &str ) -> bool
  {
    true
  }
}

#[ derive( Default ) ]
// #[ derive( former::Former ) ] // xxx : qqq : uncomment and fix problem with lifetime
// #[ derive( former::Former ) ] #[ debug ]
pub struct Styles< 'callback >
{

  // pub output_format : &'static dyn AsRef< str >,
  pub filter : &'callback dyn FilterCol,

}

// === begin_coercing of generated

#[automatically_derived]
impl< 'callback > Styles< 'callback > where
{
  #[doc = r""]
  #[doc = r" Provides a mechanism to initiate the formation process with a default completion behavior."]
  #[doc = r""]
  #[inline(always)]
  pub fn former() -> StylesFormer< 'callback, StylesFormerDefinition< 'callback, (), Styles< 'callback >, former::ReturnPreformed > >
  {
    StylesFormer::< 'callback, StylesFormerDefinition< 'callback, (), Styles< 'callback >, former::ReturnPreformed > >::new_coercing(former::ReturnPreformed)
  }
}

impl< 'callback, Definition > former::EntityToFormer< Definition > for Styles< 'callback >
where
  Definition : former::FormerDefinition< Storage = StylesFormerStorage< 'callback > >,
{
  type Former = StylesFormer< 'callback, Definition >;
}

impl< 'callback > former::EntityToStorage for Styles< 'callback >
where
{
  type Storage = StylesFormerStorage< 'callback >;
}

impl< 'callback, __Context, __Formed, __End > former::EntityToDefinition< __Context, __Formed, __End > for Styles< 'callback >
where
  __End : former::FormingEnd< StylesFormerDefinitionTypes< 'callback, __Context, __Formed > >,
{
  type Definition = StylesFormerDefinition< 'callback, __Context, __Formed, __End >;
  type Types = StylesFormerDefinitionTypes< 'callback, __Context, __Formed >;
}

impl< 'callback, __Context, __Formed > former::EntityToDefinitionTypes< __Context, __Formed > for Styles< 'callback >
where
{
  type Types = StylesFormerDefinitionTypes< 'callback, __Context, __Formed >;
}

#[doc = r" Defines the generic parameters for formation behavior including context, form, and end conditions."]
#[derive(Debug)]
pub struct StylesFormerDefinitionTypes< 'callback, __Context = (), __Formed = Styles< 'callback > >
where
{
  _phantom: ::core::marker::PhantomData< ( & 'callback (), * const __Context, * const __Formed ) >,
}

impl< 'callback, __Context, __Formed > ::core::default::Default for StylesFormerDefinitionTypes< 'callback, __Context, __Formed >
where
{
  fn default() -> Self
  {
    Self { _phantom: ::core::marker::PhantomData }
  }
}

impl< 'callback, __Context, __Formed > former::FormerDefinitionTypes for StylesFormerDefinitionTypes< 'callback, __Context, __Formed >
where
{
  type Storage = StylesFormerStorage< 'callback >;
  type Formed = __Formed;
  type Context = __Context;
}

#[doc = r" Holds the definition types used during the formation process."]
#[derive(Debug)]
pub struct StylesFormerDefinition< 'callback, __Context = (), __Formed = Styles< 'callback >, __End = former::ReturnPreformed >
where
{
  _phantom: ::core::marker::PhantomData< ( & 'callback (), * const __Context, * const __Formed, * const __End ) >,
}

impl< 'callback, __Context, __Formed, __End > ::core::default::Default for StylesFormerDefinition< 'callback, __Context, __Formed, __End >
where
{
  fn default() -> Self
  {
    Self { _phantom: ::core::marker::PhantomData }
  }
}

impl< 'callback, __Context, __Formed, __End > former::FormerDefinition for StylesFormerDefinition< 'callback, __Context, __Formed, __End >
where
  __End : former::FormingEnd< StylesFormerDefinitionTypes< 'callback, __Context, __Formed > >,
{
  type Types = StylesFormerDefinitionTypes< 'callback, __Context, __Formed >;
  type End = __End;
  type Storage = StylesFormerStorage< 'callback >;
  type Formed = __Formed;
  type Context = __Context;
}

impl< 'callback, __Context, __Formed > former::FormerMutator for StylesFormerDefinitionTypes< 'callback, __Context, __Formed >
where
{}

#[doc = "Stores potential values for fields during the formation process."]
#[allow(explicit_outlives_requirements)]
pub struct StylesFormerStorage< 'callback >
where
{
  #[doc = r" A field"]
  pub filter: ::core::option::Option< & 'callback dyn FilterCol >,
}

impl< 'callback > ::core::default::Default for StylesFormerStorage< 'callback >
where
{
  #[inline(always)]
  fn default() -> Self
  {
    Self { filter: ::core::option::Option::None }
  }
}

impl< 'callback > former::Storage for StylesFormerStorage< 'callback >
where
{
  type Preformed = Styles< 'callback >;
}

impl< 'callback > former::StoragePreform for StylesFormerStorage< 'callback >
where
{
  fn preform(mut self) -> Self::Preformed
  {
    let filter = if self.filter.is_some()
    {
      self.filter.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault<T>
        {
          fn maybe_default(self: &Self) -> T
          {
            panic!("Field 'filter' isn't initialized")
          }
        }
        impl<T> MaybeDefault<T> for &::core::marker::PhantomData<T>
        {}
        impl<T> MaybeDefault<T> for ::core::marker::PhantomData<T>
        where
          T: ::core::default::Default,
        {
          fn maybe_default(self: &Self) -> T
          {
            T::default()
          }
        }
        (&::core::marker::PhantomData::<&'callback dyn FilterCol>).maybe_default()
      }
    };
    let result = Styles::< 'callback > { filter };
    return result;
  }
}

#[doc = "\nStructure to form [Styles]. Represents a forming entity designed to construct objects through a builder pattern.\n\nThis structure holds temporary storage and context during the formation process and\nutilizes a defined end strategy to finalize the object creation.\n"]
pub struct StylesFormer< 'callback, Definition = StylesFormerDefinition< 'callback, (), Styles< 'callback >, former::ReturnPreformed > >
where
  Definition: former::FormerDefinition< Storage = StylesFormerStorage< 'callback > >,
  Definition::Types: former::FormerDefinitionTypes< Storage = StylesFormerStorage< 'callback > >,
{
  #[doc = r" Temporary storage for all fields during the formation process. It contains"]
  #[doc = r"   partial data that progressively builds up to the final object."]
  pub storage: Definition::Storage,
  #[doc = r" An optional context providing additional data or state necessary for custom"]
  #[doc = r"   formation logic or to facilitate this former's role as a subformer within another former."]
  pub context: ::core::option::Option< Definition::Context >,
  #[doc = r" An optional closure or handler that is invoked to transform the accumulated"]
  #[doc = r"   temporary storage into the final object structure once formation is complete."]
  pub on_end: ::core::option::Option< Definition::End >,
}

#[automatically_derived]
impl< 'callback, Definition > StylesFormer< 'callback, Definition >
where
  Definition: former::FormerDefinition< Storage = StylesFormerStorage< 'callback > >,
  Definition::Types: former::FormerDefinitionTypes< Storage = StylesFormerStorage< 'callback > >,
{
  #[doc = r""]
  #[doc = r" Initializes a former with an end condition and default storage."]
  #[doc = r""]
  #[inline(always)]
  pub fn new(on_end: Definition::End) -> Self
  {
    Self::begin_coercing(::core::option::Option::None, ::core::option::Option::None, on_end)
  }

  #[doc = r""]
  #[doc = r" Initializes a former with a coercible end condition."]
  #[doc = r""]
  #[inline(always)]
  pub fn new_coercing<IntoEnd>(end: IntoEnd) -> Self
  where
    IntoEnd: ::core::convert::Into<Definition::End>,
  {
    Self::begin_coercing(::core::option::Option::None, ::core::option::Option::None, end)
  }

  #[doc = r""]
  #[doc = r" Begins the formation process with specified context and termination logic."]
  #[doc = r""]
  #[inline(always)]
  pub fn begin(
    mut storage: ::core::option::Option<Definition::Storage>,
    context: ::core::option::Option<Definition::Context>,
    on_end: <Definition as former::FormerDefinition>::End,
  ) -> Self
  {
    if storage.is_none()
    {
      storage = ::core::option::Option::Some(::core::default::Default::default());
    }
    Self
    {
      storage: storage.unwrap(),
      context: context,
      on_end: ::core::option::Option::Some(on_end),
    }
  }

  #[doc = r""]
  #[doc = r" Starts the formation process with coercible end condition and optional initial values."]
  #[doc = r""]
  #[inline(always)]
  pub fn begin_coercing<IntoEnd>(
    mut storage: ::core::option::Option<Definition::Storage>,
    context: ::core::option::Option<Definition::Context>,
    on_end: IntoEnd,
  ) -> Self
  where
    IntoEnd: ::core::convert::Into<<Definition as former::FormerDefinition>::End>,
  {
    if storage.is_none()
    {
      storage = ::core::option::Option::Some(::core::default::Default::default());
    }
    Self
    {
      storage: storage.unwrap(),
      context: context,
      on_end: ::core::option::Option::Some(::core::convert::Into::into(on_end)),
    }
  }

  #[doc = r""]
  #[doc = r" Wrapper for `end` to align with common builder pattern terminologies."]
  #[doc = r""]
  #[inline(always)]
  pub fn form(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
  {
    self.end()
  }

  #[doc = r""]
  #[doc = r" Completes the formation and returns the formed object."]
  #[doc = r""]
  #[inline(always)]
  pub fn end(mut self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let mut context = self.context.take();
    <Definition::Types as former::FormerMutator>::form_mutation(&mut self.storage, &mut context);
    former::FormingEnd::<Definition::Types>::call(&on_end, self.storage, context)
  }

  #[doc = "Scalar setter for the 'filter' field."]
  #[inline]
  pub fn filter<Src>(mut self, src: Src) -> Self
  where
    Src: ::core::convert::Into<& 'callback dyn FilterCol>,
  {
    debug_assert!(self.storage.filter.is_none());
    self.storage.filter = ::core::option::Option::Some(::core::convert::Into::into(src));
    self
  }
}

impl< 'callback, Definition > StylesFormer< 'callback, Definition >
where
  Definition: former::FormerDefinition< Storage = StylesFormerStorage< 'callback >, Formed = Styles< 'callback > >,
  Definition::Types: former::FormerDefinitionTypes< Storage = StylesFormerStorage< 'callback >, Formed = Styles< 'callback > >,
  Definition: former::FormerDefinition< Storage = StylesFormerStorage< 'callback > >,
  Definition::Types: former::FormerDefinitionTypes< Storage = StylesFormerStorage< 'callback > >,
{
  #[doc = r" Executes the transformation from the former's storage state to the preformed object as specified by the definition."]
  pub fn preform(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
  {
    former::StoragePreform::preform(self.storage)
  }
}

#[automatically_derived]
impl< 'callback, Definition > StylesFormer< 'callback, Definition >
where
  Definition: former::FormerDefinition< Storage = StylesFormerStorage< 'callback >, Formed = Styles< 'callback > >,
  Definition::Types: former::FormerDefinitionTypes< Storage = StylesFormerStorage< 'callback >, Formed = Styles< 'callback > >,
{
  #[doc = r""]
  #[doc = r" Finish setting options and call perform on formed entity."]
  #[doc = r""]
  #[doc = r" If `perform` defined then associated method is called and its result returned instead of entity."]
  #[doc = r" For example `perform()` of structure with : `#[ perform( fn after1() -> &str > )` returns `&str`."]
  #[doc = r""]
  #[inline(always)]
  pub fn perform(self) -> Definition::Formed
  {
    let result = self.form();
    return result;
  }
}

impl< 'callback, Definition > former::FormerBegin< Definition > for StylesFormer< 'callback, Definition >
where
  Definition: former::FormerDefinition< Storage = StylesFormerStorage< 'callback > >,
{
  #[inline(always)]
  fn former_begin(
    storage: ::core::option::Option<Definition::Storage>,
    context: ::core::option::Option<Definition::Context>,
    on_end: Definition::End,
  ) -> Self
  {
    debug_assert!(storage.is_none());
    Self::begin(::core::option::Option::None, context, on_end)
  }
}

#[doc = r" Provides a specialized former for structure using predefined settings for superformer and end conditions."]
#[doc = r""]
#[doc = r" This type alias configures former of the structure with a specific definition to streamline its usage in broader contexts,"]
#[doc = r" especially where structure needs to be integrated into larger structures with a clear termination condition."]
pub type StylesAsSubformer< 'callback, __Superformer, __End > = StylesFormer< 'callback, StylesFormerDefinition< 'callback, __Superformer, __Superformer, __End > >;

#[doc = "\nRepresents an end condition for former of [`$Styles`], tying the lifecycle of forming processes to a broader context.\n\nThis trait is intended for use with subformer alias, ensuring that end conditions are met according to the\nspecific needs of the broader forming context. It mandates the implementation of `former::FormingEnd`.\n    "]
pub trait StylesAsSubformerEnd< 'callback, SuperFormer >
where
  Self: former::FormingEnd< StylesFormerDefinitionTypes< 'callback, SuperFormer, SuperFormer > >,
{}
impl< 'callback, SuperFormer, __T > StylesAsSubformerEnd< 'callback, SuperFormer > for __T
where
  Self: former::FormingEnd< StylesFormerDefinitionTypes< 'callback, SuperFormer, SuperFormer > >,
{}

// === end of generated

#[ test ]
fn basic()
{
}