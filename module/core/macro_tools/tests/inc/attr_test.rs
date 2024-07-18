
use super::*;
use the_module::{ attr, qt, Result };

//

#[ test ]
fn is_standard_standard()
{
  // Test a selection of attributes known to be standard
  assert!( attr::is_standard( "cfg" ), "Expected 'cfg' to be a standard attribute." );
  assert!( attr::is_standard( "derive" ), "Expected 'derive' to be a standard attribute." );
  assert!( attr::is_standard( "inline" ), "Expected 'inline' to be a standard attribute." );
  assert!( attr::is_standard( "test" ), "Expected 'test' to be a standard attribute." );
  assert!( attr::is_standard( "doc" ), "Expected 'doc' to be a standard attribute." );
}

#[ test ]
fn is_standard_non_standard()
{
  // Test some made-up attributes that should not be standard
  assert!( !attr::is_standard( "custom_attr" ), "Expected 'custom_attr' to not be a standard attribute." );
  assert!( !attr::is_standard( "my_attribute" ), "Expected 'my_attribute' to not be a standard attribute." );
  assert!( !attr::is_standard( "special_feature" ), "Expected 'special_feature' to not be a standard attribute." );
}

#[ test ]
fn is_standard_edge_cases()
{
  // Test edge cases like empty strings or unusual input
  assert!( !attr::is_standard( "" ), "Expected empty string to not be a standard attribute." );
  assert!( !attr::is_standard( " " ), "Expected a single space to not be a standard attribute." );
  assert!( !attr::is_standard( "cfg_attr_extra" ), "Expected 'cfg_attr_extra' to not be a standard attribute." );
}

#[ test ]
fn attribute_component_from_meta()
{
  use the_module::AttributeComponent;
  struct MyComponent;

  impl AttributeComponent for MyComponent
  {
    const KEYWORD : &'static str = "my_component";

    fn from_meta( attr : &syn::Attribute ) -> Result< Self >
    {
      match &attr.meta
      {
        syn::Meta::NameValue( meta_name_value ) if meta_name_value.path.is_ident( Self::KEYWORD ) =>
        {
          Ok( MyComponent )
        }
        _ => Err( syn::Error::new_spanned( attr, "Failed to parse attribute as MyComponent" ) ),
      }
    }
  }

  // Define a sample attribute
  let attr : syn::Attribute = syn::parse_quote!( #[ my_component = "value" ] );

  // Attempt to construct MyComponent from the attribute
  let result = MyComponent::from_meta( &attr );

  // Assert that the construction was successful
  assert!( result.is_ok() );

  // Negative testing

  // Define a sample invalid attribute
  let attr : syn::Attribute = syn::parse_quote!( #[ other_component = "value" ] );

  // Attempt to construct MyComponent from the invalid attribute
  let result = MyComponent::from_meta( &attr );

  // Assert that the construction failed
  assert!( result.is_err() );
}

#[ test ]
fn attribute_basic() -> Result< () >
{
  use macro_tools::syn::parse::Parser;

  // test.case( "AttributesOuter" );
  let code = qt!
  {
    #[ derive( Copy ) ]
    #[ derive( Clone ) ]
    #[ derive( Debug ) ]
  };
  let got = syn::parse2::< the_module::AttributesOuter >( code ).unwrap();
  let exp = the_module::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!
  {
    #[ derive( Copy ) ]
    #[ derive( Clone ) ]
    #[ derive( Debug ) ]
  } )? );
  a_id!( got, exp );

  // test.case( "AttributesInner" );
  let code = qt!
  {
    // #![ deny( missing_docs ) ]
    #![ warn( something ) ]
  };
  let got = syn::parse2::< the_module::AttributesInner >( code ).unwrap();
  let exp = the_module::AttributesInner::from( syn::Attribute::parse_inner.parse2( qt!
  {
    // #![ deny( missing_docs ) ]
    #![ warn( something ) ]
  } )? );
  a_id!( got, exp );

  // test.case( "AttributesInner" );
  let code = qt!
  {
    #![ warn( missing_docs1 ) ]
    #![ warn( missing_docs2 ) ]
    #[ warn( something1 ) ]
    #[ warn( something2 ) ]
  };
  let got = syn::parse2::< the_module::Pair< the_module::AttributesInner, the_module::AttributesOuter > >( code ).unwrap();
  let exp = the_module::Pair::from
  ((
    the_module::AttributesInner::from( syn::Attribute::parse_inner.parse2( qt!
    {
      #![ warn( missing_docs1 ) ]
      #![ warn( missing_docs2 ) ]
    } )? ),
    the_module::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!
    {
      #[ warn( something1 ) ]
      #[ warn( something2 ) ]
    } )? ),
  ));
  a_id!( got, exp );

  //

  Ok( () )
}
