#!/usr/bin/env bash

# create_blank_crate - Create minimal wTools crate scaffolds
# Version: 1.0.0

set -euo pipefail

# ============================================================================
# Version Check
# ============================================================================

if (( BASH_VERSINFO[0] < 4 )); then
  echo "[ERROR] [SYSTEM]: Bash 4.0+ required, found ${BASH_VERSION}" >&2
  exit 1
fi

# ============================================================================
# Parameter Definitions
# ============================================================================

# Associative array for parameter values
declare -A params

# Parameter metadata: name -> required|optional|type|pattern|description|default
declare -A param_meta=(
  [name]="required|string|^[a-zA-Z][a-zA-Z0-9_]*$|Crate name following Rust conventions|"
  [description]="optional|string|.*|Human-readable crate description|A wTools crate"
  [function]="optional|string|^[a-zA-Z_][a-zA-Z0-9_]*$|Primary placeholder function name|"
  [categories]="optional|csv|.*|Comma-separated Cargo categories|development-tools"
  [keywords]="optional|csv|.*|Comma-separated Cargo keywords|fundamental,general-purpose"
  [verify]="optional|bool|^[01]$|Run cargo test and clippy after creation|0"
  [dry_run]="optional|bool|^[01]$|Preview without creating files|0"
  [verbosity]="optional|int|^[0-5]$|Output verbosity level (0-5)|1"
)

# Initialize parameters with defaults
for param_name in "${!param_meta[@]}"; do
  IFS='|' read -r req type pattern desc default <<< "${param_meta[$param_name]}"
  params[$param_name]="$default"
done

# ============================================================================
# Help Display
# ============================================================================

show_help() {
  cat <<'EOF'
create_blank_crate - Create minimal wTools crate scaffolds

USAGE
  ./create_blank_crate name::CRATE_NAME [PARAMS]

REQUIRED PARAMETERS
  name::VALUE              Crate name following Rust conventions

OPTIONAL PARAMETERS
  description::VALUE       Human-readable crate description
                           (default: "A wTools crate")

  function::VALUE          Primary placeholder function name
                           (default: derived from crate name)

  categories::VALUE        Comma-separated Cargo categories
                           (default: "development-tools")

  keywords::VALUE          Comma-separated Cargo keywords
                           (default: "fundamental,general-purpose")

  verify::BOOL             Run cargo test and clippy after creation
                           (0|1, default: 0)

  dry_run::BOOL           Preview without creating files
                           (0|1, default: 0)

  verbosity::LEVEL        Output verbosity (0-5, default: 1)
                           0: Silent (errors only)
                           1: Minimal (errors + warnings)
                           2: Normal (errors + warnings + info)
                           3: Verbose (above + debug)
                           4: Trace (above + command execution)
                           5: Maximum (above + file contents)

UNIVERSAL COMMANDS
  .                        Display this help
  .help                    Display this help

EXIT CODES
  0    Success
  1    General error
  2    Usage error (invalid parameters)
  3    File system error
  4    Verification failed

EXAMPLES
  # Minimal usage
  ./create_blank_crate name::my_crate

  # With description
  ./create_blank_crate name::my_crate description::"My awesome crate"

  # Full specification
  ./create_blank_crate name::my_crate description::"Config management" \
    function::load_config categories::config,development-tools \
    keywords::config,fundamental verify::1 verbosity::2

  # Dry run to preview
  ./create_blank_crate name::test_crate dry_run::1 verbosity::2

  # Show help
  ./create_blank_crate .

PROJECT
  Part of the wTools ecosystem: https://github.com/Wandalen/wTools
EOF
}

# ============================================================================
# Logging Functions
# ============================================================================

log() {
  local level=$1
  local component=$2
  local message=$3
  local required_verbosity=${4:-1}

  if (( ${params[verbosity]:-1} >= required_verbosity )); then
    echo "[$level] [$component]: $message" >&2
  fi
}

# ============================================================================
# Parameter Parsing
# ============================================================================

