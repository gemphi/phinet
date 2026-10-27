use super::*;

#[test]
fn a_phase_round_trips_through_its_wavelength() {
    for i in 0..64 {
        let phase = TWO_PI * i as f64 / 64.0;
        let nm = Phiton::phase_to_wavelength(phase, 0);
        assert!((Phiton::VISIBLE_MIN_NM..=Phiton::VISIBLE_MAX_NM).contains(&nm), "{nm}");
        let back = Phiton::wavelength_to_phase(nm);
        assert!((back - phase).abs() < 1e-9, "phase {phase} → {nm} nm → {back}");
    }
}

#[test]
fn the_sub_band_shift_is_clamped() {
    assert_eq!(quantum(5), 5);
    assert_eq!(quantum(1_800_000), BAND_N_EFFECTIVE_MAX);
    let low = Phiton::phase_to_wavelength(1.0, 0);
    let clamped = Phiton::phase_to_wavelength(1.0, 1_800_000);
    assert!(clamped > low && clamped < Phiton::VISIBLE_MAX_NM, "a huge counter shifts a little, not to the red end");
}

#[test]
fn light_obeys_its_physics() {
    // ν = c/λ and E = hc/λ at 500 nm: about 599.6 THz and 2.48 eV.
    assert!((Phiton::frequency_thz(500.0) - 599.585).abs() < 0.01);
    assert!((Phiton::energy_ev(500.0) - 2.4797).abs() < 0.001);
}

#[test]
fn a_sector_has_a_colour() {
    let c = Phiton::sector_color(0, 64, 0);
    assert!((c.wavelength_nm - Phiton::VISIBLE_MIN_NM).abs() < 1e-9);
    assert!(c.hex().starts_with('#') && c.hex().len() == 7);
    let colours: std::collections::HashSet<&str> = (0..64).map(|s| Phiton::sector_color(s, 64, 0).name()).collect();
    assert!(colours.len() > 5, "64 sectors span more than a handful of bands: {colours:?}");
}

#[test]
fn the_spectrum_s_top_edge_is_crimson_and_the_bands_have_no_gaps() {
    assert_eq!(SpectralBand::from_wavelength(750.0).name, "crimson", "750 nm is the last band, not violet");
    assert_eq!(SpectralBand::from_wavelength(900.0).name, "crimson", "clamped to the top edge");
    assert_eq!(SpectralBand::from_wavelength(380.0).name, "violet");
    assert_eq!(SpectralBand::from_wavelength(100.0).name, "violet");
    assert_eq!(SpectralBand::index_of(Phiton::phase_to_wavelength(6.28, 1000)), SpectralBand::BANDS.len() - 1);
    for w in SpectralBand::BANDS.windows(2) {
        assert_eq!(w[0].max_nm, w[1].min_nm, "{} runs into {}", w[0].name, w[1].name);
        assert_eq!(SpectralBand::from_wavelength(w[1].min_nm).name, w[1].name, "a band owns its lower edge");
    }
}

#[test]
fn band_colours_hold_at_any_sector_count() {
    for count in [16u16, 4369, 4370, u16::MAX] {
        let colours = PhitonSpectrum::band_colors(count);
        assert_eq!(colours.len(), SpectralBand::BANDS.len(), "{count} sectors");
    }
}

#[test]
fn a_wave_and_its_roll_resonate_fully() {
    let q = LightQuantum::from_phase(1.2, 1.0, 0);
    let (wave, roll) = (Chiton::from_quantum(&q), Chiton::roll(&q));
    assert!(!wave.spin_clockwise && roll.spin_clockwise, "the wave turns ↺, the roll ↻");
    assert!((wave.resonance(&roll) - 1.0).abs() < 1e-12, "opposite spins at one frequency: the standing wave");
    assert!((wave.resonance(&wave) - 0.5).abs() < 1e-12, "two waves of one spin: half");
    assert_eq!(wave.opposite().spin_clockwise, roll.spin_clockwise);
    assert_eq!(roll.angular_frequency, wave.angular_frequency, "the same quantum, the same frequency");
}

