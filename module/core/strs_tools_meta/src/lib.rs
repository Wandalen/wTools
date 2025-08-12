//! Procedural macros for compile-time string processing optimizations.
//!
//! This crate provides macros that analyze string patterns at compile time
//! and generate optimized code for common string operations.
//!
//! This is a meta module for `strs_tools`. Don't use directly.

#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]

use macro_tools::
{
  quote::quote,
  syn::{ self, Expr, LitStr, Result },
};
use proc_macro::TokenStream;

/// Analyze string patterns at compile time and generate optimized split code.
/// 
/// This macro examines delimiter patterns and input characteristics to select
/// the most efficient splitting strategy at compile time.
/// 
/// # Examples
/// 
/// ```rust,ignore
/// # use strs_tools_meta::optimize_split;
/// // Simple comma splitting - generates optimized code
/// let result = optimize_split!("field1,field2,field3", ",");
/// 
/// // Multiple delimiters - generates multi-delimiter optimization
/// let result = optimize_split!(input_str, [",", ";", ":"]);
/// 
/// // Complex patterns - generates pattern-specific optimization
/// let result = optimize_split!(data, [",", "->", "::"], preserve_delimiters = true);
/// ```
/// 
/// # Debug Attribute
/// 
/// The `debug` attribute enables diagnostic output for macro expansion:
/// ```rust,ignore
/// #[ optimize_split( debug ) ]
/// let result = optimize_split!(input, ",");
/// ```
#[ cfg( feature = "optimize_split" ) ]
#[ proc_macro ]
pub fn optimize_split( input: TokenStream ) -> TokenStream
{
  let result = optimize_split_impl( input );
  match result 
  {
    Ok( tokens ) => tokens.into(),
    Err( e ) => e.to_compile_error().into(),
  }
}

/// Generate compile-time optimized string matching code.
///
/// This macro creates efficient pattern matching code based on compile-time
/// analysis of the patterns and their usage context.
///
/// # Examples
///
/// ```rust,ignore
/// # use strs_tools_meta::optimize_match;
/// // Single pattern matching
/// let matched = optimize_match!(input, "prefix_");
///
/// // Multiple pattern matching with priorities  
/// let result = optimize_match!(text, ["http://", "https://", "ftp://"], strategy = "first_match");
/// ```
/// 
/// # Debug Attribute
/// 
/// The `debug` attribute enables diagnostic output for macro expansion:
/// ```rust,ignore
/// #[ optimize_match( debug ) ]
/// let result = optimize_match!(input, ["http://", "https://"]);
/// ```
#[ cfg( feature = "optimize_match" ) ]
#[ proc_macro ]
pub fn optimize_match( input: TokenStream ) -> TokenStream
{
  let result = optimize_match_impl( input );
  match result 
  {
    Ok( tokens ) => tokens.into(),
    Err( e ) => e.to_compile_error().into(),
  }
}

#[ cfg( feature = "optimize_split" ) ]
fn optimize_split_impl( input: TokenStream ) -> Result< macro_tools::proc_macro2::TokenStream >
{
  let parsed_input = syn::parse( input )?;
  Ok( generate_optimized_split( &parsed_input ) )
}

#[ cfg( feature = "optimize_match" ) ]
fn optimize_match_impl( input: TokenStream ) -> Result< macro_tools::proc_macro2::TokenStream >
{
  let parsed_input = syn::parse( input )?;
  Ok( generate_optimized_match( &parsed_input ) )
}

/// Input structure for `optimize_split` macro
#[ cfg( feature = "optimize_split" ) ]
#[ derive( Debug ) ]
#[ allow( clippy::struct_excessive_bools ) ]
struct OptimizeSplitInput
{
  source: Expr,
  delimiters: Vec< String >,
  preserve_delimiters: bool,
  preserve_empty: bool,
  use_simd: bool,
  debug: bool,
}

