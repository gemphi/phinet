//! Draws: a 64-bit word read as a uniform in [0, 1), the two generators the
//! workspace uses, and a draw by integer weights. phient wrote each of these
//! inline, in several places (the ruling of 2026-10-28: the calculations
//! live below phient); they are written here exactly as they were.

/// A 64-bit word as a uniform in [0, 1): its top 53 bits.
#[inline]
pub fn unit_interval(x: u64) -> f64 {
    (x >> 11) as f64 / (1u64 << 53) as f64
}

/// A 64-bit word as an f32 uniform in [0, 1): its top 24 bits.
#[inline]
pub fn unit_interval_f32(x: u64) -> f32 {
    (x >> 40) as f32 / (1u64 << 24) as f32
}

/// One step of Knuth's MMIX linear congruential generator; the new state.
#[inline]
pub fn lcg(state: &mut u64) -> u64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *state
}

/// One step of xorshift64 (shifts 13, 7, 17).
#[inline]
pub fn xorshift64(x: u64) -> u64 {
    let mut x = x;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

/// One step of xorshift64* (shifts 12, 25, 27): the next state, and the
/// output word (the state times the multiplier).
#[inline]
pub fn xorshift64_star(x: u64) -> (u64, u64) {
    let mut x = x;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    (x, x.wrapping_mul(0x2545F4914F6CDD1D))
}

/// An index drawn by integer weights, from a uniform `u` in [0, 1): the pick
/// `u · total` walks the weights, subtracting each until it falls inside
/// one. Past the end (rounding), the last index.
pub fn draw_by_weight(weights: &[u32], u: f64) -> usize {
    let total: u32 = weights.iter().sum();
    let mut pick = (u * total as f64) as u32;
    for (i, w) in weights.iter().enumerate() {
        if pick < *w {
            return i;
        }
        pick -= w;
    }
    weights.len().saturating_sub(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_uniforms_stay_in_the_unit_interval() {
        for x in [0u64, 1, u64::MAX, 0x8000_0000_0000_0000] {
            assert!((0.0..1.0).contains(&unit_interval(x)));
            assert!((0.0..1.0).contains(&unit_interval_f32(x)));
        }
    }

    #[test]
    fn a_draw_lands_where_its_weight_is() {
        let w = [1, 0, 3];
        assert_eq!(draw_by_weight(&w, 0.0), 0);
        assert_eq!(draw_by_weight(&w, 0.3), 2);
        assert_eq!(draw_by_weight(&w, 0.999), 2);
        assert_ne!(draw_by_weight(&w, 0.5), 1, "a zero weight is never drawn");
    }

    #[test]
    fn the_generators_move() {
        let mut s = 7;
        assert_ne!(lcg(&mut s), 7);
        assert_ne!(xorshift64(7), 7);
        assert_ne!(xorshift64_star(7).0, 7);
    }
}
