
use super::*;

//

tests_impls!
{

  #[ test ]
  fn equation_test() -> Result< () >
  {
    use syn::spanned::Spanned;

    // test.case( "basic" );
    let input = qt!
    {
      #[ derive( Former ) ]
      pub struct Struct1
      {
        #[former( default = 31 ) ]
        pub int_1 : i32,
      }
    };

    let ast = match syn::parse2::< syn::DeriveInput >( input )
    {
      Ok( syntax_tree ) => syntax_tree,
      Err( err ) => return Err( err ),
    };

    let fields = match ast.data
    {
      syn::Data::Struct( ref data_struct ) => match data_struct.fields
      {
        syn::Fields::Named( ref fields_named ) =>
        {
          &fields_named.named
        },
        _ => return Err( syn::Error::new( ast.span(), "Unknown format of data, expected syn::Fields::Named( ref fields_named )" ) ),
      },
      _ => return Err( syn::Error::new( ast.span(), "Unknown format of data, expected syn::Data::Struct( ref data_struct )" ) ),
    };

    let attr = fields.first().ok_or_else( || err( "No field" ) )?.attrs.first().ok_or_else( || err( "No attr" ) )?;

    let exp = equation::Equation
    {
      left : parse_quote!{ default },
      op : parse_quote!{ = },
      right : parse_quote!{ 31 },
    };
    let got = equation::from_meta( &attr )?;
    a_id!( got.left, exp.left );
    a_id!( format!( "{:?}", got ), format!( "{:?}", exp ) );
    // a_id!( got.right, exp.right );

    return Ok( () );

    fn err( src : &str ) -> syn::Error
    {
      syn::Error::new( proc_macro2::Span::call_site(), src )
    }
  }

  fn equation_parse_test()
  {

    let got : the_module::Equation = syn::parse_quote!( default = 31 );
    tree_print!( got );
    a_id!( code_to_str!( got ), "default = 31".to_string() );

    a_id!( got.left, syn::parse_quote!( default ) );
    a_id!( got.op, syn::token::Eq::default() );
    a_id!( code_to_str!( got.right ), "31".to_string() );

  }

  fn equation_from_meta_test()
  {

    let attr1 : syn::Attribute = syn::parse_quote!( #[ default( 31 ) ] );
    tree_print!( attr1 );

    let attr1 : syn::Attribute = syn::parse_quote!( #[ default[ 31 ] ] );
    tree_print!( attr1 );

    let attr1 : syn::Attribute = syn::parse_quote!( #[ former( default = 31 ) ] );
    // tree_print!( attr1 );
    let got = equation::from_meta( &attr1 ).unwrap();
    a_id!( code_to_str!( got ), "default = 31".to_string() );
    a_id!( got.left, syn::parse_quote!( default ) );
    a_id!( got.op, syn::token::Eq::default() );
    a_id!( code_to_str!( got.right ), "31".to_string() );

  }

}

//

//

tests_index!
{
  equation_test,
  equation_parse_test,
  equation_from_meta_test,
}
