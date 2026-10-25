"""phinet in PyTorch — every phinet operation as a torch function, with the
same shapes, conventions and gradients, checked against `reference.json`
(written by `cargo run --release --example torch_reference`).

Conventions that must match, and do:
  * A tensor is complex128 of shape [batch, width, space].
  * The FFT is unitary in both directions (`norm="ortho"`).
  * GELU is the tanh approximation, applied to the real and imaginary parts
    separately.
  * Gradients are torch's conjugate-Wirtinger derivative (`dL/dRe + i dL/dIm`),
    which is exactly what phinet's tape accumulates (`Mul`: g · conj(b)).
  * Adam is applied to real and imaginary parts as two independent real
    parameters — phinet's `Adam` and `torch.optim.Adam` on a complex parameter
    agree, because torch also treats a complex parameter as its real view.

Run:  python phinet_torch.py [reference.json]
"""
import json
import math
import sys

import torch

torch.set_default_dtype(torch.float64)
C = torch.complex128


# ── conversions ──────────────────────────────────────────────────────────────

def from_json(t):
    """[re, im] pairs (+ shape) → complex tensor."""
    if isinstance(t, dict):
        data = torch.tensor(t["data"], dtype=torch.float64)
        return torch.complex(data[:, 0], data[:, 1]).reshape(t["shape"])
    data = torch.tensor(t, dtype=torch.float64)
    return torch.complex(data[:, 0], data[:, 1])


# ── the operations (tensor.rs) ───────────────────────────────────────────────

def fft(x):            # Tape::fft — unitary forward FFT along the sample axis
    return torch.fft.fft(x, dim=-1, norm="ortho")


def ifft(x):           # Tape::ifft
    return torch.fft.ifft(x, dim=-1, norm="ortho")


def fft2(x, s1, s2):   # Tape::fft2 — the sample axis read as an s1×s2 torus, unitary
    b, w, s = x.shape
    return torch.fft.fft2(x.reshape(b, w, s1, s2), norm="ortho").reshape(b, w, s)


def ifft2(x, s1, s2):
    b, w, s = x.shape
    return torch.fft.ifft2(x.reshape(b, w, s1, s2), norm="ortho").reshape(b, w, s)


def gelu(x):           # Tape::gelu — tanh GELU on re and im separately
    g = lambda r: 0.5 * r * (1.0 + torch.tanh(0.7978845608 * (r + 0.044715 * r ** 3)))
    return torch.complex(g(x.real), g(x.imag))


def linear(x, w):      # Tape::linear — [b,i,s] × [i,o,1] → [b,o,s]
    return torch.einsum("bis,io->bos", x, w[:, :, 0])


def add_bias(x, b):    # Tape::add_bias — b is [1,w,1]
    return x + b


def mode_mix(x, r):    # Tape::mode_mix — one [i,o] complex matrix per mode: [b,i,m] × [i,o,m] → [b,o,m]
    return torch.einsum("bim,iom->bom", x, r)


def unit(x):           # Tape::unit — every [·,·,s] row to unit energy (zero rows stay zero)
    e = (x.abs() ** 2).sum(-1, keepdim=True)
    return torch.where(e > 1e-24, x / torch.sqrt(e.clamp_min(1e-300)), x)


def cosine(p, t):      # Tape::cosine — real cosine per batch row, [b,1,1]
    dot = (p * t.conj()).real.sum(-1, keepdim=True)
    d = (p.abs().pow(2).sum(-1, keepdim=True).sqrt() * t.abs().pow(2).sum(-1, keepdim=True).sqrt()).clamp_min(1e-12)
    return dot / d


def cosine_table(p, table):  # Tape::cosine_table — [b,1,s] vs [V,1,s] → [b,V,1] real
    pn = p[:, 0].abs().pow(2).sum(-1).sqrt().clamp_min(1e-12)          # [b]
    tn = table[:, 0].abs().pow(2).sum(-1).sqrt().clamp_min(1e-12)      # [V]
    dot = torch.einsum("bs,vs->bv", p[:, 0], table[:, 0].conj()).real
    return (dot / (pn[:, None] * tn[None, :]))[:, :, None]


def gather(table, idx):      # Tape::gather — rows of a [V,1,s] table
    return table[list(idx)]


def hinge(pos, neg, margin):  # Tape::hinge — mean_b max(0, margin − pos + neg)
    return torch.clamp(margin - pos + neg, min=0.0).mean()


def mean_real(x):
    return x.real.mean()


def mobius(z, abcd):         # Tape::mobius — (a z + b)/(c z + d) on every sample, abcd is [1,1,4]
    a, b, c, d = abcd[0, 0, 0], abcd[0, 0, 1], abcd[0, 0, 2], abcd[0, 0, 3]
    return (a * z + b) / (c * z + d)


