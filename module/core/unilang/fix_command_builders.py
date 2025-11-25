#!/usr/bin/env python3
"""
Fix CommandDefinition builders to use type-state pattern.
Adds missing .description() field to all CommandDefinition builders.
"""

import re
import sys
from pathlib import Path

def fix_command_definition(content):
    """
    Fix CommandDefinition builders by adding .description() field.
    """
    # Pattern to match CommandDefinition::builder() through .build()
    # This captures the entire builder chain
    pattern = r'(CommandDefinition::builder\(\)\s*\n\s*\.name\(\s*"([^"]+)"\s*\)\s*\n)'

    def replacement(match):
        full_match = match.group(0)
        name = match.group(2)

        # Check if .description() already exists in the next few lines
        # Look ahead to see if description is already present
        start_pos = match.end()
        next_chars = content[start_pos:start_pos+200]

        if '.description(' in next_chars:
            return full_match

        # Check if .namespace() or .hint() comes next
        # We'll extract hint value to use as description
        namespace_pattern = r'\.namespace\(\s*"([^"]*)"\s*\)\s*\n'
        hint_pattern = r'\.hint\(\s*"([^"]+)"\s*\)\s*\n'

        # Find hint value
        hint_match = re.search(hint_pattern, content[start_pos:start_pos+500])
        if hint_match:
            hint_value = hint_match.group(1)
        else:
            # Default description based on name
            hint_value = f"{name} command"

        # Add .description() after .name()
        result = full_match + f'  .description( "{hint_value}" )\n'
        return result

    # Apply the fix
    fixed_content = re.sub(pattern, replacement, content)
    return fixed_content

def process_file(filepath):
    """Process a single file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content
        fixed_content = fix_command_definition(content)

        if fixed_content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(fixed_content)
            print(f"Fixed: {filepath}")
            return True
        else:
            print(f"No changes: {filepath}")
            return False
    except Exception as e:
        print(f"Error processing {filepath}: {e}", file=sys.stderr)
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: fix_command_builders.py <file1> <file2> ...")
        sys.exit(1)

    files = sys.argv[1:]
    fixed_count = 0

    for filepath in files:
        if process_file(filepath):
            fixed_count += 1

    print(f"\nFixed {fixed_count} out of {len(files)} files")

if __name__ == '__main__':
    main()
