# Cache Hierarchy

**A Rust library for simulating CPU cache hierarchies** — models L1, L2, and L3 caches with configurable size, associativity, and line size, tracking hit/miss statistics for memory access analysis.

## Why It Matters

The memory hierarchy is the most important performance characteristic of modern hardware. A cache hit to L1 takes ~1 ns; a miss that goes to main memory takes ~100 ns — a 100× difference. Understanding cache behavior is critical for:

- **Performance optimization** — data layout (AoS vs. SoA), cache-friendly algorithms, prefetching
- **Systems programming** — why certain patterns are fast (sequential access) vs. slow (pointer chasing)
- **Database design** — why B-trees (cache-friendly) outperform BSTs (pointer-heavy) on real hardware
- **Competitive programming** — cache-oblivious algorithms like the van Emde Boas layout

Modern CPUs have three cache levels:
- **L1**: 32–64 KB, 4-cycle latency, per-core
- **L2**: 256 KB – 1 MB, 10–12 cycle latency, per-core
- **L3**: 8–32 MB, 30–50 cycle latency, shared across cores

Cache associativity determines how many possible locations a cache line can occupy: direct-mapped (1 location), set-associative (N locations per set), or fully-associative (any location). Higher associativity reduces conflict misses but increases lookup time.

## How It Works

**Cache configuration**: Each level has a size (bytes), associativity (ways), and line size (typically 64 bytes). The number of sets is `size / (associativity × line_size)`. For a 32 KB, 8-way L1 with 64-byte lines: 4096 sets × 8 ways × 64 bytes = 2 MB.

**Address decomposition**: A memory address splits into:
- **Block offset** (bits 0–5): position within the 64-byte cache line
- **Set index** (next log₂(sets) bits): which set to check
- **Tag** (remaining bits): identifies the memory block

**Access simulation**: The `access()` function computes the set index for each cache level, checks for a hit (simplified pseudo-random hit pattern based on address hashing), and records statistics. Real hardware would use LRU replacement and actual tag arrays — this simulator uses a hash-based heuristic to model realistic hit rates.

**Statistics**: Per-level hit and miss counts, plus `hit_rate()` computing `hits / (hits + misses)`.

## Quick Start

```rust
use cache_hierarchy::{access, get_stats, reset_stats, CacheConfig};

// Use standard cache configurations
let configs = [
    CacheConfig::l1_dcache(),  // 32 KB, 8-way
    CacheConfig::l2(),         // 256 KB, 8-way
    CacheConfig::l3(),         // 8 MB, 16-way
];

reset_stats();

// Simulate memory accesses
for addr in (0..4096).step_by(64) { // sequential 4KB scan
    let level = access(addr, &configs);
    // level 0 = L1 hit, 1 = L2, 2 = L3, 3 = main memory
}

let stats = get_stats();
for (i, s) in stats.iter().enumerate() {
    println!("L{}: {} hits, {} misses ({:.1}% hit rate)",
        i + 1, s.hits, s.misses, s.hit_rate() * 100.0);
}
```

## API

- **`CacheConfig`** — Size, associativity, line size; presets for L1/L2/L3
- **`CacheStats`** — Hits, misses, `hit_rate()`
- **`access(addr, [L1, L2, L3])` → `usize`** — Simulate one access (0=L1, 1=L2, 2=L3, 3=miss)
- **`get_stats()` → `[CacheStats; 3]`** — Per-level statistics
- **`reset_stats()`** — Clear all counters

## Architecture Notes

Provides the microarchitecture simulation model for SuperInstance performance analysis. Used to estimate cache impact of data layout decisions before implementation. See the [architecture overview](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
