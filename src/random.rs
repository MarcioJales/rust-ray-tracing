use rand::Rng;

pub fn random() -> f64 {
        /* Rand ref:
    ** https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-numbers-within-a-range
    */
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}