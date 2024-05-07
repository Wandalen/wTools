#[ allow( unused_imports ) ]
use super::*;

// use std::collections::HashMap;
// use std::collections::HashSet;

// #[ derive( Default, Debug, PartialEq, former::Former ) ]
// #[ derive( Default, Debug, PartialEq, former::Former ) ] #[ debug ]
#[ derive( Default, Debug, PartialEq ) ]
pub struct Struct1
{
  // #[ container( definition = former::VectorDefinition ) ]
  vec_1 : Vec< String >,
  // #[ container( definition = former::HashMapDefinition ) ]
  // hashmap_1 : std::collections::HashMap< String, String >,
  // #[ container( definition = former::HashSetDefinition ) ]
  // hashset_1 : std::collections::HashSet< String >,
  // xxx
}

// == generated begin

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
  {
      type Definition = Struct1FormerDefinition < __Context, __Formed, __End > ;
      type Types = Struct1FormerDefinitionTypes < __Context, __Formed > ;
  } impl < __Context, __Formed > former :: EntityToDefinitionTypes < __Context,
  __Formed > for Struct1 < > where
  { type Types = Struct1FormerDefinitionTypes < __Context, __Formed > ; }
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
  } impl < __Context, __Formed, > former :: FormerMutator for
  Struct1FormerDefinitionTypes < __Context, __Formed, > where {}
  #[doc = "Container of a corresponding former."]
  #[allow(explicit_outlives_requirements)] pub struct Struct1FormerStorage < >
  where
  {
      #[doc = r" A field"] pub vec_1 : :: core :: option :: Option < Vec <
      String > > ,
  } impl < > :: core :: default :: Default for Struct1FormerStorage < > where
  {
      #[inline(always)] fn default() -> Self
      { Self { vec_1 : :: core :: option :: Option :: None, } }
  } impl < > former :: Storage for Struct1FormerStorage < > where
  { type Formed = Struct1 < > ; } impl < > former :: StoragePreform for
  Struct1FormerStorage < > where
  {
      type Preformed = Struct1 < > ; fn preform(mut self) -> Self :: Preformed
      {
          let vec_1 = if self.vec_1.is_some() { self.vec_1.take().unwrap() }
          else
          {
              {
                  trait MaybeDefault < T >
                  {
                      fn maybe_default(self : & Self) -> T
                      { panic! ("Field 'vec_1' isn't initialized") }
                  } impl < T > MaybeDefault < T > for & :: core :: marker ::
                  PhantomData < T > {} impl < T > MaybeDefault < T > for :: core
                  :: marker :: PhantomData < T > where T : :: core :: default ::
                  Default,
                  { fn maybe_default(self : & Self) -> T { T :: default() } }
                  (& :: core :: marker :: PhantomData :: < Vec < String >
                  >).maybe_default()
              }
          }; let result = Struct1 :: < > { vec_1, }; return result;
      }
  }
  #[doc =
  "\nStructure to form [Struct1]. Represents a forming entity designed to construct objects through a builder pattern.\n\nThis structure holds temporary storage and context during the formation process and\nutilizes a defined end strategy to finalize the object creation. It facilitates the flexible\nconstruction of complex objects by allowing step-by-step configuration.\n```\n\nuse former::Former;\n#[ derive( Former ) ]\npub struct Struct1\n{\n  #[default( 31 ) ]\n  field1 : i32,\n}\n\n```\n"]
  pub struct Struct1Former < Definition = Struct1FormerDefinition < (), Struct1
  < > , former :: ReturnPreformed > , > where Definition : former ::
  FormerDefinition < Storage = Struct1FormerStorage < > > , Definition :: Types
  : former :: FormerDefinitionTypes < Storage = Struct1FormerStorage < > > ,
  {
      #[doc =
      r" Temporary storage for all fields during the formation process. It contains"]
      #[doc =
      r"   partial data that progressively builds up to the final object."] pub
      storage : Definition :: Storage,
      #[doc =
      r" An optional context providing additional data or state necessary for custom"]
      #[doc =
      r"   formation logic or to facilitate this former's role as a subformer within another former."]
      pub context : core :: option :: Option < Definition :: Context > ,
      #[doc =
      r" An optional closure or handler that is invoked to transform the accumulated"]
      #[doc =
      r"   temporary storage into the final object structure once formation is complete."]
      pub on_end : core :: option :: Option < Definition :: End > ,
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
          let on_end = self.on_end.take().unwrap(); let mut context =
          self.context.take(); < Definition :: Types as former :: FormerMutator
          > :: form_mutation(& mut self.storage, & mut context); former ::
          FormingEnd :: < Definition :: Types > ::
          call(& on_end, self.storage, context)
      }
      #[doc =
      "Container setter for the 'vec_1' field. Method _vec_1_container_former unlike method vec_1 accept custom container subformer."]
      #[inline(always)] pub fn _vec_1_container_former < Former2 > (self) ->
      Former2 where Former2 : former :: FormerBegin < former :: VectorDefinition
      < String, Self, Self, Struct1FormerAssignVec1End < Definition > , > , > ,
      former :: VectorDefinition < String, Self, Self,
      Struct1FormerAssignVec1End < Definition > , > : former :: FormerDefinition
      < Storage : former :: ContainerAdd < Entry = < Vec < String > as former ::
      Container > :: Entry > , Context = Struct1Former < Definition, > , End =
      Struct1FormerAssignVec1End < Definition > , > ,
      {
          Former2 ::
          former_begin(None, Some(self), Struct1FormerAssignVec1End :: <
          Definition > :: default())
      }

      #[inline(always)]
      pub fn vec_1(self) -> former :: ContainerSubformer ::
      < String, former :: VectorDefinition < String, Self, Self, Struct1FormerAssignVec1End < Definition > , > , >
      where
        former :: VectorDefinition < String, Self, Self, Struct1FormerAssignVec1End < Definition > , > : former :: FormerDefinition < Storage : former :: ContainerAdd < Entry = < Vec < String > as former :: Container > :: Entry >
        , Context = Struct1Former < Definition > , End = Struct1FormerAssignVec1End < Definition > , > ,
      {
          self._vec_1_container_former :: < former :: ContainerSubformer :: <
          String, former :: VectorDefinition < String, Self, Self,
          Struct1FormerAssignVec1End < Definition > , > , > > ()
      }

  }

  impl < Definition, > Struct1Former < Definition, > where Definition : former
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

  pub struct Struct1FormerAssignVec1End< Definition >
  {
    _phantom : core::marker::PhantomData< (Definition,) >,
  }

  impl< Definition > Default for Struct1FormerAssignVec1End< Definition >
  {
    #[inline(always)]
    fn default() -> Self
    {
      Self
      {
        _phantom : core::marker::PhantomData,
      }
    }
  }

  // impl< Definition, > former::FormingEnd
  // < former::VectorDefinitionTypes< String, Struct1Former< Definition >, Struct1Former< Definition > > >
  // for Struct1FormerAssignVec1End< Definition >
  // where
  //   Definition : former::FormerDefinition< Storage = Struct1FormerStorage >,
  //   Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage >,
  // {
  //   #[ inline( always ) ]
  //   fn call( &self, storage : collection_tools::Vec< String >, super_former : Option< Struct1Former< Definition > > )
  //   -> Struct1Former< Definition, >
  //   {
  //     let mut super_former = super_former.unwrap();
  //     if let Some( ref mut field ) = super_former.storage.vec_1
  //     {
  //       former::ContainerAssign::assign( field, storage );
  //     }
  //     else
  //     {
  //       super_former.storage.vec_1 = Some( storage );
  //     }
  //     super_former
  //   }
  // }

  // xxx

  #[automatically_derived]
  impl< Definition, > former::FormingEnd
  <
    <
      former::VectorDefinition
      <
        String,
        Struct1Former< Definition, >,
        Struct1Former< Definition, >,
      >
      as former::FormerDefinition
    > :: Types
  >
  for Struct1FormerAssignVec1End< Definition >
  where
    Definition : former::FormerDefinition< Storage = Struct1FormerStorage< > >,
    Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage< > >,
  {
    #[inline(always)]
    fn call( &self, storage : Vec< String >, super_former : Option< Struct1Former< Definition, >, > )
    -> Struct1Former< Definition, >
    {
      let mut super_former = super_former.unwrap();
      if let Some( ref mut field ) = super_former.storage.vec_1
      {
        former::ContainerAssign::assign( field, storage );
      }
      else
      {
        super_former.storage.vec_1 = Some( storage );
      }
      super_former
    }
  }

// == generated end

// include!( "./only_test/containers_with_subformer.rs" );
// xxx