parse_params() {
  if (( $# == 0 )); then
    show_help
    exit 0
  fi

  for arg in "$@"; do
    # Handle help commands
    if [[ "$arg" == "." || "$arg" == ".help" ]]; then
      show_help
      exit 0
    fi

    # Parse param::value format
    if [[ "$arg" =~ ^([a-zA-Z_][a-zA-Z0-9_]*)::(.*)$ ]]; then
      local key="${BASH_REMATCH[1]}"
      local value="${BASH_REMATCH[2]}"

      # Check if parameter is known
      if [[ -z "${param_meta[$key]:-}" ]]; then
        log "ERROR" "VALIDATION" "Unknown parameter: $key" 0
        log "INFO" "VALIDATION" "Use '.' for help and list of valid parameters" 0
        exit 2
      fi

      params[$key]="$value"
    else
      log "ERROR" "VALIDATION" "Invalid parameter format: $arg" 0
      log "ERROR" "VALIDATION" "Expected format: param::value" 0
      log "INFO" "VALIDATION" "Use '.' for help" 0
      exit 2
    fi
  done
}

# ============================================================================
# Parameter Validation
# ============================================================================

validate_params() {
  local validation_failed=0

  # Check required parameters and validate all
  for param_name in "${!param_meta[@]}"; do
    IFS='|' read -r req type pattern desc default <<< "${param_meta[$param_name]}"
    local value="${params[$param_name]}"

    # Check required parameters
    if [[ "$req" == "required" && -z "$value" ]]; then
      log "ERROR" "VALIDATION" "Parameter '$param_name' is required" 0
      validation_failed=1
      continue
    fi

    # Skip validation for empty optional parameters
    if [[ "$req" == "optional" && -z "$value" ]]; then
      continue
    fi

    # Validate pattern
    if [[ ! "$value" =~ $pattern ]]; then
      log "ERROR" "VALIDATION" "Invalid value for '$param_name': '$value'" 0
      log "ERROR" "VALIDATION" "Expected pattern: $pattern" 0
      validation_failed=1
    fi
  done

  if (( validation_failed )); then
    log "INFO" "VALIDATION" "Use '.' for help and examples" 0
    exit 2
  fi

  # Normalize boolean values
  for param_name in verify dry_run; do
    case "${params[$param_name]}" in
      1|true|yes|on) params[$param_name]=1 ;;
      0|false|no|off|"") params[$param_name]=0 ;;
      *)
        log "ERROR" "VALIDATION" "$param_name must be boolean (0|1), got '${params[$param_name]}'" 0
        exit 2
        ;;
    esac
  done

  # Derive function name if not provided
  if [[ -z "${params[function]}" ]]; then
    # Smart derivation based on crate name
    local crate_name="${params[name]}"

    # Common patterns
    if [[ "$crate_name" == *"_hierarchy" ]]; then
      params[function]="load_hierarchy"
    elif [[ "$crate_name" == *"_config"* ]]; then
      params[function]="load_config"
    elif [[ "$crate_name" == "saferun" ]]; then
      params[function]="safe_execute"
    elif [[ "$crate_name" == "runbox" ]]; then
      params[function]="run_sandboxed"
    elif [[ "$crate_name" == "silo" ]]; then
      params[function]="create_silo"
    elif [[ "$crate_name" == "obox" ]]; then
      params[function]="init"
    else
      params[function]="init"
    fi

    log "DEBUG" "VALIDATION" "Derived function name: ${params[function]}" 3
  fi

  # Validate crate name doesn't conflict with existing directory
  if [[ -e "${params[name]}" && ${params[dry_run]} -eq 0 ]]; then
    log "ERROR" "VALIDATION" "Directory or file '${params[name]}' already exists" 0
    log "INFO" "VALIDATION" "Choose a different name or remove existing file/directory" 0
    exit 2
  fi
}

# ============================================================================
# Execution Wrapper
# ============================================================================

execute() {
  local description=$1
  shift
  local command="$*"

  log "DEBUG" "EXEC" "$description" 3
  log "TRACE" "EXEC" "Command: $command" 4

  if (( ${params[dry_run]} )); then
    log "DRY-RUN" "EXEC" "Would execute: $description" 1
    log "DRY-RUN" "EXEC" "  Command: $command" 2
    return 0
  else
    if (( ${params[verbosity]} >= 4 )); then
      local start=$(date +%s%N)
      eval "$command" || return $?
      local end=$(date +%s%N)
      local duration=$(( (end - start) / 1000000 ))
      log "TRACE" "EXEC" "Completed in ${duration}ms" 4
    else
      eval "$command" || return $?
    fi
  fi
}

# ============================================================================
# File Generation Functions
# ============================================================================

generate_cargo_toml() {
  local crate_name="${params[name]}"
  local description="${params[description]}"
  local categories="${params[categories]}"
  local keywords="${params[keywords]}"

  # Convert CSV to TOML array format
  local categories_array=""
  IFS=',' read -ra CATS <<< "$categories"
  for cat in "${CATS[@]}"; do
    cat=$(echo "$cat" | xargs)  # Trim whitespace
    categories_array+="\"$cat\", "
  done
  categories_array="${categories_array%, }"  # Remove trailing comma

  local keywords_array=""
  IFS=',' read -ra KEYS <<< "$keywords"
  for key in "${KEYS[@]}"; do
    key=$(echo "$key" | xargs)  # Trim whitespace
    keywords_array+="\"$key\", "
  done
  keywords_array="${keywords_array%, }"  # Remove trailing comma

  cat <<EOF
[package]
name = "${crate_name}"
version = "0.1.0"
edition = "2021"
authors = [
  "Kostiantyn Wandalen <wandalen@obox.systems>",
]
license = "MIT"
readme = "readme.md"
documentation = "https://docs.rs/${crate_name}"
repository = "https://github.com/Wandalen/wTools/tree/master/module/blank/${crate_name}"
homepage = "https://github.com/Wandalen/wTools/tree/master/module/blank/${crate_name}"
description = """
${description}
"""
categories = [ ${categories_array} ]
keywords = [ ${keywords_array} ]

[workspace]

[package.metadata.docs.rs]
features = [ "full" ]
all-features = false

[features]
default = [ "enabled" ]
full = [ "enabled" ]
enabled = []

[dependencies]

[dev-dependencies]
EOF
}

