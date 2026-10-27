#[allow(unused_imports)]
use super::*;

impl Vertex {
    pub fn new(point: ColorSpaceTimePoint, word_hash: u64) -> Self {
        Self { point, word_hash }
    }
}

impl Edge {
    pub fn new(from: Vertex, to: Vertex) -> Self {
        let length = ColorSpaceTimeManifold::geodesic(&from.point, &to.point);
        let qa = LightQuantum::from_phase(from.point.effective_phase, 1.0, from.point.band_n);
        let qb = LightQuantum::from_phase(to.point.effective_phase, 1.0, to.point.band_n);
        let ca = Chiton::from_quantum(&qa);
        let cb = Chiton::from_quantum(&qb);
        let coupling = ca.resonance(&cb);
        Self { from, to, length, coupling }
    }
}

impl Path {
    pub fn from_edges(edges: Vec<Edge>) -> Self {
        let total_length: f64 = edges.iter().map(|e| e.length).sum();
        let coherence: f64 = if edges.is_empty() {
            0.0
        } else {
            edges.iter().map(|e| e.coupling).sum::<f64>() / edges.len() as f64
        };
        let winding = Self::compute_winding(&edges);

        Self { edges, total_length, coherence, winding }
    }

    pub(super) fn compute_winding(edges: &[Edge]) -> f64 {
        let mut total_phase: f64 = 0.0;
        for e in edges {
            let mut delta = e.to.point.effective_phase - e.from.point.effective_phase;
            if delta > PI { delta -= TWO_PI; }
            if delta < -PI { delta += TWO_PI; }
            total_phase += delta;
        }
        total_phase / TWO_PI
    }

    /// Returns the number of edges (sentence length in word transitions).
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Returns true if the path has no edges.
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

impl Surface {
    pub fn from_paths(paths: Vec<Path>) -> Self {
        let area: f64 = paths.iter().map(|p| p.total_length).sum();
        let adjacency = Self::find_adjacencies(&paths);
        let genus = Self::estimate_genus(&paths, &adjacency);
        Self { paths, adjacency, area, genus }
    }

    pub(super) fn find_adjacencies(paths: &[Path]) -> Vec<(usize, usize)> {
        let mut adj = Vec::new();
        for i in 0..paths.len() {
            for j in (i + 1)..paths.len() {
                if Self::paths_share_vertex(&paths[i], &paths[j]) {
                    adj.push((i, j));
                }
            }
        }
        adj
    }

    pub(super) fn paths_share_vertex(a: &Path, b: &Path) -> bool {
        for ea in &a.edges {
            for eb in &b.edges {
                if ea.from.word_hash == eb.from.word_hash
                    || ea.from.word_hash == eb.to.word_hash
                    || ea.to.word_hash == eb.from.word_hash
                    || ea.to.word_hash == eb.to.word_hash
                {
                    return true;
                }
            }
        }
        false
    }

    pub(super) fn estimate_genus(paths: &[Path], adj: &[(usize, usize)]) -> u32 {
        let v = paths.len();
        let e = adj.len();
        let f = 1;
        let euler = v as i32 - e as i32 - f;
        (2 - euler).max(0) as u32
    }
}

impl Region {
    pub fn from_vertices(vertices: Vec<Vertex>) -> Self {
        if vertices.is_empty() {
            return Self { vertices, centroid_phase: 0.0, spread: 0.0, density: 0.0 };
        }

        let n = vertices.len() as f64;
        let sum_x: f64 = vertices.iter().map(|v| v.point.effective_phase.cos()).sum();
        let sum_y: f64 = vertices.iter().map(|v| v.point.effective_phase.sin()).sum();
        let centroid_phase = sum_y.atan2(sum_x).rem_euclid(TWO_PI);

        let mut spread = 0.0;
        for v in &vertices {
            let mut delta = (v.point.effective_phase - centroid_phase).abs();
            if delta > PI { delta = TWO_PI - delta; }
            spread += delta;
        }
        spread /= n;

        let density = if spread > 0.0 { 1.0 / (1.0 + spread * PHI) } else { 1.0 };

        Self { vertices, centroid_phase, spread, density }
    }

    /// Returns the number of vertices in this concept region.
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Returns true if the region has no vertices.
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}
