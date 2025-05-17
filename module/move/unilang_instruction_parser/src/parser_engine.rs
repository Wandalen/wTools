//! The core parsing engine for unilang instructions.

use crate::config::UnilangParserOptions;
use crate::error::{ParseError, ErrorKind, SourceLocation};
use crate::instruction::{Argument, GenericInstruction};
use strs_tools::string::split::Split as StrsSplit;
use std::borrow::Cow;

/// The main parser for unilang syntax.
#[derive(Debug)]
pub struct Parser {
    options: UnilangParserOptions,
}

impl Parser {
    pub fn new(options: UnilangParserOptions) -> Self {
        Self { options }
    }

    pub fn parse_single_str<'a>(&self, input: &'a str) -> Result<Vec<GenericInstruction<'a>>, ParseError> {
        // Filter out comment-only input before splitting
        if input.trim_start().starts_with('#') {
            return Ok(vec![]);
        }

        let mut former = strs_tools::string::split::split();
        former.src(input)
            .delimeter(self.options.delimiters_and_operators.clone())
            .preserving_empty(self.options.preserve_empty)
            .preserving_delimeters(self.options.preserve_delimiters)
            .preserving_quoting(self.options.preserve_quoting)
            .stripping(self.options.stripping)
            .quoting(self.options.quoting)
            .quoting_prefixes(self.options.quoting_prefixes.clone())
            .quoting_postfixes(self.options.quoting_postfixes.clone());

        let split_iterator = former.perform();
        let raw_splits: Vec<StrsSplit<'a>> = split_iterator.collect();

        // Detailed Plan Step 4 (Revised - Stuck Resolution): Populate start and end in RichItem for single string input.
        let rich_items: Vec<RichItem<'a>> = raw_splits.into_iter().map(|s| {
            // Use the actual start and end indices from Split
            let start = s.start;
            let end = s.end;
            RichItem {
              inner_split: s,
              segment_idx: None,
                start, // Populate start
                end,   // Populate end
            }
        }).collect();
        self.analyze_items_to_instructions_rich(rich_items)
    }

    pub fn parse_slice<'a>(&self, input_segments: &'a [&'a str]) -> Result<Vec<GenericInstruction<'a>>, ParseError> {
        let mut all_rich_items: Vec<RichItem<'a>> = Vec::new();
        for (seg_idx, segment_str) in input_segments.iter().enumerate() {
            // Filter out comment-only segments before splitting
            if segment_str.trim_start().starts_with('#') {
                continue;
            }

            let mut former = strs_tools::string::split::split();
            former.src(segment_str)
                .delimeter(self.options.delimiters_and_operators.clone())
                .preserving_empty(self.options.preserve_empty)
                .preserving_delimeters(self.options.preserve_delimiters) // Fixed typo here
                .preserving_quoting(self.options.preserve_quoting)
                .stripping(self.options.stripping)
                .quoting(self.options.quoting)
                .quoting_prefixes(self.options.quoting_prefixes.clone())
                .quoting_postfixes(self.options.quoting_postfixes.clone());
            let split_iterator = former.perform();
            // Detailed Plan Step 5 (Revised - Stuck Resolution): Populate start and end in RichItem for slice input.
            for split_item in split_iterator {
                // Use the actual start and end indices from Split
                let start = split_item.start;
                let end = split_item.end;
                all_rich_items.push(RichItem {
                  inner_split: split_item,
                  segment_idx: Some(seg_idx),
                    start, // Populate start
                    end,   // Populate end
                });
            }
        }
        self.analyze_items_to_instructions_rich(all_rich_items)
    }
}

// Detailed Plan Step 3 (Revised - Stuck Resolution): Modify RichItem to include start and end indices.
#[derive(Debug, Clone)]
struct RichItem<'a> {
    inner_split: StrsSplit<'a>,
    segment_idx: Option<usize>,
    start: usize, // Start index relative to the original input (string or slice segment)
    end: usize,   // End index relative to the original input (string or slice segment)
}

