#![ allow( dead_code ) ]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct TemplateParameterDescriptor
{
  descriptor : String,
  is_mandatory : bool,
}

/// Parameters required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct TemplateParameters
{
  // #[ debug = the_module::VectorSubformer, descriptor, descriptor( name ) ]
  // #[ subformer( the_module::VectorSubformer ) ]
  #[ subformer( former::VectorDefinition ) ]
  descriptors : Vec< TemplateParameterDescriptor >,

  // #[ subformer_setter = the_module::VectorSubformer ]
  // pub fn descriptor( self, name : &str )
  // {
  //   descriptor( name )
  // }

}

// impl< Definition > TemplateParametersFormer< Definition >
// where
//   Definition : former::FormerDefinition,
//   Definition::Types : former::FormerDefinitionTypes< Storage = TemplateParameterDescriptorFormerStorage >,
// {

// impl< Context, End > former::FormerBegin< TemplateParameterDescriptorFormerStorage, TemplateParameterDescriptor, Context >
// for TemplateParameterDescriptorFormer< Context, End >
// where
//   End : the_module::FormingEnd< TemplateParameterDescriptor, Context >,
impl< Definition > former::FormerBegin< Definition >
for TemplateParameterDescriptorFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = TemplateParameterDescriptorFormerStorage >,
  // End : the_module::FormingEnd< TemplateParameterDescriptor, Context >,
{

  // type End = End;

  #[ inline( always ) ]
  fn _begin
  (
    // storage : core::option::Option< TemplateParameterDescriptorFormerStorage >,
    // context : core::option::Option< Context >,
    // on_end : End,
    storage : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
    on_end : Definition::End,
  ) -> Self
  {
    debug_assert!( storage.is_none() );
    Self::begin( None, context, on_end )
  }

}

// impl< Context, End > TemplateParametersFormer< Context, End >
// where
//   End : former::FormingEnd< TemplateParameters, Context >,
impl< Definition > TemplateParametersFormer< Definition >
where
  // End : former::FormingEnd< TemplateParameters, Context >,
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = TemplateParametersFormerStorage >,
{

  // pub trait FormerDefinitionTypes : Sized
  // {
  //   type Storage : Default;
  //   type Formed;
  //   type Context;
  // }

// xxx : uncomment
  #[ inline( always ) ]
  pub fn descriptor3< Former2, Definition2, Types2, End >( self ) ->
  Former2
  where
    Types2 : former::FormerDefinitionTypes
    <
      Storage = TemplateParameterDescriptor,
      Formed = Self,
      Context = Self,
    >,
    // Definition2 : former::FormerDefinition< End = former::FormingEndClosure< Definition2::Types > >,
    Definition2 : former::FormerDefinition< End = former::FormingEndClosure< Types2 >, Types = Types2 >,
    // Definition2 : former::FormerDefinition< End = End >,
    Definition2 : former::FormerDefinition,
    Definition2::End : former::FormingEnd< Definition2::Types >,

    // Definition2 : former::FormerDefinition< Types = Former2::Types, End = End >,
    // Former2 : former::FormerBegin< Definition2 >,
    // End : former::FormingEnd< Former2::Types >,

    // Definition2::Types : former::FormerDefinitionTypes
    // <
    //   Storage = TemplateParameterDescriptorFormerStorage,
    //   Formed = TemplateParameterDescriptor,
    //   Context = Self,
    // >,

    Former2 : former::FormerBegin
    <
      Definition2,
      // TemplateParameterDescriptorFormerStorage,
      // TemplateParameterDescriptor,
      // Self,
      // End = former::FormingEndClosure< TemplateParameterDescriptor, Self >,
    >,
    // FieldContainer : ContainerAdd,
  {

    let on_end = | descriptor : TemplateParameterDescriptor, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      if super_former.storage.descriptors.is_none()
      {
        super_former.storage.descriptors = Some( Default::default() );
      }
      if let Some( ref mut descriptors ) = super_former.storage.descriptors
      {
        former::ContainerAdd::add( descriptors, descriptor );
      }
      super_former
    };
    Former2::_begin( None, Some( self ), former::FormingEndClosure::new( on_end ) )
  }

  // xxx2 : move to a trait and make easier to use subformer, trait with generic interface of a container should help

// xxx : uncomment
  // #[ inline( always ) ]
  // pub fn descriptor( self, name : &str ) ->
  // // TemplateParameterDescriptorFormer< Self, impl former::FormingEnd< TemplateParameterDescriptor, Self > >
  // TemplateParameterDescriptorFormer< Definition >
  // {
  //   self.descriptor3::< TemplateParameterDescriptorFormer< _ > >().descriptor( name )
  // }

}

// xxx : uncomment
// #[ test ]
// fn basic()
// {
//
//   let got = TemplateParameters::former()
//   .descriptors()
//     .push( TemplateParameterDescriptor::former().descriptor( "a" ).form() )
//     .push( TemplateParameterDescriptor::former().descriptor( "b" ).form() )
//     .end()
//   .form();
//
//   let descriptors = vec!
//   [
//     TemplateParameterDescriptor { descriptor : "a".to_string(), is_mandatory : false },
//     TemplateParameterDescriptor { descriptor : "b".to_string(), is_mandatory : false },
//   ];
//   let exp = TemplateParameters { descriptors };
//   a_id!( got, exp );
//
// }
