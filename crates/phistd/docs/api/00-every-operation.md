---
id: phinet-every-operation
title: Every Operation
description: The complete catalogue of phinet operations — shape, mathematics, PyTorch equivalent, and whether its gradient is checked — one row per public function.
sidebar_position: 0
badge: Reference
---

# Every Operation

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet computes those relations: a function is `Vec<Complex64>` on a closed circle, an operator maps functions to functions, and every operation below is exact on that circle.

A tensor is complex128 of shape `[batch, width, space]`. `space` is a power of two. Gradients are the conjugate-Wirtinger derivative `∂L/∂Re + i ∂L/∂Im` — the convention `torch.autograd` uses for complex tensors — so a phinet gradient and a torch gradient are the same numbers (checked in [PyTorch parity](/docs/phinet/pytorch-colab)).

## Tape operations (`tensor.rs`)

| phinet | shapes | mathematics | PyTorch | gradient |
|---|---|---|---|---|
| `Tape::add(a, b)` | `[b,w,s]+[b,w,s]` | `a + b` | `a + b` | ✓ |
| `Tape::add_bias(x, bias)` | `[b,w,s]+[1,w,1]` | broadcast over batch and sample | `x + b` | ✓ |
| `Tape::mul(a, b)` | pointwise | `a · b` | `a * b` | ✓ (`g·conj(b)`) |
| `Tape::scale(a, k)` | pointwise | `k · a`, real `k` | `k * a` | ✓ |
| `Tape::linear(x, w)` | `[b,i,s]×[i,o,1]→[b,o,s]` | pointwise map across the width | `einsum("bis,io->bos")` | ✓ |
| `Tape::mode_mix(x, r)` | `[b,i,m]×[i,o,m]→[b,o,m]` | one complex `i×o` matrix per Fourier mode | `einsum("bim,iom->bom")` | ✓ |
| `Tape::fft(x)` | along `s` | unitary DFT, `1/√N` both ways | `torch.fft.fft(x, norm="ortho")` | ✓ (adjoint = ifft) |
| `Tape::ifft(x)` | along `s` | unitary inverse DFT | `torch.fft.ifft(x, norm="ortho")` | ✓ |
| `Tape::fft2(x, s1, s2)` / `ifft2` | along `s` as an `s1×s2` torus | unitary 2-D DFT | `torch.fft.fft2(x.reshape(b,w,s1,s2), norm="ortho")` | ✓ |
| `Tape::gelu(x)` | pointwise | tanh-GELU on Re and Im separately | `complex(gelu(x.real), gelu(x.imag))` | ✓ |
| `Tape::unit(x)` | per row | `x / ‖x‖`, zero rows stay zero | `x / x.abs().pow(2).sum(-1).sqrt()` | ✓ |
| `Tape::cosine(p, t)` | `[b,1,s]→[b,1,1]` | `Re⟨p, t⟩ / (‖p‖‖t‖)` | see `phinet_torch.cosine` | ✓ |
| `Tape::cosine_table(p, T)` | `[b,1,s]×[V,1,s]→[b,V,1]` | cosine of every prediction against every table row (rayon-parallel) | `einsum("bs,vs->bv", p, T.conj()).real / norms` | ✓ |
| `Tape::gather(T, idx)` | `[V,1,s]→[b,1,s]` | row selection; backward scatter-adds into the table | `T[idx]` | ✓ |
| `Tape::hinge(pos, neg, m)` | `[b,1,1]→[1,1,1]` | `mean_b max(0, m − pos + neg)` | `clamp(m − pos + neg, min=0).mean()` | ✓ |
| `Tape::mean_real(x)` | `→[1,1,1]` | mean of real parts | `x.real.mean()` | ✓ |
| `Tape::mobius(z, abcd)` | pointwise, `abcd` `[1,1,4]` | `(a z + b)/(c z + d)` — rotation, dilation, inversion are special cases | `(a*z+b)/(c*z+d)` | ✓ (w.r.t. `z` and `abcd`) |
| `Tape::mixture_nll(S, terms)` | `[b,V,1]→[1,1,1]` | `mean_b −log(k_b + coef_b (γ softmax(β S_b)[t_b] + (1−γ) other_b))` | `phinet_torch.mixture_nll` | ✓ |
| `Tape::cross_entropy(S, targets, β)` | `[b,V,1]→[1,1,1]` | `mean_b −log softmax(β S_b)[t_b]` | `F.cross_entropy(β·S, t)` | ✓ |
| `Tape::backward(out)` | scalar | reverse-mode replay of the tape | `out.backward()` | — |
| `Adam::step(tape, params)` | — | Adam on Re and Im as independent reals, bias-corrected | `torch.optim.Adam` on complex parameters | matches after one step to 1e-16 |
| `Adam::with_decay(wd)` | — | decoupled weight decay before the Adam update (AdamW) | `torch.optim.AdamW` | matches after one step to 1e-16 |
| `Adam::with_schedule(s)` | — | `lr × Schedule::factor(step)`: constant, warm-up + cosine, step | `torch.optim.lr_scheduler` | tested |
| `fft_inplace(x, inverse)` | slice | radix-2, in place, unitary | `torch.fft.fft(norm="ortho")` | — |

## The operator (`operator.rs`)