def mixture_nll(scores, terms):  # Tape::mixture_nll — the mixed-LM likelihood of the true word
    s = scores[:, :, 0].real * terms["beta"]
    p_t = torch.softmax(s, dim=-1)[torch.arange(s.shape[0]), torch.tensor(terms["targets"])]
    k, coef, other = (torch.tensor(terms[n]) for n in ("k", "coef", "other"))
    p = k + coef * (terms["gamma"] * p_t + (1.0 - terms["gamma"]) * other)
    return -(p.clamp_min(1e-12).log()).mean()


def cross_entropy(scores, targets, beta):  # Tape::cross_entropy — mean −log softmax(β s)[t]
    s = scores[:, :, 0].real * beta
    return torch.nn.functional.cross_entropy(s, torch.tensor(targets))


# ── phasor.rs — resolutions and quanta ───────────────────────────────────────

def resample(f, m):
    """phasor::resample — spectral zero-pad / truncate; sample values interpolated."""
    n = f.shape[-1]
    if n == m:
        return f.clone()
    spec = fft(f)
    out = torch.zeros(m, dtype=C)
    keep = min(n, m) // 2
    for k in range(keep + 1):
        if k < m and k < n:
            out[k] = spec[k]
        if 0 < k < keep:
            out[m - k] = spec[n - k]
    out[keep] = spec[keep] if n < m else spec[keep] + spec[n - keep]
    return ifft(out * math.sqrt(m / n))


def quantise(f, sectors):
    """phasor::quantise — snap every sample's phase to its sector centre."""
    ph = torch.angle(f) % (2 * math.pi)
    sec = torch.floor(ph / (2 * math.pi) * sectors) % sectors
    return torch.polar(f.abs(), (sec + 0.5) / sectors * 2 * math.pi)


# ── hrr.rs ───────────────────────────────────────────────────────────────────

def bind(f, r):       return f * r
def unbind(f, r):     return f * r.conj()
def compose(parts):
    s = torch.stack(parts).sum(0)
    e = (s.abs() ** 2).sum()
    return s / e.sqrt() if e >= 1e-24 else s


# ── hexagram.rs — the sixty-four spectral gates ──────────────────────────────

def octave(mode, space):
    """Octave of a mode: floor(log2 m) clamped to 5; negative frequencies mirror."""
    if mode == 0:
        return None
    if mode > space // 2:
        mode = space - mode
    return min(int(math.floor(math.log2(mode))), 5)


def gate(hid, space):
    return [True if m == 0 else bool((hid >> octave(m, space)) & 1) for m in range(space)]


def hexagram_apply(f, hid):
    g = torch.tensor(gate(hid, f.shape[-1]), dtype=torch.bool)
    return ifft(torch.where(g, fft(f), torch.zeros_like(f)))


def hexagram_of(f):
    c = f.sum()
    ph = math.atan2(c.imag.item(), c.real.item()) % (2 * math.pi)
    return int(math.floor(ph / (2 * math.pi) * 64)) % 64


def lines(f):
    F = fft(f)
    e = F.abs() ** 2
    out = [0.0] * 6
    for m in range(1, f.shape[-1]):          # mode 0 (the mean) has no line
        out[octave(m, f.shape[-1])] += e[m].item()
    tot = max(sum(out), 1e-300)
    return [v / tot for v in out]


def novelty(f, hid):
    h = hexagram_apply(f, hid)
    return 1.0 - cosine(f[None, None], h[None, None]).item()


def changing_lines(a, b):
    return a ^ b


# ── operator.rs — the Fourier neural operator as an nn.Module ────────────────

def zeta_weights(modes, s):
    norm = sum(m ** (-s) for m in range(1, 200_001)) if s > 1.0 else 1.0
    return [1.0 if m == 0 else m ** (-s) / norm for m in range(modes)]


