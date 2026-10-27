use super::{Mirror, Moment, Told};

fn argmax(s: &[f64]) -> usize {
    s.iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)).map(|x| x.0).unwrap_or(0)
}

#[test]
fn the_conjugate_filter_reads_the_relation_backwards() {
    let names: Vec<String> = (0..6).map(|i| format!("w{i}")).collect();
    let mut m = Mirror::new(names, vec!["next".into()], 16, false, 7);
    m.phases[0] = (0..16).map(|k| 0.3 * k as f64 + 0.1).collect();
    for (w, v) in [(0, 1), (2, 5), (4, 4)] {
        let back = m.correlate(&m.wave(v, 0, -1))[w];
        assert!((m.score(w, 0, v) - back).abs() < 1e-12, "Re⟨H a_w, a_v⟩ = Re⟨a_w, H̄ a_v⟩");
    }
    let path = std::env::temp_dir().join("phical_mirror_test.mirror");
    m.save(path.to_str().unwrap()).unwrap();
    let l = Mirror::load(path.to_str().unwrap()).unwrap();
    assert!((l.score(0, 0, 1) - m.score(0, 0, 1)).abs() < 1e-12 && l.kind("next") == Some(0));
}

#[test]
fn a_cycle_learned_forward_is_read_back_and_two_steps_on_untrained() {
    // Twelve words in a ring, told only `w_i next w_{i+1}`.
    let n = 12;
    let names: Vec<String> = (0..n).map(|i| format!("w{i}")).collect();
    let mut m = Mirror::new(names, vec!["next".into()], 32, false, 3);
    let rows: Vec<Told> = (0..n).map(|i| Told { about: i, kind: 0, other: (i + 1) % n }).collect();
    let pool: Vec<usize> = (0..n).collect();
    let mut mo = Moment::new(&m);
    for e in 0..300 {
        m.train_epoch(&rows, &pool, 0.5, 0.02, &mut mo, e);
    }
    for i in 0..n {
        assert_eq!(argmax(&m.read(&format!("w{i}"), "next", 1).unwrap()), (i + 1) % n, "told forward");
        assert_eq!(argmax(&m.read(&format!("w{i}"), "next", -1).unwrap()), (i + n - 1) % n, "the conjugate: one step back, never told");
    }
    let two = (0..n).filter(|&i| argmax(&m.read(&format!("w{i}"), "next", 2).unwrap()) == (i + 2) % n).count();
    assert!(two >= n / 2, "H² carries a word two steps on for most of the ring: {two} of {n}");
}

#[test]
fn a_pumped_mirror_learns_its_gains_and_reads_back_by_the_adjoint() {
    // Six words in a ring, told `w_i next w_{i+1}`, with gains free to move.
    let n = 6;
    let names: Vec<String> = (0..n).map(|i| format!("w{i}")).collect();
    let mut m = Mirror::new(names, vec!["next".into()], 16, true, 5);
    let rows: Vec<Told> = (0..n).map(|i| Told { about: i, kind: 0, other: (i + 1) % n }).collect();
    let pool: Vec<usize> = (0..n).collect();
    let mut mo = Moment::new(&m);
    for e in 0..300 {
        m.train_epoch(&rows, &pool, 0.5, 0.02, &mut mo, e);
    }
    assert!(m.gains[0].iter().any(|g| g.abs() > 1e-3), "the gains moved");
    for i in 0..n {
        assert_eq!(argmax(&m.read(&format!("w{i}"), "next", 1).unwrap()), (i + 1) % n, "told forward");
        // The adjoint reads the same score from the other end: Re⟨H a_w, a_v⟩ = Re⟨a_w, H̄ a_v⟩.
        let (w, v) = (i, (i + 1) % n);
        let back = m.correlate(&m.wave(v, 0, -1))[w];
        let forward: f64 = m.wave(w, 0, 1).iter().zip(&m.spectra[v]).map(|(x, y)| (x * y.conj()).re).sum();
        assert!((back - forward).abs() < 1e-9, "adjoint identity");
    }
    let path = std::env::temp_dir().join("phical_pumped_test.mirror");
    m.save(path.to_str().unwrap()).unwrap();
    let l = Mirror::load(path.to_str().unwrap()).unwrap();
    assert!(l.pumped && l.gains == m.gains, "gains survive a save");
}
