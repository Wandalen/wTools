# Test Spec: Memory Efficiency

- **Source**: `docs/invariant/002_memory_efficiency.md`
- **Prefix**: `IN-02`
- **Min cases**: 2

## Cases

| ID | Name | Status |
|----|------|--------|
| IN-02-1 | heap_allocation_within_ceiling_for_typical_workload | ⏳ |
| IN-02-2 | memory_profiler_measures_peak_allocation | ⏳ |

[PENDING — memory profiling infrastructure not yet in place]

---

### IN-02-1: heap_allocation_within_ceiling_for_typical_workload

- **Given:** A template operation processing up to 100 files with 1MB of total template content using `MemoryFileSystem`
- **When:** Peak heap allocation is measured via memory profiler
- **Then:** Peak heap allocation is ≤ 10MB

[PENDING — requires memory profiling setup — see task for benchmark infrastructure]

---

### IN-02-2: memory_profiler_measures_peak_allocation

- **Given:** The memory profiling harness is in place
- **When:** The in-memory filesystem test workload is run under the profiler
- **Then:** A peak heap allocation value is recorded for comparison against the 10MB ceiling
