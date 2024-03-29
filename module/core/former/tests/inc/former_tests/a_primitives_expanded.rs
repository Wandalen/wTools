#[ allow( unused_imports ) ]
use super::*;

// #[ derive( Debug, PartialEq ) ]
#[ derive( Debug, PartialEq, the_module::Former ) ] #[ debug ]
pub struct Struct1
{
  pub int_1 : i32,
  string_1 : String,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

// = generated

// #[automatically_derived] impl Struct1
// {
//     #[doc = r""]
//     #[doc =
//     r" Make former, variation of builder pattern to form structure defining values of fields step by step."]
//     #[doc = r""] #[inline(always)] pub fn former() -> Struct1Former < >
//     { Struct1Former :: new() }
// } #[derive(Debug)] pub struct Struct1FormerDefinition < Context = (), Formed =
// Struct1, End = former :: ReturnPreformed >
// { _phantom : core :: marker :: PhantomData < (Context, Formed, End) >, } impl
// < Context, Formed, End > Default for Struct1FormerDefinition < Context,
// Formed, End >
// {
//     fn default() -> Self
//     { Self { _phantom : core :: marker :: PhantomData, } }
// } impl < Context, Formed > former :: FormerDefinitionTypes for
// Struct1FormerDefinition < Context, Formed, former :: NoEnd >
// {
//     type Storage = Struct1FormerStorage ; type Formed = Formed ; type Context
//     = Context ;
// } impl < Context, Formed, End > former :: FormerDefinition for
// Struct1FormerDefinition < Context, Formed, End > where End : former ::
// FormingEnd < Struct1FormerDefinition < Context, Formed, former :: NoEnd > >,
// {
//     type Types = Struct1FormerDefinition < Context, Formed, former :: NoEnd >
//     ; type End = End ;
// } #[doc = "Container of a corresponding former."] pub struct
// Struct1FormerStorage
// {
//     #[doc = r" A field"] pub int_1 : :: core :: option :: Option < i32 >,
//     #[doc = r" A field"] pub string_1 : :: core :: option :: Option < String
//     >, #[doc = r" A field"] pub int_optional_1 : core :: option :: Option <
//     i32 >, #[doc = r" A field"] pub string_optional_1 : Option < String >,
// } impl :: core :: default :: Default for Struct1FormerStorage
// {
//     #[inline(always)] fn default() -> Self
//     {
//         Self
//         {
//             int_1 : :: core :: option :: Option :: None, string_1 : :: core ::
//             option :: Option :: None, int_optional_1 : :: core :: option ::
//             Option :: None, string_optional_1 : :: core :: option :: Option ::
//             None,
//         }
//     }
// } impl former :: Storage for Struct1FormerStorage { type Formed = Struct1 ; }
// impl former :: StoragePerform for Struct1FormerStorage
// {
//     fn preform(mut self) -> < Self as former :: Storage > :: Formed
//     {
//         let int_1 = if self.int_1.is_some() { self.int_1.take().unwrap() }
//         else
//         {
//             {
//                 trait MaybeDefault < T >
//                 {
//                     fn maybe_default(self : & Self) -> T
//                     { panic! ("Field 'int_1' isn't initialized") }
//                 } impl < T > MaybeDefault < T > for & :: core :: marker ::
//                 PhantomData < T > {} impl < T > MaybeDefault < T > for :: core
//                 :: marker :: PhantomData < T > where T : :: core :: default ::
//                 Default,
//                 { fn maybe_default(self : & Self) -> T { T :: default() } }
//                 (& :: core :: marker :: PhantomData :: < i32
//                 >).maybe_default()
//             }
//         } ; let string_1 = if self.string_1.is_some()
//         { self.string_1.take().unwrap() } else
//         {
//             {
//                 trait MaybeDefault < T >
//                 {
//                     fn maybe_default(self : & Self) -> T
//                     { panic! ("Field 'string_1' isn't initialized") }
//                 } impl < T > MaybeDefault < T > for & :: core :: marker ::
//                 PhantomData < T > {} impl < T > MaybeDefault < T > for :: core
//                 :: marker :: PhantomData < T > where T : :: core :: default ::
//                 Default,
//                 { fn maybe_default(self : & Self) -> T { T :: default() } }
//                 (& :: core :: marker :: PhantomData :: < String
//                 >).maybe_default()
//             }
//         } ; let int_optional_1 = if self.int_optional_1.is_some()
//         {
//             :: core :: option :: Option ::
//             Some(self.int_optional_1.take().unwrap())
//         } else { :: core :: option :: Option :: None } ; let string_optional_1
//         = if self.string_optional_1.is_some()
//         {
//             :: core :: option :: Option ::
//             Some(self.string_optional_1.take().unwrap())
//         } else { :: core :: option :: Option :: None } ; let result = Struct1
//         { int_1, string_1, int_optional_1, string_optional_1, } ; return
//         result ;
//     }
// }
// #[doc =
// " Object to form [Struct1]. If field's values is not set then default value of the field is set.\n\nFor specifying custom default value use attribute `default`. For example:\n```\n\nuse former::Former;\n#[ derive( Former ) ]\npub struct Struct1\n{\n  #[default( 31 ) ]\n  field1 : i32,\n}\n\n```\n"]
// pub struct Struct1Former < Definition = Struct1FormerDefinition > where
// Definition : former :: FormerDefinition, < Definition :: Types as former ::
// FormerDefinitionTypes > :: Storage : former :: StoragePerform, Definition ::
// Types : former :: FormerDefinitionTypes < Storage = Struct1FormerStorage >,
// {
//     storage : < Definition :: Types as former :: FormerDefinitionTypes > ::
//     Storage, context : core :: option :: Option < < Definition :: Types as
//     former :: FormerDefinitionTypes > :: Context >, on_end : core :: option ::
//     Option < Definition :: End >,
// } #[automatically_derived] impl < Definition > Struct1Former < Definition >
// where Definition : former :: FormerDefinition, < Definition :: Types as former
// :: FormerDefinitionTypes > :: Storage : former :: StoragePerform, Definition
// :: Types : former :: FormerDefinitionTypes < Storage = Struct1FormerStorage >,
// {
//     #[doc = r""]
//     #[doc = r" Finish setting options and call perform on formed entity."]
//     #[doc = r""]
//     #[doc =
//     r" If `perform` defined then associated method is called and its result returned instead of entity."]
//     #[doc =
//     r" For example `perform()` of structure with : `#[ perform( fn after1() -> &str > )` returns `&str`."]
//     #[doc = r""] #[inline(always)] pub fn perform(self) -> < Definition ::
//     Types as former :: FormerDefinitionTypes > :: Formed
//     { let result = self.form() ; return result ; } #[doc = r""]
//     #[doc =
//     r" Begin the process of forming. Expects context of forming to return it after forming."]
//     #[doc = r""] #[inline(always)] pub fn
//     begin(mut storage : core :: option :: Option < < Definition :: Types as
//     former :: FormerDefinitionTypes > :: Storage >, context : core :: option
//     :: Option < < Definition :: Types as former :: FormerDefinitionTypes > ::
//     Context >, on_end : < Definition as former :: FormerDefinition > :: End,)
//     -> Self
//     {
//         if storage.is_none()
//         { storage = Some(:: core :: default :: Default :: default()) ; } Self
//         {
//             storage : storage.unwrap(), context : context, on_end : :: core ::
//             option :: Option :: Some(on_end),
//         }
//     } #[doc = r""]
//     #[doc =
//     r" End the process of forming returning original context of forming."]
//     #[doc = r""] #[inline(always)] pub fn form(self) -> < Definition :: Types
//     as former :: FormerDefinitionTypes > :: Formed { self.end() } #[doc = r""]
//     #[doc =
//     r" End the process of forming returning original context of forming."]
//     #[doc = r""] #[inline(always)] pub fn end(mut self) -> < Definition ::
//     Types as former :: FormerDefinitionTypes > :: Formed
//     {
//         let on_end = self.on_end.take().unwrap() ; let context =
//         self.context.take() ; former :: FormingEnd :: < Definition :: Types >
//         :: call(& on_end, self.storage, context)
//     } #[doc = "Setter for the 'int_1' field."] #[inline] pub fn int_1 < Src >
//     (mut self, src : Src) -> Self where Src : :: core :: convert :: Into < i32
//     >,
//     {
//         debug_assert! (self.storage.int_1.is_none()) ; self.storage.int_1 = ::
//         core :: option :: Option :: Some(src.into()) ; self
//     } #[doc = "Setter for the 'string_1' field."] #[inline] pub fn string_1 <
//     Src > (mut self, src : Src) -> Self where Src : :: core :: convert :: Into
//     < String >,
//     {
//         debug_assert! (self.storage.string_1.is_none()) ;
//         self.storage.string_1 = :: core :: option :: Option ::
//         Some(src.into()) ; self
//     } #[doc = "Setter for the 'int_optional_1' field."] #[inline] pub fn
//     int_optional_1 < Src > (mut self, src : Src) -> Self where Src : :: core
//     :: convert :: Into < i32 >,
//     {
//         debug_assert! (self.storage.int_optional_1.is_none()) ;
//         self.storage.int_optional_1 = :: core :: option :: Option ::
//         Some(src.into()) ; self
//     } #[doc = "Setter for the 'string_optional_1' field."] #[inline] pub fn
//     string_optional_1 < Src > (mut self, src : Src) -> Self where Src : ::
//     core :: convert :: Into < String >,
//     {
//         debug_assert! (self.storage.string_optional_1.is_none()) ;
//         self.storage.string_optional_1 = :: core :: option :: Option ::
//         Some(src.into()) ; self
//     }
// } #[automatically_derived] impl Struct1Former < >
// {
//     #[doc = r""]
//     #[doc = r" Construct new instance of former with default parameters."]
//     #[doc = r""] #[inline(always)] pub fn new() -> Self
//     { Self :: begin(None, None, former :: ReturnPreformed,) }
// }
//
// impl < Definition > Struct1FormerDefinition < Definition >
// where
//   Definition : former :: FormerDefinition,
//   <
//     Definition :: Types as former :: FormerDefinitionTypes > :: Storage : former :: StoragePerform, Definition :: Types : former :: FormerDefinitionTypes < Storage = Struct1FormerStorage,
// Formed = Struct1 >,
// {
//     pub fn preform(self) -> < Definition :: Types as former :: FormerDefinitionTypes > :: Formed
//     {
//       former :: StoragePerform :: preform(self.storage)
//     }
// }

// = end of generated

//  include!( "./only_test/primitives.rs" );
