use proc_macro::TokenStream;
use proc_macro_tools::{parse_macro_input, ParseStream, syn, syn_err};
use proc_macro_tools::quote::quote;
use proc_macro_tools::syn::{Ident, Type};
use proc_macro_tools::syn::parse::Parse;
use crate::syn::{Expr, Fields};

struct Properties
{
  fields : Vec< Field >,
  struct_ident : Ident,
}

struct Attributes
{
  parser : Option< ParserAttribute >,
}

struct ParserAttribute
{
  parser : Expr,
}

enum Field
{
  Optional( FieldType, Ident ),
  Required( FieldType, Ident ),
}

enum FieldType
{
  PlainType(),
  CustomParser( Expr ),
  Vec( Type ),
  Array( Type, Expr ),
}

#[ proc_macro_derive( Properties, attributes( doc, parser ) ) ]
pub fn derive_command_properties( input : TokenStream ) -> TokenStream
{
  let properties = parse_macro_input!( input as Properties );
  let struct_ident = properties.struct_ident;

  let idents : Vec< Ident > = properties.fields.iter().map
  (
    | field |
      {
        let ( Field::Optional( _, field_ident ) | Field::Required( _, field_ident ) ) = &field;
        field_ident.clone()
      }
  ).collect();
  let values_parsing : Vec< proc_macro_tools::proc_macro2::TokenStream > = properties.fields.into_iter().map( parse_field ).collect();

  quote!
  (
    impl Properties for #struct_ident
    {
      fn parse( properties : &::std::collections::HashMap< String, wstring_tools::parse_request::OpType< String > > ) -> ::std::result::Result< Self, error_tools::BasicError >
      {
        #(#values_parsing)*

        Ok ( #struct_ident { #(#idents),* } )
      }
    }
  )
    .into()
}

fn parse_field( field : Field ) -> proc_macro_tools::proc_macro2::TokenStream
{
  let none_result = match &field
  {
    Field::Optional( _, _ ) => quote!( None ),
    Field::Required( _, field_ident ) =>
      {
        let error_msg = format!( "Field '{}' is required", field_ident );
        quote!( return ::std::result::Result::Err( error_tools::BasicError::new( #error_msg ) ); )
      },
  };

  let ( Field::Optional( field_type, field_ident ) | Field::Required( field_type, field_ident ) ) = field;
  let value_parsing = match field_type
  {
    FieldType::PlainType() => parse_plain_type( &field_ident ),
    FieldType::Vec( type_ ) => parse_vector_type( &type_, &field_ident ),
    FieldType::Array( type_ , elements_count) => parse_array_type( &type_, &field_ident, &elements_count ),
    FieldType::CustomParser( expr ) => parse_with_custom_parser( &expr ),
  };

  let key = field_ident.to_string();
  quote!
  (
    let #field_ident = if let Some( op_type ) = properties.get( #key )
    {
      #value_parsing
    }
    else
    {
      #none_result
    };
  )
}

fn parse_plain_type( field_ident : &Ident ) -> proc_macro_tools::proc_macro2::TokenStream
{
  let error_msg = format!( "Failed to parse field '{}'", field_ident );
  quote!
  (
    if let Some( primitive ) = op_type.clone().primitive()
    {
      primitive.parse()
      .map_err( | err | error_tools::BasicError::new( #error_msg ) )?
    }
    else
    {
      return ::std::result::Result::Err( error_tools::BasicError::new( "Primitive type expected" ) );
    }
  )
}

fn parse_array_type( type_ : &Type, field_ident : &Ident, elements_count : &Expr ) -> proc_macro_tools::proc_macro2::TokenStream
{
  let error_msg = format!( "Failed to parse field '{}'", field_ident );
  quote!
  (
    if let Some( vector ) = op_type.clone().vector()
    {
      if vector.len() != #elements_count
      {
        return ::std::result::Result::Err( error_tools::BasicError::new( ::std::fmt::format( format_args!( "Expected {} elements, got {}", #elements_count, vector.len() ) ) ) );
      }

      vector.into_iter().map( | el | el.parse() ).collect::< ::std::result::Result< ::std::vec::Vec< #type_ >, < #type_ as ::std::str::FromStr >::Err > >()
      .map_err( | err | error_tools::BasicError::new( #error_msg ) )?
      .try_into()
      .unwrap()
    }
    else
    {
      return ::std::result::Result::Err( error_tools::BasicError::new( "Array type expected" ) );
    }
  )
}

fn parse_vector_type( type_ : &Type, field_ident : &Ident ) -> proc_macro_tools::proc_macro2::TokenStream
{
  let error_msg = format!( "Failed to parse field '{}'", field_ident );
  quote!
  (
    if let Some( vector ) = op_type.clone().vector()
    {
      vector.into_iter().map( | el | el.parse() ).collect::< ::std::result::Result< ::std::vec::Vec< #type_ >, < #type_ as ::std::str::FromStr >::Err > >()
      .map_err( | err | error_tools::BasicError::new( #error_msg ) )?
    }
    else
    {
      return ::std::result::Result::Err( error_tools::BasicError::new( "Vector type expected" ) );
    }
  )
}

fn parse_with_custom_parser( parser : &Expr ) -> proc_macro_tools::proc_macro2::TokenStream
{
  quote!
  (
    if let Some( primitive ) = op_type.clone().primitive()
    {
      ( #parser )( primitive )?
    }
    else
    {
      return ::std::result::Result::Err( error_tools::BasicError::new( "Primitive type expected" ) );
    }
  )
}

fn extract_field_type( type_ : &Type ) -> FieldType
{
  if let Type::Array( array ) = type_
  {
    FieldType::Array( ( *array.elem ).clone(), array.len.clone() )
  }
  else if proc_macro_tools::type_rightmost( type_ ).unwrap() == "Vec"
  {
    let types = proc_macro_tools::type_parameters( type_, 0..1 );
    let type_ = types.first().unwrap();
    FieldType::Vec( ( *type_ ).clone() )
  }
  else
  {
    FieldType::PlainType()
  }
}

impl Attributes
{
  fn parse( attributes : &Vec< syn::Attribute > ) -> syn::Result< Self >
  {
    let mut parser = None;
    for attr in attributes
    {
      let key_ident = attr.path.get_ident()
        .ok_or_else( || syn_err!( attr, "Expects simple key of an attirbute, but got:\n  {}", quote!{ #attr } ) )?;
      let key_str = format!( "{}", key_ident );
      match key_str.as_ref()
      {
        "parser" =>
          {
            let parser_attr = syn::parse2::< ParserAttribute >( attr.tokens.clone() )?;
            parser.replace( parser_attr );
          }
        "doc" =>
          {
          }
        _ =>
          {
            return Err( syn_err!( attr, "Unknown attribute {}", quote!{ #attr } ) );
          }
      }
    }

    Ok( Attributes { parser } )
  }
}

impl Parse for Properties
{
  fn parse( input : ParseStream ) -> syn::Result< Self >
  {
    let item : syn::ItemStruct = input.parse()?;
    let struct_ident = item.ident;
    let fields : Vec< Field > = match item.fields
    {
      Fields::Named( fields ) =>
        {
          fields.named.into_iter().map
          (
            | field: syn::Field |
              {
                let ident = field.ident.unwrap();

                let attrs = Attributes::parse( &field.attrs ).unwrap();
                if let Some( expr ) = attrs.parser
                {
                  return Field::Required( FieldType::CustomParser( expr.parser ), ident );
                }

                let type_ = proc_macro_tools::type_rightmost( &field.ty );
                if type_.is_some() && type_.unwrap() == "Option"
                {
                  let types = proc_macro_tools::type_parameters( &field.ty, 0..1 );
                  let type_ = types.first().unwrap();
                  let field_type = extract_field_type( type_ );
                  Field::Optional( field_type, ident )
                }
                else
                {
                  let field_type = extract_field_type( &field.ty );
                  Field::Required( field_type, ident )
                }
              }
          )
            .collect()
        },
      Fields::Unnamed( fields ) => return Err( syn_err!( fields, "Unnamed fields are not supported." ) ),
      Fields::Unit => Vec::new(),
    };

    Ok( Properties { struct_ident, fields } )
  }
}

impl Parse for ParserAttribute
{
  fn parse( input : ParseStream ) -> syn::Result< Self >
  {
    let expr_tokens;
    syn::parenthesized!( expr_tokens in input );
    Ok
      ( ParserAttribute
      {
        parser : expr_tokens.parse()?,
      }
      )
  }
}
