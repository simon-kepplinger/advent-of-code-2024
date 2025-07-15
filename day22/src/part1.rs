use aoc_core::{end_measure, read, start_measure};

struct SecretGen {
    secret: i64,
}

impl SecretGen {
    fn new(initial: i64) -> Self {
        SecretGen { secret: initial }
    }

    fn next(&mut self) -> i64 {
        self.mix(self.secret * 64);
        self.prune();

        self.mix(self.secret / 32);
        self.prune();

        self.mix(self.secret * 2048);
        self.prune();

        self.secret
    }

    fn mix(&mut self, value: i64) {
        self.secret = value ^ self.secret;
    }

    fn prune(&mut self) {
        self.secret = self.secret % 16777216;
    }
}

fn main() {
    let mes = start_measure();
    let input = read("in/input");

    let gens = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .map(|n| SecretGen::new(n));

    let sum: i64 = gens
        .map(|mut g| {
            for _ in 0..1999 {
                g.next();
            }

            g.next()
        })
        .sum();

    println!("{sum}");
    println!("");

    end_measure(mes);
}
