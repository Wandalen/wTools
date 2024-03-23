#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq ) ]
// #[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ debug ]
pub struct Struct1
{
  pub int_1 : i32,
  string_1 : String,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

// = formed

#[automatically_derived]
impl Struct1
{
  #[doc = r""]
  #[doc = r" Make former, variation of builder pattern to form structure defining values of fields step by step."]
  #[doc = r""]
  #[inline(always)]
  pub fn former() -> Struct1Former<(), former::ReturnStorage>
  {
    Struct1Former::new()
  }
}

#[derive(Debug)]
pub struct Struct1FormerDescriptor;

impl Struct1FormerDescriptor
{
  pub fn new() -> Self
  {
    Self
  }
}

impl former::FormerDescriptor for Struct1FormerDescriptor
{
  type Storage = Struct1FormerStorage;
  type Formed = Struct1;
}

#[doc = "Container of a corresponding former."]
pub struct Struct1FormerStorage
{
  #[doc = r" A field"]
  pub int_1 : ::core::option::Option<i32>,
  #[doc = r" A field"]
  pub string_1 : ::core::option::Option<String>,
  #[doc = r" A field"]
  pub int_optional_1 : core::option::Option<i32>,
  #[doc = r" A field"]
  pub string_optional_1 : Option<String>,
}

impl ::core::default::Default for Struct1FormerStorage
{
  #[inline(always)]
  fn default() -> Self
  {
    Self
    {
      int_1 : ::core::option::Option::None,
      string_1 : ::core::option::Option::None,
      int_optional_1 : ::core::option::Option::None,
      string_optional_1 : ::core::option::Option::None,
    }
  }
}

impl former::Storage for Struct1FormerStorage
{
  type Descriptor = Struct1FormerDescriptor;
}

impl former::StoragePerform for Struct1FormerStorage
{
  fn preform(mut self) -> Struct1
  {
    Struct1
    {
      int_1 : self.int_1.take().or_else(|| Some(::core::marker::PhantomData::<i32>::maybe_default())).unwrap(),
      string_1 : self.string_1.take().or_else(|| Some(::core::marker::PhantomData::<String>::maybe_default())).unwrap(),
      int_optional_1 : self.int_optional_1,
      string_optional_1 : self.string_optional_1,
    }
  }
}

trait MaybeDefault<T>
{
  fn maybe_default() -> T;
}

impl MaybeDefault<i32> for ::core::marker::PhantomData<i32>
where i32 : ::core::default::Default,
{
  fn maybe_default() -> i32
  {
    i32::default()
  }
}

impl MaybeDefault<String> for ::core::marker::PhantomData<String>
where String : ::core::default::Default,
{
  fn maybe_default() -> String
  {
    String::default()
  }
}

#[doc =
" Object to form [Struct1]. If field's values is not set then default value of the field is set.\n\nFor specifying custom default value use attribute `default`. For example:\n```\n\nuse former::Former;\n#[ derive( Former ) ]\npub struct Struct1\n{\n  #[default( 31 ) ]\n  field1 : i32,\n}\n\n```\n"]
#[automatically_derived]
pub struct Struct1Former<__FormerContext = Struct1, __FormerEnd = former::ReturnStorage>
where __FormerEnd : former::FormingEnd<Struct1FormerDescriptor, __FormerContext>,
{
  storage : Struct1FormerStorage,
  context : core::option::Option<__FormerContext>,
  on_end : core::option::Option<__FormerEnd>,
}

#[automatically_derived]
impl<__FormerContext, __FormerEnd> Struct1Former<__FormerContext, __FormerEnd>
where __FormerEnd : former::FormingEnd<Struct1FormerDescriptor, __FormerContext>,
{
  #[doc = r""]
  #[doc = r" Finish setting options and return formed entity."]
  #[doc = r""]
  #[inline(always)]
  pub fn preform(self) -> <Struct1FormerDescriptor as former::FormerDescriptor>::Formed
  {
    <Struct1FormerStorage as former::StoragePerform>::preform(self.storage)
  }

  #[doc = r""]
  #[doc = r" Finish setting options and call perform on formed entity."]
  #[doc = r""]
  #[doc = r" If `perform` defined then associated method is called and its result returned instead of entity."]
  #[doc = r" For example `perform()` of structure with : `#[ perform( fn after1() -> &str > )` returns `&str`."]
  #[doc = r""]
  #[inline(always)]
  pub fn perform(self) -> <Struct1FormerDescriptor as former::FormerDescriptor>::Formed
  {
    let result = self.form();
    return result;
  }

  #[doc = r""]
  #[doc = r" Begin the process of forming. Expects context of forming to return it after forming."]
  #[doc = r""]
  #[inline(always)]
  pub fn begin(mut storage : core::option::Option<Struct1FormerStorage>, context : core::option::Option<__FormerContext>, on_end : __FormerEnd,) -> Self
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

  #[doc = r""]
  #[doc = r" End the process of forming returning original context of forming."]
  #[doc = r""]
  #[inline(always)]
  pub fn form(self) -> <Struct1FormerDescriptor as former::FormerDescriptor>::Formed
  {
    self.end()
  }

  #[doc = r""]
  #[doc = r" End the process of forming returning original context of forming."]
  #[doc = r""]
  #[inline(always)]
  pub fn end(mut self) -> <Struct1FormerDescriptor as former::FormerDescriptor>::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    on_end.call(self.storage, context)
  }

  #[doc = "Setter for the 'int_1' field."]
  #[inline]
  pub fn int_1<Src>(mut self, src: Src) -> Self
  where Src : ::core::convert::Into<i32>,
  {
    debug_assert!(self.storage.int_1.is_none());
    self.storage.int_1 = ::core::option::Option::Some(src.into());
    self
  }

  #[doc = "Setter for the 'string_1' field."]
  #[inline]
  pub fn string_1<Src>(mut self, src: Src) -> Self
  where Src : ::core::convert::Into<String>,
  {
    debug_assert!(self.storage.string_1.is_none());
    self.storage.string_1 = ::core::option::Option::Some(src.into());
    self
  }

  #[doc = "Setter for the 'int_optional_1' field."]
  #[inline]
  pub fn int_optional_1<Src>(mut self, src: Src) -> Self
  where Src : ::core::convert::Into<i32>,
  {
    debug_assert!(self.storage.int_optional_1.is_none());
    self.storage.int_optional_1 = ::core::option::Option::Some(src.into());
    self
  }

  #[doc = "Setter for the 'string_optional_1' field."]
  #[inline]
  pub fn string_optional_1<Src>(mut self, src: Src) -> Self
  where Src : ::core::convert::Into<String>,
  {
    debug_assert!(self.storage.string_optional_1.is_none());
    self.storage.string_optional_1 = ::core::option::Option::Some(src.into());
    self
  }
}

#[automatically_derived]
impl Struct1Former<(), former::ReturnStorage>
{
  #[doc = r""]
  #[doc = r" Construct new instance of former with default parameters."]
  #[doc = r""]
  #[inline(always)]
  pub fn new() -> Self
  {
    Self::begin(None, None, former::ReturnStorage,)
  }
}

//

//  include!( "./only_test/primitives.rs" );
