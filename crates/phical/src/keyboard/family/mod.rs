//! The integration point where the six family tables meet the keyboard.
//!
//! Each family — Circle, Q64, Helix, Classes, Liskov, Relate — lands in its
//! own single-word file beside this one (`circle.rs`, `q64.rs`, …) as tasks
//! 3.4 to 3.9, each lifting its existing `phical` functions to the declared
//! ABI. [`all`] concatenates them in [`Family::ALL`](super::operator::Family)
//! order so [`table`](super::operator::table) can assert the index bijection
//! and the family partition over them.
//!
//! **State of this file.** All six families have landed, so [`all`] assembles
//! the whole 64: Circle (0–13), Q64 (14–21), Helix (22–27), Classes (28–39),
//! Liskov (40–47) and Relate (48–63), each a contiguous block in
//! [`Family::ALL`](super::operator::Family) order. `circle.rs` owns the
//! identity as its first row. [`all`] concatenates the six family tables so
//! [`table`](super::operator::table) can assert the index bijection and the
//! family partition over the complete set.

use super::operator::Operator;

pub mod circle;
pub mod q64;
pub mod helix;
pub mod classes;
pub mod liskov;
pub mod relate;

/// The whole 64-Operator keyboard, in index order. Each family appends a
/// contiguous block, the six running 0..64 with no gap: Circle 0–13, Q64
/// 14–21, Helix 22–27, Classes 28–39, Liskov 40–47, Relate 48–63.
pub fn all() -> Vec<Operator> {
    let mut ops: Vec<Operator> = Vec::new();

    // Family::Circle leads the table — fourteen rows, indices 0 to 13, the
    // identity its first.
    ops.extend(circle::table());
    ops.extend(q64::table());
    ops.extend(helix::table());
    ops.extend(classes::table());
    ops.extend(liskov::table()); // indices 40–47
    ops.extend(relate::table()); // indices 48–63

    ops
}

/// The identity Operator, re-exported from `circle.rs` where it is declared
/// as Circle's first row. `operator::identity()` calls through here, so the
/// one Operator declared the identity has a single home.
pub fn identity() -> Operator {
    circle::identity()
}
