<p align="center"><img src="web/public/logo.svg" width="96" alt="Phistd" /></p>

# phistd

The engine of Phient's PML family, `phinet::std`: what torch is to neuraloperator. A Fourier tensor engine: complex tensors `[batch, width, space]`, a reverse-mode tape with a differentiable unitary FFT, Adam, the sixty-four gates and the phasor ranges. Rust, no unsafe; `num-complex` for the numbers, `rayon` for the table-wide scoring ops, `serde` for saving Adam's state.

phistd knows no words and holds no operator. The workspace lays out one crate per use (copies of the reference libraries read under `gemphi/ext`):

| crate | what | answers |
|---|---|---|
| `phistd` | the engine | torch |
| `phifno` | the sphere and the Fourier operators: harmonics, grids, `Fno`, heat | torch-harmonics |
| `phical` | the operators built on both: `Mirror`, `Joint`, relation operators, Liskov's three, Q64 | neuraloperator, makani |
| `phiecs` | words as entities: the world, the phasor, the quest; the kit, with the eye (`kit::eye`, active vision told as rows) | phi-oml |
| `phient` | the brain: the arena of models, the study protocol | context-language-models |
| `phiclm` | the compiler: BCPL's architecture and the judge | BCPL |
| `phiton` | quantum light: the unit system under α ≈ 1/137, holographic binding, the physics of colour, space and time, P64 | — |
| `phiano` | the agents, apps and binaries | — |

A *function* is a `Vec<Complex64>`: samples of a complex function on a circle at a power-of-two number of points. The operators between such functions are in `phifno` and `phical`; the example below is the engine alone.

```toml
[dependencies]
phistd = { path = "../phistd" }
```

```rust
use phistd::{cosine, fft_inplace, unit_random, Tape, Tensor};

let f = unit_random(64, 7);
let mut g = f.clone();
fft_inplace(&mut g, false);                       // unitary: energy is kept
assert!((g.iter().map(|z| z.norm_sqr()).sum::<f64>() - 1.0).abs() < 1e-9);
assert!((cosine(&f, &f) - 1.0).abs() < 1e-12);
let mut tape = Tape::new();
let x = tape.constant(Tensor::from_vec([1, 1, 64], f.clone()));
let y = tape.fft(x);                              // differentiable on the tape
assert_eq!(tape.value(y).shape, [1, 1, 64]);
```

The operator that learns a circular shift (what the identity cannot do and a spectral filter can) is `phical::Fno`; its README has that example.

## What is in it

| module | what |
|---|---|
| `tensor` | `Tensor` of shape `[batch, width, space]`; `Tape` with `add`, `add_bias`, `mul`, `scale`, `linear`, `mode_mix`, `fft`/`ifft`, `fft2`/`ifft2` (unitary radix-2), `gelu`, `gather`, `unit`, `cosine`, `cosine_table`, `hinge`, `cross_entropy`, `mixture_nll`, `mobius`, `tie`, `mean_real`, and `backward`; `Adam` (AdamW) on `(Re, Im)` pairs with a `Schedule`. |
| `phasor` | the phasor ranges: `Resolution` `P16 P32 P64 P128` × `Quantum` `Q16 Q32 Q64`, twelve `Range`s (`Range::ALL`), `resample`, `quantise`, `sector_of`. |
| `hexagram` | the sixty-four operations: I Ching hexagrams as spectral gates (`apply`, `gate`, `changing_lines`), the hexagram a function is (`hexagram_of`, `lines`), `novelty`; the figure (`Hexagram`, `Trigram`) and its name (`archetype_name`). |
| `codon` | the sixty-four as three bases: `codon`, `codon_index`, `hexagram_glyph`; where each sits on the B-DNA helix (`helix_position`, `PITCH_PER_TURN`, `RISE`). |
| the crate root | `c64`, `Function`, the one seed and hash (`fnv1a`, `mix64`, `seed_phase`, `seed_angles`, `GOLDEN_ANGLE`), the angle wrapped to (−π, π] (`wrap`), and function helpers (`unit_random`, `unit_phase`, `normalise`, `stack`, `unstack`, `cosine`). |

## Conventions

Gradients of a real loss with respect to a complex value are `g = ∂L/∂Re(z) + i·∂L/∂Im(z)`. Under it `y = x·w` gives `g_x = g_y·conj(w)`, a unitary `y = U x` gives `g_x = U⁻¹ g_y`, and a real non-linearity applied to `Re` and `Im` separately back-propagates through each part on its own. The FFT is scaled by `1/√N` both ways, so it is unitary and its adjoint is its inverse.

## Where it came from

Built for [phiano](../phiano), where a word is 64 complex numbers on a circle of relations. `phiano/src/phinet/` is the application layer (a lexicon as a function space, the operator inside a perplexity harness, joint training with write-back, recursive composition with the operator as cleanup); the engine has no notion of words. See `phiano/docs/65_phinet.md` and `66_phinet_in_the_loop.md` for what it measured.

## Tests

```
cargo test --release -p phistd
```

26 test functions and the doc example: FFT and 2-D FFT unitarity and a pure tone landing in one bin; finite-difference checks of the tape's gradients (the general ops, `mobius`, `cross_entropy`, `mixture_nll`); Adam lowering a loss, AdamW decay and its saved state, the schedules; the twelve ranges, snapping within its bound, resampling up then down; the hexagram gates, figures and trigrams, the codons and the helix.

## Phasor ranges

`phistd::phasor`: resolutions `P16 P32 P64 P128` × quanta `Q16 Q32 Q64`, twelve `Range`s every computation runs on (`resample`, `quantise`, `sector_of`). phifno's `Fno::at_resolution` moves an operator between them, and phical's `ranges` example measures all of them:

```
cargo run --release -p phical --example ranges
```

## Examples

`profile` (per-op forward and backward timing at the operator's working sizes) and `table` (one batch against a 6,221-row table).

## PyTorch parity

`torch/phinet_torch.py` implements the tape's operations in PyTorch, with the same shapes and conventions (unitary FFT, tanh-GELU on Re and Im, conjugate-Wirtinger gradients). phical's `torch` example writes the reference dump; the Python side replays it with `torch.autograd`. From `crates/phistd`:

```
cargo run --release -p phical --example torch > torch/reference.json
python torch/phinet_torch.py torch/reference.json
```

`torch/phinet_colab.ipynb` runs the checks in Colab.

## Documentation

The site (install, tutorials, the guide, the API reference, the PyTorch twin, measurements) is in `docs/` and served by `web/`, a [PhiDoc](../../../phidoc) site on puijs:

```
cd web && npm install && npm run dev     # http://127.0.0.1:5183
npm run sync-hub                          # mirror docs/ into the PhiDoc hub
```

The architecture is drawn in [ARCHITECTURE.md](ARCHITECTURE.md); what is open is in [TODO.md](TODO.md).
