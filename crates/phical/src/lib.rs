//! # phical — Phient's operators, learned and exact
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and
//! composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical
//! stores no point: a word is 32–64 relations, each a complex phasor on a
//! closed circle; meaning is a side effect of pairs of words moving under a
//! condition, and they move by the error of what was expected — toward what
//! came, away from what did not.
//!
//! phistd is the engine (tensors, tape, FFT, Adam — what torch is); phifno
//! is the sphere (the harmonics — what torch-harmonics is); phical is the
//! operators built on both — what neuraloperator is to torch — together with
//! the exact geometry of the relation circle that no learning touches.
//! Reference code read for the learned half: `gemphi/ext/neuraloperator`
//! (Li et al.), `torch-harmonics` and `makani` (Bonev et al.; FourCastNet 3).
//!
//! | module | what |
//! |---|---|
//! | [`operator`] (in phifno) | `Fno`, `FnoConfig`, `Tie` (circle, sphere, anisotropic), `Example`, `rank`, `zeta_weights`, `Fno::scale_modes`, `train_epoch_softmax`, versioned save with optimiser state, `Module`, `Sequential` |
//! | [`joint`] | `Joint`: the table of functions as a parameter on the operator's tape, with a proximal tether |
//! | [`relate`] | relation operators: one `Joint` for every kind (`Triple`, `example`, `pools`), the laws the kinds obey (`find_laws`), closure training, inheritance along a kind (`Inherit`), the reading and its bar (`score`, `prior`, `two_hop`, `chance`) |
//! | [`liskov`] | the three operators `better`, `worse`, `loss`, and `forcing_choice` — Liskov's 1968 endgame read for words; `liskov::staged`, her order on a stage and a measure |
//! | [`classes`] | word classes by exchange: a class bigram table's `likelihood`, one word's `move_gain` and `apply_move`, and `split_cost`, what one more class must pay for; `halve`, the exchange that grows a code until no split pays (Brown et al. 1992) |
//! | [`q64`] | the C64/Q64 juncture: quantise a phase to one of 64 symbols and lift it back, error ≤ π/64 (exact) |
//! | [`keyboard`] | the closed designed instruction set: the nine-type table with its fixed widths and little-endian layouts, one equality rule, and (as it is built) the 64 named Operators over phical's existing pieces |
//! | [`heat`] (in phifno) | Laplace–Beltrami heat flow on the circle (exact in Fourier modes) and band-limited on the sphere |

pub mod circle;
pub mod classes;
pub mod helix;
pub mod joint;
pub mod keyboard;
pub mod liskov;
pub mod q64;
pub mod relate;

// The Fourier things live in phifno (docs/173) — one address; these keep
// every `phical::operator` / `phical::heat` path reading as before.
pub use phifno::{heat, operator};

pub use joint::{IndexExample, Joint};
pub use operator::{rank, zeta_weights, Example, Fno, FnoConfig, FrequencySampler, MixtureExample, Module, Sequential, Tie};
// The engine's pieces the operators are written against, so `crate::` paths
// inside them and `phical::` paths outside read the same.
pub use phistd::{cosine, mix64, normalise, stack, unit_phase, unit_random, unstack, Adam, AdamState, Function, MixtureTerms, Tape, Tensor, Var};
pub use phistd::{phasor, tensor};
pub use num_complex::Complex64 as c64;

/// Number of relation channels, and the size of the Q64 alphabet.
pub const CHANNELS: usize = 64;
/// 2π.
pub const TAU: f64 = std::f64::consts::TAU;

/// Signed wrap of an angle difference into (−π, π]: phistd's, kept at
/// this path for every `phical::wrap` caller.
pub use phistd::wrap;
