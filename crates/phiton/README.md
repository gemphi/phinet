# phiton — quantum light

In Phient's world, a word (w) is a seed of what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle; meaning is a side effect
of pairs of words moving under a condition, and they move by the error of what
was expected — toward what came, away from what did not.

PhiTon is light as Phient's unit system, governed by the fine-structure
constant α ≈ 1/137: a phase read as a wavelength across the visible band
(380–750 nm), a `SpectralBand` (16), a `PhitonColor`, a `LightQuantum` (the
particle, "roll", ↻) and a `Chiton` (the wave, ↺) whose meeting is the
standing wave. It holds everything of the workspace that is light: the
holographic binding (`hrr`), the physics of colour, space and time with its
standing-wave modes (`physics`), and P64 — the 16/32/64 variation engine that
reads language in the circle's 64 sectors (`p64`, which was phiano's `phinum`).

| module | what |
|---|---|
| `Phiton` (lib) | `phase_to_wavelength`, `wavelength_to_phase`, `frequency_thz` (ν = c/λ), `energy_ev` (E = hc/λ), `sector_color` |
| `units` | `ALPHA` (1/137), `PHI`, `PHI_CONJUGATE` (1/φ), `TWO_PI`, `BAND_N_EFFECTIVE_MAX` (13), `PHASE_CHANNELS` (64), `CHANNEL_QUANTA` (256), `quantum`, `effective_phase` |
| `bands`, `types`, `spectrum` | the 16 named bands, colour and quantum, the spectrum mapper |
| `chiton` | the wave: `from_quantum` (↺), `roll` (↻), `opposite`, `resonance`, `standing_wave_amplitude` |
| `hrr` | holographic binding on the circle: `role`, `bind`, `unbind`, `compose` |
| `physics` | `Physics`, `PhysicsOp`, `ColorSpaceTimePoint`, `ColorSpaceTimeManifold`, `Topology`; `standing::harmonic_rotation` (Bentov's integer modes) |
| `p64` | `Level {P16, P32, P64}`, `Engine`, `Config`, `Lexicon`, `SyntaxParser`, `SpeechAct`, `SentenceType`, `SpiderNet`, `words::Tokenizer` |

phiano re-exports it as `phiano::phiton`; phiecs's spectral phasor reads its
fine-structure phase and colour. It depends on phistd for the numbers.

```
cargo test --release -p phiton
```

The architecture is drawn in [ARCHITECTURE.md](ARCHITECTURE.md); what is open is in [TODO.md](TODO.md).
