# Manual Tests

Manual testing plan for the `for_each` crate.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `readme.md` | Document manual testing procedures requiring human verification |

## Trigger

Run these manual checks when adding new macro invocation styles, changing token tree parsing, or before publishing.

## Steps

1. **No-std build:** Verify `cargo build --no-default-features --features no_std` compiles without error.
2. **Empty invocation error:** Confirm `for_each!()` produces a clear compile error (not a silent pass).
3. **Non-brace prefix syntax:** Verify `@Prefix ( ... )` and `@Prefix [ ... ]` expand correctly — these are not exercised by the automated suite.
4. **Unicode identifiers:** Check `for_each!(dbg, α, β)` expands to `dbg!(α); dbg!(β);` without ICE.
5. **Raw identifiers:** Confirm `for_each!(dbg, r#type, r#use)` expands correctly.
6. **Scalability:** Manually verify a 100+ element invocation compiles in reasonable time.
