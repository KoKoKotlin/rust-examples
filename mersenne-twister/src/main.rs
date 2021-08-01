
struct MersenneTwister {
    seed: u64,
}

impl MersenneTwister {
    fn new(seed: u64) -> Self {
        MersenneTwister {
            seed: seed,
        }
    }

    fn next_bytes(& mut self) -> u64 {
        
    }
}

fn main() {
    println!("Hello, world!");

    let mersenne_twister = MersenneTwister::new(0u64);
}
