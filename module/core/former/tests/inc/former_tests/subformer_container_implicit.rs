#![ allow( dead_code ) ]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child
{
  name : String,
  data : bool,
}

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  // #[ container( definition = former::VectorDefinition ) ]
  #[ container ]
  children : Vec< Child >,
}

// == begin of generated

// // xxx : clean
//
//   #[automatically_derived] impl < > Parent < > where
//   {
//       #[doc = r""]
//       #[doc =
//       r" Make former, variation of builder pattern to form structure defining values of fields step by step."]
//       #[doc = r""] #[inline(always)] pub fn former() -> ParentFormer <
//       ParentFormerDefinition < (), Parent < > , former :: ReturnPreformed > >
//       {
//           ParentFormer :: < ParentFormerDefinition < (), Parent < > , former ::
//           ReturnPreformed > > :: new_coercing(former :: ReturnPreformed)
//       }
//   } impl < Definition > former :: EntityToFormer < Definition > for Parent < >
//   where Definition : former :: FormerDefinition < Storage = ParentFormerStorage
//   < > > , { type Former = ParentFormer < Definition > ; } impl < > former ::
//   EntityToStorage for Parent < > where
//   { type Storage = ParentFormerStorage < > ; } impl < __Context, __Formed, __End
//   > former :: EntityToDefinition < __Context, __Formed, __End > for Parent < >
//   where __End : former :: FormingEnd < ParentFormerDefinitionTypes < __Context,
//   __Formed > > ,
//   {
//       type Definition = ParentFormerDefinition < __Context, __Formed, __End > ;
//       type Types = ParentFormerDefinitionTypes < __Context, __Formed > ;
//   } impl < __Context, __Formed > former :: EntityToDefinitionTypes < __Context,
//   __Formed > for Parent < > where
//   { type Types = ParentFormerDefinitionTypes < __Context, __Formed > ; }
//   #[derive(Debug)] pub struct ParentFormerDefinitionTypes < __Context = (),
//   __Formed = Parent < > , > where
//   {
//       _phantom : core :: marker :: PhantomData <
//       (* const __Context, * const __Formed) > ,
//   } impl < __Context, __Formed, > :: core :: default :: Default for
//   ParentFormerDefinitionTypes < __Context, __Formed, > where
//   {
//       fn default() -> Self
//       { Self { _phantom : core :: marker :: PhantomData, } }
//   } impl < __Context, __Formed, > former :: FormerDefinitionTypes for
//   ParentFormerDefinitionTypes < __Context, __Formed, > where
//   {
//       type Storage = ParentFormerStorage < > ; type Formed = __Formed; type
//       Context = __Context;
//   } #[derive(Debug)] pub struct ParentFormerDefinition < __Context = (),
//   __Formed = Parent < > , __End = former :: ReturnPreformed, > where
//   {
//       _phantom : core :: marker :: PhantomData <
//       (* const __Context, * const __Formed, * const __End) > ,
//   } impl < __Context, __Formed, __End, > :: core :: default :: Default for
//   ParentFormerDefinition < __Context, __Formed, __End, > where
//   {
//       fn default() -> Self
//       { Self { _phantom : core :: marker :: PhantomData, } }
//   } impl < __Context, __Formed, __End, > former :: FormerDefinition for
//   ParentFormerDefinition < __Context, __Formed, __End, > where __End : former ::
//   FormingEnd < ParentFormerDefinitionTypes < __Context, __Formed, > > ,
//   {
//       type Types = ParentFormerDefinitionTypes < __Context, __Formed, > ; type
//       End = __End; type Storage = ParentFormerStorage < > ; type Formed =
//       __Formed; type Context = __Context;
//   } impl < __Context, __Formed, > former :: FormerMutator for
//   ParentFormerDefinitionTypes < __Context, __Formed, > where {}
//   #[doc = "Container of a corresponding former."]
//   #[allow(explicit_outlives_requirements)] pub struct ParentFormerStorage < >
//   where
//   {
//       #[doc = r" A field"] pub children : :: core :: option :: Option < Vec <
//       Child > > ,
//   } impl < > :: core :: default :: Default for ParentFormerStorage < > where
//   {
//       #[inline(always)] fn default() -> Self
//       { Self { children : :: core :: option :: Option :: None, } }
//   } impl < > former :: Storage for ParentFormerStorage < > where
//   { type Formed = Parent < > ; } impl < > former :: StoragePreform for
//   ParentFormerStorage < > where
//   {
//       type Preformed = Parent < > ; fn preform(mut self) -> Self :: Preformed
//       {
//           let children = if self.children.is_some()
//           { self.children.take().unwrap() } else
//           {
//               {
//                   trait MaybeDefault < T >
//                   {
//                       fn maybe_default(self : & Self) -> T
//                       { panic! ("Field 'children' isn't initialized") }
//                   } impl < T > MaybeDefault < T > for & :: core :: marker ::
//                   PhantomData < T > {} impl < T > MaybeDefault < T > for :: core
//                   :: marker :: PhantomData < T > where T : :: core :: default ::
//                   Default,
//                   { fn maybe_default(self : & Self) -> T { T :: default() } }
//                   (& :: core :: marker :: PhantomData :: < Vec < Child >
//                   >).maybe_default()
//               }
//           }; let result = Parent :: < > { children, }; return result;
//       }
//   }
//   #[doc =
//   "\nStructure to form [Parent]. Represents a forming entity designed to construct objects through a builder pattern.\n\nThis structure holds temporary storage and context during the formation process and\nutilizes a defined end strategy to finalize the object creation. It facilitates the flexible\nconstruction of complex objects by allowing step-by-step configuration.\n```\n\nuse former::Former;\n#[ derive( Former ) ]\npub struct Struct1\n{\n  #[default( 31 ) ]\n  field1 : i32,\n}\n\n```\n"]
//   pub struct ParentFormer < Definition = ParentFormerDefinition < (), Parent < >
//   , former :: ReturnPreformed > , > where Definition : former ::
//   FormerDefinition < Storage = ParentFormerStorage < > > , Definition :: Types :
//   former :: FormerDefinitionTypes < Storage = ParentFormerStorage < > > ,
//   {
//       #[doc =
//       r" Temporary storage for all fields during the formation process. It contains"]
//       #[doc =
//       r"   partial data that progressively builds up to the final object."] pub
//       storage : Definition :: Storage,
//       #[doc =
//       r" An optional context providing additional data or state necessary for custom"]
//       #[doc =
//       r"   formation logic or to facilitate this former's role as a subformer within another former."]
//       pub context : core :: option :: Option < Definition :: Context > ,
//       #[doc =
//       r" An optional closure or handler that is invoked to transform the accumulated"]
//       #[doc =
//       r"   temporary storage into the final object structure once formation is complete."]
//       pub on_end : core :: option :: Option < Definition :: End > ,
//   } #[automatically_derived] impl < Definition, > ParentFormer < Definition, >
//   where Definition : former :: FormerDefinition < Storage = ParentFormerStorage
//   < > > , Definition :: Types : former :: FormerDefinitionTypes < Storage =
//   ParentFormerStorage < > > ,
//   {
//       #[doc = r""]
//       #[doc = r" Construct new instance of former with default parameters."]
//       #[doc = r""] #[inline(always)] pub fn new(on_end : Definition :: End) ->
//       Self { Self :: begin_coercing(None, None, on_end) } #[doc = r""]
//       #[doc = r" Construct new instance of former with default parameters."]
//       #[doc = r""] #[inline(always)] pub fn new_coercing < IntoEnd >
//       (end : IntoEnd) -> Self where IntoEnd : Into < Definition :: End > ,
//       { Self :: begin_coercing(None, None, end,) } #[doc = r""]
//       #[doc =
//       r" Begin the process of forming. Expects context of forming to return it after forming."]
//       #[doc = r""] #[inline(always)] pub fn
//       begin(mut storage : core :: option :: Option < Definition :: Storage > ,
//       context : core :: option :: Option < Definition :: Context > , on_end : <
//       Definition as former :: FormerDefinition > :: End,) -> Self
//       {
//           if storage.is_none()
//           { storage = Some(:: core :: default :: Default :: default()); } Self
//           {
//               storage : storage.unwrap(), context : context, on_end : :: core ::
//               option :: Option :: Some(on_end),
//           }
//       } #[doc = r""]
//       #[doc =
//       r" Begin the process of forming. Expects context of forming to return it after forming."]
//       #[doc = r""] #[inline(always)] pub fn begin_coercing < IntoEnd >
//       (mut storage : core :: option :: Option < Definition :: Storage > ,
//       context : core :: option :: Option < Definition :: Context > , on_end :
//       IntoEnd,) -> Self where IntoEnd : :: core :: convert :: Into < <
//       Definition as former :: FormerDefinition > :: End > ,
//       {
//           if storage.is_none()
//           { storage = Some(:: core :: default :: Default :: default()); } Self
//           {
//               storage : storage.unwrap(), context : context, on_end : :: core ::
//               option :: Option ::
//               Some(:: core :: convert :: Into :: into(on_end)),
//           }
//       } #[doc = r""]
//       #[doc =
//       r" End the process of forming returning original context of forming."]
//       #[doc = r""] #[inline(always)] pub fn form(self) -> < Definition :: Types
//       as former :: FormerDefinitionTypes > :: Formed { self.end() } #[doc = r""]
//       #[doc =
//       r" End the process of forming returning original context of forming."]
//       #[doc = r""] #[inline(always)] pub fn end(mut self) -> < Definition ::
//       Types as former :: FormerDefinitionTypes > :: Formed
//       {
//           let on_end = self.on_end.take().unwrap(); let mut context =
//           self.context.take(); < Definition :: Types as former :: FormerMutator
//           > :: form_mutation(& mut self.storage, & mut context); former ::
//           FormingEnd :: < Definition :: Types > ::
//           call(& on_end, self.storage, context)
//       }
//
// // xxx
//
//       #[inline(always)]
//       pub fn _children_container_former< Former2 >( self ) -> Former2
//       where
//         Former2 : former::FormerBegin
//         <
//           <
//             Vec< Child > as former::EntityToDefinition< Self, Self, ParentFormerAssignChildrenEnd< Definition > >
//           >
//           ::Definition,
//         >,
//         <
//           Vec< Child > as former::EntityToDefinition
//           <
//             Self,
//             Self,
//             ParentFormerAssignChildrenEnd< Definition >
//           >
//         >
//         ::Definition
//         :
//         former::FormerDefinition
//         <
//           Storage = Vec< Child >,
//           // Storage : former::ContainerAdd
//           // <
//           //   Entry = < Vec< Child > as former::Container >::Entry
//           // >,
//           Context = ParentFormer< Definition, >,
//           End = ParentFormerAssignChildrenEnd< Definition >,
//         >,
//       {
//         Former2::former_begin( None, Some( self ), ParentFormerAssignChildrenEnd::< Definition >::default() )
//       }
//
// // xxx
//
//       // #[inline(always)]
//       // pub fn children( self ) -> former::ContainerSubformer::
//       // <
//       //   Child,
//       //   < Vec< Child > as former::EntityToDefinition< Self, Self, ParentFormerAssignChildrenEnd< Definition > > >::Definition,
//       // >
//       // where
//       //   <
//       //     Vec< Child > as former::EntityToDefinition
//       //     <
//       //       Self,
//       //       Self,
//       //       ParentFormerAssignChildrenEnd< Definition >,
//       //     >
//       //   >::Definition
//       //   :
//       //   former::FormerDefinition
//       //   <
//       //     Storage : former::ContainerAdd
//       //     <
//       //       Entry = < Vec< Child > as former::Container > ::Entry
//       //     >,
//       //     Context = ParentFormer< Definition, >,
//       //     End = ParentFormerAssignChildrenEnd< Definition >,
//       //   >,
//       //   // < Vec< Child > as former::EntityToDefinition< Self, Self, ParentFormerAssignChildrenEnd< Definition > > >::Definition : former::FormerDefinition< Storage : former::ContainerAdd< Entry = < Vec< Child > as former::Container >::Entry >, Context = ParentFormer< Definition, >, End = ParentFormerAssignChildrenEnd< Definition >, >,
//       // {
//       //   self._children_container_former::
//       //   <
//       //     former::ContainerSubformer::
//       //     <
//       //       Child,
//       //       < Vec< Child > as former::EntityToDefinition< Self, Self, ParentFormerAssignChildrenEnd< Definition > > >::Definition,
//       //     >
//       //   >()
//       // }
//
//   } impl < Definition, > ParentFormer < Definition, > where Definition : former
//   :: FormerDefinition < Storage = ParentFormerStorage < > , Formed = Parent < >
//   > , Definition :: Types : former :: FormerDefinitionTypes < Storage =
//   ParentFormerStorage < > , Formed = Parent < > > , Definition : former ::
//   FormerDefinition < Storage = ParentFormerStorage < > > , Definition :: Types :
//   former :: FormerDefinitionTypes < Storage = ParentFormerStorage < > > ,
//   {
//       pub fn preform(self) -> < Definition :: Types as former ::
//       FormerDefinitionTypes > :: Formed
//       { former :: StoragePreform :: preform(self.storage) }
//   } #[automatically_derived] impl < Definition, > ParentFormer < Definition, >
//   where Definition : former :: FormerDefinition < Storage = ParentFormerStorage
//   < > , Formed = Parent < > , > , Definition :: Types : former ::
//   FormerDefinitionTypes < Storage = ParentFormerStorage < > , Formed = Parent <
//   > , > ,
//   {
//       #[doc = r""]
//       #[doc = r" Finish setting options and call perform on formed entity."]
//       #[doc = r""]
//       #[doc =
//       r" If `perform` defined then associated method is called and its result returned instead of entity."]
//       #[doc =
//       r" For example `perform()` of structure with : `#[ perform( fn after1() -> &str > )` returns `&str`."]
//       #[doc = r""] #[inline(always)] pub fn perform(self) -> Definition ::
//       Formed { let result = self.form(); return result; }
//   } impl < Definition > former :: FormerBegin < Definition > for ParentFormer <
//   Definition, > where Definition : former :: FormerDefinition < Storage =
//   ParentFormerStorage < > > ,
//   {
//       #[inline(always)] fn
//       former_begin(storage : core :: option :: Option < Definition :: Storage >
//       , context : core :: option :: Option < Definition :: Context > , on_end :
//       Definition :: End,) -> Self
//       {
//           debug_assert! (storage.is_none()); Self ::
//           begin(None, context, on_end)
//       }
//   }
//   #[doc =
//   r" Use as subformer of a field during process of forming of super structure."]
//   pub type ParentAsSubformer < __Superformer, __End > = ParentFormer <
//   ParentFormerDefinition < __Superformer, __Superformer, __End, > , > ;
//   #[doc =
//   "Alias for trait former::FormingEnd with context and formed the same type and definition of structure [`$(stru)`]. Use as subformer end of a field during process of forming of super structure."]
//   pub trait ParentAsSubformerEnd < SuperFormer > where Self : former ::
//   FormingEnd < ParentFormerDefinitionTypes < SuperFormer, SuperFormer > , > , {}
//   impl < SuperFormer, __T > ParentAsSubformerEnd < SuperFormer > for __T where
//   Self : former :: FormingEnd < ParentFormerDefinitionTypes < SuperFormer,
//   SuperFormer > , > , {}
//   #[doc =
//   "Callback to return original former after forming of container for `$Parent` is done.#\n\n  Callback replace content of container assigning new content from subformer's storage."]
//   pub struct ParentFormerAssignChildrenEnd < Definition >
//   { _phantom : core :: marker :: PhantomData < (Definition,) > , } impl <
//   Definition > Default for ParentFormerAssignChildrenEnd < Definition >
//   {
//       #[inline(always)] fn default() -> Self
//       { Self { _phantom : core :: marker :: PhantomData, } }
//   }
//
//   #[automatically_derived]
//   impl < Definition, > former :: FormingEnd < < Vec <
//   Child > as former :: EntityToDefinitionTypes < ParentFormer < Definition, > ,
//   ParentFormer < Definition, > , > > :: Types, >
//   for
//   ParentFormerAssignChildrenEnd < Definition > where Definition : former ::
//   FormerDefinition < Storage = ParentFormerStorage < > > , Definition :: Types :
//   former :: FormerDefinitionTypes < Storage = ParentFormerStorage < > > ,
//   {
//       #[inline(always)] fn
//       call(& self, storage : Vec < Child > , super_former : Option <
//       ParentFormer < Definition, > > ,) -> ParentFormer < Definition, >
//       {
//           let mut super_former = super_former.unwrap(); if let
//           Some(ref mut field) = super_former.storage.children
//           { former :: ContainerAssign :: assign(field, storage); } else
//           { super_former.storage.children = Some(storage); } super_former
//       }
//   }

// == end of generated

include!( "./only_test/subformer_container.rs" );

// xxx