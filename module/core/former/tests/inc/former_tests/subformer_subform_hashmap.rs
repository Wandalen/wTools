#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashMap;

// Command struct with Former derived for builder pattern support
#[ derive( Debug, PartialEq, former::Former ) ]
pub struct Command
{
  name : String,
  description : String,
}

// Aggregator struct to hold commands
// #[ derive( Debug, PartialEq, former::Former ) ]
// #[ debug ]
#[ derive( Debug, PartialEq ) ]
pub struct Aggregator
{
  // #[ subform ]
  // #[ setter( false ) ]
  command : HashMap< String, Command >,
}

// // Use CommandFormer as custom subformer for AggregatorFormer to add commands by name.
// impl< Definition > AggregatorFormer< Definition >
// where
//   Definition : former::FormerDefinition< Storage = < Aggregator as former::EntityToStorage >::Storage >,
// {
//
//   #[ inline( always ) ]
//   pub fn command( self, name : &str ) -> CommandAsSubformer< Self, impl CommandAsSubformerEnd< Self > >
//   {
//     self._command_add_subformer::< CommandFormer< _ >, _, >()
//     .name( name )
//   }
//
// }

// // Use CommandFormer as custom subformer for AggregatorFormer to add commands by name.
// impl< Definition > AggregatorFormer< Definition >
// where
//   End : former::FormingEnd< Aggregator, Context >,
// {
//   #[ inline( always ) ]
//   pub fn command< IntoName >( self, name : IntoName ) -> CommandFormer< Self, impl former::FormingEnd< Command, Self > >
//   where
//     IntoName : core::convert::Into< String >,
//   {
//     let on_end = | command : Command, super_former : core::option::Option< Self > | -> Self
//     {
//       let mut super_former = super_former.unwrap();
//       if let Some( ref mut commands ) = super_former.storage.command
//       {
//         commands.insert( command.name.clone(), command );
//       }
//       else
//       {
//         let mut commands: HashMap< String, Command > = Default::default();
//         commands.insert( command.name.clone(), command );
//         super_former.storage.command = Some( commands );
//       }
//       super_former
//     };
//     let former = CommandFormer::begin( None, Some( self ), on_end );
//     former.name( name )
//   }
//   // xxx : review
// }

// == begin of generated

#[automatically_derived] impl < > Aggregator < > where
{
    #[doc = r""]
    #[doc =
    r" Make former, variation of builder pattern to form structure defining values of fields step by step."]
    #[doc = r""] #[inline(always)] pub fn former() -> AggregatorFormer <
    AggregatorFormerDefinition < (), Aggregator < > , former ::
    ReturnPreformed > >
    {
        AggregatorFormer :: < AggregatorFormerDefinition < (), Aggregator < >
        , former :: ReturnPreformed > > ::
        new_coercing(former :: ReturnPreformed)
    }
}

