//! Lock-free sample taps for monitoring modules (audio thread → GUI).

use std::sync::atomic::{AtomicU32, Ordering};

/// Samples retained per tap: 16 internal blocks of 64 frames.
pub const SCOPE_CAPACITY: usize = 1024;

/// Fixed-size ring of the newest `f32` samples.
///
/// The owning processor pushes from the audio thread; the GUI only reads
/// snapshots. Both sides are lock-free and allocation-free.
pub struct ScopeTap {
    samples: [AtomicU32; SCOPE_CAPACITY],
    write: AtomicU32,
}

impl ScopeTap {
    /// Empty tap.
    #[must_use]
    pub fn new() -> Self {
        Self {
            samples: std::array::from_fn(|_| AtomicU32::new(0)),
            write: AtomicU32::new(0),
        }
    }

    /// Audio thread: append one sample. Single writer, no allocation or blocking.
    pub fn push(&self, value: f32) {
        let cursor = self.write.load(Ordering::Relaxed);
        let slot = cursor as usize % SCOPE_CAPACITY;
        self.samples[slot].store(value.to_bits(), Ordering::Relaxed);
        self.write.store(cursor.wrapping_add(1), Ordering::Release);
    }

    /// GUI: copy the newest samples, oldest first. Returns how many were filled.
    ///
    /// Writes at most `out.len()` and leaves the rest of `out` untouched.
    pub fn snapshot(&self, out: &mut [f32]) -> usize {
        let end = self.write.load(Ordering::Acquire);
        let available = (end as usize).min(SCOPE_CAPACITY);
        let take = out.len().min(available);
        let start = end.wrapping_sub(take as u32);
        for (offset, slot) in out[..take].iter_mut().enumerate() {
            let index = start.wrapping_add(offset as u32) as usize % SCOPE_CAPACITY;
            *slot = f32::from_bits(self.samples[index].load(Ordering::Relaxed));
        }
        take
    }
}

impl Default for ScopeTap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{SCOPE_CAPACITY, ScopeTap};

    #[test]
    fn snapshot_returns_newest_samples_oldest_first() {
        let tap = ScopeTap::new();
        let mut out = [f32::NAN; 4];
        assert_eq!(tap.snapshot(&mut out), 0);
        assert!(out.iter().all(|sample| sample.is_nan()));

        for value in [1.0, 2.0, 3.0] {
            tap.push(value);
        }
        assert_eq!(tap.snapshot(&mut out), 3);
        assert_eq!(out[..3], [1.0, 2.0, 3.0]);
        assert!(out[3].is_nan());
    }

    #[test]
    fn snapshot_keeps_ring_order_across_wrap() {
        let tap = ScopeTap::new();
        for step in 0..(SCOPE_CAPACITY as u32 + 37) {
            tap.push(step as f32);
        }
        let mut out = [0.0f32; SCOPE_CAPACITY];
        assert_eq!(tap.snapshot(&mut out), SCOPE_CAPACITY);
        let first = SCOPE_CAPACITY as f32 + 37.0 - SCOPE_CAPACITY as f32;
        assert_eq!(out[0], first);
        assert_eq!(out[SCOPE_CAPACITY - 1], (SCOPE_CAPACITY + 36) as f32);
    }

    #[test]
    fn nan_bits_survive_the_roundtrip() {
        let tap = ScopeTap::new();
        tap.push(f32::NAN);
        let mut out = [0.0f32; 1];
        assert_eq!(tap.snapshot(&mut out), 1);
        assert!(out[0].is_nan());
    }
}
