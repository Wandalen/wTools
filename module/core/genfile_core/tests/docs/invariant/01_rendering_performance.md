# Test Spec: Rendering Performance

- **Source**: `docs/invariant/001_rendering_performance.md`
- **Prefix**: `IN-01`
- **Min cases**: 2

## Cases

| ID | Name | Status |
|----|------|--------|
| IN-01-1 | rendering_completes_within_latency_bound | ⏳ |
| IN-01-2 | benchmark_suite_measures_median_latency | ⏳ |

[PENDING — benchmark infrastructure not yet in place]

---

### IN-01-1: rendering_completes_within_latency_bound

- **Given:** A 10KB template with 50 parameter substitutions rendered by `HandlebarsRenderer` on 2020+ hardware
- **When:** `cargo criterion` benchmark suite is run and median latency is recorded
- **Then:** Median rendering latency is ≤ 100ms

[PENDING — requires `benches/` setup and `cargo criterion` — see task for benchmark infrastructure]

---

### IN-01-2: benchmark_suite_measures_median_latency

- **Given:** The benchmark suite exists in `benches/` and can be run with `cargo criterion`
- **When:** The rendering benchmark is executed
- **Then:** A median latency value is reported for comparison against the 100ms threshold
