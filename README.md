# cache-hierarchy

A Rust library for **CPU cache hierarchy simulation**, modeling L1/L2/L3 access latency, hit rates, and set-associative addressing with configurable geometries for each cache level.

## Why It Matters

Memory hierarchy design is the single most impactful hardware-software co-design decision in modern computing. The latency gap between L1 cache (~1ns) and DRAM (~100ns) spans two orders of magnitude — the "memory wall." Understanding cache behavior is essential for:

- **Performance engineering** — cache-friendly data layouts (SoA vs AoS, tiling)
- **Systems programming** — false sharing, cache line alignment, prefetching
- **Algorithm design** — cache-oblivious algorithms, external memory models
- **Security** — cache-timing side channels (Flush+Reload, Prime+Probe)

## How It Works

### Cache Geometry

Each cache level is configured by three parameters:

- **Size** (S) — total cache capacity in bytes
- **Associativity** (A) — ways per set (direct-mapped=1, fully associative=S/line)
- **Line size** (B) — cache block size (typically 64 bytes)

The number of sets is:

$$N_{\text{sets}} = \frac{S}{A \times B}$$

### Address Decomposition

A memory address is decomposed into three fields:

```
┌──────────────┬──────────┬──────────┐
│     Tag      │  Set Index│  Offset  │
│  log₂(tag)   │ log₂(sets)│ log₂(B)  │
└──────────────┴──────────┴──────────┘
```

- **Offset**: `log₂(B)` bits → selects byte within line
- **Set index**: `log₂(N_sets)` bits → selects cache set
- **Tag**: remaining upper bits → disambiguates lines within a set

### Default Configurations

| Level | Size | Assoc. | Sets (at 64B lines) |
|-------|------|--------|---------------------|
| L1 (data) | 32 KB | 8-way | 64 |
| L2 | 256 KB | 8-way | 512 |
| L3 | 8 MB | 16-way | 8192 |

These match common Intel/AMD desktop configurations (e.g., Skylake, Zen 4).

### Access Model

The `access()` function walks the hierarchy: L1 → L2 → L3 → memory. It returns the hitting level (0–3). Statistics are accumulated per level.

### Average Memory Access Time (AMAT)

The standard AMAT model for a three-level hierarchy:

$$\text{AMAT} = t_{L1} + m_{L1} \cdot (t_{L2} + m_{L2} \cdot (t_{L3} + m_{L3} \cdot t_{\text{mem}}))$$

where $t_i$ is the hit latency and $m_i$ is the miss rate at level $i$.

### Big-O Complexity

| Operation | Time | Space |
|-----------|------|-------|
| `access(addr)` | O(L) where L = levels (constant 3) | O(1) |
| `get_stats()` | O(L) | O(L) |
| `reset_stats()` | O(L) | O(1) |

## Quick Start

```rust
use cache_hierarchy::{access, get_stats, reset_stats, CacheConfig};

reset_stats();
let configs = [CacheConfig::l1_dcache(), CacheConfig::l2(), CacheConfig::l3()];
let hitting_level = access(0xDEAD_BEEF, &configs);
// hitting_level: 0=L1, 1=L2, 2=L3, 3=memory

let stats = get_stats();
for (i, s) in stats.iter().enumerate() {
    println!("L{}: {} hits, {} misses, {:.1}% hit rate",
        i + 1, s.hits, s.misses, s.hit_rate() * 100.0);
}
```

## API

| Function / Type | Description |
|-----------------|-------------|
| `CacheConfig::l1_icache()` / `l1_dcache()` / `l2()` / `l3()` | Preset geometries |
| `CacheConfig::num_sets() → usize` | Compute set count |
| `access(addr, &[CacheConfig; 3]) → usize` | Simulate hierarchy access |
| `get_stats() → [CacheStats; 3]` | Per-level hit/miss counters |
| `reset_stats()` | Zero all counters |
| `CacheStats::hit_rate() → f64` | Hit rate fraction |

## Architecture Notes

The **γ + η = C** link: the address decomposition (γ) maps physical addresses to cache set/tag locations, while the associativity constraint (η) bounds the number of candidates per set. Together they conserve the invariant C — every address maps to exactly one set per level, and the replacement policy determines which tag occupies a way within that set. The global statistics accumulate the observable consequence of this mapping.

## References

- Hennessy, J. L., & Patterson, D. A. (2019). *Computer Architecture: A Quantitative Approach,* 6th ed. Chapter 2: Memory Hierarchy Design.
- Smith, A. J. (1982). *Cache Memories.* ACM Computing Surveys, 14(3), 473–530.
- Aggarwal, A., Alpern, B., Chandra, A., & Snir, M. (1987). *A Model for Heuristic Analysis of Parallel Algorithms.* (I/O complexity model.)
- Yarom, Y., & Falkner, K. (2014). *FLUSH+RELOAD: A High Resolution, Low Noise, L3 Cache Side-Channel Attack.* USENIX Security.
- Intel® 64 and IA-32 Architectures Optimization Reference Manual. Chapter 2: Cache Architecture.

## License

MIT
