#![ allow( dead_code ) ]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct TemplateParameterDescriptor
{
  name : String,
  is_mandatory : bool,
}

/// Parameters required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct TemplateParameters
{
  #[ subformer( former::VectorDefinition ) ]
  descriptors : Vec< TemplateParameterDescriptor >,
}

impl< Definition > former::FormerBegin< Definition >
for TemplateParameterDescriptorFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = TemplateParameterDescriptorFormerStorage >,
{

  #[ inline( always ) ]
  fn _begin
  (
    storage : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
    on_end : Definition::End,
  ) -> Self
  {
    debug_assert!( storage.is_none() );
    Self::begin_coercing( None, context, on_end )
  }

}

impl< Definition > TemplateParametersFormer< Definition >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = TemplateParametersFormerStorage >,
{

  #[ inline( always ) ]
  pub fn descriptor3< Former2, Definition2, Types2 >( self ) ->
  Former2
  where
    Types2 : former::FormerDefinitionTypes
    <
      Storage = TemplateParameterDescriptorFormerStorage,
      Formed = Self,
      Context = Self,
    >,
    Definition2 : former::FormerDefinition< Types = Types2, End = former::FormingEndClosure< Types2 > >,
    // Definition2 : former::FormerDefinition< Types = Types2 >,
    Definition2::End : former::FormingEnd< Definition2::Types >,
    Former2 : former::FormerBegin
    <
      Definition2,
    >,
  {
    let on_end = | descriptor : TemplateParameterDescriptorFormerStorage, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      if super_former.storage.descriptors.is_none()
      {
        super_former.storage.descriptors = Some( Default::default() );
      }
      if let Some( ref mut descriptors ) = super_former.storage.descriptors
      {
        former::ContainerAdd::add( descriptors, former::StoragePreform::preform( descriptor ) );
      }
      super_former
    };
    Former2::_begin( None, Some( self ), former::FormingEndClosure::new( on_end ) )
    // Former2::_begin( None, Some( self ), on_end )
  }

  // xxx2 : move to a trait and make easier to use subformer, trait with generic interface of a container should help

  #[ inline( always ) ]
  pub fn descriptor( self, name : &str ) ->
  TemplateParameterDescriptorFormer
  <
    TemplateParameterDescriptorFormerDefinition
    <
      Self,
      Self,
      impl TemplateParameterDescriptorSubformerEnd< Self >,
      // former::FormingEndClosure< TemplateParameterDescriptorFormerDefinitionTypes< Self, Self > >,
    >
  >
  {
    self.descriptor3::
    <
      TemplateParameterDescriptorFormer< _ >,
      _,
      _,
    >()
    .name( name )
  }

}

#[ test ]
fn basic()
{

  let got = TemplateParameters::former()
  .descriptors()
    .add( TemplateParameterDescriptor::former().name( "a" ).form() )
    .add( TemplateParameterDescriptor::former().name( "b" ).form() )
    .end()
  .form();

  let descriptors = vec!
  [
    TemplateParameterDescriptor { name : "a".to_string(), is_mandatory : false },
    TemplateParameterDescriptor { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = TemplateParameters { descriptors };
  a_id!( got, exp );

}
