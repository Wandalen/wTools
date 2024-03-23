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

// = descriptor

#[ derive( Debug ) ]
pub struct Struct1FormerDescriptor;

impl Struct1FormerDescriptor
{
  pub fn new() -> Self
  {
    Self
  }
}

impl former::FormerDescriptor
for Struct1FormerDescriptor
{
  type Storage = Struct1FormerStorage;
  type Formed = Struct1;
}

// = storage

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

impl former::Storage
for Struct1FormerStorage
{
  type Descriptor = Struct1FormerDescriptor;
}

impl former::StoragePerform
for Struct1FormerStorage
{

  fn preform( mut self ) -> Struct1
  {

    let int_1 = if self.int_1.is_some()
    {
      self.int_1.take().unwrap()
    }
    else
    {
      let val : i32 = Default::default();
      val
    };

    let string_1 = if self.string_1.is_some()
    {
      self.string_1.take().unwrap()
    }
    else
    {
      let val : String = Default::default();
      val
    };

    let int_optional_1 = if self.int_optional_1.is_some()
    {
      Some( self.int_optional_1.take().unwrap() )
    }
    else
    {
      None
    };

    let string_optional_1 = if self.string_optional_1.is_some()
    {
      Some( self.string_optional_1.take().unwrap() )
    }
    else
    {
      None
    };

    // xxx : Rust failt to use parameter here
    // < < Self as former::Storage >::Descriptor as former::FormerDescriptor >::Formed
    Struct1
    {
      int_1,
      string_1,
      int_optional_1,
      string_optional_1,
    }

  }

}

// = former

#[automatically_derived]
pub struct Struct1Former<FormerContext = Struct1, FormerEnd = former::ReturnStorage>
where FormerEnd : former::FormingEnd<Struct1FormerDescriptor, FormerContext>,
{
  storage : Struct1FormerStorage,
  context : core::option::Option<FormerContext>,
  on_end : core::option::Option<FormerEnd>,
}

#[automatically_derived]
impl<FormerContext, FormerEnd> Struct1Former<FormerContext, FormerEnd>
where FormerEnd : former::FormingEnd<Struct1FormerDescriptor, FormerContext>,
{
  #[doc = r""]
  #[doc = r" Finish setting options and return formed entity."]
  #[doc = r""]
  #[doc =  r" `perform` has no effect on method `form`, but change behavior and returned type of method `perform`."]
  #[doc = r""]
  #[inline(always)]
  pub fn preform(self) -> < Struct1FormerDescriptor as former::FormerDescriptor >::Formed
  {
    < Struct1FormerStorage as former::StoragePerform >::preform( self.storage )
  }

  #[doc = r""]
  #[doc = r" Finish setting options and call perform on formed entity."]
  #[doc = r""]
  #[doc =  r" If `perform` defined then associated method is called and its result returned instead of entity."]
  #[doc =  r" For example `perform()` of structure with : `#[ perform( fn after1() -> &str > )` returns `&str`."]
  #[doc = r""]
  #[inline(always)]
  pub fn perform( self ) -> < Struct1FormerDescriptor as former::FormerDescriptor >::Formed
  {
    let result = self.form();
    return result;
  }

  #[doc = r""]
  #[doc =  r" Begin the process of forming. Expects context of forming to return it after forming."]
  #[doc = r""]
  #[inline(always)]
  pub fn begin(mut storage : core::option::Option<Struct1FormerStorage>, context : core::option::Option<FormerContext>, on_end : FormerEnd,) -> Self
  {
    if storage.is_none()
    {
      storage = Some(::core::default::Default::default());
    }
    Self
    {
      storage: storage.unwrap(),
      context: context,
      on_end: ::core::option::Option::Some(on_end),
    }
  }

  #[doc = r""]
  #[doc =  r" End the process of forming returning original context of forming."]
  #[doc = r""]
  #[inline(always)]
  pub fn form( self ) -> < Struct1FormerDescriptor as former::FormerDescriptor >::Formed
  {
    self.end()
  }

  #[doc = r""]
  #[doc =  r" End the process of forming returning original context of forming."]
  #[doc = r""]
  #[inline(always)]
  pub fn end(mut self) -> < Struct1FormerDescriptor as former::FormerDescriptor >::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    // let storage = self.form();
    on_end.call(self.storage, context)
  }

  #[doc = "Setter for the 'int_1' field."]
  #[inline]
  pub fn int_1<Src>(mut self, src: Src) -> Self
  where Src: ::core::convert::Into<i32>,
  {
    debug_assert!(self.storage.int_1.is_none());
    self.storage.int_1 = ::core::option::Option::Some(src.into());
    self
  }

  #[doc = "Setter for the 'string_1' field."]
  #[inline]
  pub fn string_1<Src>(mut self, src: Src) -> Self
  where Src: ::core::convert::Into<String>,
  {
    debug_assert!(self.storage.string_1.is_none());
    self.storage.string_1 = ::core::option::Option::Some(src.into());
    self
  }

  #[doc = "Setter for the 'int_optional_1' field."]
  #[inline]
  pub fn int_optional_1<Src>(mut self, src: Src) -> Self
  where Src: ::core::convert::Into<i32>,
  {
    debug_assert!(self.storage.int_optional_1.is_none());
    self.storage.int_optional_1 = ::core::option::Option::Some(src.into());
    self
  }

  #[doc = "Setter for the 'string_optional_1' field."]
  #[inline]
  pub fn string_optional_1<Src>(mut self, src: Src) -> Self
  where Src: ::core::convert::Into<String>,
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
