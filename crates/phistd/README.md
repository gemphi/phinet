<p align="center"><img src="web/public/logo.svg" width="96" alt="Phinet" /></p>

# phinet

The engine of Phient's PML family — what torch is to neuraloperator. A Fourier tensor engine: complex tensors `[batch, width, space]`, a reverse-mode tape with a differentiable unitary FFT, Adam, holographic bind/unbind, the sixty-four gates and the phasor ranges. Rust, one dependency (`num-complex`, plus serde/bincode for saving), no unsafe, every gradient finite-difference checked.

phinet knows no words and holds no operator. The family lays out one crate per reference library (copies read under `gemphi/ext`):

| crate | what | answers |
|---|---|---|
| `phinet` | the engine | torch |
| `phifno` | the sphere: real spherical harmonics, grids, analysis/synthesis | torch-harmonics |
| `phical` | the operators: `Fno` (circle, sphere, anisotropic ties), `Joint`, relation operators, Liskov's three; the exact geometry (Möbius, Q64, winding, heat) | neuraloperator, makani |
| `phiplm` | the Phasor Language Model: counts, levels, the cast, the field, kin, the relation model; prose and code as trees | context-language-models |
| `phiecs` | the entity-component world; the quest | — |
| `phiano` | the applications and binaries | — |

A *function* is a `Vec<Complex64>`: samples of a complex function on a circle at a power-of-two number of points. The operators between such functions are in `phical`; the example below is the engine alone.

```toml
[dependencies]
phinet = { path = "../phinet" }
```

```rust
use phinet::{cosine, fft_inplace, unit_random, Tape, Tensor};

let f = unit_random(64, 7);
let mut g = f.clone();
fft_inplace(&mut g, false);                       // unitary: energy is kept
assert!((g.iter().map(|z| z.norm_sqr()).sum::<f64>() - 1.0).abs() < 1e-9);
let mut tape = Tape::new();
let x = tape.constant(Tensor::from_vec([1, 1, 64], f.clone()));
let y = tape.fft(x);                              // differentiable on the tape
assert_eq!(tape.value(y).shape, [1, 1, 64]);
```

The operator that learns a circular shift — what the identity cannot do and a spectral filter can — is `phical::Fno`; its README has that example.

## What is in it

| module | what |
|---|---|
| `tensor` | `Tensor` of shape `[batch, width, space]`; `Tape` with `add`, `add_bias`, `mul`, `scale`, `linear`, `mode_mix`, `fft`/`ifft` (unitary radix-2), `gelu`, `gather`, `unit`, `cosine`, `hinge`, `mean_real`, and `backward`; `Adam` on `(Re, Im)` pairs. |
| `operator` | `Fno` — the Fourier neural operator `h ← σ(F⁻¹(R·F h) + W h + b)`, lift and projection, kept-mode mask, hinge-on-cosine ranking objective, save/load. `modes = 0` is the pointwise control. `rank`, `cosine`. |
| `joint` | `Joint` — the table of functions as a parameter on the operator's tape; contexts gathered on the tape; a proximal tether against memorisation. |
| `hrr` | `bind`, `unbind`, `compose`, `role` — holographic reduced representations in the Fourier domain. |

## Conventions

Gradients of a real loss with respect to a complex value are `g = ∂L/∂Re(z) + i·∂L/∂Im(z)`. Under it `y = x·w` gives `g_x = g_y·conj(w)`, a unitary `y = U x` gives `g_x = U⁻¹ g_y`, and a real non-linearity applied to `Re` and `Im` separately back-propagates through each part on its own. The FFT is scaled by `1/√N` both ways, so it is unitary and its adjoint is its inverse.

The operator is resolution-free: `FnoConfig::space` only has to be a power of two, and the same `R` applies at any of them.

## Where it came from

Built for [phiano](../phiano), where a word is 64 complex numbers on a circle of relations. `phiano/src/phinet/` is the application layer (a lexicon as a function space, the operator inside a perplexity harness, joint training with write-back, recursive composition with the operator as cleanup); the engine has no notion of words. See `phiano/docs/65_phinet.md` and `66_phinet_in_the_loop.md` for what it measured.

## Tests

```
cargo test --release
```

Nine tests and the doc example: FFT unitarity and a pure tone landing in one bin; a finite-difference check of every op through a small network including `gather` and `unit`; Adam lowering a loss; the operator learning a circular shift the identity cannot and the pointwise control failing it; construction and save/load at other resolutions; joint training moving items and lowering loss; bind/unbind round-trip and single-level recovery.

## Phasor ranges

`phinet::phasor`: resolutions `P16 P32 P64 P128` × quanta `Q16 Q32 Q64` — twelve `Range`s every computation runs on (`resample`, `quantise`, `sector_of`, `Fno::at_resolution`). `cargo run --release --example ranges` measures all of them.

## PyTorch parity

`torch/phinet_torch.py` implements every operation above in PyTorch, with phinet's shapes and conventions (unitary FFT, tanh-GELU on Re and Im, conjugate-Wirtinger gradients). `examples/torch_reference.rs` writes a reference dump; the Python side replays it with `torch.autograd`:

```
cargo run --release --example torch_reference > torch/reference.json
python torch/phinet_torch.py torch/reference.json      # 60 checks, ALL PASS, max |Δ| < 1e-15
```

`torch/phinet_colab.ipynb` runs the checks in Colab and trains the sixty-second example in torch.

## Documentation

The full site — install, seven tutorials, the guide, every public item as API reference, the PyTorch twin, measurements, benchmarks, and what is missing against PyTorch/TensorFlow — is in `docs/` and served by `web/` (a [PhiDoc](../phidoc) site on puijs):

```
cd web && npm install && npm run dev     # http://127.0.0.1:5183
npm run sync-hub                          # mirror docs/ into ../phidoc/docs/phinet
```
