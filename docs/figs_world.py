"""Figures for docs/world_model.md: the phiecs world, the phinet engine, the
learning steps with the world model, and the levels of abstraction. Same
style as crates/phiano/docs/papers/figs25.py. Every number drawn here is
from the docs it names."""
import os
import numpy as np, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch, Circle, Arc

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "fig")
os.makedirs(OUT, exist_ok=True)
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10, "axes.titleweight": "bold"})
INK = "#1f2430"; ACC = "#c8451f"; ACC2 = "#1f6fb2"; ACC3 = "#2e8b57"; GREY = "#9aa0a6"; PALE = "#f4f6f9"; WARM = "#fdf1ec"; COOL = "#eaf2fb"; LEAF = "#eaf6ee"


def save(fig, name):
    fig.savefig(f"{OUT}/{name}.png", dpi=170, bbox_inches="tight", facecolor="white")
    fig.savefig(f"{OUT}/{name}.svg", bbox_inches="tight", facecolor="white")
    plt.close(fig)


def box(ax, x, y, w, h, title, lines, fc=PALE, ec=INK, tc=INK, lw=1.4, fs=8.8, tfs=None):
    ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.02,rounding_size=0.025", fc=fc, ec=ec, lw=lw))
    n = max(len(lines), 1)
    top = 0.84 if lines else 0.5
    ax.text(x + w / 2, y + h * top, title, ha="center", va="center", fontsize=tfs or fs + 1.5, fontweight="bold", color=tc)
    for i, l in enumerate(lines):
        ax.text(x + w / 2, y + h * (0.66 - 0.60 * (i + 0.5) / n + 0.30 / n), l, ha="center", va="center", fontsize=fs, color=INK)


def arrow(ax, p, q, color=INK, lw=1.4, style="-|>", ls="-", rad=0.0):
    ax.add_patch(FancyArrowPatch(p, q, arrowstyle=style, mutation_scale=12, color=color, lw=lw, linestyle=ls, connectionstyle=f"arc3,rad={rad}"))


