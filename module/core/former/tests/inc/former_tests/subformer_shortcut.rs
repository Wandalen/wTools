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


/// Interface to construct a subformer passing `itself` as `context` and `on_end` which should return `itself` back.
pub trait FormerBegin< Struct, Context >
{
  type End : the_module::ToSuperFormer< Struct, Context >;

  fn _begin
  (
    context : core::option::Option< Context >,
    on_end : Self::End,
  ) -> Self;

}

/// Interface to add new elements into a container.
pub trait ContainerAdd
{
  type Element;

  fn add( &mut self, e : Self::Element ) -> bool;

}

impl< T > ContainerAdd for collection_tools::Vec< T >
{
  type Element = T;

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Element ) -> bool
  {
    self.push( e );
    true
  }

}

impl< E > ContainerAdd for collection_tools::HashSet< E >
where
  E : core::cmp::Eq + core::hash::Hash,
{
  type Element = E;

  #[ inline( always ) ]
  fn add( &mut self, e : Self::Element ) -> bool
  {
    self.insert( e )
  }

}

impl< K, V > ContainerAdd for collection_tools::HashMap< K, V >
where
  K : core::cmp::Eq + core::hash::Hash,
{
  type Element = ( K, V );

  #[ inline( always ) ]
  fn add( &mut self, ( k, v ) : Self::Element ) -> bool
  {
    self.insert( k, v ).map_or_else( || true, | _ | false )
  }

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
  pub fn descriptor3< Former2 >( self ) ->
  Former2
  where
    Former2 : FormerBegin< TemplateParameterDescriptor, Self, End = former::ToSuperFormerWrapper< TemplateParameterDescriptor, Self > >,
    // FieldContainer : ContainerAdd,
  {
    let on_end = | descriptor : TemplateParameterDescriptor, super_former : core::option::Option< Self > | -> Self
    {
      let mut super_former = super_former.unwrap();
      if super_former.container.descriptors.is_none()
      {
        super_former.container.descriptors = Some( Default::default() );
      }
      if let Some( ref mut descriptors ) = super_former.container.descriptors
      {
        ContainerAdd::add( descriptors, descriptor );
      }
      super_former
    };
    Former2::_begin( Some( self ), former::ToSuperFormerWrapper::new( on_end ) )
  }
  // xxx2 : move to a trait and make easier to use subformer, trait with generic interface of a container should help

  #[ inline( always ) ]
  pub fn descriptor( self, name : &str ) ->
  TemplateParameterDescriptorFormer< Self, impl former::ToSuperFormer< TemplateParameterDescriptor, Self > >
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
