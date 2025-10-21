#!/bin/bash
#
# Test Quality Monitoring Script
#
# This script provides comprehensive monitoring of test quality metrics
# including historical tracking, alerting, and automated quality gates.
#
# Usage:
#   ./quality_monitor.sh [command] [options]
#
# Commands:
#   assess    - Run quality assessment (default)
#   trend     - Show quality trend analysis
#   alert     - Check for quality alerts
#   report    - Generate comprehensive report
#   dashboard - Start quality dashboard
#   ci        - CI/CD integration mode

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TESTS_DIR="${PROJECT_ROOT}/tests"
REPORTS_DIR="${TESTS_DIR}/quality_reports"
CONFIG_FILE="${TESTS_DIR}/quality_config.json"
HISTORY_FILE="${REPORTS_DIR}/quality_history.json"

# Default thresholds
DEFAULT_FAIL_THRESHOLD=90.0
DEFAULT_WARN_THRESHOLD=85.0
DEFAULT_CRITICAL_THRESHOLD=70.0

# Create directories if they don't exist
mkdir -p "$REPORTS_DIR"

# Functions

show_help() {
  cat << EOF
🧪 Test Quality Monitoring Tool

USAGE:
    quality_monitor.sh [COMMAND] [OPTIONS]

COMMANDS:
    assess      Run quality assessment (default)
    trend       Show quality trend analysis over time
    alert       Check for quality alerts and notifications
    report      Generate comprehensive quality report
    dashboard   Start interactive quality dashboard
    ci          CI/CD integration mode with strict checks
    init        Initialize quality monitoring configuration
    help        Show this help message

OPTIONS:
    --tests-dir DIR         Tests directory [default: tests]
    --threshold SCORE       Fail threshold [default: 90.0]
    --format FORMAT         Output format: text, json, html, markdown [default: text]
    --output FILE           Output file (default: stdout)
    --verbose               Enable verbose output
    --no-history            Skip updating quality history
    --alert-webhook URL     Webhook URL for quality alerts
    --config FILE           Custom configuration file

EXAMPLES:
    quality_monitor.sh                              # Basic assessment
    quality_monitor.sh assess --format html -o report.html
    quality_monitor.sh trend --last 30             # Last 30 assessments
    quality_monitor.sh alert --webhook http://...  # Alert with webhook
    quality_monitor.sh ci --threshold 95.0         # CI mode with 95% threshold
    quality_monitor.sh dashboard --port 8080       # Start web dashboard

QUALITY GRADES:
    🌟 Excellent (95-100%) - Outstanding test quality
    ✅ Good (85-94%)       - High quality with minor improvements
    ⚠️ Fair (70-84%)       - Adequate quality needing attention
    ❌ Poor (50-69%)       - Significant issues requiring action
    🚨 Critical (<50%)     - Major problems needing immediate fix

For more information, see: tests/quality_metrics.md
EOF
}

log_info() {
  echo -e "${BLUE}ℹ️ $1${NC}"
}

log_success() {
  echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
  echo -e "${YELLOW}⚠️ $1${NC}"
}

log_error() {
  echo -e "${RED}❌ $1${NC}"
}

log_critical() {
  echo -e "${RED}🚨 $1${NC}"
}

# Parse command line arguments
COMMAND="assess"
TESTS_DIR_ARG=""
THRESHOLD=""
OUTPUT_FORMAT="text"
OUTPUT_FILE=""
VERBOSE=false
UPDATE_HISTORY=true
ALERT_WEBHOOK=""
CONFIG_FILE_ARG=""
EXTRA_ARGS=()

while [[ $# -gt 0 ]]; do
  case $1 in
    assess|trend|alert|report|dashboard|ci|init|help)
      COMMAND="$1"
      shift
      ;;
    --tests-dir)
      TESTS_DIR_ARG="$2"
      shift 2
      ;;
    --threshold)
      THRESHOLD="$2"
      shift 2
      ;;
    --format)
      OUTPUT_FORMAT="$2"
      shift 2
      ;;
    --output)
      OUTPUT_FILE="$2"
      shift 2
      ;;
    --verbose)
      VERBOSE=true
      shift
      ;;
    --no-history)
      UPDATE_HISTORY=false
      shift
      ;;
    --alert-webhook)
      ALERT_WEBHOOK="$2"
      shift 2
      ;;
    --config)
      CONFIG_FILE_ARG="$2"
      shift 2
      ;;
    *)
      EXTRA_ARGS+=("$1")
      shift
      ;;
  esac
done

