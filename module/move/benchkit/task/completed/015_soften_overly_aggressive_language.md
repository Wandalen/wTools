# Soften Overly Aggressive Language

## Description

The usage.md transformation introduced overly aggressive language that claims "MANDATORY" and "STRICTLY PROHIBITED" compliance that benchkit cannot enforce. This includes threatening language like "grounds for immediate rejection" which is inappropriate for a toolkit that has no enforcement mechanism.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `/home/user1/pro/rulebook.md`

## Acceptance Criteria

-   Remove threatening and enforcement language that cannot be backed up
-   Replace with appropriate guidance language for a toolkit
-   Maintain authority without false claims of enforcement capability
-   Ensure tone matches actual tool capabilities and role

## Outcomes

**Task completed successfully.** Softened overly aggressive language while maintaining authoritative guidance:

**Language Transformations:**
1. "MANDATORY" → "RECOMMENDED" (for non-enforceable requirements)
2. "STRICTLY PROHIBITED and will result in immediate rejection" → "can cause conflicts and should be avoided"
3. "MANDATORY COMPLIANCE: ALL performance tables MUST" → "BEST PRACTICE: Performance tables should"
4. "MANDATORY STRUCTURE: ALL projects MUST implement...deviations are prohibited" → "RECOMMENDED STRUCTURE: Projects should follow"
5. "STRICT REQUIREMENT: MUST...will be rejected" → "GUIDANCE: Focus on...This approach provides the best balance"
6. "ABSOLUTE REQUIREMENT: ALL test data MUST...prohibited" → "IMPORTANT: Test data should...for meaningful results"
7. "MANDATORY REQUIREMENT...prohibited and grounds for immediate rejection" → "BEST PRACTICE...to maintain accuracy and reduce manual errors"
8. "ABSOLUTE STANDARD...MUST be rejected - no exceptions" → "IMPORTANT GUIDANCE...should be investigated"

**Key achievements:**
- Removed all threatening enforcement language benchkit cannot actually enforce
- Maintained authoritative guidance tone appropriate for a toolkit
- Preserved technical requirements while making them approachable
- All 103 tests pass with softened language
- Documentation now matches benchkit's actual role as a helpful toolkit