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
use strs_tools::string::split::{ Split, SplitType, SplitOptionsFormer }; // Added SplitOptionsFormer import

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
    let rich_items_vec = self.tokenize_input( input, None )?;
    self.analyze_items_to_instructions( &rich_items_vec )
  }

  /// Parses a slice of input strings into a vector of [`GenericInstruction`]s.
  #[allow(clippy::missing_errors_doc)]
  pub fn parse_slice<'input>( &'input self, input_segments : &'input [&'input str] ) -> Result< Vec< GenericInstruction >, ParseError >
  {
    let mut rich_items_accumulator_vec : Vec<RichItem<'input>> = Vec::new();

    for ( seg_idx, segment_str ) in input_segments.iter().enumerate()
    {
      let segment_rich_items = self.tokenize_input( segment_str, Some( seg_idx ) )?;
      rich_items_accumulator_vec.extend( segment_rich_items );
    }
    self.analyze_items_to_instructions( &rich_items_accumulator_vec )
  }

  /// Tokenizes the input string using `strs_tools` and classifies each split item.
  fn tokenize_input<'input>
  (
    &'input self,
    input : &'input str,
    segment_idx : Option<usize>,
  ) -> Result<Vec<RichItem<'input>>, ParseError>
  {
    let mut rich_items_vec : Vec<RichItem<'input>> = Vec::new();

    let delimiters_as_str_slice: Vec<&str> = self.options.main_delimiters.iter().map(|s| s.as_str()).collect();
    let split_options_former = SplitOptionsFormer::new( delimiters_as_str_slice )
    .src( input )
    .quoting( true )
    ;
    let split_iterator = split_options_former.perform();

    for split_item in split_iterator {
        // Skip empty delimited strings if whitespace is separator, as strs_tools might return them
        if self.options.whitespace_is_separator && split_item.typ == SplitType::Delimeted && split_item.string.trim().is_empty() {
            continue;
        }
        let classified_kind = classify_split( &split_item, &self.options );
        rich_items_vec.push( RichItem { inner: split_item, segment_idx, kind: classified_kind } );
    }

    Ok(rich_items_vec)
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
                        kind: ErrorKind::Syntax("Empty instruction segment due0 to ';;'".to_string()),
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

    eprintln!("DEBUG: significant_items: {:?}", significant_items);

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

    eprintln!("DEBUG: Initial items_cursor: {}", items_cursor);

    // Handle optional leading dot
    if let Some(first_item) = significant_items.get(0) {
        if let UnilangTokenKind::Delimiter(d) = &first_item.kind {
            if d == "." {
                items_cursor += 1; // Consume the leading dot
                eprintln!("DEBUG: Consumed leading dot. items_cursor: {}", items_cursor);
            }
        }
    }

    // Consume command path segments
    while items_cursor < significant_items.len() {
        let current_item = significant_items[items_cursor];
        eprintln!("DEBUG: Command path loop. items_cursor: {}, current_item: {:?}", items_cursor, current_item);

        // Check for named argument delimiter first, as it always terminates command path
        if let UnilangTokenKind::Delimiter(d) = &current_item.kind {
            if d == "::" {
                eprintln!("DEBUG: Named argument delimiter. Breaking command path parsing.");
                break;
            }
        }

        if let UnilangTokenKind::Identifier(s) = &current_item.kind {
            command_path_slices.push(s.clone());
            items_cursor += 1; // Consume the identifier
            eprintln!("DEBUG: Added identifier to command_path_slices: {:?}. items_cursor: {}", command_path_slices, items_cursor);

            // After an identifier, if there are more items, check if the next is a delimiter (space or dot)
            // or another identifier (for space-separated command path segments).
            if items_cursor < significant_items.len() {
                let next_item = significant_items[items_cursor];
                match &next_item.kind {
                    UnilangTokenKind::Delimiter(d) if d == "." || (self.options.whitespace_is_separator && d.trim().is_empty()) => {
                        items_cursor += 1; // Consume the delimiter
                        eprintln!("DEBUG: Consumed command path delimiter '{}'. items_cursor: {}", d, items_cursor);
                        // Continue loop to expect next identifier
                    },
                    UnilangTokenKind::Identifier(_) => {
                        // Another identifier, means it's a space-separated command path segment.
                        eprintln!("DEBUG: Identifier followed by another identifier (space-separated command path). Continuing.");
                        // Do not consume here, let the next loop iteration consume it.
                    },
                    _ => {
                        eprintln!("DEBUG: Non-command-path token after identifier. Breaking command path parsing.");
                        break; // Any other token type means end of command path
                    }
                }
            }
            // If no more items, command path ends naturally.
        } else if let UnilangTokenKind::Delimiter(d) = &current_item.kind {
            // If the current item is a delimiter (space or dot), skip it and continue.
            if d == "." || (self.options.whitespace_is_separator && d.trim().is_empty()) {
                items_cursor += 1; // Consume the delimiter
                eprintln!("DEBUG: Skipping command path delimiter '{}'. items_cursor: {}", d, items_cursor);
            } else {
                eprintln!("DEBUG: Non-command-path token. Breaking command path parsing.");
                break;
            }
        } else {
            // Any other token type indicates the end of the command path.
            eprintln!("DEBUG: Non-command-path token. Breaking command path parsing.");
            break;
        }
    }
    eprintln!("DEBUG: Final command_path_slices before arguments: {:?}", command_path_slices);

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
            let (value_str_raw, value_loc_raw) = match &item.kind {
                UnilangTokenKind::Identifier(val_s) => (val_s.as_str(), item.source_location()),
                UnilangTokenKind::QuotedValue(val_s) => {
                    // For QuotedValue, the `val_s` already contains the inner content without quotes
                    (val_s.as_str(), item.source_location())
                },
                _ => return Err(ParseError{ kind: ErrorKind::Syntax(format!("Expected value for named argument '{name_str_ref}' but found {:?}", item.kind)), location: Some(item.source_location()) }),
            };

            let final_value = unescape_string_with_errors(value_str_raw, &value_loc_raw)?;

            let name_key = name_str_ref.to_string();
            if self.options.error_on_duplicate_named_arguments && named_arguments.contains_key(&name_key) {
                return Err(ParseError{ kind: ErrorKind::Syntax(format!("Duplicate named argument: {name_key}")), location: Some(name_loc.clone()) });
            }

            named_arguments.insert(name_key.clone(), Argument {
                name: Some(name_key),
                value: final_value,
                name_location: Some(name_loc),
                value_location: item.source_location(),
            });
            items_cursor += 1;
        } else {
            match &item.kind {
                UnilangTokenKind::Identifier(_s_val_owned) | UnilangTokenKind::QuotedValue(_s_val_owned) => {
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
                        let (value_str_raw, value_loc_raw) = match &item.kind {
                            UnilangTokenKind::Identifier(val_s) => (val_s.as_str(), item.source_location()),
                            UnilangTokenKind::QuotedValue(val_s) => (val_s.as_str(), item.source_location()),
                            _ => unreachable!("Should be Identifier or QuotedValue here"), // Filtered by outer match
                        };
                        positional_arguments.push(Argument{
                            name: None,
                            value: unescape_string_with_errors(value_str_raw, &value_loc_raw)?,
                            name_location: None,
                            value_location: item.source_location(),
                        });
                        items_cursor += 1;
                    }
                }
                UnilangTokenKind::Unrecognized(_s) => { // Removed `if s_val_owned.starts_with("--")`
                    // Treat as a positional argument if it's not a delimiter
                    if !item.inner.string.trim().is_empty() && !self.options.main_delimiters.iter().any(|d| d == item.inner.string) {
                        if seen_named_argument && self.options.error_on_positional_after_named {
                             return Err(ParseError{ kind: ErrorKind::Syntax("Positional argument encountered after a named argument.".to_string()), location: Some(item.source_location()) });
                        }
                        positional_arguments.push(Argument{
                            name: None,
                            value: item.inner.string.to_string(),
                            name_location: None,
                            value_location: item.source_location(),
                        });
                        items_cursor += 1;
                    } else {
                        return Err(ParseError{ kind: ErrorKind::Syntax(format!("Unexpected token in arguments: '{}' ({:?})", item.inner.string, item.kind)), location: Some(item.source_location()) });
                    }
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