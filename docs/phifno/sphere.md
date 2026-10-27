# sphere

## Files

```mermaid
graph TD
    mod["sphere/mod.rs<br/>pub use basis · grid · transform<br/>degree_gate · degree_energy"]
    basis["basis.rs<br/>DEGREES = 8 · HARMONICS = DEGREES²<br/>index_of · degree_of · order_of<br/>legendre · factorial_ratio · real_harmonic"]
    grid["grid.rs<br/>gauss_legendre(n) · Grid"]
    transform["transform.rs<br/>Sphere · minimal · new<br/>synthesis · analysis · rotate_about_pole"]
    tests["tests.rs<br/>5 test functions"]
    mod --> basis & grid & transform & tests
    mod -->|"degree_of · DEGREES · HARMONICS"| basis
    transform -->|"real_harmonic · HARMONICS<br/>degree_of · index_of · order_of"| basis
    transform -->|"Grid"| grid
    LIB["lib.rs · Word"] -.-> mod
    OP["operator · FnoConfig::sphere · tests/sphere"] -.-> mod
    EX["examples/dump.rs"] -.-> transform
```

## Channel k ↔ (l, m) — k = l² + l + m

```mermaid
block-beta
    columns 1
    l0["l = 0 · k 0 · 1 order"]
    l1["l = 1 · k 1 – 3 · 3 orders"]
    l2["l = 2 · k 4 – 8 · 5 orders"]
    l3["l = 3 · k 9 – 15 · 7 orders"]
    l4["l = 4 · k 16 – 24 · 9 orders"]
    l5["l = 5 · k 25 – 35 · 11 orders"]
    l6["l = 6 · k 36 – 48 · 13 orders"]
    l7["l = 7 · k 49 – 63 · 15 orders"]
```

## One harmonic at one point — real_harmonic(k, θ, φ)

```mermaid
flowchart LR
    K["k"] --> L["degree_of · l = floor of √k"]
    K --> M["order_of · m = k − l² − l"]
    L & M --> N["norm = √((2l+1)/4π · factorial_ratio(l, abs m))"]
    TH["θ"] --> P["legendre(l, abs m, cos θ)<br/>no Condon–Shortley phase"]
    L & M --> P
    N & P --> S{"m"}
    S -->|"0"| A["norm · P"]
    S -->|"#gt; 0"| B["√2 · norm · P · cos(m φ)"]
    S -->|"#lt; 0"| C["√2 · norm · P · sin(abs m · φ)"]
```

## The grid — Grid::new(nlat, nlon)

```mermaid
flowchart LR
    GL["gauss_legendre(nlat)<br/>Newton on the Legendre polynomial<br/>up to 100 steps, stop at 1e-15"] --> X["nodes x · weights w"]
    X --> TH["theta = acos x"]
    X --> W["weight = w · 2π / nlon"]
    NL["nlon"] --> PH["phi = 2π j / nlon"]
    TH & W & PH --> G["Grid · len = nlat · nlon<br/>at(i) = theta of row, phi of column, weight of row"]
```

## The transform pair

```mermaid
flowchart LR
    NEW["Sphere::new(nlat, nlon)<br/>synth table: Y_k at every sample, for every k"] -.-> SY & AN
    MIN["Sphere::minimal = new(8, 16)<br/>exact when nlat ≥ 8 and nlon ≥ 16"] -.-> NEW
    C["64 coefficients · c64"] --> SY["synthesis<br/>f(x_i) = Σ_k c_k Y_k(x_i)"]
    SY --> F["values on nlat × nlon"]
    F --> AN["analysis<br/>c_k = Σ_i w_i f(x_i) Y_k(x_i)"]
    AN --> C2["64 coefficients"]
```

## Rotation about the pole — rotate_about_pole(coeffs, α)

```mermaid
flowchart LR
    K["each k"] --> Z{"m = 0"}
    Z -->|"yes"| SAME["coefficient kept"]
    Z -->|"no"| PAIR["kc = index_of(l, abs m) · ks = index_of(l, −abs m)<br/>a = c at kc · b = c at ks"]
    PAIR --> ROT["out at kc = a cos mα + b sin mα<br/>out at ks = b cos mα − a sin mα"]
```

## Resolution and scale

```mermaid
flowchart LR
    C["coeffs"] --> DG["degree_gate(coeffs, max_degree)<br/>zero every k with degree_of(k) #gt; max_degree"]
    C --> DE["degree_energy(coeffs)<br/>norm_sqr summed per degree · first HARMONICS only"]
    DG --> G["Vec of c64 · degrees ≤ max_degree"]
    DE --> E["f64 per degree · DEGREES of them"]
```

## The types

```mermaid
classDiagram
    class Grid {
        +usize nlat
        +usize nlon
        +Vec of f64 theta
        +Vec of f64 phi
        +Vec of f64 weight
        +new(nlat, nlon) Grid
        +len() usize
        +is_empty() bool
        +at(i) theta phi weight
    }
    class Sphere {
        +Grid grid
        synth Vec of f64
        +minimal() Sphere
        +new(nlat, nlon) Sphere
        +synthesis(coeffs) Vec of c64
        +analysis(values) Vec of c64
        +rotate_about_pole(coeffs, angle) Vec of c64
    }
    Sphere *-- Grid
```

## The tests

```mermaid
graph LR
    T["sphere/tests.rs"] --> T1["indexing_packs_sixty_four"]
    T --> T2["harmonics_are_orthonormal_on_the_grid"]
    T --> T3["analysis_inverts_synthesis"]
    T --> T4["rotation_about_the_pole_matches_a_shifted_grid"]
    T --> T5["degree_gate_is_a_projection"]
    P["examples/sphere_parity.py<br/>parity of real_harmonic against scipy"] -.-> D["examples/dump.rs · TSV of the 64 harmonics on Sphere::minimal"]
```