# Use argument values or defaults
if [[ -n "$TESTS_DIR_ARG" ]]; then
  TESTS_DIR="$TESTS_DIR_ARG"
fi

if [[ -n "$CONFIG_FILE_ARG" ]]; then
  CONFIG_FILE="$CONFIG_FILE_ARG"
fi

if [[ -z "$THRESHOLD" ]]; then
  THRESHOLD="$DEFAULT_FAIL_THRESHOLD"
fi

# Validate tests directory
if [[ ! -d "$TESTS_DIR" ]]; then
  log_error "Tests directory '$TESTS_DIR' does not exist"
  exit 1
fi

# Main command execution
case $COMMAND in
  help)
    show_help
    exit 0
    ;;

  init)
    log_info "Initializing quality monitoring configuration..."

    # Create default configuration
    cat > "$CONFIG_FILE" << EOF
{
  "target_line_coverage": 95.0,
  "target_function_coverage": 98.0,
  "target_structure_compliance": 98.0,
  "target_naming_compliance": 98.0,
  "max_test_duration_ms": 100.0,
  "max_function_length": 50,
  "min_documentation_coverage": 85.0,
  "alert_thresholds": {
    "critical": 70.0,
    "warning": 85.0,
    "fail": 90.0
  },
  "monitoring": {
    "history_retention_days": 90,
    "trend_analysis_window": 30,
    "alert_cooldown_hours": 4
  }
}
EOF

    # Initialize history file
    echo "[]" > "$HISTORY_FILE"

    log_success "Quality monitoring initialized"
    log_info "Configuration: $CONFIG_FILE"
    log_info "History: $HISTORY_FILE"
    exit 0
    ;;

  assess)
    if [[ "$VERBOSE" == "true" ]]; then
      log_info "Running quality assessment..."
      log_info "Tests directory: $TESTS_DIR"
      log_info "Threshold: $THRESHOLD%"
      log_info "Format: $OUTPUT_FORMAT"
    fi

    # Build assessment command
    ASSESS_CMD="cargo run --bin assess_quality_cli --"
    ASSESS_CMD="$ASSESS_CMD --tests-dir '$TESTS_DIR'"
    ASSESS_CMD="$ASSESS_CMD --format '$OUTPUT_FORMAT'"
    ASSESS_CMD="$ASSESS_CMD --fail-threshold '$THRESHOLD'"

    if [[ -n "$OUTPUT_FILE" ]]; then
      ASSESS_CMD="$ASSESS_CMD --output '$OUTPUT_FILE'"
    fi

    if [[ "$VERBOSE" == "true" ]]; then
      ASSESS_CMD="$ASSESS_CMD --verbose"
    fi

    if [[ -f "$CONFIG_FILE" ]]; then
      ASSESS_CMD="$ASSESS_CMD --config '$CONFIG_FILE'"
    fi

    # Run assessment
    if eval "$ASSESS_CMD"; then
      ASSESSMENT_RESULT=0
      if [[ "$VERBOSE" == "true" ]]; then
        log_success "Quality assessment passed"
      fi
    else
      ASSESSMENT_RESULT=$?
      if [[ "$VERBOSE" == "true" ]]; then
        log_error "Quality assessment failed"
      fi
    fi

    # Update history if enabled
    if [[ "$UPDATE_HISTORY" == "true" ]]; then
      if [[ "$VERBOSE" == "true" ]]; then
        log_info "Updating quality history..."
      fi
      update_quality_history "$ASSESSMENT_RESULT"
    fi

    exit $ASSESSMENT_RESULT
    ;;

  trend)
    log_info "Analyzing quality trends..."

    if [[ ! -f "$HISTORY_FILE" ]]; then
      log_warning "No quality history found. Run 'quality_monitor.sh init' first."
      exit 1
    fi

    # Default to last 30 entries if not specified
    WINDOW_SIZE=30
    if [[ "${EXTRA_ARGS[0]}" =~ ^[0-9]+$ ]]; then
      WINDOW_SIZE="${EXTRA_ARGS[0]}"
    fi

    log_info "Showing last $WINDOW_SIZE quality assessments"

    # Generate trend analysis (simplified version)
    if command -v jq >/dev/null 2>&1; then
      RECENT_SCORES=$(jq -r ".[-$WINDOW_SIZE:] | .[] | .overall_score" "$HISTORY_FILE" 2>/dev/null | tail -n "$WINDOW_SIZE")

      if [[ -n "$RECENT_SCORES" ]]; then
        echo "Quality Score Trend:"
        echo "==================="

        SCORE_ARRAY=($RECENT_SCORES)
        for i in "${!SCORE_ARRAY[@]}"; do
          SCORE="${SCORE_ARRAY[$i]}"
          GRADE=""

          if (( $(echo "$SCORE >= 95" | bc -l 2>/dev/null || echo "0") )); then
            GRADE="🌟 Excellent"
          elif (( $(echo "$SCORE >= 85" | bc -l 2>/dev/null || echo "0") )); then
            GRADE="✅ Good"
          elif (( $(echo "$SCORE >= 70" | bc -l 2>/dev/null || echo "0") )); then
            GRADE="⚠️ Fair"
          elif (( $(echo "$SCORE >= 50" | bc -l 2>/dev/null || echo "0") )); then
            GRADE="❌ Poor"
          else
            GRADE="🚨 Critical"
          fi

          printf "%2d. %5.1f%% %s\n" $((i+1)) "$SCORE" "$GRADE"
        done

        # Calculate trend
        if [[ ${#SCORE_ARRAY[@]} -ge 2 ]]; then
          FIRST_SCORE="${SCORE_ARRAY[0]}"
          LAST_SCORE="${SCORE_ARRAY[-1]}"
          TREND_DIFF=$(echo "$LAST_SCORE - $FIRST_SCORE" | bc -l 2>/dev/null || echo "0")

          if (( $(echo "$TREND_DIFF > 1" | bc -l 2>/dev/null || echo "0") )); then
            log_success "Trend: Improving (+${TREND_DIFF}%)"
          elif (( $(echo "$TREND_DIFF < -1" | bc -l 2>/dev/null || echo "0") )); then
            log_warning "Trend: Declining (${TREND_DIFF}%)"
          else
            log_info "Trend: Stable"
          fi
        fi
      else
        log_warning "No trend data available"
      fi
    else
      log_warning "jq not available. Install jq for detailed trend analysis."
      cat "$HISTORY_FILE"
    fi
    ;;

  alert)
    log_info "Checking quality alerts..."

    # Run assessment and capture score
    TEMP_REPORT="/tmp/quality_report_$$.json"
    if cargo run --bin assess_quality_cli -- --tests-dir "$TESTS_DIR" --format json --output "$TEMP_REPORT" >/dev/null 2>&1; then
      CURRENT_SCORE=$(jq -r '.overall_score' "$TEMP_REPORT" 2>/dev/null || echo "0")
      rm -f "$TEMP_REPORT"

      # Check alert thresholds
      if (( $(echo "$CURRENT_SCORE < $DEFAULT_CRITICAL_THRESHOLD" | bc -l 2>/dev/null || echo "0") )); then
        ALERT_LEVEL="CRITICAL"
        ALERT_MESSAGE="🚨 CRITICAL: Quality score ($CURRENT_SCORE%) below critical threshold ($DEFAULT_CRITICAL_THRESHOLD%)"
        log_critical "$ALERT_MESSAGE"
      elif (( $(echo "$CURRENT_SCORE < $DEFAULT_WARN_THRESHOLD" | bc -l 2>/dev/null || echo "0") )); then
        ALERT_LEVEL="WARNING"
        ALERT_MESSAGE="⚠️ WARNING: Quality score ($CURRENT_SCORE%) below warning threshold ($DEFAULT_WARN_THRESHOLD%)"
        log_warning "$ALERT_MESSAGE"
      else
        ALERT_LEVEL="OK"
        ALERT_MESSAGE="✅ Quality score ($CURRENT_SCORE%) is healthy"
        log_success "$ALERT_MESSAGE"
      fi

      # Send webhook alert if configured
      if [[ -n "$ALERT_WEBHOOK" && "$ALERT_LEVEL" != "OK" ]]; then
        log_info "Sending alert webhook..."

        WEBHOOK_PAYLOAD=$(cat << EOF
{
  "text": "$ALERT_MESSAGE",
  "level": "$ALERT_LEVEL",
  "score": $CURRENT_SCORE,
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ)"
}
EOF
)

        if command -v curl >/dev/null 2>&1; then
          curl -s -X POST -H "Content-Type: application/json" -d "$WEBHOOK_PAYLOAD" "$ALERT_WEBHOOK" >/dev/null
          log_success "Alert webhook sent"
        else
          log_warning "curl not available. Cannot send webhook alert."
        fi
      fi
    else
      log_error "Failed to run quality assessment for alerts"
      exit 1
    fi
    ;;

  report)
    log_info "Generating comprehensive quality report..."

    REPORT_FILE="${OUTPUT_FILE:-$REPORTS_DIR/quality_report_$(date +%Y%m%d_%H%M%S).html}"

    if cargo run --bin assess_quality_cli -- --tests-dir "$TESTS_DIR" --format html --output "$REPORT_FILE" --verbose; then
      log_success "Comprehensive report generated: $REPORT_FILE"

      # Also generate JSON for history
      JSON_REPORT="${REPORT_FILE%.html}.json"
      cargo run --bin assess_quality_cli -- --tests-dir "$TESTS_DIR" --format json --output "$JSON_REPORT" >/dev/null 2>&1

      if [[ "$VERBOSE" == "true" ]]; then
        log_info "JSON report: $JSON_REPORT"
      fi
    else
      log_error "Failed to generate quality report"
      exit 1
    fi
    ;;

  dashboard)
    log_info "Starting quality dashboard..."

    PORT="${EXTRA_ARGS[0]:-8080}"
    DASHBOARD_DIR="$REPORTS_DIR/dashboard"

    mkdir -p "$DASHBOARD_DIR"

    # Generate latest report for dashboard
    cargo run --bin assess_quality_cli -- --tests-dir "$TESTS_DIR" --format html --output "$DASHBOARD_DIR/index.html" >/dev/null 2>&1

    log_success "Quality dashboard available at: http://localhost:$PORT"
    log_info "Press Ctrl+C to stop the dashboard"

    # Simple HTTP server (Python 3)
    if command -v python3 >/dev/null 2>&1; then
      cd "$DASHBOARD_DIR"
      python3 -m http.server "$PORT"
    elif command -v python >/dev/null 2>&1; then
      cd "$DASHBOARD_DIR"
      python -m http.server "$PORT"
    else
      log_error "Python not available. Cannot start dashboard server."
      log_info "Dashboard files available in: $DASHBOARD_DIR"
      exit 1
    fi
    ;;

  ci)
    log_info "Running CI/CD quality gate..."

    # Stricter defaults for CI
    CI_THRESHOLD="${THRESHOLD:-95.0}"

    # Run assessment with CI-specific settings
    if cargo run --bin assess_quality_cli -- --tests-dir "$TESTS_DIR" --format text --fail-threshold "$CI_THRESHOLD" --verbose; then
      log_success "✅ CI Quality Gate: PASSED"

      # Update history
      update_quality_history 0

      exit 0
    else
      log_error "❌ CI Quality Gate: FAILED"
      log_error "Quality score below CI threshold ($CI_THRESHOLD%)"

      # Update history
      update_quality_history 1

      # Generate failure report
      FAILURE_REPORT="$REPORTS_DIR/ci_failure_$(date +%Y%m%d_%H%M%S).html"
      cargo run --bin assess_quality_cli -- --tests-dir "$TESTS_DIR" --format html --output "$FAILURE_REPORT" >/dev/null 2>&1

      log_info "Failure report: $FAILURE_REPORT"

      exit 1
    fi
    ;;

  *)
    log_error "Unknown command: $COMMAND"
    log_info "Use 'quality_monitor.sh help' for usage information"
    exit 1
    ;;
