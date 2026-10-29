//! Agreement between offset vectors — one angle per channel, the step from
//! a head to its filler — as the mean cosine of their differences: 1 when
//! two pairs stand in the same relation, 0 when they are unrelated. With it:
//! the nearest of a set of centroids, the per-channel circular mean of a
//! group, and the group's mean agreement to a centre.

use super::resultant::Resultant;
use std::cmp::Ordering;

/// Mean cosine between two offset vectors: Σ cos(aₖ − bₖ) over the pairs
/// `zip` gives, divided by `a`'s length (at least 1).
pub fn agreement(a: &[f64], b: &[f64]) -> f64 {
    let mut s = 0.0;
    for (x, y) in a.iter().zip(b.iter()) {
        s += (x - y).cos();
    }
    s / a.len().max(1) as f64
}

/// The centroid an offset agrees with most, and that agreement; `None` when
/// there are no centroids. An unordered comparison (a NaN) counts as equal,
/// and ties go to the later centroid, as `max_by` gives them.
pub fn nearest<V: AsRef<[f64]>>(offset: &[f64], centroids: &[V]) -> Option<(usize, f64)> {
    centroids
        .iter()
        .enumerate()
        .map(|(i, c)| (i, agreement(offset, c.as_ref())))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
}

/// Per-channel circular mean of offset vectors over `channels` channels:
/// the direction of Σ (cos, sin) on each, 0 where the channel cancelled.
pub fn centroid<V: AsRef<[f64]>>(offsets: &[V], channels: usize) -> Vec<f64> {
    (0..channels)
        .map(|k| {
            let mut r = Resultant::new();
            for o in offsets {
                r.add_unit(o.as_ref()[k]);
            }
            r.direction_or_none().unwrap_or(0.0)
        })
        .collect()
}

/// Mean agreement of offsets to a centre: Σ [`agreement`] (by `sum`) over
/// the count. The caller guards the empty group.
pub fn mean_agreement<V: AsRef<[f64]>>(offsets: &[V], centre: &[f64]) -> f64 {
    offsets.iter().map(|o| agreement(o.as_ref(), centre)).sum::<f64>() / offsets.len() as f64
}

/// Mean, over offsets, of the agreement with each one's [`nearest`]
/// centroid, counting only finite agreements; 0 when none counts. This is
/// held-out agreement when the centroids were fitted elsewhere.
pub fn mean_nearest<V: AsRef<[f64]>, W: AsRef<[f64]>>(offsets: &[V], centroids: &[W]) -> f64 {
    let mut sum = 0.0f64;
    let mut n = 0usize;
    for o in offsets {
        if let Some((_, a)) = nearest(o.as_ref(), centroids) {
            if a.is_finite() {
                sum += a;
                n += 1;
            }
        }
    }
    match n {
        0 => 0.0,
        _ => sum / n as f64,
    }
}

/// Similarity of two shapes wherever they sit: each set of offsets sorted,
/// then the mean cosine of the paired differences over the shorter, clamped
/// to [−1, 1]; 0 when either set is empty.
pub fn shape_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let (mut x, mut y) = (a.to_vec(), b.to_vec());
    x.sort_by(|p, q| p.partial_cmp(q).unwrap_or(Ordering::Equal));
    y.sort_by(|p, q| p.partial_cmp(q).unwrap_or(Ordering::Equal));

    let n = x.len().min(y.len());
    let total: f64 = (0..n).map(|i| (x[i] - y[i]).cos()).sum();
    (total / n as f64).clamp(-1.0, 1.0)
}
