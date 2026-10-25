# phical — Phient's operators, learned and exact

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

phinet is the engine (what torch is), phifno the sphere (what torch-harmonics is), and phical the operators built on both — what neuraloperator is to torch (read under `gemphi/ext/neuraloperator`, with `makani` for the spherical operator of FourCastNet 3) — together with the exact geometry of the relation circle that no learning touches.

| module | what |
|---|---|
| `operator` | `Fno`, `FnoConfig` (`for_space`, `sphere`, `anisotropic`, `circle64`), `Tie` (circle, sphere, anisotropic), `Example`, `rank`, `zeta_weights`, spectral priors, versioned save with optimiser state, `Module`, `Sequential` |
| `conjugate` | `Mirror`: a relation as an all-pass filter `e^{iθ_k}` on the circle; read forward by `H`, backwards by its conjugate `H̄` (never trained) and `n` steps on by `Hⁿ`; closed-form training (`Moments`, `Told`). RotatE as a Fourier operator (docs/153, paper 27) |
| `joint` | `Joint`: the table of functions as a parameter on the operator's tape, with a proximal tether |
| `relate` | one `Joint` for every kind of relation; the laws the kinds obey, closure, inheritance along a kind, the reading and its bar |
| `liskov` | `better`, `worse`, `loss`, `forcing_choice` |
| `sphere`, `mobius`, `q64`, `winding`, `heat` | the exact operators: the Riemann sphere per channel, Möbius maps, the C64/Q64 quantum juncture, winding numbers, heat flow |

```rust
use phical::{rank, Adam, Example, Fno, FnoConfig, Tie};
use phinet::unit_random;

// Sixty items on a 64-point circle; the target of each is itself shifted by
// one sample — a relation the identity cannot express and a spectral filter can.
let inputs: Vec<_> = (0..60).map(|i| unit_random(64, i)).collect();
let targets: Vec<_> = inputs.iter().map(|f| { let mut g = f.clone(); g.rotate_right(1); g }).collect();
let examples: Vec<Example> = (0..60).map(|i| Example { input: inputs[i].clone(), target: i }).collect();
let pool: Vec<usize> = (0..60).collect();
let cfg = FnoConfig { space: 64, width: 4, layers: 1, modes: 32, margin: 0.5, lr: 1e-2, tie: Tie::Circle };
let mut fno = Fno::new(cfg, 1);
let mut adam = Adam::new(cfg.lr);
for epoch in 0..60 {
    fno.train_epoch(&targets, &examples, &pool, 20, &mut adam, epoch);
}
let (mrr, _) = rank(&targets, &fno.apply(&inputs), &examples, &pool);
assert!(mrr > 0.9);
```

Examples: `bench`, `profile_apply`, `profile_softmax`, `ranges`, `torch_reference` (parity with PyTorch), `degree_filter` (circle vs sphere vs anisotropic on a per-degree relation). Measurements: phiano docs/78, 84, 86, 91–97, 105.
