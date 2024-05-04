#[ allow( unused_imports ) ]
use super::*;
// xxx2 : implement

// #[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ storage_fields( a : i32, b : Option< String > ) ]
// #[ debug ]
#[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  // #[ former( only_storage = true ) ]
  // pub a : i32,
  // #[ former( only_storage = true ) ]
  // b : Option< String >,
}

// == begin of generated

  #[automatically_derived] impl < > Struct1 < > where
  {
      #[doc = r""]
      #[doc =
      r" Make former, variation of builder pattern to form structure defining values of fields step by step."]
      #[doc = r""] #[inline(always)] pub fn former() -> Struct1Former <
      Struct1FormerDefinition < (), Struct1 < > , former :: ReturnPreformed > >
      {
          Struct1Former :: < Struct1FormerDefinition < (), Struct1 < > , former
          :: ReturnPreformed > > :: new_coercing(former :: ReturnPreformed)
      }
  } impl < Definition > former :: EntityToFormer < Definition > for Struct1 < >
  where Definition : former :: FormerDefinition < Storage = Struct1FormerStorage
  < > > , { type Former = Struct1Former < Definition > ; } impl < > former ::
  EntityToStorage for Struct1 < > where
  { type Storage = Struct1FormerStorage < > ; } impl < __Context, __Formed,
  __End > former :: EntityToDefinition < __Context, __Formed, __End > for
  Struct1 < > where __End : former :: FormingEnd < Struct1FormerDefinitionTypes
  < __Context, __Formed > > ,
  { type Definition = Struct1FormerDefinition < __Context, __Formed, __End > ; }
  #[derive(Debug)] pub struct Struct1FormerDefinitionTypes < __Context = (),
  __Formed = Struct1 < > , > where
  {
      _phantom : core :: marker :: PhantomData <
      (* const __Context, * const __Formed) > ,
  } impl < __Context, __Formed, > :: core :: default :: Default for
  Struct1FormerDefinitionTypes < __Context, __Formed, > where
  {
      fn default() -> Self
      { Self { _phantom : core :: marker :: PhantomData, } }
  } impl < __Context, __Formed, > former :: FormerDefinitionTypes for
  Struct1FormerDefinitionTypes < __Context, __Formed, > where
  {
      type Storage = Struct1FormerStorage < > ; type Formed = __Formed; type
      Context = __Context;
  } #[derive(Debug)] pub struct Struct1FormerDefinition < __Context = (),
  __Formed = Struct1 < > , __End = former :: ReturnPreformed, > where
  {
      _phantom : core :: marker :: PhantomData <
      (* const __Context, * const __Formed, * const __End) > ,
  } impl < __Context, __Formed, __End, > :: core :: default :: Default for
  Struct1FormerDefinition < __Context, __Formed, __End, > where
  {
      fn default() -> Self
      { Self { _phantom : core :: marker :: PhantomData, } }
  } impl < __Context, __Formed, __End, > former :: FormerDefinition for
  Struct1FormerDefinition < __Context, __Formed, __End, > where __End : former
  :: FormingEnd < Struct1FormerDefinitionTypes < __Context, __Formed, > > ,
  {
      type Types = Struct1FormerDefinitionTypes < __Context, __Formed, > ; type
      End = __End; type Storage = Struct1FormerStorage < > ; type Formed =
      __Formed; type Context = __Context;
  } #[doc = "Container of a corresponding former."]
  #[allow(explicit_outlives_requirements)] pub struct Struct1FormerStorage < >
  where { a : i32, b : Option < String > } impl < > :: core :: default ::
  Default for Struct1FormerStorage < > where
  { #[inline(always)] fn default() -> Self { Self {} } } impl < > former ::
  Storage for Struct1FormerStorage < > where { type Formed = Struct1 < > ; }
  impl < > former :: StoragePreform for Struct1FormerStorage < > where
  {
      type Preformed = Struct1 < > ; fn preform(mut self) -> Self :: Preformed
      { let result = Struct1 :: < > {}; return result; }
  }
  #[doc =
  " Object to form [Struct1]. If field's values is not set then default value of the field is set.\n\nFor specifying custom default value use attribute `default`. For example:\n```\n\nuse former::Former;\n#[ derive( Former ) ]\npub struct Struct1\n{\n  #[default( 31 ) ]\n  field1 : i32,\n}\n\n```\n"]
  pub struct Struct1Former < Definition = Struct1FormerDefinition < (), Struct1
  < > , former :: ReturnPreformed > , > where Definition : former ::
  FormerDefinition < Storage = Struct1FormerStorage < > > , Definition :: Types
  : former :: FormerDefinitionTypes < Storage = Struct1FormerStorage < > > ,
  {
      storage : Definition :: Storage, context : core :: option :: Option <
      Definition :: Context > , on_end : core :: option :: Option < Definition
      :: End > ,
  } #[automatically_derived] impl < Definition, > Struct1Former < Definition, >
  where Definition : former :: FormerDefinition < Storage = Struct1FormerStorage
  < > > , Definition :: Types : former :: FormerDefinitionTypes < Storage =
  Struct1FormerStorage < > > ,
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
      begin(mut storage : core :: option :: Option < Definition :: Storage > ,
      context : core :: option :: Option < Definition :: Context > , on_end : <
      Definition as former :: FormerDefinition > :: End,) -> Self
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
      (mut storage : core :: option :: Option < Definition :: Storage > ,
      context : core :: option :: Option < Definition :: Context > , on_end :
      IntoEnd,) -> Self where IntoEnd : :: core :: convert :: Into < <
      Definition as former :: FormerDefinition > :: End > ,
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
      }
  } impl < Definition, > Struct1Former < Definition, > where Definition : former
  :: FormerDefinition < Storage = Struct1FormerStorage < > , Formed = Struct1 <
  > > , Definition :: Types : former :: FormerDefinitionTypes < Storage =
  Struct1FormerStorage < > , Formed = Struct1 < > > , Definition : former ::
  FormerDefinition < Storage = Struct1FormerStorage < > > , Definition :: Types
  : former :: FormerDefinitionTypes < Storage = Struct1FormerStorage < > > ,
  {
      pub fn preform(self) -> < Definition :: Types as former ::
      FormerDefinitionTypes > :: Formed
      { former :: StoragePreform :: preform(self.storage) }
  } #[automatically_derived] impl < Definition, > Struct1Former < Definition, >
  where Definition : former :: FormerDefinition < Storage = Struct1FormerStorage
  < > , Formed = Struct1 < > , > , Definition :: Types : former ::
  FormerDefinitionTypes < Storage = Struct1FormerStorage < > , Formed = Struct1
  < > , > ,
  {
      #[doc = r""]
      #[doc = r" Finish setting options and call perform on formed entity."]
      #[doc = r""]
      #[doc =
      r" If `perform` defined then associated method is called and its result returned instead of entity."]
      #[doc =
      r" For example `perform()` of structure with : `#[ perform( fn after1() -> &str > )` returns `&str`."]
      #[doc = r""] #[inline(always)] pub fn perform(self) -> Definition ::
      Formed { let result = self.form(); return result; }
  } impl < Definition > former :: FormerBegin < Definition > for Struct1Former <
  Definition, > where Definition : former :: FormerDefinition < Storage =
  Struct1FormerStorage < > > ,
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
  pub type Struct1AsSubformer < __Superformer, __End > = Struct1Former <
  Struct1FormerDefinition < __Superformer, __Superformer, __End, > , > ;
  #[doc =
  "Alias for trait former::FormingEnd with context and formed the same type and definition of structure [`$(stru)`]. Use as subformer end of a field during process of forming of super structure."]
  pub trait Struct1AsSubformerEnd < SuperFormer > where Self : former ::
  FormingEnd < Struct1FormerDefinitionTypes < SuperFormer, SuperFormer > , > ,
  {} impl < SuperFormer, __T > Struct1AsSubformerEnd < SuperFormer > for __T
  where Self : former :: FormingEnd < Struct1FormerDefinitionTypes <
  SuperFormer, SuperFormer > , > , {}

// == end of generated

tests_impls!
{
  fn test_complex()
  {
    // let got = Struct1::former().a( 13 ).b( "abc" ).form();
    // let exp = Struct1
    // {
    //   a : 13,
    //   b : Some( "abc".to_string() ),
    // };
    // a_id!( got, exp );
  }
}

tests_index!
{
  test_complex,
}
