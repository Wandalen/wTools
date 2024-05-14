crate::mod_interface!
{
  /// This module defines a raw representation of parsed commands, providing a foundation for further processing and
  /// transformation into other formats. The raw representation captures the essential information about each command in
  /// a straightforward and easy-to-work-with format, allowing for efficient manipulation and subsequent conversion to
  /// other representations.
  layer command;
  
  /// This module is responsible for processing command-line arguments and parsing them into a raw representation of a
  /// program containing multiple parsed commands. The input list of arguments is transformed into a structured format,
  /// allowing the program to efficiently handle and manipulate the parsed commands.
  layer parser;
}
