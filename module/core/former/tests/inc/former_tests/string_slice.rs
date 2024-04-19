use super::*;

// #[ derive( Debug, PartialEq, former::Former ) ]
// #[ derive( Debug, PartialEq, former::Former ) ] #[ debug ]
#[ derive( Debug, PartialEq ) ]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

// === begin of generated

  #[automatically_derived] impl < 'a, > Struct1 < 'a, > where
  {
      #[doc = r""]
      #[doc =
      r" Make former, variation of builder pattern to form structure defining values of fields step by step."]
      #[doc = r""] #[inline(always)] pub fn former() -> Struct1Former < 'a,
      Struct1FormerDefinition < 'a, (), Struct1 < 'a, >, former ::
      ReturnPreformed > >
      {
          Struct1Former :: < 'a, Struct1FormerDefinition < 'a, (), Struct1 < 'a,
          >, former :: ReturnPreformed > > :: new(former :: ReturnPreformed)
      }
  }

  #[derive(Debug)] pub struct Struct1FormerDefinitionTypes < 'a, Context,
  Formed, > where
  { _phantom : core :: marker :: PhantomData < (Context, Formed) >, }

  impl < 'a,
  Context, Formed, > Default for Struct1FormerDefinitionTypes < 'a, Context,
  Formed, > where
  {
      fn default() -> Self
      { Self { _phantom : core :: marker :: PhantomData, } }
  }

  impl < 'a, Context, Formed, > former :: FormerDefinitionTypes for
  Struct1FormerDefinitionTypes < 'a, Context, Formed, >
  {
      type Storage = Struct1FormerStorage < 'a, > ; type Formed = Formed ; type
      Context = Context ;
  } #[derive(Debug)] pub struct Struct1FormerDefinition < 'a, Context, Formed,
  End, > where
  { _phantom : core :: marker :: PhantomData < (Context, Formed, End) >, } impl
  < 'a, Context, Formed, End, > Default for Struct1FormerDefinition < 'a,
  Context, Formed, End, > where
  {
      fn default() -> Self
      { Self { _phantom : core :: marker :: PhantomData, } }
  } impl < 'a, Context, Formed, End, > former :: FormerDefinition for
  Struct1FormerDefinition < 'a, Context, Formed, End, > where End : former ::
  FormingEnd < Struct1FormerDefinitionTypes < 'a, Context, Formed, > >,
  {
      type Types = Struct1FormerDefinitionTypes < 'a, Context, Formed, > ; type
      End = End ;
  } pub type Struct1FormerWithClosure < 'a, Context, Formed, > =
  Struct1FormerDefinition < 'a, Context, Formed, former :: FormingEndClosure <
  Struct1FormerDefinitionTypes < 'a, Context, Formed, > > > ;
  #[doc = "Container of a corresponding former."] pub struct
  Struct1FormerStorage < 'a, > where
  {
      #[doc = r" A field"] pub string_slice_1 : :: core :: option :: Option < &
      'a str >,
  } impl < 'a, > :: core :: default :: Default for Struct1FormerStorage < 'a, >
  where
  {
      #[inline(always)] fn default() -> Self
      { Self { string_slice_1 : :: core :: option :: Option :: None, } }
  } impl < 'a, > former :: Storage for Struct1FormerStorage < 'a, > where
  { type Formed = Struct1 < 'a, > ; } impl < 'a, > former :: StoragePreform for
  Struct1FormerStorage < 'a, > where
  {
      fn preform(mut self) -> < Self as former :: Storage > :: Formed
      {
          let string_slice_1 = if self.string_slice_1.is_some()
          { self.string_slice_1.take().unwrap() } else
          {
              {
                  trait MaybeDefault < T >
                  {
                      fn maybe_default(self : & Self) -> T
                      { panic! ("Field 'string_slice_1' isn't initialized") }
                  } impl < T > MaybeDefault < T > for & :: core :: marker ::
                  PhantomData < T > {} impl < T > MaybeDefault < T > for :: core
                  :: marker :: PhantomData < T > where T : :: core :: default ::
                  Default,
                  { fn maybe_default(self : & Self) -> T { T :: default() } }
                  (& :: core :: marker :: PhantomData :: < & 'a str
                  >).maybe_default()
              }
          } ; let result = Struct1 :: < 'a, > { string_slice_1, } ; return
          result ;
      }
  }
  #[doc =
  " Object to form [Struct1]. If field's values is not set then default value of the field is set.\n\nFor specifying custom default value use attribute `default`. For example:\n```\n\nuse former::Former;\n#[ derive( Former ) ]\npub struct Struct1\n{\n  #[default( 31 ) ]\n  field1 : i32,\n}\n\n```\n"]
  pub struct Struct1Former < 'a, Definition = Struct1FormerDefinition < 'a, (),
  Struct1 < 'a, >, former :: ReturnPreformed >, > where Definition : former ::
  FormerDefinition, Definition :: Types : former :: FormerDefinitionTypes <
  Storage = Struct1FormerStorage < 'a, > >,
  {
      storage : < Definition :: Types as former :: FormerDefinitionTypes > ::
      Storage, context : core :: option :: Option < < Definition :: Types as
      former :: FormerDefinitionTypes > :: Context >, on_end : core :: option ::
      Option < Definition :: End >,
  } #[automatically_derived] impl < 'a, Definition, > Struct1Former < 'a,
  Definition, > where Definition : former :: FormerDefinition, Definition ::
  Types : former :: FormerDefinitionTypes < Storage = Struct1FormerStorage < 'a,
  > >,
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
      { let result = self.form() ; return result ; } #[doc = r""]
      #[doc = r" Construct new instance of former with default parameters."]
      #[doc = r""] #[inline(always)] pub fn
      _new_precise(on_end : Definition :: End) -> Self
      { Self :: begin(None, None, on_end) } #[doc = r""]
      #[doc = r" Construct new instance of former with default parameters."]
      #[doc = r""] #[inline(always)] pub fn new < IntoEnd > (end : IntoEnd) ->
      Self where IntoEnd : Into < Definition :: End >,
      { Self :: begin(None, None, end,) } #[doc = r""]
      #[doc =
      r" Begin the process of forming. Expects context of forming to return it after forming."]
      #[doc = r""] #[inline(always)] pub fn
      _begin_precise(mut storage : core :: option :: Option < < Definition ::
      Types as former :: FormerDefinitionTypes > :: Storage >, context : core ::
      option :: Option < < Definition :: Types as former ::
      FormerDefinitionTypes > :: Context >, on_end : < Definition as former ::
      FormerDefinition > :: End,) -> Self
      {
          if storage.is_none()
          { storage = Some(:: core :: default :: Default :: default()) ; } Self
          {
              storage : storage.unwrap(), context : context, on_end : :: core ::
              option :: Option :: Some(on_end),
          }
      } #[doc = r""]
      #[doc =
      r" Begin the process of forming. Expects context of forming to return it after forming."]
      #[doc = r""] #[inline(always)] pub fn begin < IntoEnd >
      (mut storage : core :: option :: Option < < Definition :: Types as former
      :: FormerDefinitionTypes > :: Storage >, context : core :: option ::
      Option < < Definition :: Types as former :: FormerDefinitionTypes > ::
      Context >, on_end : IntoEnd,) -> Self where IntoEnd : :: core :: convert
      :: Into < < Definition as former :: FormerDefinition > :: End >,
      {
          if storage.is_none()
          { storage = Some(:: core :: default :: Default :: default()) ; } Self
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
          let on_end = self.on_end.take().unwrap() ; let context =
          self.context.take() ; former :: FormingEnd :: < Definition :: Types >
          :: call(& on_end, self.storage, context)
      } #[doc = "Setter for the 'string_slice_1' field."] #[inline] pub fn
      string_slice_1 < Src > (mut self, src : Src) -> Self where Src : :: core
      :: convert :: Into < & 'a str >,
      {
          debug_assert! (self.storage.string_slice_1.is_none()) ;
          self.storage.string_slice_1 = :: core :: option :: Option ::
          Some(:: core :: convert :: Into :: into(src)) ; self
      }
  } impl < 'a, Definition, > Struct1Former < 'a, Definition, > where Definition
  : former :: FormerDefinition, Definition :: Types : former ::
  FormerDefinitionTypes < Storage = Struct1FormerStorage < 'a, >, Formed =
  Struct1 < 'a, > >, < Definition :: Types as former :: FormerDefinitionTypes >
  :: Storage : former :: StoragePreform, < Definition :: Types as former ::
  FormerDefinitionTypes > :: Storage : former :: Storage < Formed = Struct1 <
  'a, > >,
  {
      pub fn preform(self) -> < Definition :: Types as former ::
      FormerDefinitionTypes > :: Formed
      { former :: StoragePreform :: preform(self.storage) }
  }

// === end of generated

// include!( "./only_test/string_slice.rs" );
// xxx : uncomment
