# numerics: random, stats, prob, real, optim

The plain numerics phient used to write inline (batch 2 of docs/phient/calculations, 2026-10-28), each written here exactly as the sites wrote it, so no recorded number moved.

Added 2026-10-29 for phisor's checks, new rather than moved: `precision` and `recall` (`f1` now reads them, with the same arithmetic), and a contingency table's `purity`, `mutual_information` in bits, and a `z_score` against a null.

```mermaid
graph LR
    subgraph phistd["phistd"]
        Random["random.rs<br/>unit_interval · unit_interval_f32<br/>lcg · xorshift64 · xorshift64_star<br/>draw_by_weight"]
        Stats["stats.rs<br/>mean · population_sd · precision · recall · f1 · log_share<br/>purity · mutual_information · z_score"]
        Prob["prob.rs<br/>perplexity · perplexity_of_nll<br/>softmax_z · softmax_p · log_softmax_at<br/>softmax_minus_onehot · sample_softmax"]
        Real["real.rs<br/>normalise_f32 · dot_f32<br/>norm · re_dot_interleaved"]
        Optim["optim.rs<br/>adam_step on slices"]
        Lib["lib.rs<br/>seed_phase · seed_angles · unit_phase"]
    end
    Scribe["phient scribe<br/>coin, draw, mold::pick"]
    Learner["phient lineup::learner<br/>chance"]
    Held["phient prose::heldout<br/>relate::quests (held-out splits)"]
    Sampling["phient generate::sampling<br/>next_uniform, sample_index"]
    Harness["phient metrics::harness::lm<br/>negatives, context readout,<br/>gamma, sweep, affine"]
    Metrics["phient metrics<br/>baseline, forgetting, arc"]
    Evaluator["phient generate::compose::better<br/>average_score, score_spread"]
    Baseline["phient baseline (Kneser–Ney)"]
    Study["phient study::Score"]
    Levels["phient relate::levels"]
    Tuner["phient generate::learn::Tuner<br/>nll, grad, fit"]
    Synthesis["phient train::synthesis::library"]
    Lm["phient net::lm cosines"]
    Phisor["phisor<br/>the slice's checks: purity, MI, z against shuffles"]

    Lib --> Random
    Scribe --> Random
    Learner --> Random
    Held --> Random
    Sampling --> Random
    Sampling --> Prob
    Harness --> Random
    Harness --> Prob
    Metrics --> Stats
    Metrics --> Prob
    Evaluator --> Stats
    Baseline --> Prob
    Study --> Prob
    Held --> Prob
    Levels --> Prob
    Tuner --> Prob
    Tuner --> Optim
    Synthesis --> Real
    Lm --> Real
    Phisor --> Stats
```
