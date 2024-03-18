#![ allow( dead_code ) ]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct TemplateParameterDescriptor
{
  parameter : String,
  is_mandatory : bool,
}

/// Parameters required for the template.
#[ derive( Debug, Default, PartialEq, the_module::Former ) ]
pub struct TemplateParameters
{
  #[ subformer( the_module::VectorSubformer ) ]
  // #[ subformer_vec => parameter => parameter( name ) ]
  descriptors : Vec< TemplateParameterDescriptor >
}

impl< Context, End > TemplateParametersFormer< Context, End >
where
  End : former::ToSuperFormer< TemplateParameters, Context >,
{
  #[ inline( always ) ]
  pub fn parameter( self, name : &str ) ->
  TemplateParameterDescriptorFormer< Self, impl former::ToSuperFormer< TemplateParameterDescriptor, Self > >
  {
    let on_end = | descriptor : TemplateParameterDescriptor, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      if let Some( ref mut descriptors ) = super_former.container.descriptors
      {
        descriptors.push( descriptor );
      }
      else
      {
        super_former.container.descriptors = Some( vec![ descriptor ] );
      }
      super_former
    };
    TemplateParameterDescriptorFormer::begin( Some( self ), on_end ).parameter( name )
  }
}

#[ test ]
fn basic()
{

  let got = TemplateParameters::former()
  .descriptors()
    .push( TemplateParameterDescriptor::former().parameter( "a" ).form() )
    .push( TemplateParameterDescriptor::former().parameter( "b" ).form() )
    .end()
  .form();

  let descriptors = vec!
  [
    TemplateParameterDescriptor { parameter : "a".to_string(), is_mandatory : false },
    TemplateParameterDescriptor { parameter : "b".to_string(), is_mandatory : false },
  ];
  let exp = TemplateParameters { descriptors };
  a_id!( got, exp );

}
