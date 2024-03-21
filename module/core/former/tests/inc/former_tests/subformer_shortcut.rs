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
  #[ subformer( the_module::VectorSubformer ) ]
  descriptors : Vec< TemplateParameterDescriptor >,

  // #[ subformer_setter = the_module::VectorSubformer ]
  // pub fn descriptor( self, name : &str )
  // {
  //   descriptor( name )
  // }

}

impl< Context, End > former::FormerBegin< TemplateParameterDescriptorFormerStorage, TemplateParameterDescriptor, Context >
for TemplateParameterDescriptorFormer< Context, End >
where
  End : the_module::FormingEnd< TemplateParameterDescriptor, Context >,
{
  type End = End;

  #[ inline( always ) ]
  fn _begin
  (
    storage : core::option::Option< TemplateParameterDescriptorFormerStorage >, /* xxx2 : that should be storage */
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
    Former2 : former::FormerBegin< TemplateParameterDescriptorFormerStorage, TemplateParameterDescriptor, Self, End = former::FormingEndWrapper< TemplateParameterDescriptor, Self > >,
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
    Former2::_begin( None, Some( self ), former::FormingEndWrapper::new( on_end ) )
  }

  // xxx2 : move to a trait and make easier to use subformer, trait with generic interface of a container should help

  #[ inline( always ) ]
  pub fn descriptor( self, name : &str ) ->
  TemplateParameterDescriptorFormer< Self, impl former::FormingEnd< TemplateParameterDescriptor, Self > >
  {
    self.descriptor3::< TemplateParameterDescriptorFormer< _, _ > >().descriptor( name )
  }

}

#[ test ]
fn basic()
{

  let got = TemplateParameters::former()
  .descriptors()
    .push( TemplateParameterDescriptor::former().descriptor( "a" ).form() )
    .push( TemplateParameterDescriptor::former().descriptor( "b" ).form() )
    .end()
  .form();

  let descriptors = vec!
  [
    TemplateParameterDescriptor { descriptor : "a".to_string(), is_mandatory : false },
    TemplateParameterDescriptor { descriptor : "b".to_string(), is_mandatory : false },
  ];
  let exp = TemplateParameters { descriptors };
  a_id!( got, exp );

}
