# Source Files

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `lib.rs` | Crate entry point with feature-conditional exports |
| `hrng_deterministic.rs` | Deterministic hierarchical RNG using ChaCha8Rng |
| `hrng_non_deterministic.rs` | Non-deterministic hierarchical RNG using ThreadRng |
| `seed.rs` | Master seed type with creation utilities |
| `iter.rs` | Iterator extensions for deterministic sorting |
