#!/usr/bin/env bash
# Basic genfile workflow example
#
# This script demonstrates the fundamental genfile workflow:
# 1. Starting REPL mode
# 2. Creating a new archive
# 3. Adding template files
# 4. Defining parameters
# 5. Saving the archive
#
# The commands are sent to genfile's REPL mode, which maintains
# archive state across commands within a single session.

set -e  # Exit on any error

echo "=== Genfile Basic Workflow Example ==="
echo
echo "This example demonstrates creating a simple template archive."
echo "Commands are executed in REPL mode to maintain state."
echo

# Create temporary directory for output
TEMP_DIR="/tmp/genfile-example-$$"
mkdir -p "$TEMP_DIR"
OUTPUT_FILE="$TEMP_DIR/basic-example.json"

echo "Step 1: Building genfile binary..."
cargo build --quiet --release

echo "Step 2: Executing workflow in REPL mode..."
echo

# Execute commands in REPL mode via heredoc
cargo run --quiet --release <<EOF
.archive.new name::"basic-example" description::"Basic template example"
.file.add path::"readme.txt" content::"Hello from {{project_name}}"
.parameter.add name::"project_name" description::"Project name" mandatory::true
.archive.save path::"$OUTPUT_FILE"
exit
EOF

echo
echo "=== Results ==="
echo

# Check if file was created
if [ -f "$OUTPUT_FILE" ]; then
  echo "✅ Success! Archive created at:"
  echo "   $OUTPUT_FILE"
  echo
  echo "Archive contents:"
  echo "---"
  cat "$OUTPUT_FILE" | head -20
  echo "..."
  echo
  echo "You can now load this archive with:"
  echo "   genfile .archive.load path::\"$OUTPUT_FILE\""
  echo
  echo "Or explore it interactively:"
  echo "   genfile"
  echo "   genfile[0]> .archive.load path::\"$OUTPUT_FILE\""
  echo "   genfile[1]> .file.list"
  echo "   genfile[2]> .parameter.list"
else
  echo "❌ Error: Archive file was not created"
  exit 1
fi
