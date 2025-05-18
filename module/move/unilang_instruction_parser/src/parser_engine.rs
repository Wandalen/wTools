//! Contains the core parsing logic for unilang instructions.

use crate::config::UnilangParserOptions;
use crate::error::{ ParseError, ErrorKind, SourceLocation };
use crate::instruction::{ GenericInstruction, Argument };
use crate::item_adapter::{ classify_split, RichItem, UnilangTokenKind, unescape_string };
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
  pub fn parse_single_str<'input>( &'input self, input : &'input str ) -> Result< Vec< GenericInstruction >, ParseError >
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
  pub fn parse_slice<'input>( &'input self, input_segments : &'input [&'input str] ) -> Result< Vec< GenericInstruction >, ParseError >
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
  -> Result<Vec<GenericInstruction>, ParseError>
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
  -> Result<GenericInstruction, ParseError>
  {
    if instruction_rich_items.is_empty()
    {
      return Err( ParseError {
        kind: ErrorKind::Syntax( "Internal error: parse_single_instruction_from_rich_items called with empty items".to_string() ),
        location: None,
      });
    }

    let first_item_loc = instruction_rich_items.first().unwrap().source_location();
    let last_item_loc = instruction_rich_items.last().unwrap().source_location();
    let overall_location = match ( &first_item_loc, &last_item_loc )
    {
        ( SourceLocation::StrSpan{ start: s1, .. }, SourceLocation::StrSpan{ end: e2, .. } ) =>
            SourceLocation::StrSpan{ start: *s1, end: *e2 },
        ( SourceLocation::SliceSegment{ segment_index: idx1, start_in_segment: s1, .. }, SourceLocation::SliceSegment{ segment_index: idx2, end_in_segment: e2, .. } ) if idx1 == idx2 =>
            SourceLocation::SliceSegment{ segment_index: *idx1, start_in_segment: *s1, end_in_segment: *e2 },
        _ => first_item_loc,
    };

    let mut command_path_slices = Vec::new();
    let mut help_requested = false;
    let mut items_cursor = 0;

    // Parse Command Path
    while items_cursor < instruction_rich_items.len() {
        let item = &instruction_rich_items[items_cursor];

        // Peek ahead: if current is Ident/Unquoted and next is '::', it's an arg name.
        if (matches!(item.kind, UnilangTokenKind::Identifier(_)) || matches!(item.kind, UnilangTokenKind::UnquotedValue(_)))
            && items_cursor + 1 < instruction_rich_items.len()
            && instruction_rich_items[items_cursor + 1].kind == UnilangTokenKind::Delimiter(Cow::Borrowed("::"))
        {
            break;
        }

        match &item.kind {
            UnilangTokenKind::Identifier(s) | UnilangTokenKind::UnquotedValue(s) => {
                command_path_slices.push(s.as_ref().to_string());
                items_cursor += 1;
            }
            UnilangTokenKind::Operator(op_cow) if op_cow.as_ref() == "?" => {
                break;
            }
            _ => {
                break;
            }
        }
    }

    // Check for Help Operator
    if items_cursor < instruction_rich_items.len() {
        let item = &instruction_rich_items[items_cursor];
        if item.kind == UnilangTokenKind::Operator(Cow::Borrowed("?")) {
            if items_cursor == instruction_rich_items.len() - 1 {
                help_requested = true;
                items_cursor += 1;
            } else {
                if command_path_slices.is_empty() && items_cursor == 0 {
                    help_requested = true;
                    items_cursor += 1;
                }
            }
        }
    }

    let mut named_arguments = HashMap::new();
    let mut positional_arguments = Vec::new();
    let mut expect_named_arg_value = false;
    let mut current_named_arg_name : Option<(String, SourceLocation)> = None;
    // TODO: Implement E6 argument order rules (e.g. positional before named) more strictly.

    while items_cursor < instruction_rich_items.len() {
        let item = &instruction_rich_items[items_cursor];
        let current_item_location = item.source_location(); // Store for potential error reporting

        if expect_named_arg_value {
            items_cursor += 1; // Consume item that will be the value
            match &item.kind {
                UnilangTokenKind::Identifier(val_s) | UnilangTokenKind::UnquotedValue(val_s) | UnilangTokenKind::QuotedValue(val_s) => {
                    let (name, name_loc) = current_named_arg_name.take().unwrap();
                    if named_arguments.contains_key(&name) {
                        return Err(ParseError{ kind: ErrorKind::Syntax(format!("Duplicate named argument: {}", name)), location: Some(name_loc) });
                    }
                    named_arguments.insert(name, Argument {
                        name_slice: None,
                        value: unescape_string(val_s.as_ref()),
                        name_location: Some(name_loc),
                        value_location: item.source_location(),
                    });
                    expect_named_arg_value = false;
                }
                _ => return Err(ParseError{ kind: ErrorKind::Syntax("Expected value after '::' for named argument".to_string()), location: Some(current_item_location) }),
            }
        } else {
            // item is current_item_at_cursor (before potential increment below)
            match &item.kind {
                UnilangTokenKind::Identifier(s) | UnilangTokenKind::UnquotedValue(s) => {
                    // Look ahead to see if the *next* token is "::"
                    if items_cursor + 1 < instruction_rich_items.len() &&
                       instruction_rich_items[items_cursor + 1].kind == UnilangTokenKind::Delimiter(Cow::Borrowed("::"))
                    {
                        // Current 'item' is the name
                        current_named_arg_name = Some((s.as_ref().to_string(), item.source_location()));
                        items_cursor += 2; // Consume name and '::'
                        expect_named_arg_value = true;
                    } else {
                        // Positional argument
                        positional_arguments.push(Argument{
                            name_slice: None,
                            value: unescape_string(s.as_ref()),
                            name_location: None,
                            value_location: item.source_location(),
                        });
                        items_cursor += 1; // Consume item
                    }
                }
                UnilangTokenKind::QuotedValue(s) => {
                    // Always a positional argument if not expecting a named value
                    positional_arguments.push(Argument{
                        name_slice: None,
                        value: unescape_string(s.as_ref()),
                        name_location: None,
                        value_location: item.source_location(),
                    });
                    items_cursor += 1; // Consume item
                }
                UnilangTokenKind::Delimiter(d_cow) if d_cow.as_ref() == "::" => {
                     return Err(ParseError{ kind: ErrorKind::Syntax("Unexpected '::' without preceding argument name".to_string()), location: Some(item.source_location()) });
                }
                _ => return Err(ParseError{ kind: ErrorKind::Syntax(format!("Unexpected token in arguments: '{}'", item.inner.string)), location: Some(item.source_location()) }),
            }
        }
    }
    if expect_named_arg_value {
        return Err(ParseError{ kind: ErrorKind::Syntax("Expected value for named argument but found end of instruction".to_string()), location: current_named_arg_name.map(|(_,loc)| loc).or_else(|| instruction_rich_items.last().map(|i|i.source_location())) });
    }

    Ok( GenericInstruction {
      command_path_slices,
      named_arguments,
      positional_arguments,
      help_requested,
      overall_location,
    })
  }
}