impl < Definition > former :: EntityToFormer < Definition >
for Aggregator <
>
where Definition : former :: FormerDefinition < Storage =
AggregatorFormerStorage < > > ,
{ type Former = AggregatorFormer < Definition > ; } impl < > former ::
EntityToStorage for Aggregator < > where
{ type Storage = AggregatorFormerStorage < > ; } impl < __Context, __Formed,
__End > former :: EntityToDefinition < __Context, __Formed, __End > for
Aggregator < > where __End : former :: FormingEnd <
AggregatorFormerDefinitionTypes < __Context, __Formed > > ,
{
    type Definition = AggregatorFormerDefinition < __Context, __Formed, __End
    > ;
} #[derive(Debug)] pub struct AggregatorFormerDefinitionTypes < __Context =
(), __Formed = Aggregator < > , > where
{
    _phantom : core :: marker :: PhantomData <
    (* const __Context, * const __Formed) > ,
} impl < __Context, __Formed, > :: core :: default :: Default for
AggregatorFormerDefinitionTypes < __Context, __Formed, > where
{
    fn default() -> Self
    { Self { _phantom : core :: marker :: PhantomData, } }
} impl < __Context, __Formed, > former :: FormerDefinitionTypes for
AggregatorFormerDefinitionTypes < __Context, __Formed, > where
{
    type Storage = AggregatorFormerStorage < > ; type Formed = __Formed; type
    Context = __Context;
} #[derive(Debug)] pub struct AggregatorFormerDefinition < __Context = (),
__Formed = Aggregator < > , __End = former :: ReturnPreformed, > where
{
    _phantom : core :: marker :: PhantomData <
    (* const __Context, * const __Formed, * const __End) > ,
} impl < __Context, __Formed, __End, > :: core :: default :: Default for
AggregatorFormerDefinition < __Context, __Formed, __End, > where
{
    fn default() -> Self
    { Self { _phantom : core :: marker :: PhantomData, } }
} impl < __Context, __Formed, __End, > former :: FormerDefinition for
AggregatorFormerDefinition < __Context, __Formed, __End, > where __End :
former :: FormingEnd < AggregatorFormerDefinitionTypes < __Context, __Formed,
> > ,
{
    type Types = AggregatorFormerDefinitionTypes < __Context, __Formed, > ;
    type End = __End; type Storage = AggregatorFormerStorage < > ; type Formed
    = __Formed; type Context = __Context;
} #[doc = "Container of a corresponding former."]
#[allow(explicit_outlives_requirements)] pub struct AggregatorFormerStorage <
> where
{
    #[doc = r" A field"] pub command : :: core :: option :: Option < HashMap <
    String, Command > > ,
} impl < > :: core :: default :: Default for AggregatorFormerStorage < > where
{
    #[inline(always)] fn default() -> Self
    { Self { command : :: core :: option :: Option :: None, } }
} impl < > former :: Storage for AggregatorFormerStorage < > where
{ type Formed = Aggregator < > ; } impl < > former :: StoragePreform for
AggregatorFormerStorage < > where
{
    type Preformed = Aggregator < > ; fn preform(mut self) -> Self ::
    Preformed
    {
        let command = if self.command.is_some()
        { self.command.take().unwrap() } else
        {
            {
                trait MaybeDefault < T >
                {
                    fn maybe_default(self : & Self) -> T
                    { panic! ("Field 'command' isn't initialized") }
                } impl < T > MaybeDefault < T > for & :: core :: marker ::
                PhantomData < T > {} impl < T > MaybeDefault < T > for :: core
                :: marker :: PhantomData < T > where T : :: core :: default ::
                Default,
                { fn maybe_default(self : & Self) -> T { T :: default() } }
                (& :: core :: marker :: PhantomData :: < HashMap < String,
                Command > >).maybe_default()
            }
        }; let result = Aggregator :: < > { command, }; return result;
    }
}
#[doc =
" Object to form [Aggregator]. If field's values is not set then default value of the field is set.\n\nFor specifying custom default value use attribute `default`. For example:\n```\n\nuse former::Former;\n#[ derive( Former ) ]\npub struct Struct1\n{\n  #[default( 31 ) ]\n  field1 : i32,\n}\n\n```\n"]
pub struct AggregatorFormer < Definition = AggregatorFormerDefinition < (),
Aggregator < > , former :: ReturnPreformed > , > where Definition : former ::
FormerDefinition < Storage = AggregatorFormerStorage < > > , Definition ::
Types : former :: FormerDefinitionTypes < Storage = AggregatorFormerStorage <
> > ,
{
    storage : Definition :: Storage, context : core :: option :: Option <
    Definition :: Context > , on_end : core :: option :: Option < Definition
    :: End > ,
} #[automatically_derived] impl < Definition, > AggregatorFormer < Definition,
> where Definition : former :: FormerDefinition < Storage =
AggregatorFormerStorage < > > , Definition :: Types : former ::
FormerDefinitionTypes < Storage = AggregatorFormerStorage < > > ,
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
    } #[doc = "Setter for the 'command' field."] #[inline] pub fn command <
    Src > (mut self, src : Src) -> Self where Src : :: core :: convert :: Into
    < HashMap < String, Command > > ,
    {
        debug_assert! (self.storage.command.is_none()); self.storage.command =
        :: core :: option :: Option ::
        Some(:: core :: convert :: Into :: into(src)); self
    } #[doc = r" Custom setter which produce container element subformer."]
    #[inline(always)] pub fn _command_add_subformer < Former2, Definition2 >
    (self) -> Former2 where Definition2 : former :: FormerDefinition < End =
    AggregatorFormerAddCommandEnd < Definition > , Storage = < < HashMap <
    String, Command > as former :: Container > :: Val as former ::
    EntityToStorage > :: Storage, Formed = Self, Context = Self, > ,
    Definition2 :: Types : former :: FormerDefinitionTypes < Storage = < <
    HashMap < String, Command > as former :: Container > :: Val as former ::
    EntityToStorage > :: Storage, Formed = Self, Context = Self, > , Former2 :
    former :: FormerBegin < Definition2 > ,
    {
        Former2 ::
        former_begin(None, Some(self), AggregatorFormerAddCommandEnd ::
        default())
    }
} impl < Definition, > AggregatorFormer < Definition, > where Definition :
former :: FormerDefinition < Storage = AggregatorFormerStorage < > , Formed =
Aggregator < > > , Definition :: Types : former :: FormerDefinitionTypes <
Storage = AggregatorFormerStorage < > , Formed = Aggregator < > > , Definition
: former :: FormerDefinition < Storage = AggregatorFormerStorage < > > ,
Definition :: Types : former :: FormerDefinitionTypes < Storage =
AggregatorFormerStorage < > > ,
{
    pub fn preform(self) -> < Definition :: Types as former ::
    FormerDefinitionTypes > :: Formed
    { former :: StoragePreform :: preform(self.storage) }
} #[automatically_derived] impl < Definition, > AggregatorFormer < Definition,
> where Definition : former :: FormerDefinition < Storage =
AggregatorFormerStorage < > , Formed = Aggregator < > , > , Definition ::
Types : former :: FormerDefinitionTypes < Storage = AggregatorFormerStorage <
> , Formed = Aggregator < > , > ,
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
} impl < Definition > former :: FormerBegin < Definition > for
AggregatorFormer < Definition, > where Definition : former :: FormerDefinition
< Storage = AggregatorFormerStorage < > > ,
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
pub type AggregatorAsSubformer < __Superformer, __End > = AggregatorFormer <
AggregatorFormerDefinition < __Superformer, __Superformer, __End, > , > ;
#[doc =
"Alias for trait former::FormingEnd with context and formed the same type and definition of structure [`$(stru)`]. Use as subformer end of a field during process of forming of super structure."]
pub trait AggregatorAsSubformerEnd < SuperFormer > where Self : former ::
FormingEnd < AggregatorFormerDefinitionTypes < SuperFormer, SuperFormer > , >
, {} impl < SuperFormer, __T > AggregatorAsSubformerEnd < SuperFormer > for
__T where Self : former :: FormingEnd < AggregatorFormerDefinitionTypes <
SuperFormer, SuperFormer > , > , {}
#[doc = r" Handles the completion of an element of subformer's container."]
pub struct AggregatorFormerAddCommandEnd < Definition >
{ _phantom : core :: marker :: PhantomData < fn(Definition) > , } impl <
Definition > Default for AggregatorFormerAddCommandEnd < Definition >
{
    #[inline(always)] fn default() -> Self
    { Self { _phantom : core :: marker :: PhantomData, } }
}

