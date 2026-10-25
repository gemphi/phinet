use phistd::{Tape, Tensor};
fn main() {
    let (b, v, s) = (20, 6221, 64);
    let mut tape = Tape::new();
    let p = tape.param(Tensor::randn([b, 1, s], 1.0, 1));
    let t = tape.param(Tensor::randn([v, 1, s], 1.0, 2));
    let t0 = std::time::Instant::now();
    for _ in 0..20 {
        tape.truncate(2);
        let c = tape.cosine_table(p, t);
        let l = tape.mean_real(c);
        tape.backward(l);
    }
    println!("{} threads: {:.1} ms per forward+backward", rayon::current_num_threads(), t0.elapsed().as_secs_f64() * 1000.0 / 20.0);
}