# ------------------------------------------------------------- 1. the phiecs world
def fig_phiecs():
    fig, ax = plt.subplots(figsize=(12.6, 7.6)); ax.set_xlim(0, 12.6); ax.set_ylim(0, 7.6); ax.axis("off")
    ax.set_title("phiecs — the world: entities are words, components are relations, systems move them under conditions", fontsize=11.5)
    # the World frame
    ax.add_patch(FancyBboxPatch((0.25, 0.45), 8.5, 6.7, boxstyle="round,pad=0.02,rounding_size=0.04", fc="white", ec=INK, lw=1.8))
    ax.text(0.45, 6.95, "World", fontsize=13, fontweight="bold", color=INK)
    ax.text(1.6, 6.95, "coin · forget · declare · relate · relate_by · unrelate · calls_for · bind · read · add_system · step", fontsize=8.2, color=GREY, va="center")
    box(ax, 0.45, 5.0, 2.6, 1.6, "Lexicon", ["spelling ⇄ Word(id)", "intern · get · spelling", "a scope is a word;", "a kind is a word"], fc=COOL, ec=ACC2)
    box(ax, 3.25, 5.0, 2.6, 1.6, "Relations", ["64 channels, Relation(u8)", "16 named kinds + declared", "observe · evidence per kind"], fc=COOL, ec=ACC2)
    box(ax, 6.05, 5.0, 2.5, 1.6, "Edges", ["(from, kind, to, weight)", "relate adds, weight sums", "calls_for(w, kind): X ⇒ Y"], fc=COOL, ec=ACC2)
    # the Phasor aspect
    box(ax, 0.45, 2.6, 4.0, 2.05, "stores: Dense<Phasor>", ["one Aspect per word", "θ[64]  one angle per channel", "evidence[64]  how often coupled", "amplitude[64] = ln(1 + evidence)", "seeded(spelling): a start of its own"], fc=WARM, ec=ACC)
    # the circle drawing of couple
    cx, cy, r = 6.9, 3.75, 0.72
    ax.add_patch(Circle((cx, cy), r, fc="white", ec=INK, lw=1.3))
    ta, tb = np.deg2rad(150), np.deg2rad(30)
    for t, c, lab in [(ta, ACC, "θ_a"), (tb, ACC2, "θ_b")]:
        ax.plot([cx, cx + r * np.cos(t)], [cy, cy + r * np.sin(t)], color=c, lw=2)
        ax.text(cx + 1.2 * r * np.cos(t), cy + 1.2 * r * np.sin(t), lab, color=c, fontsize=9, ha="center", va="center")
    ax.add_patch(Arc((cx, cy), 1.3 * r, 1.3 * r, theta1=30, theta2=150, color=GREY, lw=1.0, ls="--"))
    ax.text(cx, cy + 1.35 * r, "relate(a, kind, b)", ha="center", va="bottom", fontsize=9.5, fontweight="bold", color=INK)
    ax.text(cx, cy - 1.2 * r, "couple(kind, target, rate):\nθ ← θ + rate·Δ, the short way round\nrate 0.08 per observation; both ends move", ha="center", va="top", fontsize=7.6, color=INK)
    # systems
    ax.add_patch(FancyBboxPatch((0.45, 0.6), 5.4, 1.75, boxstyle="round,pad=0.02,rounding_size=0.025", fc=LEAF, ec=ACC3, lw=1.4))
    ax.text(3.15, 2.1, "systems: Vec<Box<dyn System>> — step(dt) runs the stages in order", ha="center", va="center", fontsize=8.6, fontweight="bold", color=INK)
    stages = ["Ingest", "PreCouple", "Couple", "PostCouple", "Readout"]
    for i, st in enumerate(stages):
        x = 0.6 + i * 1.04
        ax.add_patch(FancyBboxPatch((x, 1.2), 0.9, 0.55, boxstyle="round,pad=0.01", fc="white", ec=ACC3, lw=1.1))
        ax.text(x + 0.45, 1.475, st, ha="center", va="center", fontsize=8, color=INK)
        if i < 4:
            arrow(ax, (x + 0.9, 1.475), (x + 1.04, 1.475), color=ACC3, lw=1.0)
    ax.text(3.15, 0.85, "new rows · settle the phasors · read out  ·  phikit::Watch: one frame per step", ha="center", fontsize=7.4, color=GREY)
    box(ax, 6.05, 0.6, 2.5, 1.75, "Condition · Resources", ["now / was · enter · tick", "world-owned resources", "Quest: rows → ask · oracle"], fc=LEAF, ec=ACC3)
    # arrows inside
    arrow(ax, (1.75, 5.0), (1.75, 4.65), color=ACC2); arrow(ax, (4.55, 5.0), (3.9, 4.65), color=ACC2); arrow(ax, (7.3, 5.0), (7.1, 4.75), color=ACC2)
    arrow(ax, (2.45, 2.6), (2.45, 2.35), color=ACC3)
    # outside: the sources and the readers
    box(ax, 9.1, 5.3, 3.2, 1.5, "rows in", ["phiplm::prose::paragraph (text)", "phiplm::code (Rust)", "phikit::tell · Registry::rows", "quest rows from Phi-4"], fc=PALE)
    arrow(ax, (9.1, 6.05), (8.75, 5.9), color=INK, lw=1.6)
    box(ax, 9.1, 3.1, 3.2, 1.7, "read out", ["calls_for(w, kind): what w calls for", "agreement(a, b, kind) = cos(θ_a − θ_b)", "None without evidence — silence", "resonance: all channels, by evidence"], fc=PALE, fs=8.2)
    arrow(ax, (8.75, 3.95), (9.1, 3.95), color=INK, lw=1.6)
    box(ax, 9.1, 0.6, 3.2, 2.0, "the same rows, counted", ["phiplm::Counts under every scope", "ScopeTree: the tree of conditions", "one row writes lexicon, edges,", "phasors and counts alike"], fc=WARM, ec=ACC)
    arrow(ax, (12.3, 6.05), (12.3, 1.6), color=ACC, lw=1.2, ls="--", rad=-0.25)
    ax.text(12.5, 3.9, "the same rows", rotation=90, ha="center", va="center", fontsize=7.4, color=ACC)
    ax.text(6.3, 0.12, "RealityKit's shape (Entity · Component · System · World) with one substitution: the entity is a word, and what it carries is a phase on the relation circle", ha="center", fontsize=8, color=GREY)
    save(fig, "phiecs_world")


