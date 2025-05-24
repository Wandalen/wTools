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
///
/// This struct is responsible for tokenizing the input using `strs_tools` (configured by
/// [`UnilangParserOptions`]), classifying tokens, and then applying syntactic rules
/// to build a sequence of [`GenericInstruction`]s.
///
/// ## Parsing Process
///
/// 1.  **Tokenization**: The input string (or each string in a slice) is split into raw tokens
///     (called `Split` items) by `strs_tools::string::split::SplitIterator`. This is configured
///     by `UnilangParserOptions::to_split_options_former`.
/// 2.  **Classification**: Each `Split` item is classified into a [`UnilangTokenKind`] (e.g., Identifier,
///     Operator, QuotedValue) and wrapped in a [`RichItem`] which also includes source location info.
/// 3.  **Instruction Grouping**: The stream of `RichItem`s is divided into segments based on the
///     instruction separator `;;`.
/// 4.  **Single Instruction Parsing**: Each segment of `RichItem`s is then parsed into a single
///     [`GenericInstruction`]. This involves:
///     *   **Path Parsing**: Identifying the command path (sequence of identifiers/unquoted values).
///     *   **Help Operator Parsing**: Checking for a trailing `?`.
///     *   **Argument Parsing**: Processing named (`name::value`) and positional arguments, including
///         handling quotes and unescaping values.
///
/// Errors encountered at any stage are reported as a [`ParseError`].
#[derive(Debug)]
pub struct Parser
{
  options : UnilangParserOptions,
}

impl Parser
{
  /// Creates a new `Parser` with the specified [`UnilangParserOptions`].
  ///
  /// # Arguments
  ///
  /// * `options`: The configuration options that will guide the parsing process.
  pub fn new( options : UnilangParserOptions ) -> Self
  {
    Self { options }
  }

  /// Parses a single input string into a vector of [`GenericInstruction`]s.
  ///
  /// The input string can contain multiple instructions separated by `;;`.
  ///
  /// # Arguments
  ///
  /// * `input`: The input string to parse.
  ///
  /// # Returns
  ///
  /// * `Ok(Vec<GenericInstruction>)` if parsing is successful.
  /// * `Err(ParseError)` if a parsing error occurs.
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
    // eprintln!("[DEBUG] Input: \"{}\", RichItems from parse_single_str: {:?}", input, rich_items_vec);

    self.analyze_items_to_instructions( &rich_items_vec )
  }

  /// Parses a slice of input strings into a vector of [`GenericInstruction`]s.
  ///
  /// Each string in the slice is treated as a segment. The parser processes these segments
  /// sequentially. Instruction separators `;;` can still be used within individual segments.
  /// `SourceLocation` in errors or parsed items will use `SliceSegment` to indicate
  /// the origin segment and position within that segment.
  ///
  /// # Arguments
  ///
  /// * `input_segments`: A slice of string slices, where each inner slice is a segment of the input.
  ///
  /// # Returns
  ///
  /// * `Ok(Vec<GenericInstruction>)` if parsing is successful.
  /// * `Err(ParseError)` if a parsing error occurs.
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
    // eprintln!("[DEBUG] Input Slice: {:?}, RichItems from parse_slice: {:?}", input_segments, rich_items_accumulator_vec);

    self.analyze_items_to_instructions( &rich_items_accumulator_vec )
  }

  /// Analyzes a stream of `RichItem`s, groups them by the `;;` separator,
  /// and parses each group into a `GenericInstruction`.
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
            kind: ErrorKind::Syntax("Empty instruction segment due to ';;'".to_string()),
            location: Some(items[0].source_location()),
        });
    }


    Ok(instructions)
  }

  /// Parses a single instruction from a slice of `RichItem`s.
  /// This is the core logic for interpreting the command path, help operator, and arguments.
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

    // Phase 1: Consume Command Path
    while items_cursor < instruction_rich_items.len() {
        let current_item = &instruction_rich_items[items_cursor];

        if let UnilangTokenKind::Identifier(_) | UnilangTokenKind::UnquotedValue(_) = &current_item.kind {
            if items_cursor + 1 < instruction_rich_items.len() &&
               instruction_rich_items[items_cursor + 1].kind == UnilangTokenKind::Delimiter("::".to_string()) {
                break;
            }
        }

        match &current_item.kind {
            UnilangTokenKind::Identifier(s) | UnilangTokenKind::UnquotedValue(s) => {
                if !command_path_slices.is_empty() {
                    if items_cursor > 0 {
                         let previous_item_in_path_source = &instruction_rich_items[items_cursor -1];
                         if current_item.segment_idx != previous_item_in_path_source.segment_idx {
                             break;
                         }
                    }
                }
                command_path_slices.push(s.clone());
                // eprintln!("[PATH_DEBUG] Pushed to path: '{}', current path_slices: {:?}", s, command_path_slices);
                items_cursor += 1;
            }
            _ => {
                break;
            }
        }
    }

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

    let mut named_arguments = HashMap::new();
    let mut positional_arguments = Vec::new();
    let mut current_named_arg_name_data : Option<(&'input str, SourceLocation)> = None;
    let mut seen_named_argument = false;

    while items_cursor < instruction_rich_items.len() {
        let item = &instruction_rich_items[items_cursor];
        let current_item_location = item.source_location();

        if let Some((name_str_ref, name_loc)) = current_named_arg_name_data.take() {
            match &item.kind {
                UnilangTokenKind::Identifier(val_s) | UnilangTokenKind::UnquotedValue(val_s)
                | UnilangTokenKind::QuotedValue(val_s) => {
                    let name_key = name_str_ref.to_string();
                    if self.options.error_on_duplicate_named_arguments && named_arguments.contains_key(&name_key) {
                        return Err(ParseError{ kind: ErrorKind::Syntax(format!("Duplicate named argument: {}", name_key)), location: Some(name_loc.clone()) });
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
                                end_in_segment: end_in_segment - postfix_len,
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
                            value: s_val_owned.to_string(),
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

                    let (prefix_len, postfix_len) = self.options.quote_pairs.iter()
                        .find(|(p, _postfix)| item.inner.string.starts_with(*p))
                        .map_or((0,0), |(p, pf)| (p.len(), pf.len()));

                    let inner_content_location = match item.source_location() {
                        SourceLocation::StrSpan { start, end } => SourceLocation::StrSpan {
                            start: start + prefix_len,
                            end: end - postfix_len
                        },
                        SourceLocation::SliceSegment { segment_index, start_in_segment, end_in_segment } => SourceLocation::SliceSegment {
                            segment_index,
                            start_in_segment: start_in_segment + prefix_len,
                            end_in_segment: end_in_segment - postfix_len,
                        },
                    };
                    let unescaped_value = unescape_string_with_errors(s_val_owned, &inner_content_location)?;

                    positional_arguments.push(Argument{
                        name: None,
                        value: unescaped_value,
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
        return Err(ParseError{ kind: ErrorKind::Syntax(format!("Expected value for named argument '{}' but found end of instruction", name_str_ref)), location: Some(name_loc) });
    }

    // eprintln!("[FINAL_PATH_DEBUG] Final command_path_slices before Ok: {:?}", command_path_slices);

    Ok( GenericInstruction {
      command_path_slices,
      named_arguments,
      positional_arguments,
      help_requested,
      overall_location,
    })
  }
}