esac

# Helper function to update quality history
update_quality_history() {
  local result_code=$1

  if [[ ! -f "$HISTORY_FILE" ]]; then
    echo "[]" > "$HISTORY_FILE"
  fi

  # Get latest assessment data
  TEMP_REPORT="/tmp/quality_history_$$.json"
  if cargo run --bin assess_quality_cli -- --tests-dir "$TESTS_DIR" --format json --output "$TEMP_REPORT" >/dev/null 2>&1; then
    # Add timestamp and result to the report
    if command -v jq >/dev/null 2>&1; then
      jq --arg timestamp "$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ)" --arg result_code "$result_code" \
         '. + {history_timestamp: $timestamp, result_code: ($result_code | tonumber)}' \
         "$TEMP_REPORT" > "${TEMP_REPORT}.tmp" && mv "${TEMP_REPORT}.tmp" "$TEMP_REPORT"

      # Append to history
      jq --slurpfile new_entry "$TEMP_REPORT" '. + $new_entry' "$HISTORY_FILE" > "${HISTORY_FILE}.tmp" && mv "${HISTORY_FILE}.tmp" "$HISTORY_FILE"

      # Keep only last 100 entries
      jq '.[-100:]' "$HISTORY_FILE" > "${HISTORY_FILE}.tmp" && mv "${HISTORY_FILE}.tmp" "$HISTORY_FILE"
    fi

    rm -f "$TEMP_REPORT"
  fi
}