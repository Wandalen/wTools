//! Contains the core parsing logic for unilang instructions.
//!
//! The main entry point is the [`Parser`] struct, which can be configured with
//! [`UnilangParserOptions`]. It provides methods to parse instruction strings
//! or slices of strings into a `Vec<GenericInstruction>`.

use crate::config::UnilangParserOptions;
use crate::error::{ ParseError, ErrorKind, SourceLocation };
use crate::instruction::{ GenericInstruction, Argument };
use crate::item_adapter::{ classify_split, RichItem, UnilangTokenKind, unescape_string_with_errors };
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
  /// Creates a new `Parser` with the specified [`UnilangParserOptions`].
  #[allow(clippy::must_use_candidate)]
  pub fn new( options : UnilangParserOptions ) -> Self
  {
    Self { options }
  }

  /// Parses a single input string into a vector of [`GenericInstruction`]s.
  #[allow(clippy::missing_errors_doc)]
  pub fn parse_single_str<'input>( &'input self, input : &'input str ) -> Result< Vec< GenericInstruction >, ParseError >
  {
    let mut rich_items_vec : Vec<RichItem<'input>> = Vec::new();
    let mut split_iterator = self.options.to_split_options_former( input ).perform();

  #[allow(clippy::while_let_on_iterator)]
    while let Some( split_item ) = split_iterator.next()
    {
      if self.options.whitespace_is_separator && (split_item.typ == SplitType::Delimeted || split_item.typ == SplitType::Delimiter) && split_item.string.trim().is_empty()
      {
        continue;
      }
      let classified_kind = classify_split( &split_item, &self.options );
      rich_items_vec.push( RichItem { inner: split_item, segment_idx: None, kind: classified_kind } );
    }
    self.analyze_items_to_instructions( &rich_items_vec )
  }

  /// Parses a slice of input strings into a vector of [`GenericInstruction`]s.
  #[allow(clippy::missing_errors_doc)]
  pub fn parse_slice<'input>( &'input self, input_segments : &'input [&'input str] ) -> Result< Vec< GenericInstruction >, ParseError >
  {
    let mut rich_items_accumulator_vec : Vec<RichItem<'input>> = Vec::new();

    for ( seg_idx, segment_str ) in input_segments.iter().enumerate()
    {
      let mut split_iterator = self.options.to_split_options_former( segment_str ).perform();
      #[allow(clippy::while_let_on_iterator)]
      while let Some( split_item ) = split_iterator.next()
      {
        if self.options.whitespace_is_separator && split_item.typ == SplitType::Delimeted && split_item.string.trim().is_empty()
        {
          continue;
        }
        let classified_kind = classify_split( &split_item, &self.options );
        rich_items_accumulator_vec.push( RichItem { inner: split_item, segment_idx: Some( seg_idx ), kind: classified_kind } );
      }
    }
    self.analyze_items_to_instructions( &rich_items_accumulator_vec )
  }

  /// Analyzes a stream of `RichItem`s, groups them by `;;` or change in `segment_idx`,
  /// and parses each group into a `GenericInstruction`.
  fn analyze_items_to_instructions<'input>
  (
    &'input self,
    items : &'input [RichItem<'input>],
  )
  -> Result<Vec<GenericInstruction>, ParseError>
  {
    let mut instructions = Vec::new();
    if items.is_empty() {
        return Ok(instructions);
    }

    let mut start_index = 0;
    let mut current_segment_idx_val = items[0].segment_idx;

    for i in 0..items.len() {
        let item_ref = &items[i];

        let is_boundary_delimiter = item_ref.kind == UnilangTokenKind::Delimiter(";;".to_string());
        let is_segment_idx_change = item_ref.segment_idx != current_segment_idx_val && item_ref.segment_idx.is_some();

        if is_boundary_delimiter || is_segment_idx_change {
            let segment_to_parse = &items[start_index..i]; // Segment before boundary

            if !segment_to_parse.is_empty() {
                let first_significant_token_opt = segment_to_parse.iter().find(|item| {
                    match &item.kind {
                        UnilangTokenKind::Delimiter(s) | UnilangTokenKind::Unrecognized(s) => !s.trim().is_empty(),
                        _ => true,
                    }
                });

                if let Some(first_significant_token) = first_significant_token_opt {
                    if let UnilangTokenKind::Unrecognized(s) = &first_significant_token.kind {
                        if s == "#" { /* Comment segment, skip */ }
                        else { instructions.push(self.parse_single_instruction_from_rich_items(segment_to_parse)?); }
                    } else {
                        instructions.push(self.parse_single_instruction_from_rich_items(segment_to_parse)?);
                    }
                } // Else: segment was all whitespace, skip.
            } else if is_boundary_delimiter { // Empty segment specifically due to ';;'
                 if start_index == i { // Handles `;; cmd` or `cmd ;;;; cmd`
                    return Err(ParseError {
                        kind: ErrorKind::Syntax("Empty instruction segment due to ';;'".to_string()),
                        location: Some(item_ref.source_location()),
                    });
                }
            }

            start_index = if is_boundary_delimiter { i + 1 } else { i };
            current_segment_idx_val = item_ref.segment_idx;
        }
    }

    // Process the final segment after the loop
    if start_index < items.len() {
        let segment_to_parse = &items[start_index..];
        if !segment_to_parse.is_empty() {
            let first_significant_token_opt = segment_to_parse.iter().find(|item| {
                match &item.kind {
                    UnilangTokenKind::Delimiter(s) | UnilangTokenKind::Unrecognized(s) => !s.trim().is_empty(),
                    _ => true,
                }
            });

            if let Some(first_significant_token) = first_significant_token_opt {
                if let UnilangTokenKind::Unrecognized(s) = &first_significant_token.kind {
                    if s == "#" { /* Comment segment, skip */ }
                    else { instructions.push(self.parse_single_instruction_from_rich_items(segment_to_parse)?); }
                } else {
                    instructions.push(self.parse_single_instruction_from_rich_items(segment_to_parse)?);
                }
            } // Else: final segment was all whitespace, skip.
        }
    }

    // Check for trailing delimiter that results in an empty instruction segment
    if !items.is_empty() && items.last().unwrap().kind == UnilangTokenKind::Delimiter(";;".to_string()) && start_index == items.len() {
        // This means the last instruction was followed by a trailing delimiter,
        // and no new instruction was formed from the segment after it.
        return Err(ParseError {
            kind: ErrorKind::TrailingDelimiter,
            location: Some(items.last().unwrap().source_location()),
        });
    }

    // Specific check for input that is *only* a comment (already handled by loop logic if it results in empty instructions)
    // Specific check for input that is *only* ";;"
    if instructions.is_empty() && items.len() == 1 && items[0].kind == UnilangTokenKind::Delimiter(";;".to_string())
    {
       return Err(ParseError {
            kind: ErrorKind::Syntax("Empty instruction segment due to ';;'".to_string()),
            location: Some(items[0].source_location()),
        });
    }

    Ok(instructions)
  }

  /// Parses a single instruction from a slice of `RichItem`s.
  #[allow(clippy::too_many_lines)]
  #[allow(unreachable_patterns)]
  fn parse_single_instruction_from_rich_items<'input>
  (
    &'input self,
    instruction_rich_items : &'input [RichItem<'input>]
  )
  -> Result<GenericInstruction, ParseError>
  {
    let significant_items: Vec<&RichItem<'input>> = instruction_rich_items.iter().filter(|item| {
        match &item.kind {
            UnilangTokenKind::Delimiter(s) | UnilangTokenKind::Unrecognized(s) => !s.trim().is_empty(),
            _ => true,
        }
    }).collect();

    if significant_items.is_empty()
    {
      return Err( ParseError {
        kind: ErrorKind::Syntax( "Internal error or empty/comment segment: parse_single_instruction_from_rich_items called with effectively empty items".to_string() ),
        location: if instruction_rich_items.is_empty() { None } else { Some(instruction_rich_items.first().unwrap().source_location()) },
      });
    }

    let first_item_loc = significant_items.first().unwrap().source_location();
    let last_item_loc = significant_items.last().unwrap().source_location();
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

    // Phase 1: Consume Command Path
    while items_cursor < significant_items.len() {
        let current_item = significant_items[items_cursor];

        // This `if let` block is for named argument detection, not path termination.
        // It should remain as is, as it correctly breaks if a named argument is next.
        if items_cursor + 1 < significant_items.len() &&
           significant_items[items_cursor + 1].kind == UnilangTokenKind::Delimiter("::".to_string()) {
            break; // Break to handle named argument
        }

        match &current_item.kind {
            UnilangTokenKind::Identifier(s) => {
                // Existing logic for segment index change
                #[allow(clippy::collapsible_if)]
                if !command_path_slices.is_empty() {
                    if items_cursor > 0 {
                         let previous_item_in_path_source = significant_items[items_cursor -1];
                         if current_item.segment_idx != previous_item_in_path_source.segment_idx {
                             break; // Segment change, end of path
                         }
                    }
                }
                command_path_slices.push(s.clone());
                items_cursor += 1;
            },
            UnilangTokenKind::QuotedValue(_) => {
                // Quoted values are always arguments, not part of the command path
                break;
            },
            UnilangTokenKind::Unrecognized(s) => {
                // If an Unrecognized token contains '.' or '/', treat it as a path segment
                if s.contains('.') || s.contains('/') {
                    let segments: Vec<String> = s.split(['.', '/']).map(ToString::to_string).collect();
                    for segment in segments {
                        if !segment.is_empty() {
                            command_path_slices.push(segment);
                        }
                    }
                    items_cursor += 1;
                } else {
                    // Otherwise, it's an unexpected token, so break
                    break;
                }
            },
            _ => {
                // Any other token type (including other delimiters/operators) also ends the command path
                break;
            }
        }
    }

    let mut help_requested = false;
    if items_cursor < significant_items.len() {
        let potential_help_item = significant_items[items_cursor];
        #[allow(clippy::collapsible_if)]
        if potential_help_item.kind == UnilangTokenKind::Operator("?".to_string()) {
            if items_cursor == significant_items.len() - 1 {
                help_requested = true;
                items_cursor += 1;
            }
        }
    }

    let mut named_arguments = HashMap::new();
    let mut positional_arguments = Vec::new();
    let mut current_named_arg_name_data : Option<(&'input str, SourceLocation)> = None;
    let mut seen_named_argument = false;

    // eprintln!("[ARG_LOOP_START] Initial items_cursor: {}, significant_items_len: {}", items_cursor, significant_items.len());
    while items_cursor < significant_items.len() {
        let item = significant_items[items_cursor];
        // let current_item_location = item.source_location();
        // eprintln!("[ARG_MATCH_ITEM] items_cursor: {}, item: {:?}", items_cursor, item);


        if let Some((name_str_ref, name_loc)) = current_named_arg_name_data.take() {
            match &item.kind {
                UnilangTokenKind::Identifier(val_s) | UnilangTokenKind::QuotedValue(val_s) => {
                    let name_key = name_str_ref.to_string();
                    if self.options.error_on_duplicate_named_arguments && named_arguments.contains_key(&name_key) {
                        return Err(ParseError{ kind: ErrorKind::Syntax(format!("Duplicate named argument: {name_key}")), location: Some(name_loc.clone()) });
                    }

                    let value_str_to_unescape = val_s;
                    let base_loc_for_unescape = if let UnilangTokenKind::QuotedValue(_) = &item.kind {
                        let (prefix_len, postfix_len) = self.options.quote_pairs.iter()
                            .find(|(p, _postfix)| item.inner.string.starts_with(*p))
                            .map_or((0,0), |(p, pf)| (p.len(), pf.len()));

                        match item.source_location() {
                            SourceLocation::StrSpan { start, end } => SourceLocation::StrSpan {
                                start: start + prefix_len,
                                end: end - postfix_len
                            },
                            SourceLocation::SliceSegment { segment_index, start_in_segment, end_in_segment } => SourceLocation::SliceSegment {
                                segment_index,
                                start_in_segment: start_in_segment + prefix_len,
                                end_in_segment: end_in_segment - postfix_len
                            },
                        }
                    } else {
                        item.source_location()
                    };

                    let final_value = if let UnilangTokenKind::QuotedValue(_) = &item.kind {
                        unescape_string_with_errors(value_str_to_unescape, &base_loc_for_unescape)?
                    } else {
                        value_str_to_unescape.to_string()
                    };


                    named_arguments.insert(name_key.clone(), Argument {
                        name: Some(name_key),
                        value: final_value,
                        name_location: Some(name_loc),
                        value_location: item.source_location(),
                    });
                    items_cursor += 1;
                }
                _ => return Err(ParseError{ kind: ErrorKind::Syntax(format!("Expected value for named argument '{name_str_ref}' but found {:?}", item.kind)), location: Some(item.source_location()) }),
            }
        } else {
            match &item.kind {
                UnilangTokenKind::Identifier(s_val_owned) | UnilangTokenKind::QuotedValue(s_val_owned) => {
                    if items_cursor + 1 < significant_items.len() &&
                       significant_items[items_cursor + 1].kind == UnilangTokenKind::Delimiter("::".to_string())
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
                            value: if let UnilangTokenKind::QuotedValue(_) = &item.kind {
                                let (prefix_len, postfix_len) = self.options.quote_pairs.iter()
                                    .find(|(p, _postfix)| item.inner.string.starts_with(*p))
                                    .map_or((0,0), |(p, pf)| (p.len(), pf.len()));

                                let base_loc_for_unescape = match item.source_location() {
                                    SourceLocation::StrSpan { start, end } => SourceLocation::StrSpan {
                                        start: start + prefix_len,
                                        end: end - postfix_len
                                    },
                                    SourceLocation::SliceSegment { segment_index, start_in_segment, end_in_segment } => SourceLocation::SliceSegment {
                                        segment_index,
                                        start_in_segment: start_in_segment + prefix_len,
                                        end_in_segment: end_in_segment - postfix_len
                                    },
                                };
                                unescape_string_with_errors(s_val_owned, &base_loc_for_unescape)?
                            } else {
                                s_val_owned.to_string()
                            },
                            name_location: None,
                            value_location: item.source_location(),
                        });
                        items_cursor += 1;
                    }
                }
                UnilangTokenKind::Unrecognized(s_val_owned) if s_val_owned.starts_with("--") => {
                    // Treat as a positional argument
                    if seen_named_argument && self.options.error_on_positional_after_named {
                         return Err(ParseError{ kind: ErrorKind::Syntax("Positional argument encountered after a named argument.".to_string()), location: Some(item.source_location()) });
                    }
                    positional_arguments.push(Argument{
                        name: None,
                        value: s_val_owned.to_string(),
                        name_location: None,
                        value_location: item.source_location(),
                    });
                    items_cursor += 1;
                }
                UnilangTokenKind::Delimiter(d_s) if d_s == "::" => {
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
        return Err(ParseError{ kind: ErrorKind::Syntax(format!("Expected value for named argument '{name_str_ref}' but found end of instruction")), location: Some(name_loc) });
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