class Fno(torch.nn.Module):
    """h ← σ(F⁻¹(R·F h) + W h + b), lift and projection; identical parameter
    layout to phinet's `Fno` so that its saved parameters load 1:1."""

    def __init__(self, space, width, layers, modes):
        super().__init__()
        self.space, self.width, self.modes = space, width, modes
        P = lambda *shape: torch.nn.Parameter(torch.zeros(*shape, dtype=C))
        self.lift, self.lift_b = P(1, width, 1), P(1, width, 1)
        self.r = torch.nn.ParameterList([P(width, width, space) for _ in range(layers)])
        self.w = torch.nn.ParameterList([P(width, width, 1) for _ in range(layers)])
        self.b = torch.nn.ParameterList([P(1, width, 1) for _ in range(layers)])
        self.proj = P(width, 1, 1)
        self.register_buffer("mask", torch.tensor([m < modes or m + modes >= space for m in range(space)]))

    def load_phinet(self, params):
        with torch.no_grad():
            self.lift.copy_(from_json(params["lift"])); self.lift_b.copy_(from_json(params["lift_b"]))
            for l in range(len(self.r)):
                self.r[l].copy_(from_json(params[f"l{l}_r"]))
                self.w[l].copy_(from_json(params[f"l{l}_w"]))
                self.b[l].copy_(from_json(params[f"l{l}_b"]))
            self.proj.copy_(from_json(params["proj"]))

    def forward(self, x):
        h = gelu(add_bias(linear(x, self.lift), self.lift_b))
        n = len(self.r)
        for i in range(n):
            s = add_bias(linear(h, self.w[i]), self.b[i])
            if self.modes > 0:
                s = s + ifft(mode_mix(fft(h), self.r[i]))
            h = gelu(s) if i + 1 < n else s
        return linear(h, self.proj)

    def mask_modes(self):
        with torch.no_grad():
            for r in self.r:
                r.mul_(self.mask)

    def parameter_count(self):
        return sum(p.numel() for p in self.parameters())


# ── the check ────────────────────────────────────────────────────────────────