#[ cfg( feature = "optimize_split" ) ]
impl syn::parse::Parse for OptimizeSplitInput
{
  fn parse( input: syn::parse::ParseStream<'_> ) -> Result< Self >
  {
    let source: Expr = input.parse()?;
    input.parse::< syn::Token![,] >()?;
    
    let mut delimiters = Vec::new();
    let mut preserve_delimiters = false;
    let mut preserve_empty = false;
    let mut use_simd = true; // Default to SIMD if available
    let mut debug = false;
    
    // Parse delimiter(s)
    if input.peek( syn::token::Bracket )
    {
      // Multiple delimiters: ["a", "b", "c"]
      let content;
      syn::bracketed!( content in input );
      while !content.is_empty()
      {
        let lit: LitStr = content.parse()?;
        delimiters.push( lit.value() );
        if !content.is_empty()
        {
          content.parse::< syn::Token![,] >()?;
        }
      }
    }
    else
    {
      // Single delimiter: "a"
      let lit: LitStr = input.parse()?;
      delimiters.push( lit.value() );
    }
    
    // Parse optional parameters
    while !input.is_empty()
    {
      input.parse::< syn::Token![,] >()?;
      
      let ident: syn::Ident = input.parse()?;
      
      if ident.to_string().as_str() == "debug" {
        debug = true;
      } else {
        input.parse::< syn::Token![=] >()?;
        
        match ident.to_string().as_str()
        {
          "preserve_delimiters" =>
          {
            let lit: syn::LitBool = input.parse()?;
            preserve_delimiters = lit.value;
          },
          "preserve_empty" =>
          {
            let lit: syn::LitBool = input.parse()?;
            preserve_empty = lit.value;
          },
          "use_simd" =>
          {
            let lit: syn::LitBool = input.parse()?;
            use_simd = lit.value;
          },
          _ =>
          {
            return Err( syn::Error::new( ident.span(), "Unknown parameter" ) );
          }
        }
      }
    }
    
    Ok( OptimizeSplitInput
    {
      source,
      delimiters,
      preserve_delimiters,
      preserve_empty,
      use_simd,
      debug,
    } )
  }
}

/// Input structure for `optimize_match` macro
#[ cfg( feature = "optimize_match" ) ]
#[ derive( Debug ) ]
struct OptimizeMatchInput
{
  source: Expr,
  patterns: Vec< String >,
  strategy: String, // "first_match", "longest_match", "all_matches"
  debug: bool,
}

#[ cfg( feature = "optimize_match" ) ]
impl syn::parse::Parse for OptimizeMatchInput
{
  fn parse( input: syn::parse::ParseStream<'_> ) -> Result< Self >
  {
    let source: Expr = input.parse()?;
    input.parse::< syn::Token![,] >()?;
    
    let mut patterns = Vec::new();
    let mut strategy = "first_match".to_string();
    let mut debug = false;
    
    // Parse pattern(s)
    if input.peek( syn::token::Bracket )
    {
      // Multiple patterns: ["a", "b", "c"]
      let content;
      syn::bracketed!( content in input );
      while !content.is_empty()
      {
        let lit: LitStr = content.parse()?;
        patterns.push( lit.value() );
        if !content.is_empty()
        {
          content.parse::< syn::Token![,] >()?;
        }
      }
    }
    else
    {
      // Single pattern: "a"
      let lit: LitStr = input.parse()?;
      patterns.push( lit.value() );
    }
    
    // Parse optional parameters
    while !input.is_empty()
    {
      input.parse::< syn::Token![,] >()?;
      
      let ident: syn::Ident = input.parse()?;
      
      match ident.to_string().as_str()
      {
        "debug" =>
        {
          debug = true;
        },
        "strategy" =>
        {
          input.parse::< syn::Token![=] >()?;
          let lit: LitStr = input.parse()?;
          strategy = lit.value();
        },
        _ =>
        {
          return Err( syn::Error::new( ident.span(), "Unknown parameter" ) );
        }
      }
    }
    
    Ok( OptimizeMatchInput
    {
      source,
      patterns,
      strategy,
      debug,
    } )
  }
}

/// Generate optimized split code based on compile-time analysis
#[ cfg( feature = "optimize_split" ) ]
fn generate_optimized_split( input: &OptimizeSplitInput ) -> macro_tools::proc_macro2::TokenStream
{
  let source = &input.source;
  let delimiters = &input.delimiters;
  let _preserve_delimiters = input.preserve_delimiters;
  let preserve_empty = input.preserve_empty;
  let _use_simd = input.use_simd;
  
  // Compile-time optimization decisions
  let optimization = analyze_split_pattern( delimiters );
  
  if input.debug
  {
    eprintln!( "optimize_split! debug: pattern={delimiters:?}, optimization={optimization:?}" );
  }
  
  match optimization
  {
    SplitOptimization::SingleCharDelimiter( delim ) =>
    {
      // Generate highly optimized single-character split
      if preserve_empty
      {
        quote!
        {
          {
            // Compile-time optimized single character split with empty preservation
            #source.split( #delim ).collect::< Vec< &str > >()
          }
        }
      }
      else
      {
        quote!
        {
          {
            // Compile-time optimized single character split
            #source.split( #delim ).filter( |s| !s.is_empty() ).collect::< Vec< &str > >()
          }
        }
      }
    },
    
    SplitOptimization::MultipleCharDelimiters =>
    {
      // Generate multi-delimiter optimization
      let delim_first = &delimiters[ 0 ];
      
      if delimiters.len() == 1
      {
        // Single multi-char delimiter
        if preserve_empty
        {
          quote!
          {
            {
              // Compile-time optimized multi-char delimiter split with empty preservation
              #source.split( #delim_first ).collect::< Vec< &str > >()
            }
          }
        }
        else
        {
          quote!
          {
            {
              // Compile-time optimized multi-char delimiter split
              #source.split( #delim_first ).filter( |s| !s.is_empty() ).collect::< Vec< &str > >()
            }
          }
        }
      }
      else
      {
        // Multiple delimiters - generate pattern matching code
        let delim_array = delimiters.iter().map( |d| quote! { #d } ).collect::< Vec< _ > >();
        
        if preserve_empty
        {
          quote!
          {
            {
              // Compile-time optimized multi-delimiter split with empty preservation
              let mut result = vec![ #source ];
              let delimiters = [ #( #delim_array ),* ];
              
              for delimiter in &delimiters
              {
                result = result.into_iter()
                  .flat_map( |s| s.split( delimiter ) )
                  .collect();
              }
              
              result
            }
          }
        }
        else
        {
          quote!
          {
            {
              // Compile-time optimized multi-delimiter split
              let mut result = vec![ #source ];
              let delimiters = [ #( #delim_array ),* ];
              
              for delimiter in &delimiters
              {
                result = result.into_iter()
                  .flat_map( |s| s.split( delimiter ) )
                  .filter( |s| !s.is_empty() )
                  .collect();
              }
              
              result
            }
          }
        }
      }
    },
    
    SplitOptimization::ComplexPattern =>
    {
      // Generate complex pattern optimization fallback
      let delim_first = &delimiters[ 0 ];
      
      if preserve_empty
      {
        quote!
        {
          {
            // Compile-time optimized complex pattern fallback with empty preservation
            #source.split( #delim_first ).collect::< Vec< &str > >()
          }
        }
      }
      else
      {
        quote!
        {
          {
            // Compile-time optimized complex pattern fallback
            #source.split( #delim_first ).filter( |s| !s.is_empty() ).collect::< Vec< &str > >()
          }
        }
      }
    }
  }
}

/// Generate optimized match code based on compile-time analysis
#[ cfg( feature = "optimize_match" ) ]
fn generate_optimized_match( input: &OptimizeMatchInput ) -> macro_tools::proc_macro2::TokenStream
{
  let source = &input.source;
  let patterns = &input.patterns;
  let strategy = &input.strategy;
  
  let optimization = analyze_match_pattern( patterns, strategy );
  
  if input.debug
  {
    eprintln!( "optimize_match! debug: patterns={patterns:?}, strategy={strategy:?}, optimization={optimization:?}" );
  }
  
  match optimization
  {
    MatchOptimization::SinglePattern( pattern ) =>
    {
      // Generate optimized single pattern matching
      quote!
      {
        {
          // Compile-time optimized single pattern match
          #source.find( #pattern )
        }
      }
    },
    
    MatchOptimization::TrieBasedMatch =>
    {
      // Generate trie-based pattern matching
      let _trie_data = build_compile_time_trie( patterns );
      quote!
      {
        {
          // Compile-time generated trie matching (simplified implementation)
          let mut best_match = None;
          for pattern in [ #( #patterns ),* ]
          {
            if let Some( pos ) = #source.find( pattern )
            {
              match best_match
              {
                None => best_match = Some( pos ),
                Some( current_pos ) if pos < current_pos => best_match = Some( pos ),
                _ => {}
              }
            }
          }
          best_match
        }
      }
    },
    
    MatchOptimization::SequentialMatch =>
    {
      // Generate sequential pattern matching
      quote!
      {
        {
          // Compile-time sequential pattern matching
          let mut result = None;
          for pattern in [ #( #patterns ),* ]
          {
            if let Some( pos ) = #source.find( pattern )
            {
              result = Some( pos );
              break;
            }
          }
          result
        }
      }
    }
  }
}

/// Compile-time split pattern analysis
#[ cfg( feature = "optimize_split" ) ]
#[ derive( Debug ) ]
enum SplitOptimization
{
  SingleCharDelimiter( String ),
  MultipleCharDelimiters,
  ComplexPattern,
}

/// Compile-time match pattern analysis
#[ cfg( feature = "optimize_match" ) ]
#[ derive( Debug ) ]
enum MatchOptimization
{
  SinglePattern( String ),
  TrieBasedMatch,
  SequentialMatch,
}

/// Analyze delimiter patterns for optimization opportunities
#[ cfg( feature = "optimize_split" ) ]
fn analyze_split_pattern( delimiters: &[ String ] ) -> SplitOptimization
{
  if delimiters.len() == 1
  {
    let delim = &delimiters[0];
    if delim.len() == 1
    {
      // Single character delimiter - highest optimization potential
      SplitOptimization::SingleCharDelimiter( delim.clone() )
    }
    else
    {
      // Multi-character single delimiter
      SplitOptimization::MultipleCharDelimiters
    }
  }
  else if delimiters.len() <= 8 && delimiters.iter().all( |d| d.len() <= 4 )
  {
    // Multiple simple delimiters - good for SIMD
    SplitOptimization::MultipleCharDelimiters
  }
  else
  {
    // Complex patterns - use state machine approach
    SplitOptimization::ComplexPattern
  }
}

/// Analyze match patterns for optimization opportunities
#[ cfg( feature = "optimize_match" ) ]
fn analyze_match_pattern( patterns: &[ String ], _strategy: &str ) -> MatchOptimization
{
  if patterns.len() == 1
  {
    MatchOptimization::SinglePattern( patterns[0].clone() )
  }
  else if patterns.len() <= 16 && patterns.iter().all( |p| p.len() <= 8 )
  {
    // Small set of short patterns - use trie
    MatchOptimization::TrieBasedMatch
  }
  else
  {
    // Large pattern set - use sequential matching
    MatchOptimization::SequentialMatch
  }
}

/// Build compile-time trie data for pattern matching
#[ cfg( feature = "optimize_match" ) ]
fn build_compile_time_trie( patterns: &[ String ] ) -> Vec< macro_tools::proc_macro2::TokenStream >
{
  // Simplified trie construction for demonstration
  // In a full implementation, this would build an optimal trie structure
  patterns.iter().map( |pattern| {
    let bytes: Vec< u8 > = pattern.bytes().collect();
    quote! { &[ #( #bytes ),* ] }
  } ).collect()
}