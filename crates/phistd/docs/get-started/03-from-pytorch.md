---
id: phinet-from-pytorch
title: From PyTorch or TensorFlow
description: The cheat sheet — what each familiar thing is called here, and what has no counterpart.
sidebar_position: 3
---

# From PyTorch or TensorFlow

> **The paradigm.** In Phient's world, a word *w* is nothing but what it relates to, under what conditions *c*. Phinet is the engine that computes those relations: a function on a closed circle, an operator from functions to functions, everything exact on that circle.

## Vocabulary

| PyTorch / TensorFlow | Phinet | notes |
|---|---|---|
| `torch.Tensor` (complex128) | `Tensor { shape: [b,w,s], data: Vec<c64> }` | always rank 3 |
| autograd graph / `GradientTape` | `Tape` | explicit: `tape.param(t)`, `tape.constant(t)`, ops return `Var` |
| `requires_grad=True` | `Tape::param` | constants via `Tape::constant` |
| `loss.backward()` | `tape.backward(loss)` | loss must be `[1,1,1]` |
| `p.grad` | `tape.grad(var)` | conjugate-Wirtinger, same numbers as torch |
| `torch.optim.Adam` | `Adam::new(lr)`; `adam.step(&mut tape, &params)` | Re/Im as two reals, like torch on complex params |
| `torch.fft.fft(x, norm="ortho")` | `tape.fft(x)` / `fft_inplace` | unitary; torch's default `"backward"` differs by √N |
| `nn.Linear` across channels | `tape.linear(x, w)` with `w: [i,o,1]` | pointwise over the sample axis |
| spectral conv (FNO) | `tape.mode_mix(fft(h), r)` with `r: [i,o,m]` | one complex matrix per mode |
| `F.gelu` | `tape.gelu(x)` | tanh form, on Re and Im separately |
| `F.normalize` | `tape.unit(x)` | per row, zero rows stay zero |
| `F.cosine_similarity` | `tape.cosine(p, t)` | real part of the Hermitian inner product |
| `pred @ table.T` (normalised) | `tape.cosine_table(p, table)` | `[b,V,1]`, rayon over rows |
| `torch.index_select` / `nn.Embedding` | `tape.gather(table, idx)` | scatter-add backward: the table can be a parameter |
| `nn.MarginRankingLoss` | `tape.hinge(pos, neg, margin)` | mean over batch |
| `nn.Module` | `Fno`, `Joint` | no generic module trait yet |
| `state_dict` / `torch.save` | `Fno::to_bytes` / `from_bytes` | bincode |
| `torch.manual_seed` | every constructor and epoch takes a seed | no global state |
| `DataLoader` | a `Vec<Example>` and `batch` | no shuffling abstraction |
| `torch.optim.lr_scheduler` | — | set `adam.lr` yourself |
| `torch.nn.functional.conv1d` (circular) | `fft → mode_mix → ifft` | exact, not padded |
| `nn.MultiheadAttention`, `LayerNorm`, `Dropout` | — | see [What is missing](/docs/about/what-is-missing) |
| `.cuda()` | — | CPU only |

## The four conventions to get right

1. **Unitary FFT** both ways (`1/√N`). Otherwise `ifft(fft(x)) = x` still holds but every spectral weight differs by a factor of N between engines.
2. **GELU on parts.** Applied to Re and Im separately; `torch.nn.GELU` on a complex tensor is not defined.
3. **The complex gradient** is `∂L/∂Re + i ∂L/∂Im`. Torch's `.grad` on a complex leaf is the same quantity, so gradients match to the last digit.
4. **Adam per part.** `Adam` keeps separate first and second moments for Re and Im, exactly what `torch.optim.Adam` does through the real view.

[PyTorch parity](/docs/interop/pytorch-parity) checks all four on a reference dump; [Every operation](/docs/api/every-operation) lists the torch equivalent beside each op.

## Shapes

Phinet has no broadcasting. `[b,w,s]` is the only rank; a single function is `[1,1,s]`; a table is `[V,1,s]`; a bias is `[1,w,1]`; a linear weight is `[i,o,1]`; a spectral weight is `[i,o,s]`; a scalar loss is `[1,1,1]`. Every op asserts its shapes.