# ------------------------------------------------------------- 2. the phinet engine
def fig_phinet():
    fig, ax = plt.subplots(figsize=(12.6, 6.8)); ax.set_xlim(0, 12.6); ax.set_ylim(0, 6.8); ax.axis("off")
    ax.set_title("phinet — the engine: complex functions on the circle, a tape that differentiates through the FFT, and what is built on it", fontsize=11.5)
    box(ax, 0.3, 4.6, 2.7, 1.7, "Function · Tensor", ["Vec<c64>: samples on a circle", "at 32 / 64 / 128 points", "Tensor [batch, width, space]", "unit_random · cosine · stack"], fc=COOL, ec=ACC2)
    box(ax, 3.3, 4.6, 5.6, 1.7, "Tape (reverse mode)", ["constant · var · add · mul · linear · mode_mix · gelu · gather · unit", "fft · ifft · fft2 · ifft2 — unitary, differentiable", "cosine · hinge · cross_entropy · cosine_table · mixture_nll · mobius", "backward: g = ∂L/∂Re + i·∂L/∂Im; every op finite-difference checked"], fc=COOL, ec=ACC2, fs=7.8)
    box(ax, 9.2, 4.6, 3.1, 1.7, "Adam · Schedule", ["AdamW over Vars", "warm-up · cosine decay", "state saved with the operator", "(versioned, in phical)"], fc=COOL, ec=ACC2)
    arrow(ax, (3.0, 5.45), (3.3, 5.45), color=ACC2); arrow(ax, (8.9, 5.45), (9.2, 5.45), color=ACC2)
    box(ax, 0.3, 2.6, 3.7, 1.5, "hrr", ["holographic reduced representations", "bind · unbind · compose · role", "circular convolution = product in Fourier"], fc=PALE)
    box(ax, 4.3, 2.6, 4.0, 1.5, "hexagram", ["64 spectral gates: six lines = six octaves", "gate · apply · changing_lines", "hexagram_of(f) · phase_of · novelty"], fc=PALE)
    box(ax, 8.6, 2.6, 3.7, 1.5, "phasor ranges", ["Resolution P16–P128 × Quantum Q16–Q64", "resample · quantise · sector_of · snap", "Range::ALL; Fno::at_resolution"], fc=PALE)
    # built on it
    ax.add_patch(FancyBboxPatch((0.3, 0.35), 12.0, 1.75, boxstyle="round,pad=0.02,rounding_size=0.04", fc="white", ec=GREY, lw=1.2, ls="--"))
    ax.text(0.5, 1.9, "built on it, one way up (docs/139)", fontsize=9, color=GREY, fontweight="bold")
    box(ax, 0.5, 0.5, 2.7, 1.2, "phifno", ["the sphere: 64 real harmonics, l < 8", "analysis · synthesis · grids"], fc=PALE, fs=7.8)
    box(ax, 3.45, 0.5, 2.9, 1.2, "phical", ["Fno (circle · sphere · aniso) · Tie · Joint", "relate · liskov · exact geometry"], fc=WARM, ec=ACC, fs=7.8)
    box(ax, 6.6, 0.5, 2.9, 1.2, "phiplm", ["Field: scopes as phasors (docs/136)", "RelationModel over word functions"], fc=WARM, ec=ACC, fs=7.8)
    box(ax, 9.75, 0.5, 2.4, 1.2, "phikit · phiano", ["the body as rows; the apps", "Phasor::seeded → word functions"], fc=PALE, fs=7.8)
    for x in (3.2, 6.35, 9.5):
        arrow(ax, (x, 1.1), (x + 0.25, 1.1), color=GREY, lw=1.2)
    arrow(ax, (6.3, 2.6), (6.3, 2.1), color=GREY, lw=1.2)
    ax.text(6.3, 0.1, "phinet knows no words and holds no operator: the operator is phical's, the harmonics phifno's, the language model phiplm's", ha="center", fontsize=8, color=GREY)
    save(fig, "phinet_engine")