| phinet | what | PyTorch |
|---|---|---|
| `Fno::new(cfg, seed)` | lift `[1,w,1]`, per layer `R [w,w,s]` (masked to `modes` each side of zero), `W [w,w,1]`, `b [1,w,1]`, projection `[w,1,1]` | `phinet_torch.Fno(space, width, layers, modes)` — identical parameter layout |
| `Fno::forward(x)` | `h ← σ(F⁻¹(R·F h) + W h + b)`; GELU between layers, none after the last; lift and projection around | `Fno.forward` |
| `Fno::apply(fs)` | forward off the tape, on a batch of functions | `with torch.no_grad(): fno(x)` |
| `Fno::mask_modes()` | re-zero the masked modes after a step | `Fno.mask_modes()` |
| `Fno::scale_modes(w)` | per-mode spectral prior as a projection (negative frequencies share their partner's weight) | multiply `R[..., m]` by `w[min(m, s−m)]` |
| `zeta_weights(modes, s)` | `m^{−s}/ζ(s)`, mode 0 → 1 | `phinet_torch.zeta_weights` |
| `Fno::train_epoch` / `train_epoch_with` | hinge on cosine vs a decoy (uniform, or `FrequencySampler`) | the training cell of the Colab notebook |
| `Fno::train_epoch_mixture` | the mixed-LM likelihood objective through `cosine_table` + `mixture_nll` | same two functions |
| `Fno::to_bytes` / `from_bytes` / `to_bytes_with` / `from_bytes_with` | versioned save/load, optionally with the optimiser | `state_dict` + optimizer `state_dict` |
| `Fno::at_resolution(space)` | the same operator at another resolution (`R` by signed frequency) | copy `R[..., f]` for kept `f` into a new module |
| `Fno::train_epoch_softmax` | full-table cross-entropy at temperature β | `F.cross_entropy` over `cosine_table` |
| `Module`, `Sequential` | the forward/parameters trait; two operators on one tape | `nn.Module`, `nn.Sequential` |
| `rank(table, preds, examples, pool)` | MRR and hit@10 under cosine | see the Colab `mrr()` |
| `stack` / `unstack` | functions ⇄ `[b,1,s]` | `torch.stack(...)[:, None, :]` |
| `cosine(a, b)` | real cosine of two functions | `phinet_torch.cosine` |

## Joint training (`joint.rs`)

`Joint::new(fno, table, window)` attaches the table of items as a parameter on the operator's tape (`Fno::attach_param`) and `Joint::train_epoch` trains both, holding the table by a proximal tether (`with_tether`) toward its starting point. In torch: make the table an `nn.Parameter`, index it with `gather`, add `tether · ‖T − T₀‖²` to the loss.

## Holographic binding (`hrr.rs`)

| phinet | mathematics | PyTorch |
|---|---|---|
| `role(space, seed)` | unit-magnitude random phases | `phinet_torch` reads the reference role; `torch.polar(1, 2π·rand)` for a fresh one |
| `bind(f, r)` | pointwise `f · r` (phase addition) | `f * r` |
| `unbind(f, r)` | pointwise `f · conj(r)` | `f * r.conj()` |
| `compose(parts)` | normalised sum | `stack(parts).sum(0) / norm` |

## The sixty-four gates (`hexagram.rs`)

| phinet | mathematics | PyTorch |
|---|---|---|
| `octave(m, space)` | `⌊log₂ m⌋` clamped to 5; negatives mirror; mode 0 has none | `phinet_torch.octave` |
| `gate(id, space)` | line *j* yang passes octave *j*; mode 0 always passes | `phinet_torch.gate` |
| `apply(f, id)` | `F⁻¹(gate ⊙ F f)` — exact, idempotent; ䷀ is the identity | `phinet_torch.hexagram_apply` |
| `hexagram_of(f)` | Q64 sector of the centroid phase | `phinet_torch.hexagram_of` |
| `changing_lines(a, b)` | `a ⊕ b` | `a ^ b` |
| `novelty(f, id)` | `1 − cos(f, H f)` | `phinet_torch.novelty` |
| `lines(f)` | octave energies, normalised over the six octaves | `phinet_torch.lines` |

## Phasor ranges (`phasor.rs`)

| phinet | mathematics | PyTorch |
|---|---|---|
| `Resolution`, `Quantum`, `Range` (`::ALL`, `::DEFAULT`) | P16–P128 × Q16–Q64, the twelve combinations | dtype/device-style enumeration; no torch counterpart |
| `resample(f, res)` | spectral zero-pad / truncate, values interpolated | `phinet_torch.resample` |
| `quantise(f, q)` | phase snapped to sector centres | `phinet_torch.quantise` |
| `sector_of(f, q)`, `quantisation_loss(f, q)`, `table(f)` | centroid sector; `1 − cos(f, quantise f)`; all ranges | one-liners |

## Utilities (`lib.rs`)

`mix64` (splitmix64 finaliser), `normalise` (unit energy in place), `unit_random(space, seed)`, `unit_phase(space, seed)`. The random streams are phinet's own; the torch side loads them from the reference rather than re-deriving them.

Every ✓ above is a finite-difference test in the crate (`cargo test`) **and** a numerical match against `torch.autograd` on the reference dump (`torch/phinet_torch.py`, 62 checks).