impl Parser {
    fn parse_single_instruction_group<'input>(
        &self,
        instruction_items_group: Vec<RichItem<'input>>,
    ) -> Result<GenericInstruction<'input>, ParseError> {
        if instruction_items_group.is_empty() {
            // Detailed Plan Step 4 (Revised): Update "Empty instruction group" error location.
            // Cannot provide a location for an empty group, so location remains None.
            return Err(ParseError {
                kind: ErrorKind::Syntax("Empty instruction group".to_string()),
                location: None,
            });
        }

        let mut command_path_slices = Vec::new();
        let mut help_requested = false;
        let mut named_arguments: std::collections::HashMap<&'input str, Argument<'input>> = std::collections::HashMap::new();
        let mut positional_arguments: Vec<Argument<'input>> = Vec::new();
        let overall_location = Self::rich_item_to_source_location_placeholder(&instruction_items_group[0]);
        let mut items_iter = instruction_items_group.into_iter().peekable();

        // Phase 1: Command Path Identification
        // The command path is the first Delimeted item if one exists.
        if let Some(first_item_peek) = items_iter.peek() {
            if first_item_peek.inner_split.typ == strs_tools::string::split::SplitType::Delimeted {
                let path_item = items_iter.next().unwrap(); // Consume the first Delimeted item as path
                let candidate = path_item.inner_split.string.trim();
                if !candidate.is_empty() {
                    // Split the candidate by whitespace and add non-empty segments to the path
                    command_path_slices.extend(
                        candidate.split_whitespace().filter(|s| !s.is_empty())
                    );
                }
            }
        }

        // "Missing command path" check
        if command_path_slices.is_empty() {
            let mut is_solely_help_q = false;
            if let Some(item_peek) = items_iter.peek() {
                if item_peek.inner_split.typ == strs_tools::string::split::SplitType::Delimeter && item_peek.inner_split.string == "?" {
                    let mut temp_clone = items_iter.clone();
                    temp_clone.next();
                    if temp_clone.peek().is_none() {
                        is_solely_help_q = true;
                    }
                }
            } else {
                is_solely_help_q = true;
            }

            if !is_solely_help_q {
                let loc = items_iter.peek().map(Self::rich_item_to_source_location_placeholder).unwrap_or(overall_location.clone());
                return Err(ParseError {
                    kind: ErrorKind::Syntax("Missing command path".to_string()),
                    location: Some(loc),
                });
            }
        }

        // Phase 2 & 3 Combined: Argument Parsing (incorporating Help Operator)
        // Help operator '?' can appear anywhere in the argument list.
        // We will iterate and if '?' is found, set flag and continue (it's consumed).
        // Other argument parsing logic will apply to other tokens.
        // A stray '?' not meant as help will be caught by the final Delimiter check if not consumed here.

        while let Some(current_item) = items_iter.next() {
            if current_item.inner_split.typ == strs_tools::string::split::SplitType::Delimeter && current_item.inner_split.string == "?" {
                help_requested = true;
                continue; // Consume '?' and move to the next item for argument parsing
            }

            if current_item.inner_split.typ == strs_tools::string::split::SplitType::Delimeted {
                let name_candidate_slice = current_item.inner_split.string.trim();
                if name_candidate_slice.is_empty() { continue; }

                if let Some(peeked_next) = items_iter.peek() {
                    if peeked_next.inner_split.typ == strs_tools::string::split::SplitType::Delimeter && peeked_next.inner_split.string == "::" {
                        items_iter.next();
                        if let Some(value_item) = items_iter.next() {
                            if value_item.inner_split.typ == strs_tools::string::split::SplitType::Delimeted {
                                let value_location = Self::rich_item_to_source_location_placeholder(&value_item);
                                let arg_value = self.unescape_string(value_item.inner_split.string, value_location.clone())?; // Handle Result
                                named_arguments.insert(
                                    name_candidate_slice,
                                    Argument {
                                        name_slice: Some(name_candidate_slice),
                                        value: arg_value,
                                        name_location: Some(Self::rich_item_to_source_location_placeholder(&current_item)),
                                        value_location, // Use the captured location
                                    },
                                );
                            } else {
                                return Err(ParseError {
                                    kind: ErrorKind::Syntax(format!("Named argument '{}::' not followed by a delimited value", name_candidate_slice)),
                                    location: Some(Self::rich_item_to_source_location_placeholder(&value_item)),
                                });
                            }
                        } else {
                            return Err(ParseError {
                                kind: ErrorKind::Syntax(format!("Named argument '{}::' not followed by a value", name_candidate_slice)),
                                location: Some(Self::rich_item_to_source_location_placeholder(&current_item)),
                            });
                        }
                    } else {
                        let value_location = Self::rich_item_to_source_location_placeholder(&current_item);
                        let arg_value = self.unescape_string(name_candidate_slice, value_location.clone())?; // Handle Result
                        positional_arguments.push(Argument {
                            name_slice: None,
                            value: arg_value,
                            name_location: None,
                            value_location, // Use the captured location
                        });
                    }
                } else {
                    let value_location = Self::rich_item_to_source_location_placeholder(&current_item);
                    let arg_value = self.unescape_string(name_candidate_slice, value_location.clone())?; // Handle Result
                    positional_arguments.push(Argument {
                        name_slice: None,
                        value: arg_value,
                        name_location: None,
                        value_location, // Use the captured location
                    });
                }
            } else if current_item.inner_split.typ == strs_tools::string::split::SplitType::Delimeter {
                 return Err(ParseError {
                    kind: ErrorKind::Syntax(format!("Unexpected delimiter '{}' in arguments section", current_item.inner_split.string)),
                    location: Some(Self::rich_item_to_source_location_placeholder(&current_item)),
                });
            }
        }

        Ok(GenericInstruction {
            command_path_slices,
            named_arguments,
            positional_arguments,
            help_requested,
            overall_location,
        })
    }

    // Detailed Plan Step 2.1 (Revised): Modify unescape_string to return Result and handle errors with location
    fn unescape_string<'input>(&self, s: &'input str, location: SourceLocation) -> Result<Cow<'input, str>, ParseError> { // Corrected Cow generic
        let trimmed = s.trim();
        if trimmed.contains('\\') {
            let mut unescaped = String::with_capacity(trimmed.len());
            let mut chars = trimmed.char_indices();
            while let Some((i, c)) = chars.next() {
                if c == '\\' {
                    if let Some((next_i, next_c)) = chars.next() {
                        match next_c {
                            '"' => unescaped.push('"'),
                            '\'' => unescaped.push('\''),
                            '\\' => unescaped.push('\\'),
                            _ => {
                                // Invalid escape sequence
                                let error_location = match &location {
                                    SourceLocation::StrSpan { start, .. } => SourceLocation::StrSpan { start: start + i, end: start + next_i + next_c.len_utf8() },
                                    SourceLocation::SliceSegment { segment_index, start_in_segment, .. } => SourceLocation::SliceSegment { segment_index: *segment_index, start_in_segment: start_in_segment + i, end_in_segment: start_in_segment + next_i + next_c.len_utf8() },
                                };
                                return Err(ParseError {
                                    kind: ErrorKind::InvalidEscapeSequence,
                                    location: Some(error_location),
                                });
                            }
                        }
                    } else {
                        // Trailing backslash
                        let error_location = match &location {
                            SourceLocation::StrSpan { start, .. } => SourceLocation::StrSpan { start: start + i, end: start + i + 1 },
                            SourceLocation::SliceSegment { segment_index, start_in_segment, .. } => SourceLocation::SliceSegment { segment_index: *segment_index, start_in_segment: start_in_segment + i, end_in_segment: start_in_segment + i + 1 },
                        };
                         return Err(ParseError {
                            kind: ErrorKind::InvalidEscapeSequence, // Or a specific TrailingBackslash kind if needed
                            location: Some(error_location),
                        });
                    }
                } else {
                    unescaped.push(c);
                }
            }
            Ok(Cow::Owned(unescaped))
        } else {
            Ok(Cow::Borrowed(trimmed))
        }
    }

    fn rich_item_to_source_location_placeholder(item: &RichItem) -> SourceLocation {
        // Use the actual start and end indices from the inner_split
        let start = item.start;
        let end = item.end;

        if let Some(seg_idx) = item.segment_idx {
            SourceLocation::SliceSegment {
                segment_index: seg_idx,
                start_in_segment: start,
                end_in_segment: end,
            }
        } else {
            SourceLocation::StrSpan {
                start,
                end,
            }
        }
    }

    fn analyze_items_to_instructions_rich<'input>(
        &self,
        items: Vec<RichItem<'input>>,
    ) -> Result<Vec<GenericInstruction<'input>>, ParseError> {
        let mut instructions = Vec::new();
        let filtered_items: Vec<RichItem<'input>> = items
            .into_iter()
            .filter(|item| {
                // Filter out items that are comments (start with # after trimming leading whitespace)
                item.inner_split.string.trim_start().chars().next() != Some('#')
            })
            .collect();

        if filtered_items.is_empty() {
            return Ok(instructions);
        }

        let mut current_instruction_items: Vec<RichItem<'input>> = Vec::new();
        for item in filtered_items {
            if item.inner_split.typ == strs_tools::string::split::SplitType::Delimeter && item.inner_split.string == ";;" {
                if !current_instruction_items.is_empty() {
                    let instruction = self.parse_single_instruction_group(current_instruction_items)?;
                    instructions.push(instruction);
                    current_instruction_items = Vec::new();
                }
            } else {
                current_instruction_items.push(item);
            }
        }

        if !current_instruction_items.is_empty() {
            let instruction = self.parse_single_instruction_group(current_instruction_items)?;
            instructions.push(instruction);
        }

        Ok(instructions)
    }
}