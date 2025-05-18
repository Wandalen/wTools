//! Contains the core parsing logic for unilang instructions.

use crate::config::UnilangParserOptions;
use crate::error::{ ParseError, ErrorKind, SourceLocation };
use crate::instruction::GenericInstruction;
use crate::item_adapter::{ classify_split, RichItem, UnilangTokenKind };
use std::borrow::Cow;
use std::collections::HashMap;

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
    let mut rich_items_vec : Vec<RichItem<'input>> = Vec::new();
    let mut split_iterator = self.options.to_split_options_former( input ).perform();

    while let Some( split_item ) = split_iterator.next()
    {
      let classified_kind = classify_split( &split_item, &self.options );
      rich_items_vec.push( RichItem { inner: split_item, segment_idx: None, kind: classified_kind } );
    }

    self.analyze_items_to_instructions( &rich_items_vec )
  }

  /// Parses a slice of strings into a vector of generic instructions.
  pub fn parse_slice<'input>( &'input self, input_segments : &'input [&'input str] ) -> Result< Vec< GenericInstruction<'input> >, ParseError >
  {
    let mut rich_items_accumulator_vec : Vec<RichItem<'input>> = Vec::new();

    for ( seg_idx, segment_str ) in input_segments.iter().enumerate()
    {
      let mut split_iterator = self.options.to_split_options_former( segment_str ).perform();
      while let Some( split_item ) = split_iterator.next()
      {
        let classified_kind = classify_split( &split_item, &self.options );
        rich_items_accumulator_vec.push( RichItem { inner: split_item, segment_idx: Some( seg_idx ), kind: classified_kind } );
      }
    }

    self.analyze_items_to_instructions( &rich_items_accumulator_vec )
  }

  /// Analyzes a slice of rich items into generic instructions.
  fn analyze_items_to_instructions<'s_slice, 'input : 's_slice>
  (
    &'input self,
    items : &'s_slice [RichItem<'input>],
  )
  -> Result<Vec<GenericInstruction<'input>>, ParseError>
  {
    let mut instructions = Vec::new();
    if items.is_empty()
    {
      return Ok( instructions );
    }

    let mut start_index = 0;
    for (i, item_ref) in items.iter().enumerate() {
        if item_ref.kind == UnilangTokenKind::Delimiter(Cow::Borrowed(";;")) {
            let segment = &items[start_index..i];
            if segment.is_empty() {
                return Err(ParseError {
                    kind: ErrorKind::Syntax("Empty instruction segment due to ';;'".to_string()),
                    location: Some(item_ref.source_location()),
                });
            }
            instructions.push(self.parse_single_instruction_from_rich_items(segment)?);
            start_index = i + 1;
        }
    }

    if start_index < items.len() {
        let segment = &items[start_index..];
        instructions.push(self.parse_single_instruction_from_rich_items(segment)?);
    } else if start_index == items.len() && !items.is_empty() {
        if items.last().unwrap().kind == UnilangTokenKind::Delimiter(Cow::Borrowed(";;")) {
            return Err(ParseError {
                kind: ErrorKind::Syntax("Empty instruction segment due to trailing ';;'".to_string()),
                location: Some(items.last().unwrap().source_location()),
            });
        }
    }

    if instructions.is_empty() && items.len() == 1 && items[0].kind == UnilangTokenKind::Delimiter(Cow::Borrowed(";;"))
    {
       return Err(ParseError {
            kind: ErrorKind::Syntax("Empty instruction segment: input is only ';;'".to_string()),
            location: Some(items[0].source_location()),
        });
    }

    Ok(instructions)
  }

  /// Parses a single instruction from a slice of RichItems.
  fn parse_single_instruction_from_rich_items<'s_slice, 'input : 's_slice>
  (
    &'input self,
    instruction_rich_items : &'s_slice [RichItem<'input>]
  )
  -> Result<GenericInstruction<'input>, ParseError>
  {
    if instruction_rich_items.is_empty()
    {
      return Err( ParseError {
        kind: ErrorKind::Syntax( "Internal error: parse_single_instruction_from_rich_items called with empty items".to_string() ),
        location: None,
      });
    }

    // Determine overall location
    let first_item_loc = instruction_rich_items.first().unwrap().source_location();
    let last_item_loc = instruction_rich_items.last().unwrap().source_location();
    let overall_location = match ( &first_item_loc, &last_item_loc )
    {
        ( SourceLocation::StrSpan{ start: s1, .. }, SourceLocation::StrSpan{ end: e2, .. } ) =>
            SourceLocation::StrSpan{ start: *s1, end: *e2 },
        ( SourceLocation::SliceSegment{ segment_index: idx1, start_in_segment: s1, .. }, SourceLocation::SliceSegment{ segment_index: idx2, end_in_segment: e2, .. } ) if idx1 == idx2 =>
            SourceLocation::SliceSegment{ segment_index: *idx1, start_in_segment: *s1, end_in_segment: *e2 },
        _ => first_item_loc, // Fallback
    };

    let mut command_path_slices = Vec::new();
    let mut help_requested = false;
    let mut remaining_items_idx = 0;

    // Parse Command Path
    for (idx, item) in instruction_rich_items.iter().enumerate()
    {
      remaining_items_idx = idx;
      match &item.kind {
        UnilangTokenKind::Identifier(s) | UnilangTokenKind::UnquotedValue(s) =>
        {
          command_path_slices.push(s.as_ref().to_string());
        }
        UnilangTokenKind::Operator(op_cow) if op_cow.as_ref() == "?" =>
        {
          // If '?' is encountered, it might be a help operator.
          // Path parsing stops here. We check if it's the last significant item.
          remaining_items_idx = idx; // Current item is '?'
          break;
        }
        _ =>
        {
          // Not a path component, stop path parsing.
          // This item (at idx) will be the first potential argument or error.
          break;
        }
      }
      // If loop finishes, all items were path components.
      if idx == instruction_rich_items.len() - 1 {
        remaining_items_idx = idx + 1;
      }
    }

    // Check for Help Operator
    // It must be the *next* item after the path, or the only item if no path.
    // Or if the path loop broke on '?', check that '?'
    if remaining_items_idx < instruction_rich_items.len() {
        let current_item = &instruction_rich_items[remaining_items_idx];
        if current_item.kind == UnilangTokenKind::Operator(Cow::Borrowed("?")) {
            // Check if it's the last item in the instruction_rich_items slice
            // or if subsequent items are not suitable for arguments (e.g. another ';;' which shouldn't be here)
            if remaining_items_idx == instruction_rich_items.len() - 1 {
                help_requested = true;
                remaining_items_idx += 1; // Consume the '?'
            } else {
                // '?' is not the last significant item, this might be an error later
                // depending on argument parsing rules (e.g. "? arg").
                // For now, we assume '?' must be effectively last for help.
                // This logic will be refined with argument parsing.
                // If path was empty and this is the first item:
                if command_path_slices.is_empty() && remaining_items_idx == 0 {
                    help_requested = true;
                    remaining_items_idx += 1;
                }
            }
        }
    }


    // For Increment 4, remaining_items (instruction_rich_items[remaining_items_idx..]) are not processed further.
    // They will be handled in Increment 5 for argument parsing.
    // If after path and help, there are still items that are not arguments, it will be an error in Inc 5.

    Ok( GenericInstruction {
      command_path_slices,
      named_arguments : HashMap::new(),
      positional_arguments : Vec::new(),
      help_requested,
      overall_location,
    })
  }
}