# ------------------------------------------------------------- 3. the learning steps
def fig_steps():
    fig, ax = plt.subplots(figsize=(12.6, 6.9)); ax.set_xlim(0, 12.6); ax.set_ylim(0, 6.9); ax.axis("off")
    ax.set_title("Learning with the world model: seven steps, one data structure, no loss", fontsize=11.5)
    steps = [
        (1.0, 4.9, "1 · read", ["a source by its own grammar", "text → tokens, paragraphs", "Rust → its syntax", "a frame → what it is sure of", "nothing added, nothing dropped"], COOL, ACC2),
        (3.7, 4.9, "2 · tell as rows", ["(about, kind, other)", "under every scope on the path", "sentence ⊂ paragraph ⊂ chapter", "frame ⊂ scene ⊂ session", "one row per observation"], COOL, ACC2),
        (6.4, 4.9, "3 · count", ["Counts::learn", "D1 movement · D2 posts", "D4 nested distributions,", "each scope smoothed toward", "its parent's"], WARM, ACC),
        (9.1, 4.9, "4 · relate", ["World::relate(a, kind, b)", "edge + evidence; both phasors", "move 0.08 of the gap", "Watch: one frame per step,", "near within, before to next"], WARM, ACC),
    ]
    for (x, y, t, l, fc, ec) in steps:
        box(ax, x, y, 2.5, 1.75, t, l, fc=fc, ec=ec, fs=8.2)
    for x in (3.5, 6.2, 8.9):
        arrow(ax, (x, 5.77), (x + 0.2, 5.77), color=INK, lw=1.6)
    steps2 = [
        (9.1, 2.3, "5 · cast", ["from the roots down, enter the", "children that say most of the", "asked words beyond their parent", "D3: a word most siblings say", "draws none of them"], LEAF, ACC3),
        (6.4, 2.3, "6 · read at a level", ["the next word where the path", "ends; when that level is silent,", "read the parent's (D4) —", "the detail the lower level", "could not predict is dropped"], LEAF, ACC3),
        (3.7, 2.3, "7 · ask, be told", ["unknown: said in < 3 scopes", "→ ask a teacher; tell the answer", "as new rows (learn_new); the later", "telling wins among equals", "tellings-to-find is the measure"], PALE, INK),
    ]
    for (x, y, t, l, fc, ec) in steps2:
        box(ax, x, y, 2.5, 1.75, t, l, fc=fc, ec=ec, fs=8.2)
    arrow(ax, (10.35, 4.9), (10.35, 4.05), color=INK, lw=1.6)
    for x in (9.1, 6.4):
        arrow(ax, (x, 3.17), (x - 0.2, 3.17), color=INK, lw=1.6)
    arrow(ax, (3.7, 3.17), (2.25, 3.17), color=INK, lw=1.6)
    arrow(ax, (2.25, 3.17), (2.25, 4.9), color=INK, lw=1.6)
    ax.text(2.2, 2.95, "the answer is a source:\nback to step 1", ha="center", va="top", fontsize=8.2, color=INK)
    # what is measured
    ax.add_patch(FancyBboxPatch((0.3, 0.3), 12.0, 1.55, boxstyle="round,pad=0.02,rounding_size=0.04", fc="white", ec=GREY, lw=1.2, ls="--"))
    ax.text(0.5, 1.65, "measured, per step (the docs that hold the numbers)", fontsize=9, color=GREY, fontweight="bold")
    rows = [
        "one pointing kept over 400 and 2,000 turns: 10/10 · 20/20 · 20/20, both seeds (docs/129)   ·   held-out Emma 5 %: 20.6 %, perplexity 159; heard 77 % / unheard 23 % (docs/137)",
        "phasor cast capacity 11 · 14 · 16 · 18 of 20 vs counts 20/20 (docs/136)   ·   edge count right at one showing, phasor agreement > 0.94 at twelve (docs/141)   ·   2.8 KB a word (docs/133)",
        "operator 9.2 % and kin 14.2 % / 0.5 % on the unheard stratum — neither beats frequency yet (docs/137–138)   ·   two cast changes reverted for breaking the update test (docs/140)",
    ]
    for i, r in enumerate(rows):
        ax.text(0.5, 1.3 - i * 0.38, r, fontsize=7.3, color=INK, va="center")
    save(fig, "learning_steps")


