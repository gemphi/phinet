//! Counterfactual reasoning: swaps a key word's phase and re-evaluates.
//! Implements Ch 14.2's reasoning about hypothetical situations.

use phiton::units::TWO_PI;
use phient::eval::Evaluator;
use phiecs::phasor::facet::Facet;
use phiton::p64::words::Tokenizer;

#[derive(Debug, Default)]
pub struct Counterfactual;

impl Counterfactual {
    /// Swaps a key word's phase to the counterfactual word's phase, re-evaluates.
    /// Returns what changes in the output.
    pub fn swap(facet: &Facet, premise: &str, counterfactual: &str) -> String {
        let evaluator = Evaluator::new();

        let baseline = evaluator.eval(facet, premise);
        let baseline_coh = baseline.coherence;

        let premise_tokens = Tokenizer::tokenize(premise);
        let cf_tokens = Tokenizer::tokenize(counterfactual);

        let mut perturbed = facet.clone();

        for (p_word, cf_word) in premise_tokens.iter().zip(cf_tokens.iter()) {
            if let Some(cf_phasor) = facet.lexicon.get(cf_word) {
                if let Some(p_phasor) = perturbed.lexicon.get_mut(p_word) {
                    p_phasor.phase = cf_phasor.phase;
                }
            }
        }

        let perturbed_eval = evaluator.eval(&perturbed, premise);
        let delta = perturbed_eval.coherence - baseline_coh;

        if delta.abs() < 0.01 {
            "Counterfactual had minimal effect: the model is robust to this swap.".to_string()
        } else if delta > 0.0 {
            format!("Counterfactual improved coherence by {:.4}: {} → {} suggests the original word was suboptimal.",
                delta, premise, counterfactual)
        } else {
            format!("Counterfactual reduced coherence by {:.4}: {} → {} suggests the original word was important.",
                -delta, premise, counterfactual)
        }
    }

    /// Tests what-if scenarios by perturbing phases.
    pub fn what_if(facet: &Facet, prompt: &str, word: &str, new_phase: f64) -> String {
        let evaluator = Evaluator::new();
        let baseline = evaluator.eval(facet, prompt).coherence;

        let mut perturbed = facet.clone();
        if let Some(p) = perturbed.lexicon.get_mut(word) {
            p.phase = new_phase.rem_euclid(TWO_PI);
        }

        let perturbed_score = evaluator.eval(&perturbed, prompt).coherence;
        let delta = perturbed_score - baseline;

        format!("If '{}' had phase {:.3} instead: coherence {} → {:.4} ({:+.4})",
            word, new_phase, baseline, perturbed_score, delta)
    }
}
