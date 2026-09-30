//! Cache hierarchy simulator — models L1/L2/L3 cache behaviour.

use std::sync::Mutex;

const CACHE_LINE_SIZE: usize = 64;

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub size: usize,
    pub associativity: usize,
    pub line_size: usize,
}

impl CacheConfig {
    pub fn l1_icache() -> Self {
        Self { size: 32 * 1024, associativity: 8, line_size: CACHE_LINE_SIZE }
    }
    pub fn l1_dcache() -> Self {
        Self { size: 32 * 1024, associativity: 8, line_size: CACHE_LINE_SIZE }
    }
    pub fn l2() -> Self {
        Self { size: 256 * 1024, associativity: 8, line_size: CACHE_LINE_SIZE }
    }
    pub fn l3() -> Self {
        Self { size: 8 * 1024 * 1024, associativity: 16, line_size: CACHE_LINE_SIZE }
    }

    pub fn num_sets(&self) -> usize {
        self.size / (self.associativity * self.line_size)
    }
}

#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 { 0.0 } else { self.hits as f64 / total as f64 }
    }
}

lazy_static::lazy_static! {
    static ref STATS: Mutex<[CacheStats; 3]> = Mutex::new([
        CacheStats::default(),
        CacheStats::default(),
        CacheStats::default(),
    ]);
}

/// Simulate an access at `addr` through the cache hierarchy.
/// Returns the cache level that hit (0 = L1, 1 = L2, 2 = L3, 3 = miss).
pub fn access(addr: usize, configs: &[CacheConfig; 3]) -> usize {
    let mut stats = STATS.lock().unwrap();
    for (i, cfg) in configs.iter().enumerate() {
        let set = (addr / cfg.line_size) % cfg.num_sets();
        let _tag = addr / (cfg.num_sets() * cfg.line_size);
        // Simplified: random hit simulation based on address pattern
        let hit = (addr.wrapping_mul(0x9E3779B97F4A7C15) >> (60 - i * 4)) & 1 == 0;
        if hit {
            stats[i].hits += 1;
            return i;
        }
        stats[i].misses += 1;
    }
    3
}

/// Get current cache statistics for L1/L2/L3.
pub fn get_stats() -> [CacheStats; 3] {
    STATS.lock().unwrap().clone()
}

/// Reset all statistics.
pub fn reset_stats() {
    let mut stats = STATS.lock().unwrap();
    for s in stats.iter_mut() {
        *s = CacheStats::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access() {
        reset_stats();
        let cfgs = [CacheConfig::l1_dcache(), CacheConfig::l2(), CacheConfig::l3()];
        access(0x1000, &cfgs);
        let s = get_stats();
        let total: u64 = s.iter().map(|x| x.hits + x.misses).sum();
        assert!(total > 0);
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