def check(path):
    R = json.load(open(path))
    space = R["space"]
    fails = 0

    def eq(name, got, want, tol=1e-9):
        nonlocal fails
        got = torch.as_tensor(got) if not torch.is_tensor(got) else got
        want = torch.as_tensor(want) if not torch.is_tensor(want) else want
        if got.is_complex() or want.is_complex():
            got, want = got.to(C), want.to(C)
        else:
            got, want = got.double(), want.double()
        err = (got.reshape(-1) - want.reshape(-1)).abs().max().item() if got.numel() else 0.0
        ok = err <= tol
        fails += 0 if ok else 1
        print(f"{'ok ' if ok else 'FAIL'}  {name:<28} max|Δ| = {err:.2e}")

    x = from_json(R["x"])
    x0 = x[0, 0]
    eq("fft", fft(x0), from_json(R["fft_x0"]))
    eq("ifft∘fft = id", ifft(fft(x0)), from_json(R["ifft_fft_x0"]))

    # the tape, replayed with autograd
    xv = x.clone().requires_grad_(True)
    w = from_json(R["lift_w"]).requires_grad_(True)
    b = from_json(R["bias"]).requires_grad_(True)
    r = from_json(R["r"]).requires_grad_(True)
    pw = from_json(R["proj_w"]).requires_grad_(True)
    h = linear(xv, w);              eq("linear", h, from_json(R["linear"]))
    hb = add_bias(h, b);            eq("add_bias", hb, from_json(R["add_bias"]))
    hg = gelu(hb);                  eq("gelu", hg, from_json(R["gelu"]))
    hf = fft(hg);                   eq("fft (tape)", hf, from_json(R["fft"]))
    hm = mode_mix(hf, r);           eq("mode_mix", hm, from_json(R["mode_mix"]))
    hi = ifft(hm);                  eq("ifft (tape)", hi, from_json(R["ifft"]))
    y = linear(hi, pw);             eq("proj", y, from_json(R["proj"]))
    yu = unit(y);                   eq("unit", yu, from_json(R["unit"]))
    table = from_json(R["table"])
    eq("cosine_table", cosine_table(yu, table), from_json(R["cosine_table"]))
    cp = cosine(yu, gather(table, [1, 3])); eq("cosine (pos)", cp, from_json(R["cosine_pos"]))
    cn = cosine(yu, gather(table, [0, 2])); eq("cosine (neg)", cn, from_json(R["cosine_neg"]))
    loss = hinge(cp, cn, 0.2);      eq("hinge", loss, R["hinge"])
    loss.backward()
    eq("∂/∂lift_w", w.grad, from_json(R["grad_lift_w"]))
    eq("∂/∂r", r.grad, from_json(R["grad_r"]))
    eq("∂/∂proj_w", pw.grad, from_json(R["grad_proj_w"]))
    eq("∂/∂bias", b.grad, from_json(R["grad_bias"]))
    eq("∂/∂x", xv.grad, from_json(R["grad_x"]))

    sv = from_json(R["cosine_table"]).requires_grad_(True)
    nll = mixture_nll(sv, R["mixture_terms"]); eq("mixture_nll", nll, R["mixture_nll"])
    nll.backward();                  eq("∂mixture/∂scores", sv.grad, from_json(R["grad_mixture_scores"]))

    sv2 = from_json(R["cosine_table"]).requires_grad_(True)
    ce = cross_entropy(sv2, [1, 3], 6.0); eq("cross_entropy", ce, R["cross_entropy"])
    ce.backward();                   eq("∂cross_entropy/∂scores", sv2.grad, from_json(R["grad_cross_entropy_scores"]))

    pw = torch.nn.Parameter(from_json(R["adamw_param"]))
    optw = torch.optim.AdamW([pw], lr=0.05, betas=(0.9, 0.999), eps=1e-8, weight_decay=0.1)
    optw.zero_grad(); mean_real(pw).backward(); optw.step()
    eq("AdamW step", pw.detach(), from_json(R["adamw_after"]))

    x6 = x[0:1].clone().requires_grad_(True)
    f2 = fft2(x6, 4, 4);              eq("fft2 4×4", f2, from_json(R["fft2_x0_4x4"]))
    mean_real(f2).backward();        eq("∂fft2/∂x", x6.grad, from_json(R["grad_fft2_x"]))
    eq("resample 16→32", resample(x0, 32), from_json(R["resample_x0_to_32"]))
    eq("quantise Q16", quantise(x0, 16), from_json(R["quantise_x0_q16"]))

    zv = x.clone()
    abcd = from_json(R["mobius_abcd"]).requires_grad_(True)
    mob = mobius(zv, abcd);         eq("mobius", mob, from_json(R["mobius"]))
    mr = mean_real(mob);            eq("mobius mean_real", mr, R["mobius_mean_real"])
    mr.backward();                  eq("∂mobius/∂abcd", abcd.grad, from_json(R["grad_mobius_abcd"]))

    role = from_json(R["role"])
    bound = bind(x0, role);         eq("bind", bound, from_json(R["bind"]))
    eq("unbind", unbind(bound, role), from_json(R["unbind"]))

    hid = R["hexagram_id"]
    assert gate(hid, space) == R["hexagram_gate"], "hexagram gate"
    print("ok   hexagram_gate")
    eq("hexagram_apply", hexagram_apply(x0, hid), from_json(R["hexagram_apply"]))
    assert hexagram_of(x0) == R["hexagram_of_x0"], "hexagram_of"
    print("ok   hexagram_of")
    eq("hexagram lines", torch.tensor(lines(x0)), torch.tensor(R["hexagram_lines_x0"]))
    eq("hexagram novelty", torch.tensor(novelty(x0, hid)), torch.tensor(R["hexagram_novelty"]))
    assert changing_lines(63, hid) == R["changing_lines_63_to_hx"]
    print("ok   changing_lines")
    eq("zeta_weights", torch.tensor(zeta_weights(8, 2.0)), torch.tensor(R["zeta_weights_8_s2"]), tol=1e-6)
    eq("cosine (fn)", cosine(x[0:1], x[1:2]), torch.tensor([[[R["cosine_x0_x1"]]]]))

    # the whole operator: forward, gradients, one Adam step
    cfg = R["fno_config"]
    fno = Fno(cfg["space"], cfg["width"], cfg["layers"], cfg["modes"])
    fno.load_phinet(R["fno_params"])
    assert fno.parameter_count() == R["fno_parameter_count"], "parameter count"
    print(f"ok   fno parameter_count = {fno.parameter_count()}")
    ys = fno(x)
    want = torch.stack([from_json(f) for f in R["fno_apply"]])[:, None, :]
    eq("Fno::apply", ys, want)
    opt = torch.optim.Adam(fno.parameters(), lr=cfg["lr"], betas=(0.9, 0.999), eps=1e-8)
    opt.zero_grad()
    pred = fno(x)
    l = hinge(cosine(pred, gather(table, [1, 3])), cosine(pred, gather(table, [0, 2])), cfg["margin"])
    eq("Fno step loss", l, R["fno_step_loss"])
    l.backward()
    names = ["lift", "lift_b", "l0_r", "l0_w", "l0_b", "l1_r", "l1_w", "l1_b", "proj"]
    plist = [fno.lift, fno.lift_b, fno.r[0], fno.w[0], fno.b[0], fno.r[1], fno.w[1], fno.b[1], fno.proj]
    for n, p in zip(names, plist):
        eq(f"∂step/∂{n}", p.grad, from_json(R["fno_step_grads"][n]))
    opt.step()
    fno.mask_modes()
    for n, p in zip(names, plist):
        eq(f"adam {n}", p.detach(), from_json(R["fno_after_adam"][n]), tol=1e-7)

    print("\nALL PASS" if fails == 0 else f"\n{fails} FAILED")
    return fails


if __name__ == "__main__":
    sys.exit(check(sys.argv[1] if len(sys.argv) > 1 else "reference.json"))
