//! Contains the core parsing logic for unilang instructions.

use crate::config::UnilangParserOptions;
use crate::error::ParseError;
use crate::instruction::GenericInstruction;
use crate::item_adapter::{ classify_split, RichItem };

/// The main parser for unilang instructions.
#[derive(Debug)]
pub struct Parser
{
  options : UnilangParserOptions,
}

impl Parser
{
  /// Creates a new parser with the given options.
  pub fn new( options : UnilangParserOptions ) -> Self
  {
    Self { options }
  }

  /// Parses a single string into a vector of generic instructions.
  pub fn parse_single_str<'input>( &'input self, input : &'input str ) -> Result< Vec< GenericInstruction<'input> >, ParseError >
  {
    let mut rich_items : Vec<RichItem<'input>> = Vec::new();
    let mut split_iterator = self.options.to_split_options_former( input ).perform();

    while let Some( split_item ) = split_iterator.next()
    {
      let classified_kind = classify_split( &split_item, &self.options );
      rich_items.push( RichItem { inner: split_item, segment_idx: None, kind: classified_kind } );
    }

    self.analyze_items_to_instructions( rich_items )
  }

  /// Parses a slice of strings into a vector of generic instructions.
  pub fn parse_slice<'input>( &'input self, input_segments : &'input [&'input str] ) -> Result< Vec< GenericInstruction<'input> >, ParseError >
  {
    let mut rich_items_accumulator : Vec<RichItem<'input>> = Vec::new();

    for ( seg_idx, segment_str ) in input_segments.iter().enumerate()
    {
      let mut split_iterator = self.options.to_split_options_former( segment_str ).perform();
      while let Some( split_item ) = split_iterator.next()
      {
        let classified_kind = classify_split( &split_item, &self.options );
        rich_items_accumulator.push( RichItem { inner: split_item, segment_idx: Some( seg_idx ), kind: classified_kind } );
      }
    }

    self.analyze_items_to_instructions( rich_items_accumulator )
  }

  /// Analyzes a vector of rich items into generic instructions.
  /// This is the core syntactic analysis logic.
  #[allow(dead_code)] // Will be used and refined in later increments
  fn analyze_items_to_instructions<'input>
  (
    &self, // This &self does not need to be &'input self if it doesn't return anything tied to 'input directly
    _items : Vec<RichItem<'input>>,
  )
  -> Result<Vec<GenericInstruction<'input>>, ParseError>
  {
    // TODO: Implement full syntactic analysis in Increments 3, 4, 5.
    Ok( vec![] )
  }
}