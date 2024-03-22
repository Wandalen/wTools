#!/bin/bash

# Check if at least one argument is provided
if [ $# -eq 0 ]; then
  echo "Usage: $0 directory [directory...]"
  exit 1
fi

# Function to convert line endings
convert_line_endings() {
  local file="$1"
  # Use sed to replace CRLF with LF in-place
  sed -i 's/\r$//' "$file"
}

# Iterate over all arguments
for dir in "$@"; do
  # Check if directory exists
  if [ ! -d "$dir" ]; then
    echo "Directory not found: $dir"
    continue
  fi

  # Find all .rs and .toml files, excluding .git directories, and convert line endings
  find "$dir" -type d -name .git -prune -o -type f \( -name "*.rs" -o -name "*.toml" \) -print0 | while IFS= read -r -d $'\0' file; do
    echo "Processing: $file"
    convert_line_endings "$file"
  done
done

echo "Conversion complete."