# ------------------------------------------------------------- 4. levels of abstraction
def fig_levels():
    fig, ax = plt.subplots(figsize=(12.6, 6.8)); ax.set_xlim(0, 12.6); ax.set_ylim(0, 6.8); ax.axis("off")
    ax.set_title("Levels of abstraction: each level a model of its own, read beyond the one below it — multi-model by design", fontsize=11.5)
    cols = [
        (0.4, "the sciences (LeCun's ladder)", ["society", "organism", "cell", "molecule", "particle"], GREY, "each level ignores the details of the one\nbelow it and predicts further ahead"),
        (3.45, "prose (phiplm)", ["book", "chapter", "paragraph", "sentence", "occurrence w@path#i"], ACC2, "docs/126–129: the scope tree;\na word is read at the lowest level that has counts"),
        (6.5, "the body (phikit)", ["domain · session", "module · scene", "kind · frame", "thing", "a coefficient / a confidence"], ACC3, "docs/140–141: domain → module → kind → thing;\nwhat is below a thing is not stored"),
        (9.55, "each level, a model (phiecs)", ["its own distribution,\nsmoothed to its parent's (D4)", "ln(p_level / p_parent):\nwhat the level adds", "a word every sibling\nsays draws none (D3)", "a scope is a word:\nrecursion", "aspects and systems\ncompose: modularity"], ACC, "abstraction · modularity · recursion —\nthe dropping is the structure, not a loss"),
    ]
    for (x, title, levels, color, foot) in cols:
        ax.text(x + 1.3, 6.1, title, ha="center", fontsize=9.5, fontweight="bold", color=color)
        for i, l in enumerate(levels):
            y = 5.05 - i * 0.92
            fc = "white" if color == GREY else (COOL if color == ACC2 else (LEAF if color == ACC3 else WARM))
            ax.add_patch(FancyBboxPatch((x, y), 2.6, 0.66, boxstyle="round,pad=0.015", fc=fc, ec=color, lw=1.2))
            ax.text(x + 1.3, y + 0.33, l, ha="center", va="center", fontsize=8.2, color=INK)
            if i < len(levels) - 1:
                arrow(ax, (x + 1.3, y), (x + 1.3, y - 0.26), color=color, lw=1.1)
        ax.text(x + 1.3, 1.2, foot, ha="center", va="top", fontsize=7.4, color=GREY)
    # the matrix note
    ax.add_patch(FancyBboxPatch((0.4, 0.15), 11.75, 0.62, boxstyle="round,pad=0.02", fc="white", ec=GREY, lw=1.0, ls="--"))
    ax.text(6.3, 0.46, "the matrix is count(word, condition): rows (words) differ by spelling, columns (conditions) differ because a child is read only beyond its parent —\n"
            "its information is computed from the counts, not maximised by a loss (guard.md §7)", ha="center", va="center", fontsize=7.8, color=INK)
    save(fig, "abstraction_levels")


# ------------------------------------------------------------- 5. the paradigm: a world model that is not a simulator
def fig_control():
    fig, ax = plt.subplots(figsize=(12.6, 6.6)); ax.set_xlim(0, 12.6); ax.set_ylim(0, 6.6); ax.axis("off")
    ax.set_title("The ECS paradigm: a world model is for prediction and planning in an abstract space — not a simulator", fontsize=11.5)
    # left: what it is not
    ax.add_patch(FancyBboxPatch((0.3, 0.5), 3.3, 5.5, boxstyle="round,pad=0.02,rounding_size=0.04", fc="white", ec=GREY, lw=1.2, ls="--"))
    ax.text(1.95, 5.7, "what a world model is not", ha="center", fontsize=10, fontweight="bold", color=GREY)
    nots = [("a simulator", "every particle, every pixel;\ndiverges with its own detail"), ("a digital twin", "the system copied,\nnot understood"), ("a generative model", "reconstructs Y in detail,\npays for what it cannot know"), ("video generation", "one cute video, not\nall plausible futures")]
    for i, (t, l) in enumerate(nots):
        y = 4.55 - i * 1.15
        ax.add_patch(FancyBboxPatch((0.5, y), 2.9, 0.95, boxstyle="round,pad=0.015", fc="white", ec=GREY, lw=1.0))
        ax.text(1.95, y + 0.68, t, ha="center", va="center", fontsize=9.2, fontweight="bold", color=INK)
        ax.text(1.95, y + 0.3, l, ha="center", va="center", fontsize=7.6, color=INK)
        ax.plot([0.6, 3.3], [y + 0.1, y + 0.85], color=ACC, lw=1.2, alpha=0.55)
    # right: the loop
    cx, cy, R = 8.4, 3.2, 2.45
    nodes = [
        (90, "observe", "a source speaks in words:\na text, a teacher, a device", COOL, ACC2),
        (18, "tell", "rows (about, kind, other)\nunder the conditions now", COOL, ACC2),
        (-54, "state", "the condition path the cast\nends on; the counts there", WARM, ACC),
        (-126, "predict", "calls_for(w, kind): X ⇒ Y;\nread at the lowest level that speaks", WARM, ACC),
        (162, "plan · act", "World::plan: the heaviest chain of\nedges under a kind to the outcome", LEAF, ACC3),
    ]
    pts = []
    for (deg, t, l, fc, ec) in nodes:
        x, y = cx + R * np.cos(np.deg2rad(deg)), cy + R * np.sin(np.deg2rad(deg))
        pts.append((x, y))
        ax.add_patch(FancyBboxPatch((x - 1.25, y - 0.42), 2.5, 0.84, boxstyle="round,pad=0.015", fc=fc, ec=ec, lw=1.3))
        ax.text(x, y + 0.2, t, ha="center", va="center", fontsize=9.5, fontweight="bold", color=INK)
        ax.text(x, y - 0.14, l, ha="center", va="center", fontsize=7.3, color=INK)
    for i in range(5):
        (x1, y1), (x2, y2) = pts[i], pts[(i + 1) % 5]
        dx, dy = x2 - x1, y2 - y1
        n = (dx * dx + dy * dy) ** 0.5
        ax.add_patch(FancyArrowPatch((x1 + dx / n * 1.05, y1 + dy / n * 0.55), (x2 - dx / n * 1.05, y2 - dy / n * 0.55), arrowstyle="-|>", mutation_scale=13, color=INK, lw=1.4, connectionstyle="arc3,rad=-0.18"))
    ax.text(cx, cy + 0.3, "X ⇒ Y under c", ha="center", va="center", fontsize=12, fontweight="bold", color=INK)
    ax.text(cx, cy - 0.2, "entity = word · component = relation\nsystem = a step · condition = a scope", ha="center", va="center", fontsize=8.2, color=INK)
    ax.text(cx, cy - 0.65, "no loss; one row per observation", ha="center", va="center", fontsize=8, color=GREY)
    ax.text(6.3, 0.15, "abstract by construction: the unit is a word under a condition; a phenomenological model is the counts of what followed what (docs/126, 129, 141)", ha="center", fontsize=8, color=GREY)
    save(fig, "paradigm_control")


