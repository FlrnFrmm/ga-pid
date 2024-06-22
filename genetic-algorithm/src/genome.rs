use rand::prelude::*;

#[derive(Debug)]
pub struct Genome {
    pub p: f32,
    pub i: f32,
    pub d: f32,
}

impl Genome {
    pub fn new(p: f32, i: f32, d: f32) -> Genome {
        Genome { p, i, d }
    }

    pub fn cross(&self, other: &Genome) -> Genome {
        let (mut p, mut i, mut d) = (0., 0., 0.);
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=100) {
            50..=100 => {
                p = (self.p + other.p) / 2.0;
                i = (self.i + other.i) / 2.0;
                d = (self.d + other.d) / 2.0;
            }
            49 => {
                let offset = 0.3;
                p = (self.p + other.p) / 2.0;
                if rng.gen_bool(0.5) {
                    p = p + rng.gen_range(-offset..=offset) * p;
                }
                i = (self.i + other.i) / 2.0;
                if rng.gen_bool(0.5) {
                    i = i + rng.gen_range(-offset..=offset) * i;
                }
                d = (self.d + other.d) / 2.0;
                if rng.gen_bool(0.5) {
                    d = d + rng.gen_range(-offset..=offset) * d;
                }
            }
            41..=48 => {
                p = (self.p + other.p) / 2.8;
                i = (self.i + other.i) / 2.8;
                d = (self.d + other.d) / 2.8;
            }
            33..=40 => {
                p = (self.p + other.p) / 2.5;
                i = (self.i + other.i) / 2.5;
                d = (self.d + other.d) / 2.5;
            }
            25..=32 => {
                p = (self.p + other.p) / 2.3;
                i = (self.i + other.i) / 2.3;
                d = (self.d + other.d) / 2.3;
            }
            17..=24 => {
                p = (self.p + other.p) / 1.7;
                i = (self.i + other.i) / 1.7;
                d = (self.d + other.d) / 1.7;
            }
            9..=16 => {
                p = (self.p + other.p) / 1.5;
                i = (self.i + other.i) / 1.5;
                d = (self.d + other.d) / 1.5;
            }
            1..=8 => {
                p = (self.p + other.p) / 1.2;
                i = (self.i + other.i) / 1.2;
                d = (self.d + other.d) / 1.2;
            }
            0 => {
                let offset = 0.8;
                p = (self.p + other.p) / 2.0;
                if rng.gen_bool(0.5) {
                    p = p + rng.gen_range(-offset..=offset) * p;
                }
                i = (self.i + other.i) / 2.0;
                if rng.gen_bool(0.5) {
                    i = i + rng.gen_range(-offset..=offset) * i;
                }
                d = (self.d + other.d) / 2.0;
                if rng.gen_bool(0.5) {
                    d = d + rng.gen_range(-offset..=offset) * d;
                }
            }
            _ => {}
        }
        Genome { p, i, d }
    }
}
