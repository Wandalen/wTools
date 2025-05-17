//! Configuration for the unilang instruction parser.

// No direct import of SplitOptions needed here anymore, components will be stored.

/// Options to configure the behavior of the `unilang` parser.
///
/// This structure holds components needed to construct `strs_tools::string::split::SplitOptions`
/// for the initial splitting of the input string.
#[derive(Debug)]
pub struct UnilangParserOptions {
    // Components to build strs_tools::string::split::SplitOptions
    pub delimiters_and_operators: Vec<&'static str>,
    pub quoting_prefixes: Vec<&'static str>,
    pub quoting_postfixes: Vec<&'static str>,
    pub preserve_delimiters: bool,
    pub preserve_quoting: bool,
    pub stripping: bool,
    pub quoting: bool,
    pub preserve_empty: bool,
    // Other unilang-specific options that are not part of SplitOptions
    // will be handled post-splitting or stored here if needed.
    // For example:
    // pub escape_char: Option<char>,
    // pub comment_prefix: Option<&'static str>,
    // pub implicit_whitespace_delimit: bool,
}

impl Default for UnilangParserOptions {
    fn default() -> Self {
        const DELIMITERS_AND_OPERATORS: &[&str] = &[" ", "\t", "\n", "\r", "::", ";;", "?"]; // Added whitespace
        const QUOTE_PREFIXES: &[&str] = &["\"", "'"];
        const QUOTE_POSTFIXES: &[&str] = &["\"", "'"];

        Self {
            delimiters_and_operators: DELIMITERS_AND_OPERATORS.to_vec(),
            quoting_prefixes: QUOTE_PREFIXES.to_vec(),
            quoting_postfixes: QUOTE_POSTFIXES.to_vec(),
            preserve_delimiters: true, // Keep delimiters as separate items.
            preserve_quoting: false,   // Remove quotes from the content of quoted strings.
            stripping: true,             // Strip leading/trailing whitespace from each item.
            quoting: true,               // Enable handling of quoted strings.
            preserve_empty: false,    // Don't keep empty strings from splits.
            // escape_char: Some('\\'), // To be handled by unilang_parser
            // comment_prefix: Some("#"), // To be handled by unilang_parser
            // implicit_whitespace_delimit: true, // To be handled by unilang_parser
        }
    }
}