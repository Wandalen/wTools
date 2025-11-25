#!/usr/bin/env python3
"""
Systematically fix CommandDefinition builders across the codebase.

Fixes:
1. Add missing .namespace(), .status(), .version() fields
2. Add missing .description() field if not present
3. Replace .end() with .build()
"""

import re
import sys
from pathlib import Path

def fix_command_definition_builders(content, filepath):
    """Fix CommandDefinition::builder() calls in the content."""

    # Find all CommandDefinition::builder() blocks
    # Match from builder() to .build() or .end()
    pattern = r'(CommandDefinition::builder\(\))(.*?)(\.(build|end)\(\))'

    def replacement(match):
        prefix = match.group(1)  # "CommandDefinition::builder()"
        body = match.group(2)    # Everything between builder() and build/end()
        ending = match.group(4)  # "build" or "end"

        # Check which required fields are missing
        has_description = '.description(' in body
        has_namespace = '.namespace(' in body
        has_status = '.status(' in body
        has_version = '.version(' in body

        # Extract hint value to use as description if description is missing
        hint_match = re.search(r'\.hint\(\s*"([^"]+)"\s*\)', body)
        hint_value = hint_match.group(1) if hint_match else "Command"

        # Build the fixed version
        result = prefix + body

        # Add missing fields before the ending
        additions = []

        if not has_description:
            additions.append(f'\n  .description( "{hint_value}" )')

        if not has_namespace:
            additions.append('\n  .namespace( "" )')

        if not has_status:
            additions.append('\n  .status( "stable" )')

        if not has_version:
            additions.append('\n  .version( "1.0.0" )')

        # Add all missing fields
        for addition in additions:
            result += addition

        # Always use .build()
        result += '\n  .build()'

        return result

    # Apply the fixes
    fixed_content = re.sub(pattern, replacement, content, flags=re.DOTALL)

    return fixed_content

def process_file(filepath):
    """Process a single file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content
        fixed_content = fix_command_definition_builders(content, filepath)

        if fixed_content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(fixed_content)
            print(f"✓ Fixed: {filepath}")
            return True
        else:
            print(f"  No changes: {filepath}")
            return False
    except Exception as e:
        print(f"✗ Error processing {filepath}: {e}", file=sys.stderr)
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: fix_all_builders.py <file1> <file2> ...")
        sys.exit(1)

    files = sys.argv[1:]
    fixed_count = 0

    for filepath in files:
        if process_file(filepath):
            fixed_count += 1

    print(f"\n✓ Fixed {fixed_count} out of {len(files)} files")

if __name__ == '__main__':
    main()