impl< Types2, Definition > former::FormingEnd< Types2, >
for AggregatorFormerAddCommandEnd< Definition >
where
  Definition : former::FormerDefinition< Storage = < Aggregator< > as former::EntityToStorage >::Storage, >,
  Types2 :former::FormerDefinitionTypes
  <
    Storage = < < HashMap< String, Command > as former::Container >::Val as former::EntityToStorage >::Storage,
    Formed = AggregatorFormer< Definition, >,
    Context = AggregatorFormer< Definition, >,
  >,
{
  #[ inline( always ) ]
  fn call( &self, substorage : Types2::Storage, super_former : core::option::Option< Types2::Context > ) -> Types2::Formed
  {
    let mut super_former = super_former.unwrap();
    if super_former.storage.command.is_none()
    {
      super_former.storage.command = Some( Default::default() );
    }
    if let Some( ref mut field ) = super_former.storage.command
    {
      former::ContainerAdd::add
      (
        field,
        < Command as former::ValToElement< HashMap< String, Command > > >
        ::val_to_element( former::StoragePreform::preform( substorage ) ),
      );
    }
    super_former
  }
}

// impl former::ContainerValToElement for collection_tools::HashMap< String, Command >
// {
//   fn val_to_element( val : Self::Val ) -> Self::Element
//   {
//     ( val.name.clone(), val )
//   }
// }

impl former::ValToElement< HashMap< String, Command > > for Command
{
  type Element = ( String, Command );
  #[ inline ]
  fn val_to_element( self ) -> Self::Element
  {
    ( self.name.clone(), self )
  }
}

// pub trait ValToElement
// {
//   type Element;
//
//   /// Convert val to element. For Vector `Val` and `Element` is the same type, but for `HashMap` `Element` is pair of key-value and `Val` is value itself.
//   fn val_to_element( self ) -> Self::Element;
//
// }

// == end of generated

// #[ test ]
// fn basic()
// {
//
//   let ca = Aggregator::former()
//   .command( "echo" )
//     .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
//     .end()
//   .command( "exit" )
//     .description( "just exit" ) // Sets additional properties using using custom subformer
//     .end()
//   .form();
//
// }
// xxx