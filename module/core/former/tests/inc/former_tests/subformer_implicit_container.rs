#![ allow( dead_code ) ]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct Child
{
  name : String,
  is_mandatory : bool,
}

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent
{
  #[ subform ]
  // #[ subform( name = child ) ]
  #[ container( former::VectorDefinition ) ]
  // #[ setter( false ) ]
  children : Vec< Child >,
}

impl< Definition > ParentFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = < Parent as former::EntityToStorage >::Storage >,
{

  #[ inline( always ) ]
  pub fn child( self, name : &str ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._children_add_subformer
    ::< ChildFormer< _ >, _, >()
    .name( name )
  }

  #[ inline( always ) ]
  pub fn _child( self ) ->
  ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
  {
    self._children_add_subformer
    ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
  }

}

// == begin of generated

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
//   { type Storage = ParentFormerStorage < > ; } #[derive(Debug)] pub struct
//   ParentFormerDefinitionTypes < __Context = (), __Formed = Parent < > , > where
//   { _phantom : core :: marker :: PhantomData < (__Context, __Formed) > , } impl
//   < __Context, __Formed, > :: core :: default :: Default for
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
//       _phantom : core :: marker :: PhantomData < (__Context, __Formed, __End) >
//       ,
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
//   } #[doc = "Container of a corresponding former."] pub struct
//   ParentFormerStorage < > where
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
//   " Object to form [Parent]. If field's values is not set then default value of the field is set.\n\nFor specifying custom default value use attribute `default`. For example:\n```\n\nuse former::Former;\n#[ derive( Former ) ]\npub struct Struct1\n{\n  #[default( 31 ) ]\n  field1 : i32,\n}\n\n```\n"]
//   pub struct ParentFormer < Definition = ParentFormerDefinition < (), Parent < >
//   , former :: ReturnPreformed > , > where Definition : former ::
//   FormerDefinition, Definition :: Types : former :: FormerDefinitionTypes <
//   Storage = ParentFormerStorage < > > ,
//   {
//       storage : < Definition :: Types as former :: FormerDefinitionTypes > ::
//       Storage, context : core :: option :: Option < < Definition :: Types as
//       former :: FormerDefinitionTypes > :: Context > , on_end : core :: option
//       :: Option < Definition :: End > ,
//   } #[automatically_derived] impl < Definition, > ParentFormer < Definition, >
//   where Definition : former :: FormerDefinition, Definition :: Types : former ::
//   FormerDefinitionTypes < Storage = ParentFormerStorage < > > ,
//   {
//       #[doc = r""]
//       #[doc = r" Construct new instance of former with default parameters."]
//       #[doc = r""] #[inline(always)] pub fn
//       new_precise(on_end : Definition :: End) -> Self
//       { Self :: begin_coercing(None, None, on_end) } #[doc = r""]
//       #[doc = r" Construct new instance of former with default parameters."]
//       #[doc = r""] #[inline(always)] pub fn new_coercing < IntoEnd >
//       (end : IntoEnd) -> Self where IntoEnd : Into < Definition :: End > ,
//       { Self :: begin_coercing(None, None, end,) } #[doc = r""]
//       #[doc =
//       r" Begin the process of forming. Expects context of forming to return it after forming."]
//       #[doc = r""] #[inline(always)] pub fn
//       begin_precise(mut storage : core :: option :: Option < < Definition ::
//       Types as former :: FormerDefinitionTypes > :: Storage > , context : core
//       :: option :: Option < < Definition :: Types as former ::
//       FormerDefinitionTypes > :: Context > , on_end : < Definition as former ::
//       FormerDefinition > :: End,) -> Self
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
//       (mut storage : core :: option :: Option < < Definition :: Types as former
//       :: FormerDefinitionTypes > :: Storage > , context : core :: option ::
//       Option < < Definition :: Types as former :: FormerDefinitionTypes > ::
//       Context > , on_end : IntoEnd,) -> Self where IntoEnd : :: core :: convert
//       :: Into < < Definition as former :: FormerDefinition > :: End > ,
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
//           let on_end = self.on_end.take().unwrap(); let context =
//           self.context.take(); former :: FormingEnd :: < Definition :: Types >
//           :: call(& on_end, self.storage, context)
//       }
//       #[doc =
//       "Subformer setter for the 'children' field. Method _children_assign unlike method children accept custom container subformer."]
//       #[inline(always)] pub fn _children_assign < Former2 > (self) -> Former2
//       where Former2 : former :: FormerBegin < former :: VectorDefinition <
//       Child, Self, Self, ParentFormerAssignChildrenEnd < Definition > , > > ,
//       {
//           Former2 ::
//           former_begin(None, Some(self), ParentFormerAssignChildrenEnd :: <
//           Definition > :: default())
//       }
//       #[doc =
//       "Subformer setter for the 'children' field. Method _children_assign unlike method children accept custom container subformer."]
//       #[inline(always)] pub fn children(self) -> former :: ContainerSubformer ::
//       < Child, former :: VectorDefinition < Child, Self, Self,
//       ParentFormerAssignChildrenEnd < Definition > , > >
//       {
//           self._children_assign :: < former :: ContainerSubformer :: < Child,
//           former :: VectorDefinition < Child, Self, Self,
//           ParentFormerAssignChildrenEnd < Definition > , > >> ()
//       } #[doc = r" Custom setter which produce container element subformer."]
//       #[inline(always)] pub fn _children_add_subformer < Former2, Definition2 >
//       (self) -> Former2 where Definition2 : former :: FormerDefinition < End =
//       ParentFormerAddChildrenEnd < Definition > , Storage = < Child as former ::
//       EntityToStorage > :: Storage, Formed = Self, Context = Self, > ,
//       Definition2 :: Types : former :: FormerDefinitionTypes < Storage = < Child
//       as former :: EntityToStorage > :: Storage, Formed = Self, Context = Self,
//       > , Former2 : former :: FormerBegin < Definition2 > ,
//       {
//           Former2 ::
//           former_begin(None, Some(self), ParentFormerAddChildrenEnd ::
//           default())
//       }
//
//       #[inline(always)]
//       pub fn child(self) -> ParentAsSubformer < Self, impl ParentAsSubformerEnd < Self > >
//       {
//           self._children_add_subformer :: < < Vec < Child > as former ::
//           EntityToFormer < _ > > :: Former, _, > ()
//       }
//   }
//
//   impl < Definition, > ParentFormer < Definition, > where Definition :: Types
//   : former :: FormerDefinitionTypes < Storage = ParentFormerStorage < > , Formed
//   = Parent < > > , Definition : former :: FormerDefinition, Definition :: Types
//   : former :: FormerDefinitionTypes < Storage = ParentFormerStorage < > > ,
//   {
//       pub fn preform(self) -> < Definition :: Types as former ::
//       FormerDefinitionTypes > :: Formed
//       { former :: StoragePreform :: preform(self.storage) }
//   } #[automatically_derived] impl < Definition, > ParentFormer < Definition, >
//   where Definition : former :: FormerDefinition, Definition :: Types : former ::
//   FormerDefinitionTypes < Storage = ParentFormerStorage < > , Formed = Parent <
//   > > ,
//   {
//       #[doc = r""]
//       #[doc = r" Finish setting options and call perform on formed entity."]
//       #[doc = r""]
//       #[doc =
//       r" If `perform` defined then associated method is called and its result returned instead of entity."]
//       #[doc =
//       r" For example `perform()` of structure with : `#[ perform( fn after1() -> &str > )` returns `&str`."]
//       #[doc = r""] #[inline(always)] pub fn perform(self) -> < Definition ::
//       Types as former :: FormerDefinitionTypes > :: Formed
//       { let result = self.form(); return result; }
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
//           begin_precise(None, context, on_end)
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
//   impl < SuperFormer, T > ParentAsSubformerEnd < SuperFormer > for T where Self
//   : former :: FormingEnd < ParentFormerDefinitionTypes < SuperFormer,
//   SuperFormer > , > , {}
//   #[doc =
//   "Callback to return original former after forming of container for `$Parent` is done.#\n\nCallback replace content of container assigning new content from subformer's storage."]
//   pub struct ParentFormerAssignChildrenEnd < Definition >
//   { _phantom : core :: marker :: PhantomData < (Definition,) > , } impl <
//   Definition > Default for ParentFormerAssignChildrenEnd < Definition >
//   {
//       #[inline(always)] fn default() -> Self
//       { Self { _phantom : core :: marker :: PhantomData, } }
//   } #[automatically_derived] impl < Definition, > former :: FormingEnd < former
//   :: VectorDefinition < Child, ParentFormer < Definition, > , ParentFormer <
//   Definition, > , former :: NoEnd > , > for ParentFormerAssignChildrenEnd <
//   Definition > where Definition : former :: FormerDefinition, Definition ::
//   Types : former :: FormerDefinitionTypes < Storage = ParentFormerStorage < > >
//   ,
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
//   } #[doc = r" Handles the completion of an element of subformer's container."]
//   pub struct ParentFormerAddChildrenEnd < Definition >
//   { _phantom : core :: marker :: PhantomData < fn(Definition) > , } impl <
//   Definition > Default for ParentFormerAddChildrenEnd < Definition >
//   {
//       #[inline(always)] fn default() -> Self
//       { Self { _phantom : core :: marker :: PhantomData, } }
//   } impl < Types2, Definition > former :: FormingEnd < Types2, > for
//   ParentFormerAddChildrenEnd < Definition > where Definition : former ::
//   FormerDefinition, Definition :: Types : former :: FormerDefinitionTypes <
//   Storage = < Parent < > as former :: EntityToStorage > :: Storage, > , Types2 :
//   former :: FormerDefinitionTypes < Storage = < < Vec < Child > as former ::
//   ContainerAdd > :: Element as former :: EntityToStorage > :: Storage, Formed =
//   ParentFormer < Definition, > , Context = ParentFormer < Definition, > , > ,
//   {
//       #[inline(always)] fn
//       call(& self, substorage : Types2 :: Storage, super_former : core :: option
//       :: Option < Types2 :: Context > ,) -> Types2 :: Formed
//       {
//           let mut super_former = super_former.unwrap(); if
//           super_former.storage.children.is_none()
//           { super_former.storage.children = Some(Default :: default()); } if let
//           Some(ref mut field) = super_former.storage.children
//           {
//               former :: ContainerAdd ::
//               add(field, former :: StoragePreform :: preform(substorage));
//           } super_former
//       }
//   }

// == end of generated

// xxx
include!( "./only_test/subformer_subform.rs" );
include!( "./only_test/subformer_container.rs" );
