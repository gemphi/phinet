//! K-means on the torus: offset vectors (one angle per channel) assigned by
//! [`agreement`], centroids updated by the per-channel circular mean.
//!
//! The seeds are chosen by farthest point on the agreement metric rather
//! than at random, so the result does not depend on a seed and two runs on
//! the same data give the same clusters — which a discovery procedure has
//! to, or what it finds is an artefact of initialisation.

use super::agreement::{agreement, centroid, mean_agreement};
use std::cmp::Ordering;

/// One cluster of [`kmeans`].
#[derive(Debug, Clone, PartialEq)]
pub struct Cluster {
    /// The shared offset, one angle per channel.
    pub centroid: Vec<f64>,
    /// The points assigned here, as ascending indices into the input.
    pub members: Vec<usize>,
    /// Mean agreement between a member and the centroid; 0 when empty.
    pub coherence: f64,
}

/// Clusters `points` into `k` groups over `channels` channels, in at most
/// `rounds` Lloyd iterations (fewer when an assignment pass moves nothing).
///
/// Seeds: the first point, then repeatedly the point least like every seed
/// chosen so far. Each round assigns every point to the centroid it agrees
/// with most (ties to the later), then sets each non-empty cluster's
/// centroid to its members' [`centroid`]. Empty when there are fewer points
/// than `k`, or `k` is 0.
pub fn kmeans<V: AsRef<[f64]>>(points: &[V], k: usize, rounds: usize, channels: usize) -> Vec<Cluster> {
    if points.len() < k || k == 0 {
        return Vec::new();
    }

    let mut seeds: Vec<usize> = vec![0];
    while seeds.len() < k {
        let next = (0..points.len())
            .filter(|i| !seeds.contains(i))
            .min_by(|a, b| {
                let sim = |i: &usize| {
                    seeds
                        .iter()
                        .map(|s| agreement(points[*i].as_ref(), points[*s].as_ref()))
                        .fold(f64::NEG_INFINITY, f64::max)
                };
                sim(a).partial_cmp(&sim(b)).unwrap_or(Ordering::Equal)
            });
        match next {
            Some(n) => seeds.push(n),
            None => break,
        }
    }

    let mut centroids: Vec<Vec<f64>> = seeds.iter().map(|s| points[*s].as_ref().to_vec()).collect();
    let mut assign: Vec<usize> = vec![0; points.len()];

    for _ in 0..rounds {
        let mut moved = false;
        for (i, off) in points.iter().enumerate() {
            let off = off.as_ref();
            let best = centroids
                .iter()
                .enumerate()
                .max_by(|a, b| {
                    agreement(off, a.1)
                        .partial_cmp(&agreement(off, b.1))
                        .unwrap_or(Ordering::Equal)
                })
                .map(|(c, _)| c)
                .unwrap_or(0);
            if assign[i] != best {
                assign[i] = best;
                moved = true;
            }
        }
        for (c, cen) in centroids.iter_mut().enumerate() {
            let members: Vec<&[f64]> = points
                .iter()
                .enumerate()
                .filter(|(i, _)| assign[*i] == c)
                .map(|(_, o)| o.as_ref())
                .collect();
            if !members.is_empty() {
                *cen = centroid(&members, channels);
            }
        }
        if !moved {
            break;
        }
    }

    centroids
        .into_iter()
        .enumerate()
        .map(|(c, centre)| {
            let members: Vec<usize> = (0..points.len()).filter(|i| assign[*i] == c).collect();
            let coherence = match members.is_empty() {
                true => 0.0,
                false => {
                    let offsets: Vec<&[f64]> = members.iter().map(|i| points[*i].as_ref()).collect();
                    mean_agreement(&offsets, &centre)
                }
            };
            Cluster { centroid: centre, members, coherence }
        })
        .collect()
}
