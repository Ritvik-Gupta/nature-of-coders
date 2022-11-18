use noise::{NoiseFn, Perlin};

fn main() {
    println!("{}", -10.2 % 2.0);
    let perlin = Perlin::new(Perlin::DEFAULT_SEED);

    for i in 0..10 {
        for j in 0..10 {
            let perlin_noise = perlin.get([(i as f64) / 10.0, (j as f64) / 10.0]);
            print!("{perlin_noise:.2?}\t");
        }
        println!();
    }

    for i in 0..10u32.pow(3) {
        for j in 0..10u32.pow(3) {
            let perlin_noise = perlin.get([(i as f64) / 1000.0, (j as f64) / 1000.0]);
            assert!(-1.0 <= perlin_noise && perlin_noise <= 1.0)
        }
    }
    println!("yes");
}