# ------------------------------------------------------------- 6. SIGReg against D4
def fig_sigreg():
    fig, (al, ar) = plt.subplots(1, 2, figsize=(12.6, 4.6))
    # left: sketched isotropic Gaussian regularisation — empirical CDF vs Gaussian CDF
    rng = np.random.default_rng(3)
    pts = np.sort(np.concatenate([rng.normal(-1.2, 0.35, 9), rng.normal(0.9, 0.5, 11)]))
    xs = np.linspace(-3, 3, 400)
    from math import erf
    gauss = np.array([0.5 * (1 + erf(x / np.sqrt(2))) for x in xs])
    al.step(pts, np.arange(1, len(pts) + 1) / len(pts), where="post", color=ACC2, lw=1.8, label="empirical CDF of the projected batch")
    al.plot(xs, gauss, color=GREY, lw=1.6, ls="--", label="the Gaussian it is pulled toward")
    al.scatter(pts, np.zeros_like(pts) - 0.03, color=ACC2, s=14, zorder=3)
    for p_ in pts[::3]:
        g = 0.5 * (1 + erf(p_ / np.sqrt(2)))
        e = (np.searchsorted(pts, p_, side="right")) / len(pts)
        al.annotate("", xy=(p_ + (0.25 if e > g else -0.25), -0.03), xytext=(p_, -0.03), arrowprops=dict(arrowstyle="-|>", color=ACC, lw=1.2))
    al.set_ylim(-0.16, 1.05); al.set_xlim(-3, 3); al.set_yticks([0, 0.5, 1]); al.set_xticks([])
    al.set_title("SIGReg: a loss moves each sample toward the reference", fontsize=10)
    al.legend(loc="upper left", fontsize=7.5, frameon=False)
    al.text(0, -0.125, "project to one direction · staircase vs Gaussian · a gradient per point", ha="center", fontsize=7.5, color=GREY)
    for sp in ("top", "right"):
        al.spines[sp].set_visible(False)
    # right: D4 — a scope's counts against its parent's distribution, read not pulled
    words = ["smile", "blink", "jaw", "tongue", "brow", "cheek", "nose", "eye"]
    parent = np.array([0.20, 0.15, 0.14, 0.08, 0.13, 0.10, 0.10, 0.10])
    child = np.array([0.42, 0.30, 0.04, 0.02, 0.06, 0.08, 0.03, 0.05])
    x = np.arange(len(words)); w = 0.38
    ar.bar(x - w / 2, parent, w, color=GREY, alpha=0.7, label="parent scope p (the base)")
    ar.bar(x + w / 2, child, w, color=ACC, alpha=0.85, label="this scope p_s (its own model)")
    for i in range(len(words)):
        r = np.log(child[i] / parent[i])
        ar.text(x[i], max(parent[i], child[i]) + 0.015, f"{r:+.1f}", ha="center", fontsize=7.5, color=ACC2 if r > 0 else GREY)
    ar.set_xticks(x); ar.set_xticklabels(words, fontsize=8); ar.set_ylim(0, 0.5); ar.set_yticks([0, 0.25, 0.5])
    ar.set_title("D4: a scope read against its parent — ln(p_s / p) is what it adds", fontsize=10)
    ar.legend(loc="upper right", fontsize=7.5, frameon=False)
    ar.text(3.5, -0.11, "no projection, no pull: the distribution is the counts, the reference is the parent, the information is a number", ha="center", fontsize=7.5, color=GREY)
    for sp in ("top", "right"):
        ar.spines[sp].set_visible(False)
    fig.suptitle("Information: maximised toward a reference (SIGReg) or read against one (the tree) — the scope figures are illustrative; the rule is docs/126's", fontsize=10.5, fontweight="bold")
    fig.tight_layout(rect=(0, 0.02, 1, 0.94))
    save(fig, "paradigm_sigreg")