generate_license() {
  cat <<'EOF'
MIT License

Copyright (c) 2024 wandalen

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOF
}

generate_readme() {
  local crate_name="${params[name]}"
  local description="${params[description]}"

  cat <<EOF
# Module :: ${crate_name}

${description}

## Status

This is a blank crate template. Implementation is pending.

## Features

- Feature \`enabled\`: Core functionality (default)
- Feature \`full\`: All features

## Usage

\`\`\`rust
use ${crate_name}::*;
\`\`\`

## License

Licensed under MIT license.
EOF
}

generate_lib_rs() {
  local crate_name="${params[name]}"
  local function_name="${params[function]}"
  local description="${params[description]}"

  cat <<EOF
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/${crate_name}/latest/${crate_name}/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]

/// Placeholder function for ${description,,}.
#[ cfg( feature = "enabled" ) ]
pub fn ${function_name}()
{
}
EOF
}

generate_smoke_test() {
  cat <<'EOF'
#[ test ]
fn basic()
{
}
EOF
}

generate_tests_rs() {
  cat <<'EOF'
#[ cfg( feature = "enabled" ) ]
mod inc;
EOF
}

generate_inc_mod() {
  cat <<'EOF'
mod basic_test;
EOF
}

generate_inc_basic_test() {
  cat <<'EOF'
#[ test ]
fn basic()
{
}
EOF
}

# ============================================================================
# Main Crate Creation Logic
# ============================================================================

create_crate_structure() {
  local crate_name="${params[name]}"

  log "INFO" "CREATION" "Creating crate structure for '$crate_name'" 2

  # Create directory structure
  execute "Creating directory structure" \
    "mkdir -p '$crate_name'/{src,tests/inc}"

  # Generate Cargo.toml
  if (( ${params[dry_run]} )); then
    log "DRY-RUN" "FILE" "Would create: $crate_name/Cargo.toml" 2
    if (( ${params[verbosity]} >= 5 )); then
      log "DUMP" "FILE" "Content preview:" 5
      generate_cargo_toml | sed 's/^/    /' >&2
    fi
  else
    log "DEBUG" "FILE" "Creating $crate_name/Cargo.toml" 3
    generate_cargo_toml > "$crate_name/Cargo.toml" || {
      log "ERROR" "FILESYSTEM" "Failed to create Cargo.toml" 0
      exit 3
    }
    if (( ${params[verbosity]} >= 5 )); then
      log "DUMP" "FILE" "Created Cargo.toml:" 5
      cat "$crate_name/Cargo.toml" | sed 's/^/    /' >&2
    fi
  fi

  # Generate license
  if (( ${params[dry_run]} )); then
    log "DRY-RUN" "FILE" "Would create: $crate_name/license" 2
  else
    log "DEBUG" "FILE" "Creating $crate_name/license" 3
    generate_license > "$crate_name/license" || {
      log "ERROR" "FILESYSTEM" "Failed to create license" 0
      exit 3
    }
  fi

  # Generate readme.md
  if (( ${params[dry_run]} )); then
    log "DRY-RUN" "FILE" "Would create: $crate_name/readme.md" 2
  else
    log "DEBUG" "FILE" "Creating $crate_name/readme.md" 3
    generate_readme > "$crate_name/readme.md" || {
      log "ERROR" "FILESYSTEM" "Failed to create readme.md" 0
      exit 3
    }
  fi

  # Generate src/lib.rs
  if (( ${params[dry_run]} )); then
    log "DRY-RUN" "FILE" "Would create: $crate_name/src/lib.rs" 2
    if (( ${params[verbosity]} >= 5 )); then
      log "DUMP" "FILE" "Content preview:" 5
      generate_lib_rs | sed 's/^/    /' >&2
    fi
  else
    log "DEBUG" "FILE" "Creating $crate_name/src/lib.rs" 3
    generate_lib_rs > "$crate_name/src/lib.rs" || {
      log "ERROR" "FILESYSTEM" "Failed to create src/lib.rs" 0
      exit 3
    }
  fi

  # Generate tests/smoke_test.rs
  if (( ${params[dry_run]} )); then
    log "DRY-RUN" "FILE" "Would create: $crate_name/tests/smoke_test.rs" 2
  else
    log "DEBUG" "FILE" "Creating $crate_name/tests/smoke_test.rs" 3
    generate_smoke_test > "$crate_name/tests/smoke_test.rs" || {
      log "ERROR" "FILESYSTEM" "Failed to create tests/smoke_test.rs" 0
      exit 3
    }
  fi

  # Generate tests/tests.rs
  if (( ${params[dry_run]} )); then
    log "DRY-RUN" "FILE" "Would create: $crate_name/tests/tests.rs" 2
  else
    log "DEBUG" "FILE" "Creating $crate_name/tests/tests.rs" 3
    generate_tests_rs > "$crate_name/tests/tests.rs" || {
      log "ERROR" "FILESYSTEM" "Failed to create tests/tests.rs" 0
      exit 3
    }
  fi

  # Generate tests/inc/mod.rs
  if (( ${params[dry_run]} )); then
    log "DRY-RUN" "FILE" "Would create: $crate_name/tests/inc/mod.rs" 2
  else
    log "DEBUG" "FILE" "Creating $crate_name/tests/inc/mod.rs" 3
    generate_inc_mod > "$crate_name/tests/inc/mod.rs" || {
      log "ERROR" "FILESYSTEM" "Failed to create tests/inc/mod.rs" 0
      exit 3
    }
  fi

  # Generate tests/inc/basic_test.rs
  if (( ${params[dry_run]} )); then
    log "DRY-RUN" "FILE" "Would create: $crate_name/tests/inc/basic_test.rs" 2
  else
    log "DEBUG" "FILE" "Creating $crate_name/tests/inc/basic_test.rs" 3
    generate_inc_basic_test > "$crate_name/tests/inc/basic_test.rs" || {
      log "ERROR" "FILESYSTEM" "Failed to create tests/inc/basic_test.rs" 0
      exit 3
    }
  fi

  log "INFO" "SUCCESS" "Created crate structure at ./$crate_name" 1
}

# ============================================================================
# Verification
# ============================================================================

verify_crate() {
  if (( ${params[verify]} == 0 )); then
    log "DEBUG" "VERIFY" "Verification skipped (verify::0)" 3
    return 0
  fi

  if (( ${params[dry_run]} )); then
    log "DRY-RUN" "VERIFY" "Would run: cargo test --manifest-path ${params[name]}/Cargo.toml --all-features" 2
    log "DRY-RUN" "VERIFY" "Would run: cargo clippy --manifest-path ${params[name]}/Cargo.toml --all-targets --all-features" 2
    return 0
  fi

  local crate_name="${params[name]}"

  log "INFO" "VERIFY" "Running cargo test..." 2

  if (( ${params[verbosity]} >= 3 )); then
    cargo test --manifest-path "$crate_name/Cargo.toml" --all-features 2>&1 | sed 's/^/    /' >&2
  else
    cargo test --manifest-path "$crate_name/Cargo.toml" --all-features > /dev/null 2>&1
  fi

  if (( $? != 0 )); then
    log "ERROR" "VERIFY" "cargo test failed" 0
    log "INFO" "VERIFY" "Run with verbosity::3 to see test output" 0
    exit 4
  fi

  log "INFO" "VERIFY" "Running cargo clippy..." 2

  if (( ${params[verbosity]} >= 3 )); then
    cargo clippy --manifest-path "$crate_name/Cargo.toml" --all-targets --all-features -- -D warnings 2>&1 | sed 's/^/    /' >&2
  else
    cargo clippy --manifest-path "$crate_name/Cargo.toml" --all-targets --all-features -- -D warnings > /dev/null 2>&1
  fi

  if (( $? != 0 )); then
    log "ERROR" "VERIFY" "cargo clippy failed" 0
    log "INFO" "VERIFY" "Run with verbosity::3 to see clippy output" 0
    exit 4
  fi

  log "INFO" "SUCCESS" "Verification passed" 1
}

# ============================================================================
# Main Execution
# ============================================================================

main() {
  parse_params "$@"
  validate_params

  # Dump parameters if verbosity is maximum
  if (( ${params[verbosity]} >= 5 )); then
    log "DUMP" "PARAMS" "All parameters:" 5
    for key in "${!params[@]}"; do
      printf "    %-20s = %s\n" "$key" "${params[$key]}" >&2
    done
  fi

  create_crate_structure
  verify_crate

  if (( ${params[dry_run]} == 0 )); then
    log "INFO" "SUCCESS" "Crate '${params[name]}' created successfully" 1
  else
    log "INFO" "DRY-RUN" "Dry run completed, no files created" 1
  fi

  exit 0
}

# Execute main with all arguments
main "$@"
