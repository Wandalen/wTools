#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

// xxx : make it working

/// Parameter description.
#[ allow( explicit_outlives_requirements ) ]
// #[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ debug ]
#[ derive( Debug, PartialEq ) ]
pub struct Child< 'child, T >
where
  T : 'child + ?Sized,
{
  name : String,
  arg : &'child T,
}

// == begin of generated

  #[automatically_derived] impl < 'child, T, > Child < 'child, T, > where T :
  'child + ? Sized,
  {
      #[doc = r""]
      #[doc =
      r" Make former, variation of builder pattern to form structure defining values of fields step by step."]
      #[doc = r""] #[inline(always)] pub fn former() -> ChildFormer < 'child, T,
      ChildFormerDefinition < 'child, T, (), Child < 'child, T, > , former ::
      ReturnPreformed > >
      {
          ChildFormer :: < 'child, T, ChildFormerDefinition < 'child, T, (),
          Child < 'child, T, > , former :: ReturnPreformed > > ::
          new_coercing(former :: ReturnPreformed)
      }
  } impl < 'child, T, Definition > former :: EntityToFormer < Definition > for
  Child < 'child, T, > where Definition : former :: FormerDefinition < Storage =
  ChildFormerStorage < 'child, T, > > , T : 'child + ? Sized,
  { type Former = ChildFormer < 'child, T, Definition > ; } impl < 'child, T, >
  former :: EntityToStorage for Child < 'child, T, > where T : 'child + ? Sized,
  { type Storage = ChildFormerStorage < 'child, T, > ; } impl < 'child, T,
  __Context, __Formed, __End > former :: EntityToDefinition < __Context,
  __Formed, __End > for Child < 'child, T, > where __End : former :: FormingEnd
  < ChildFormerDefinitionTypes < 'child, T, __Context, __Formed > > , T : 'child
  + ? Sized,
  {
      type Definition = ChildFormerDefinition < 'child, T, __Context, __Formed,
      __End > ;
  }

  #[derive(Debug)]
  pub struct ChildFormerDefinitionTypes < 'child, T, __Context = (), __Formed = Child < 'child, T, > , >
  where
    T : 'child + ?Sized,
  {
    _phantom : core :: marker :: PhantomData <
    (& 'child (), *const T, __Context, __Formed) > ,
  }

  impl < 'child, T, __Context, __Formed, > :: core :: default :: Default for
  ChildFormerDefinitionTypes < 'child, T, __Context, __Formed, > where T :
  'child + ? Sized,
  {
      fn default() -> Self
      { Self { _phantom : core :: marker :: PhantomData, } }
  }

  impl < 'child, T, __Context, __Formed, > former :: FormerDefinitionTypes for
  ChildFormerDefinitionTypes < 'child, T, __Context, __Formed, > where T :
  'child + ? Sized,
  {
      type Storage = ChildFormerStorage < 'child, T, > ;
      type Formed = __Formed;
      type Context = __Context;
  }

  #[derive(Debug)] pub struct ChildFormerDefinition
  < 'child, T, __Context = (), __Formed = Child < 'child, T, > , __End = former :: ReturnPreformed, >
  where
    T : 'child + ? Sized,
  {
      _phantom : core :: marker :: PhantomData < (& 'child (), *const T, __Context, __Formed, __End) > ,
  }

  impl < 'child, T, __Context, __Formed, __End, > :: core :: default ::
  Default for ChildFormerDefinition < 'child, T, __Context, __Formed, __End, >
  where T : 'child + ? Sized,
  {
      fn default() -> Self
      { Self { _phantom : core :: marker :: PhantomData, } }
  }

  impl < 'child, T, __Context, __Formed, __End, > former :: FormerDefinition
  for ChildFormerDefinition < 'child, T, __Context, __Formed, __End, >
  where
  __End : former :: FormingEnd < ChildFormerDefinitionTypes < 'child, T,
  __Context, __Formed, > > , T : 'child + ? Sized,
  {
      type Types = ChildFormerDefinitionTypes < 'child, T, __Context, __Formed, > ;
      type End = __End;
      type Storage = ChildFormerStorage < 'child, T, > ;
      type Formed = __Formed;
      type Context = __Context;
  }

  #[doc = "Container of a corresponding former."]
  #[ allow( explicit_outlives_requirements ) ]
  pub struct ChildFormerStorage < 'child, T, >
  where
    T : 'child + ? Sized,
  {
      #[doc = r" A field"]
      pub name : :: core :: option :: Option < String > ,
      #[doc = r" A field"]
      pub arg : :: core :: option :: Option < & 'child T >
      ,
  }

  impl < 'child, T, > :: core :: default :: Default for ChildFormerStorage <
  'child, T, > where T : 'child + ? Sized,
  {
      #[inline(always)] fn default() -> Self
      {
          Self
          {
              name : :: core :: option :: Option :: None, arg : :: core ::
              option :: Option :: None,
          }
      }
  } impl < 'child, T, > former :: Storage for ChildFormerStorage < 'child, T, >
  where T : 'child + ? Sized, { type Formed = Child < 'child, T, > ; } impl <
  'child, T, > former :: StoragePreform for ChildFormerStorage < 'child, T, >
  where T : 'child + ? Sized,
  {
      type Preformed = Child < 'child, T, > ; fn preform(mut self) -> Self ::
      Preformed
      {
          let name = if self.name.is_some() { self.name.take().unwrap() } else
          {
              {
                  trait MaybeDefault < T >
                  {
                      fn maybe_default(self : & Self) -> T
                      { panic! ("Field 'name' isn't initialized") }
                  } impl < T > MaybeDefault < T > for & :: core :: marker ::
                  PhantomData < T > {} impl < T > MaybeDefault < T > for :: core
                  :: marker :: PhantomData < T > where T : :: core :: default ::
                  Default,
                  { fn maybe_default(self : & Self) -> T { T :: default() } }
                  (& :: core :: marker :: PhantomData :: < String
                  >).maybe_default()
              }
          }; let arg = if self.arg.is_some() { self.arg.take().unwrap() } else
          {
              {
                  trait MaybeDefault < T >
                  {
                      fn maybe_default(self : & Self) -> T
                      { panic! ("Field 'arg' isn't initialized") }
                  } impl < T > MaybeDefault < T > for & :: core :: marker ::
                  PhantomData < T > {} impl < T > MaybeDefault < T > for :: core
                  :: marker :: PhantomData < T > where T : :: core :: default ::
                  Default,
                  { fn maybe_default(self : & Self) -> T { T :: default() } }
                  (& :: core :: marker :: PhantomData :: < & 'child T
                  >).maybe_default()
              }
          }; let result = Child :: < 'child, T, > { name, arg, }; return result;
      }
  }
  #[doc =
  " Object to form [Child]. If field's values is not set then default value of the field is set.\n\nFor specifying custom default value use attribute `default`. For example:\n```\n\nuse former::Former;\n#[ derive( Former ) ]\npub struct Struct1\n{\n  #[default( 31 ) ]\n  field1 : i32,\n}\n\n```\n"]
  pub struct ChildFormer < 'child, T, Definition = ChildFormerDefinition <
  'child, T, (), Child < 'child, T, > , former :: ReturnPreformed > , > where T
  : 'child + ? Sized, Definition : former :: FormerDefinition, Definition ::
  Types : former :: FormerDefinitionTypes < Storage = ChildFormerStorage <
  'child, T, > > ,
  {
      storage : < Definition :: Types as former :: FormerDefinitionTypes > ::
      Storage, context : core :: option :: Option < < Definition :: Types as
      former :: FormerDefinitionTypes > :: Context > , on_end : core :: option
      :: Option < Definition :: End > ,
  } #[automatically_derived] impl < 'child, T, Definition, > ChildFormer <
  'child, T, Definition, > where T : 'child + ? Sized, Definition : former ::
  FormerDefinition, Definition :: Types : former :: FormerDefinitionTypes <
  Storage = ChildFormerStorage < 'child, T, > > ,
  {
      #[doc = r""]
      #[doc = r" Construct new instance of former with default parameters."]
      #[doc = r""] #[inline(always)] pub fn new(on_end : Definition :: End) ->
      Self { Self :: begin_coercing(None, None, on_end) } #[doc = r""]
      #[doc = r" Construct new instance of former with default parameters."]
      #[doc = r""] #[inline(always)] pub fn new_coercing < IntoEnd >
      (end : IntoEnd) -> Self where IntoEnd : Into < Definition :: End > ,
      { Self :: begin_coercing(None, None, end,) } #[doc = r""]
      #[doc =
      r" Begin the process of forming. Expects context of forming to return it after forming."]
      #[doc = r""] #[inline(always)] pub fn
      begin(mut storage : core :: option :: Option < < Definition :: Types as
      former :: FormerDefinitionTypes > :: Storage > , context : core :: option
      :: Option < < Definition :: Types as former :: FormerDefinitionTypes > ::
      Context > , on_end : < Definition as former :: FormerDefinition > :: End,)
      -> Self
      {
          if storage.is_none()
          { storage = Some(:: core :: default :: Default :: default()); } Self
          {
              storage : storage.unwrap(), context : context, on_end : :: core ::
              option :: Option :: Some(on_end),
          }
      } #[doc = r""]
      #[doc =
      r" Begin the process of forming. Expects context of forming to return it after forming."]
      #[doc = r""] #[inline(always)] pub fn begin_coercing < IntoEnd >
      (mut storage : core :: option :: Option < < Definition :: Types as former
      :: FormerDefinitionTypes > :: Storage > , context : core :: option ::
      Option < < Definition :: Types as former :: FormerDefinitionTypes > ::
      Context > , on_end : IntoEnd,) -> Self where IntoEnd : :: core :: convert
      :: Into < < Definition as former :: FormerDefinition > :: End > ,
      {
          if storage.is_none()
          { storage = Some(:: core :: default :: Default :: default()); } Self
          {
              storage : storage.unwrap(), context : context, on_end : :: core ::
              option :: Option ::
              Some(:: core :: convert :: Into :: into(on_end)),
          }
      } #[doc = r""]
      #[doc =
      r" End the process of forming returning original context of forming."]
      #[doc = r""] #[inline(always)] pub fn form(self) -> < Definition :: Types
      as former :: FormerDefinitionTypes > :: Formed { self.end() } #[doc = r""]
      #[doc =
      r" End the process of forming returning original context of forming."]
      #[doc = r""] #[inline(always)] pub fn end(mut self) -> < Definition ::
      Types as former :: FormerDefinitionTypes > :: Formed
      {
          let on_end = self.on_end.take().unwrap(); let context =
          self.context.take(); former :: FormingEnd :: < Definition :: Types >
          :: call(& on_end, self.storage, context)
      } #[doc = "Setter for the 'name' field."] #[inline] pub fn name < Src >
      (mut self, src : Src) -> Self where Src : :: core :: convert :: Into <
      String > ,
      {
          debug_assert! (self.storage.name.is_none()); self.storage.name = ::
          core :: option :: Option ::
          Some(:: core :: convert :: Into :: into(src)); self
      } #[doc = "Setter for the 'arg' field."] #[inline] pub fn arg < Src >
      (mut self, src : Src) -> Self where Src : :: core :: convert :: Into < &
      'child T > ,
      {
          debug_assert! (self.storage.arg.is_none()); self.storage.arg = :: core
          :: option :: Option :: Some(:: core :: convert :: Into :: into(src));
          self
      }
  } impl < 'child, T, Definition, > ChildFormer < 'child, T, Definition, > where
  Definition :: Types : former :: FormerDefinitionTypes < Storage =
  ChildFormerStorage < 'child, T, > , Formed = Child < 'child, T, > > , T :
  'child + ? Sized, Definition : former :: FormerDefinition, Definition :: Types
  : former :: FormerDefinitionTypes < Storage = ChildFormerStorage < 'child, T,
  > > ,
  {
      pub fn preform(self) -> < Definition :: Types as former ::
      FormerDefinitionTypes > :: Formed
      { former :: StoragePreform :: preform(self.storage) }
  } #[automatically_derived] impl < 'child, T, Definition, > ChildFormer <
  'child, T, Definition, > where T : 'child + ? Sized, Definition : former ::
  FormerDefinition, Definition :: Types : former :: FormerDefinitionTypes <
  Storage = ChildFormerStorage < 'child, T, > , Formed = Child < 'child, T, > >
  ,
  {
      #[doc = r""]
      #[doc = r" Finish setting options and call perform on formed entity."]
      #[doc = r""]
      #[doc =
      r" If `perform` defined then associated method is called and its result returned instead of entity."]
      #[doc =
      r" For example `perform()` of structure with : `#[ perform( fn after1() -> &str > )` returns `&str`."]
      #[doc = r""] #[inline(always)] pub fn perform(self) -> < Definition ::
      Types as former :: FormerDefinitionTypes > :: Formed
      { let result = self.form(); return result; }
  } impl < 'child, T, Definition > former :: FormerBegin < Definition > for
  ChildFormer < 'child, T, Definition, > where Definition : former ::
  FormerDefinition < Storage = ChildFormerStorage < 'child, T, > > , T : 'child
  + ? Sized,
  {
      #[inline(always)] fn
      former_begin(storage : core :: option :: Option < Definition :: Storage >
      , context : core :: option :: Option < Definition :: Context > , on_end :
      Definition :: End,) -> Self
      {
          debug_assert! (storage.is_none()); Self ::
          begin(None, context, on_end)
      }
  }
  #[doc =
  r" Use as subformer of a field during process of forming of super structure."]
  pub type ChildAsSubformer < 'child, T, __Superformer, __End > = ChildFormer <
  'child, T, ChildFormerDefinition < 'child, T, __Superformer, __Superformer,
  __End, > , > ;
  #[doc =
  "Alias for trait former::FormingEnd with context and formed the same type and definition of structure [`$(stru)`]. Use as subformer end of a field during process of forming of super structure."]
  pub trait ChildAsSubformerEnd < 'child, T, SuperFormer > where T : 'child + ?
  Sized, Self : former :: FormingEnd < ChildFormerDefinitionTypes < 'child, T,
  SuperFormer, SuperFormer > , > , {} impl < 'child, T, SuperFormer, __T >
  ChildAsSubformerEnd < 'child, T, SuperFormer > for __T where T : 'child + ?
  Sized, Self : former :: FormingEnd < ChildFormerDefinitionTypes < 'child, T,
  SuperFormer, SuperFormer > , > , {}

// == end of generated

// xxx : uncomment
// #[ test ]
// fn basic()
// {
//   let got = Child::< 'static, str >::former().name( "abc" ).arg( "arg1" ).end();
//   let exp = Child::< 'static, str >{ name : "abc".into(), arg : "arg1" };
//   // a_id!( got, exp );
// }