# ------------------------------------------------------------- 7. the paradigm card
def fig_card():
    fig, ax = plt.subplots(figsize=(12.6, 5.2)); ax.set_xlim(0, 12.6); ax.set_ylim(0, 5.2); ax.axis("off")
    ax.set_title("The Phical paradigm as an entity–component system — X ⇒ Y under c, and its four parts", fontsize=11.5)
    parts = [
        (0.3, "X, Y — entities", ["words, told apart by spelling", "a scope is a word; a kind too", "phiecs Lexicon · Word"], COOL, ACC2),
        (3.4, "⇒ — components", ["a relation: kind, weight, and", "the two phasors it couples", "phiecs Edges · Phasor (64 ch.)"], WARM, ACC),
        (6.5, "under c — conditions", ["the scope path a row was told in;", "nested: each a model, read", "beyond its parent (Counts, ScopeTree)"], LEAF, ACC3),
        (9.6, "the step — systems", ["one observation at a time:", "Ingest · Couple · Readout", "phiecs System · Watch · Quest"], PALE, INK),
    ]
    for (x, t, l, fc, ec) in parts:
        box(ax, x, 3.0, 2.8, 1.8, t, l, fc=fc, ec=ec, fs=8.4)
    for x in (3.1, 6.2, 9.3):
        arrow(ax, (x, 3.9), (x + 0.3, 3.9), color=GREY, lw=1.2)
    claims = [
        ("not a simulator", "no state below a word; what a level cannot\npredict is dropped by D4, not learned away"),
        ("abstract by construction", "the space is the lexicon and the tree, given\nby the source, not found by an encoder"),
        ("learns from one telling", "a row is a fact; the count sharpens it; the later\ntelling wins (docs/129: 20/20 at 2,000 turns)"),
        ("predicts and plans", "calls_for reads X ⇒ Y; World::plan follows the\nheaviest chain under a kind to an outcome (docs/141)"),
        ("information is computed", "count(word, condition) is the distribution; ln(p_s/p)\nwhat a condition adds; nothing is maximised"),
        ("cannot collapse to a constant", "no encoder; the tree's own collapses (prior, depth,\nexplosion, hub, schema, knob) have guards — guard.md"),
    ]
    for i, (t, l) in enumerate(claims):
        col, row = i % 3, i // 3
        x, y = 0.3 + col * 4.15, 1.75 - row * 1.0
        ax.add_patch(FancyBboxPatch((x, y), 3.95, 0.88, boxstyle="round,pad=0.012", fc="white", ec=GREY, lw=0.9))
        ax.text(x + 0.12, y + 0.68, t, fontsize=8.6, fontweight="bold", color=INK, va="center")
        ax.text(x + 0.12, y + 0.3, l, fontsize=7.2, color=INK, va="center")
    ax.text(6.3, 0.05, "the honest limits stay stated: no perception of its own; the unheard stratum (23 %) falls to the prior; the continuous half pays what JEPA pays and reports it (docs/136–138)", ha="center", fontsize=7.6, color=GREY)
    save(fig, "paradigm_card")


