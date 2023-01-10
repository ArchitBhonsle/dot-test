use rand::Rng;

fn generate_vector(size: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen::<f64>()).collect::<Vec<f64>>()
}

fn dot(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(ai, bi)| *ai * *bi)
        .reduce(|acc, e| acc + e)
        .expect("The vectors are empty")
}

fn bench_single(size: usize) -> u128 {
    let a = generate_vector(size);
    let b = generate_vector(size);

    let start = std::time::SystemTime::now();
    dot(&a, &b);
    let end = std::time::SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    duration.as_nanos()
}

fn bench(size: usize, iters: usize) -> f64 {
    (0..iters).map(|_| bench_single(size) as f64).sum::<f64>() / iters as f64
}

fn main() {
    let iters = usize::from_str_radix(&std::env::args().nth(1).expect("Needs one argument"), 10)
        .expect("Could not parse the argument as an unsigned integer");

    (0..6).for_each(|e| {
        println!(
            "size: {:<10} time: {:.3} μs",
            10_usize.pow(e),
            bench(10_usize.pow(e), iters),
        )
    });
}
