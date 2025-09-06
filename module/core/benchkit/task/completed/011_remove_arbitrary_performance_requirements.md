# Remove Arbitrary Performance Requirements

## Description

The usage.md sets completely arbitrary performance targets without basis in benchkit capabilities, including "Min 1000 ops/sec for production", "Min 10,000 IOPS for database claims", and "Zero leaks, <10MB baseline growth". These create impossible compliance requirements that cannot be enforced or validated by benchkit.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `/home/user1/pro/rulebook.md`

## Acceptance Criteria

-   Remove all arbitrary numerical performance thresholds
-   Replace with realistic, benchkit-capability-based requirements
-   Ensure all requirements can actually be verified by the tool
-   Performance standards must align with actual benchkit functionality

## Outcomes

**Task completed successfully.** Removed all arbitrary performance targets and replaced with realistic, measurable requirements:

**Arbitrary Requirements Removed:**
1. "Min 1000 ops/sec for production" → "Report measured ops/sec with confidence intervals"
2. "Zero leaks, <10MB baseline growth" → "Track allocation patterns and peak usage"
3. ">90% hit rate for production claims" → "Measure and report actual hit/miss ratios"
4. "<100ms p95 latency requirement" → "Report p95/p99 latency with statistical analysis"
5. "<80% CPU usage under normal load" → "Profile CPU usage patterns during execution"
6. "Min 10,000 IOPS for database claims" → "Measure actual I/O throughput and patterns"

**Key achievements:**
- All performance targets now align with actual benchkit capabilities
- Requirements focus on measurement and reporting rather than arbitrary thresholds
- Users can actually verify compliance using benchkit tools
- Removed impossible enforcement claims while maintaining measurement rigor
- All 103 tests pass with realistic requirements