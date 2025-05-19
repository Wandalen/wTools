//! Contains the core parsing logic for unilang instructions.

use crate::config::UnilangParserOptions;
use crate::error::{ ParseError, ErrorKind, SourceLocation };
use crate::instruction::{ GenericInstruction, Argument };
use crate::item_adapter::{ classify_split, RichItem, UnilangTokenKind, unescape_string };
use std::collections::HashMap;
use strs_tools::string::split::SplitType;

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
      if self.options.whitespace_is_separator && split_item.typ == SplitType::Delimeter && split_item.string.trim().is_empty()
      {
        continue;
      }
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
        if self.options.whitespace_is_separator && split_item.typ == SplitType::Delimeter && split_item.string.trim().is_empty()
        {
          continue;
        }
        let classified_kind = classify_split( &split_item, &self.options );
        rich_items_accumulator_vec.push( RichItem { inner: split_item, segment_idx: Some( seg_idx ), kind: classified_kind } );
      }
    }

    self.analyze_items_to_instructions( &rich_items_accumulator_vec )
  }

  fn analyze_items_to_instructions<'input>
  (
    &'input self,
    items : &'input [RichItem<'input>],
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
        if item_ref.kind == UnilangTokenKind::Delimiter(";;".to_string()) {
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
        if items.last().unwrap().kind == UnilangTokenKind::Delimiter(";;".to_string()) {
             return Err(ParseError {
                kind: ErrorKind::Syntax("Empty instruction segment due to trailing ';;'".to_string()),
                location: Some(items.last().unwrap().source_location()),
            });
        }
    }

    if instructions.is_empty() && items.len() == 1 && items[0].kind == UnilangTokenKind::Delimiter(";;".to_string())
    {
       return Err(ParseError {
            kind: ErrorKind::Syntax("Empty instruction segment: input is only ';;'".to_string()),
            location: Some(items[0].source_location()),
        });
    }

    Ok(instructions)
  }

  fn parse_single_instruction_from_rich_items<'input>
  (
    &'input self,
    instruction_rich_items : &'input [RichItem<'input>]
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
    let mut items_cursor = 0;

    // Phase 1: Consume Command Path (Restored to greedy version that passed temp_path_only_multi_segment_path)
    while items_cursor < instruction_rich_items.len() {
        let current_item = &instruction_rich_items[items_cursor];
        match &current_item.kind {
            UnilangTokenKind::Identifier(s) | UnilangTokenKind::UnquotedValue(s) => {
                if items_cursor + 1 < instruction_rich_items.len() {
                    if instruction_rich_items[items_cursor + 1].kind == UnilangTokenKind::Delimiter("::".to_string()) {
                        break;
                    }
                }
                command_path_slices.push(s.clone());
                items_cursor += 1;
            }
            UnilangTokenKind::Operator(_) | UnilangTokenKind::QuotedValue(_) => {
                break;
            }
            _ => {
                break;
            }
        }
    }

    // Phase 2: Check for Help Operator immediately after the path
    let mut help_requested = false;
    if items_cursor < instruction_rich_items.len() {
        let potential_help_item = &instruction_rich_items[items_cursor];
        if potential_help_item.kind == UnilangTokenKind::Operator("?".to_string()) {
            if items_cursor == instruction_rich_items.len() - 1 {
                help_requested = true;
                items_cursor += 1;
            }
        }
    }

    // Phase 3: Argument Parsing
    let mut named_arguments = HashMap::new();
    let mut positional_arguments = Vec::new();
    let mut current_named_arg_name_data : Option<(&'input str, SourceLocation)> = None;
    let mut seen_named_argument = false;

    while items_cursor < instruction_rich_items.len() {
        let item = &instruction_rich_items[items_cursor];
        let current_item_location = item.source_location();

        if let Some((name_str_ref, name_loc)) = current_named_arg_name_data.take() {
            match &item.kind {
                UnilangTokenKind::Identifier(val_s) | UnilangTokenKind::UnquotedValue(val_s) | UnilangTokenKind::QuotedValue(val_s) => {
                    let name_key = name_str_ref.to_string();
                    if self.options.error_on_duplicate_named_arguments && named_arguments.contains_key(&name_key) {
                        return Err(ParseError{ kind: ErrorKind::Syntax(format!("Duplicate named argument: {}", name_key)), location: Some(name_loc.clone()) });
                    }
                    named_arguments.insert(name_key.clone(), Argument {
                        name: Some(name_key),
                        value: unescape_string(val_s),
                        name_location: Some(name_loc),
                        value_location: item.source_location(),
                    });
                    items_cursor += 1;
                }
                _ => return Err(ParseError{ kind: ErrorKind::Syntax(format!("Expected value for named argument '{}' but found {:?}", name_str_ref, item.kind)), location: Some(current_item_location) }),
            }
        } else {
            match &item.kind {
                UnilangTokenKind::Identifier(s_val_owned) | UnilangTokenKind::UnquotedValue(s_val_owned) => {
                    if items_cursor + 1 < instruction_rich_items.len() &&
                       instruction_rich_items[items_cursor + 1].kind == UnilangTokenKind::Delimiter("::".to_string())
                    {
                        current_named_arg_name_data = Some((item.inner.string, item.source_location()));
                        items_cursor += 2;
                        seen_named_argument = true;
                    } else {
                        if seen_named_argument && self.options.error_on_positional_after_named {
                             return Err(ParseError{ kind: ErrorKind::Syntax("Positional argument encountered after a named argument.".to_string()), location: Some(item.source_location()) });
                        }
                        positional_arguments.push(Argument{
                            name: None,
                            value: unescape_string(s_val_owned),
                            name_location: None,
                            value_location: item.source_location(),
                        });
                        items_cursor += 1;
                    }
                }
                UnilangTokenKind::QuotedValue(s_val_owned) => {
                    if seen_named_argument && self.options.error_on_positional_after_named {
                         return Err(ParseError{ kind: ErrorKind::Syntax("Positional argument encountered after a named argument.".to_string()), location: Some(item.source_location()) });
                    }
                    positional_arguments.push(Argument{
                        name: None,
                        value: unescape_string(s_val_owned),
                        name_location: None,
                        value_location: item.source_location(),
                    });
                    items_cursor += 1;
                }
                UnilangTokenKind::Delimiter(d_s) if d_s == "::" => {
                     // dbg!("Inside Delimiter('::') arm, about to return Err for named_arg_missing_name_error"); // Removed
                     return Err(ParseError{ kind: ErrorKind::Syntax("Unexpected '::' without preceding argument name or after a previous value.".to_string()), location: Some(item.source_location()) });
                }
                UnilangTokenKind::Operator(op_s) if op_s == "?" => {
                     return Err(ParseError{ kind: ErrorKind::Syntax("Unexpected help operator '?' amidst arguments.".to_string()), location: Some(item.source_location()) });
                }
                _ => return Err(ParseError{ kind: ErrorKind::Syntax(format!("Unexpected token in arguments: '{}' ({:?})", item.inner.string, item.kind)), location: Some(item.source_location()) }),
            }
        }
    }

    if let Some((name_str_ref, name_loc)) = current_named_arg_name_data {
        return Err(ParseError{ kind: ErrorKind::Syntax(format!("Expected value for named argument '{}' but found end of instruction", name_str_ref)), location: Some(name_loc) });
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