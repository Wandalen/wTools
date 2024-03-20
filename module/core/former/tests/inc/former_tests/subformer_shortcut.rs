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
  // #[ debug = the_module::VectorSubformer, parameter, parameter( name ) ]
  #[ subformer( the_module::VectorSubformer ) ]
  descriptors : Vec< TemplateParameterDescriptor >,

  // #[ subformer_setter = the_module::VectorSubformer ]
  // pub fn parameter( self, name : &str )
  // {
  //   parameter( name )
  // }

}

// pub trait FormerBegin< Struct, Context, End >
// where
//   End : the_module::ToSuperFormer< Struct, Context >,
// {
//
//   fn _begin
//   (
//     context : core::option::Option< Context >,
//     on_end : End,
//   ) -> Self;
//
// }
//
// impl< Context, End > FormerBegin< TemplateParameterDescriptor, Context, End >
// for TemplateParameterDescriptorFormer< Context, End >
// where
//   End : the_module::ToSuperFormer< TemplateParameterDescriptor, Context >,
// {
//
//
//   #[ inline( always ) ]
//   fn _begin
//   (
//     context : core::option::Option< Context >,
//     on_end : End,
//   ) -> Self
//   {
//     Self::begin( context, on_end )
//   }
//
// }

pub trait FormerBegin< Struct, Context >
{
  type End : the_module::ToSuperFormer< Struct, Context >;

  fn _begin
  (
    context : core::option::Option< Context >,
    on_end : Self::End,
  ) -> Self;

}

impl< Context, End > FormerBegin< TemplateParameterDescriptor, Context >
for TemplateParameterDescriptorFormer< Context, End >
where
  End : the_module::ToSuperFormer< TemplateParameterDescriptor, Context >,
{
  type End = End;

  #[ inline( always ) ]
  fn _begin
  (
    context : core::option::Option< Context >,
    on_end : End,
  ) -> Self
  {
    Self::begin( context, on_end )
  }

}

impl< Context, End > TemplateParametersFormer< Context, End >
where
  End : former::ToSuperFormer< TemplateParameters, Context >,
{

  #[ inline( always ) ]
  pub fn __parameter< Former2, Struct >( self ) ->
  Former2
  where
    Former2 : FormerBegin< TemplateParameterDescriptor, Self, End = former::ToSuperFormerWrapper< TemplateParameterDescriptor, Self > >,
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
    Former2::_begin( Some( self ), former::ToSuperFormerWrapper::new( on_end ) )
  }

  #[ inline( always ) ]
  pub fn _parameter( self ) ->
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
    TemplateParameterDescriptorFormer::begin( Some( self ), on_end )
  }

  // #[ inline( always ) ]
  // pub fn __parameter< Former2, Struct >( self ) ->
  // // pub fn __parameter( self ) ->
  // // impl FormerBegin< TemplateParameterDescriptor, Self, Box< dyn former::ToSuperFormer< TemplateParameterDescriptor, Self > > >
  // // pub fn __parameter( self ) ->
  // // impl FormerBegin< TemplateParameterDescriptor, Self, impl former::ToSuperFormer< TemplateParameterDescriptor, Self > >
  // // Former2
  // // where
  //   // Former2 : FormerBegin< TemplateParameterDescriptor, Self, impl former::ToSuperFormer< TemplateParameterDescriptor, Self > >,
  //   // Former2 : FormerBegin< TemplateParameterDescriptor, Self >,
  //   // End2 : former::ToSuperFormer< TemplateParameterDescriptor, Self >,
  // {
  //   let on_end = | descriptor : TemplateParameterDescriptor, super_former : core::option::Option< Self > | -> Self
  //   {
  //     let mut super_former = super_former.unwrap();
  //     if let Some( ref mut descriptors ) = super_former.container.descriptors
  //     {
  //       descriptors.push( descriptor );
  //     }
  //     else
  //     {
  //       super_former.container.descriptors = Some( vec![ descriptor ] );
  //     }
  //     super_former
  //   };
  //   Former2::_begin( Some( self ), on_end.into() )
  // }

  #[ inline( always ) ]
  pub fn parameter( self, name : &str ) ->
  TemplateParameterDescriptorFormer< Self, impl former::ToSuperFormer< TemplateParameterDescriptor, Self > >
  {
    self._parameter().parameter( name )
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
