# phiton — light as Phient's unit system

In Phient's world, a word (w) is a seed of what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition.

PhiTon reads a phase as light: a wavelength across the visible band (380–750
nm) with a fine-structure shift of `quantum(band_n) · α · 10` nm, a
`SpectralBand` (16), a `PhitonColor` (name, RGB, hex), a `LightQuantum` (the
particle, "roll") and a `Chiton` (the wave). It is a child of phinet in the
PML family and was lifted out of phiano (docs/85, move 1); phiano re-exports
it as `phiano::phiton`, so nothing that used it changed.

| type | what |
|---|---|
| `Phiton` | `phase_to_wavelength`, `wavelength_to_phase`, `frequency_thz` (ν = c/λ), `energy_ev` (E = hc/λ), `sector_color` |
| `PhitonSpectrum` | `phase_to_color`, `sector_to_color`, `band_colors`, `spectral_distance`, `coupling`, `antipodal_color` |
| `SpectralBand` | the 16 named bands |
| `PhitonColor`, `LightQuantum`, `Chiton` | colour, particle, wave |
| `units` | `ALPHA` (1/137), `PHI`, `TWO_PI`, `BAND_N_EFFECTIVE_MAX` (13), `quantum` |

Its one shipping reader is phiano's helix: every codon of a strand carries its
wavelength, band and hex (`love`'s `AAA` is 418.5 nm indigo `#5900F8`).

```
cargo test --release     # 4 tests: round trip, clamp, c/λ and hc/λ, sector colours
```