# ------------------------------------------------------------- 8. the layer: projections, validators, constraints, operators
def fig_layer():
    fig, ax = plt.subplots(figsize=(12.6, 5.6)); ax.set_xlim(0, 12.6); ax.set_ylim(0, 5.6); ax.axis("off")
    ax.set_title("The layer the representation space is said to lack — heads trained there, rows read here", fontsize=11.5)
    # left: JEPA's answer
    ax.add_patch(FancyBboxPatch((0.3, 0.5), 4.1, 4.5, boxstyle="round,pad=0.02,rounding_size=0.04", fc="white", ec=GREY, lw=1.2, ls="--"))
    ax.text(2.35, 4.7, "in a learned space (the answer given)", ha="center", fontsize=9.5, fontweight="bold", color=GREY)
    box(ax, 0.5, 3.35, 3.7, 1.0, "encoder → representation s", ["a vector; nothing in it is named"], fc="white", ec=GREY, fs=8)
    heads = [("constraint head", "a projection trained per constraint:\n'not the wall', few samples"), ("objective head", "a cost trained per task:\n'is the door open?'"), ("validator", "the same head, thresholded")]
    for i, (t, l) in enumerate(heads):
        y = 2.3 - i * 0.85
        ax.add_patch(FancyBboxPatch((0.5, y), 3.7, 0.72, boxstyle="round,pad=0.012", fc="white", ec=GREY, lw=1.0))
        ax.text(0.65, y + 0.5, t, fontsize=8.6, fontweight="bold", color=INK, va="center")
        ax.text(0.65, y + 0.2, l, fontsize=7.2, color=INK, va="center")
    arrow(ax, (2.35, 3.35), (2.35, 3.02), color=GREY, lw=1.0)
    ax.text(2.5, 3.18, "one head per constraint, per task", fontsize=7, color=GREY, va="center")
    # right: the tree's layer
    ax.add_patch(FancyBboxPatch((4.8, 0.5), 7.5, 4.5, boxstyle="round,pad=0.02,rounding_size=0.04", fc="white", ec=INK, lw=1.4))
    ax.text(8.55, 4.7, "in a space of words (phiecs world/rule.rs — nothing trained)", ha="center", fontsize=9.5, fontweight="bold", color=INK)
    box(ax, 5.0, 3.35, 3.3, 1.0, "the world: rows", ["(about, kind, other); every thing has a name"], fc=COOL, ec=ACC2, fs=8)
    box(ax, 8.6, 3.35, 3.5, 1.0, "told under a condition, by anyone", ["forbid(scene, wall) · want(scene, open)"], fc=LEAF, ec=ACC3, fs=8)
    arrow(ax, (8.6, 3.85), (8.3, 3.85), color=ACC3, lw=1.3)
    cells = [
        ("constraint", "rule(scene).avoid — the words never\nto pass through, read back by name", WARM, ACC),
        ("objective", "rule(scene).reach — the words to reach;\nplan_in(scene, from, kind, depth)", WARM, ACC),
        ("validator", "holds(a, kind, b): does the world say it?\nkeeps(path, rule): does a plan obey?", LEAF, ACC3),
        ("projection", "degree(a, kind, b): how much it says it —\nthe edge weight; plan_weight along a plan", LEAF, ACC3),
        ("operator", "phical's Fno over word functions: the one\ntrained head, kept beside the counts (docs/137)", PALE, GREY),
        ("what stays upstream", "that a place is 'wall' is perception's to say;\ndownstream it is one word (paper 26 §10–11)", "white", GREY),
    ]
    for i, (t, l, fc, ec) in enumerate(cells):
        col, row = i % 2, i // 2
        x, y = 5.0 + col * 3.65, 2.3 - row * 0.85
        ax.add_patch(FancyBboxPatch((x, y), 3.45, 0.72, boxstyle="round,pad=0.012", fc=fc, ec=ec, lw=1.0))
        ax.text(x + 0.12, y + 0.5, t, fontsize=8.6, fontweight="bold", color=INK, va="center")
        ax.text(x + 0.12, y + 0.2, l, fontsize=7.1, color=INK, va="center")
    arrow(ax, (6.65, 3.35), (6.65, 3.02), color=ACC2, lw=1.2); arrow(ax, (10.3, 3.35), (10.3, 3.02), color=ACC2, lw=1.2)
    ax.text(6.3, 0.15, "measured (phiecs rule.rs test): reach `open`, never the wall ⇒ hall → door → open, weight 4; forbid the door too ⇒ no plan, honestly", ha="center", fontsize=8, color=GREY)
    save(fig, "paradigm_layer")


if __name__ == "__main__":
    fig_phiecs(); fig_phinet(); fig_steps(); fig_levels(); fig_control(); fig_sigreg(); fig_card(); fig_layer()
    print("written to", OUT)
