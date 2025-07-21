extern crate alloc;
use macro_tools::{ Result, syn };
use syn::{ Expr, Type, meta::ParseNestedMeta };
use super::OpKind;
use alloc::string::ToString;

#[ derive( Debug ) ]
enum Operation 
{
  Add,
  Sub,
  Mul,
  Div,
  Default,
}

/// Attributes of item.
#[ derive( Debug, Default ) ]
pub struct ItemAttributes 
{
  /// Error expression for the `Add` operation.
  pub add_error_expr : Option< Expr >,
  /// Error type for the `Add` operation.
  pub add_error_type : Option< Type >,

  /// Error expression for the `Sub` operation.
  pub sub_error_expr : Option< Expr >,
  /// Error type for the `Sub` operation.
  pub sub_error_type : Option< Type >,

  /// Error expression for the `Mul` operation.
  pub mul_error_expr : Option< Expr >,
  /// Error type for the `Mul` operation.
  pub mul_error_type : Option< Type >,

  /// Error expression for the `Div` operation.
  pub div_error_expr : Option< Expr >,
  /// Error type for the `Div` operation.
  pub div_error_type : Option< Type >,

  /// Default error expression for all operations.
  pub default_error_expr : Option< Expr >,
  /// Default error type for all operations.
  pub default_error_type : Option< Type >,
}

impl ItemAttributes 
{
  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut result = Self::default();

    for attr in attrs 
    {
      let ident = attr.path().get_ident().map( ToString::to_string );

      let Some( attr_name ) = ident else { continue };

      let op = match attr_name.as_str() 
      {
        "add" => Operation::Add,
        "sub" => Operation::Sub,
        "mul" => Operation::Mul,
        "div" => Operation::Div,
        "derive_ops" => Operation::Default,
        _ => continue, // skip unknown
      };

      attr.parse_nested_meta( | meta | 
      {
        handle_meta( &mut result, &meta, &op )
      })?;
    }

    Ok( result )
  }

  pub fn error_expr_for( &self, op : OpKind ) -> Option< &syn::Expr > 
  {
    match op 
    {
      OpKind::Add => self.add_error_expr.as_ref().or( self.default_error_expr.as_ref() ),
      OpKind::Sub => self.sub_error_expr.as_ref().or( self.default_error_expr.as_ref() ),
      OpKind::Mul => self.mul_error_expr.as_ref().or( self.default_error_expr.as_ref() ),
      OpKind::Div => self.div_error_expr.as_ref().or( self.default_error_expr.as_ref() ),
    }
  }

  pub fn error_type_for( &self, op : OpKind ) -> Option< &syn::Type > 
  {
    match op 
    {
      OpKind::Add => self.add_error_type.as_ref().or( self.default_error_type.as_ref() ),
      OpKind::Sub => self.sub_error_type.as_ref().or( self.default_error_type.as_ref() ),
      OpKind::Mul => self.mul_error_type.as_ref().or( self.default_error_type.as_ref() ),
      OpKind::Div => self.div_error_type.as_ref().or( self.default_error_type.as_ref() ),
    }
  }
}

fn handle_meta( result : &mut ItemAttributes, meta : &ParseNestedMeta< '_ >, op : &Operation ) -> Result< () > 
{
  if meta.path.is_ident( "error_expr" ) 
  {
    let value = meta.value()?;
    let expr : Expr = value.parse()?;

    match op 
    {
      Operation::Add => result.add_error_expr = Some( expr.clone() ),
      Operation::Sub => result.sub_error_expr = Some( expr.clone() ),
      Operation::Mul => result.mul_error_expr = Some( expr.clone() ),
      Operation::Div => result.div_error_expr = Some( expr.clone() ),
      Operation::Default => result.default_error_expr = Some( expr.clone() ),
    }

    if let Some( ty ) = extract_type_from_expr( &expr ) 
    {
      let type_slot = match op 
      {
        Operation::Add => &mut result.add_error_type,
        Operation::Sub => &mut result.sub_error_type,
        Operation::Mul => &mut result.mul_error_type,
        Operation::Div => &mut result.div_error_type,
        Operation::Default => &mut result.default_error_type,
      };
      if type_slot.is_none() 
      {
        *type_slot = Some( ty );
      }
    }
  } 
  else if meta.path.is_ident( "error_type" ) 
  {
    let value = meta.value()?;
    let ty : Type = value.parse()?;
    match op 
    {
      Operation::Add => result.add_error_type = Some( ty ),
      Operation::Sub => result.sub_error_type = Some( ty ),
      Operation::Mul => result.mul_error_type = Some( ty ),
      Operation::Div => result.div_error_type = Some( ty ),
      Operation::Default => result.default_error_type = Some( ty ),
    }
  }

    Ok( () )
}

pub fn extract_type_from_expr( expr : &syn::Expr ) -> Option< syn::Type > 
{
  match expr 
  {
    syn::Expr::Path( expr_path ) => 
    {
    let segments = &expr_path.path.segments;  
    if segments.is_empty() 
    {
        None
    }
    else 
    {
      let first = segments.first().cloned()?;
      let mut new_path = syn::Path 
      {
        leading_colon : None,
        segments : syn::punctuated::Punctuated::new(),
      };
      new_path.segments.push( first );  
      Some( syn::Type::Path( syn::TypePath 
      {
        qself: None,
        path: new_path,
      }))
    }
  },
    _ => None,
  }
}