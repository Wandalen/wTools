#![ allow( dead_code ) ]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct TemplateParameterDefinition
{
  descriptor : String,
  is_mandatory : bool,
}

/// Parameters required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct TemplateParameters
{
  // #[ debug = the_module::VectorSubformer, descriptor, descriptor( name ) ]
  #[ subformer( the_module::VectorSubformer ) ]
  descriptors : Vec< TemplateParameterDefinition >,

  // #[ subformer_setter = the_module::VectorSubformer ]
  // pub fn descriptor( self, name : &str )
  // {
  //   descriptor( name )
  // }

}

impl< Context, End > former::FormerBegin< TemplateParameterDefinitionFormerStorage, TemplateParameterDefinition, Context >
for TemplateParameterDefinitionFormer< Context, End >
where
  End : the_module::FormingEnd< TemplateParameterDefinition, Context >,
{
  type End = End;

  #[ inline( always ) ]
  fn _begin
  (
    storage : core::option::Option< TemplateParameterDefinitionFormerStorage >, /* xxx2 : that should be storage */
    context : core::option::Option< Context >,
    on_end : End,
  ) -> Self
  {
    debug_assert!( storage.is_none() );
    Self::begin( None, context, on_end )
  }

}

impl< Context, End > TemplateParametersFormer< Context, End >
where
  End : former::FormingEnd< TemplateParameters, Context >,
{

  #[ inline( always ) ]
  pub fn descriptor3< Former2 >( self ) ->
  Former2
  where
    Former2 : former::FormerBegin
    <
      TemplateParameterDefinitionFormerStorage,
      TemplateParameterDefinition,
      Self,
      End = former::FormingEndWrapper< TemplateParameterDefinition, Self >,
    >,
    // FieldContainer : ContainerAdd,
  {
    let on_end = | descriptor : TemplateParameterDefinition, super_former : core::option::Option< Self > | -> Self
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
    Former2::_begin( None, Some( self ), former::FormingEndWrapper::new( on_end ) )
  }

  // xxx2 : move to a trait and make easier to use subformer, trait with generic interface of a container should help

  #[ inline( always ) ]
  pub fn descriptor( self, name : &str ) ->
  TemplateParameterDefinitionFormer< Self, impl former::FormingEnd< TemplateParameterDefinition, Self > >
  {
    self.descriptor3::< TemplateParameterDefinitionFormer< _, _ > >().descriptor( name )
  }

}

#[ test ]
fn basic()
{

  let got = TemplateParameters::former()
  .descriptors()
    .push( TemplateParameterDefinition::former().descriptor( "a" ).form() )
    .push( TemplateParameterDefinition::former().descriptor( "b" ).form() )
    .end()
  .form();

  let descriptors = vec!
  [
    TemplateParameterDefinition { descriptor : "a".to_string(), is_mandatory : false },
    TemplateParameterDefinition { descriptor : "b".to_string(), is_mandatory : false },
  ];
  let exp = TemplateParameters { descriptors };
  a_id!( got, exp );

}
