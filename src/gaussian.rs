use std::f64::consts::PI;

pub fn pdf(x: f64) -> f64 {
    (-x * x / 2.0).exp() / (2.0 * PI).sqrt()
}

pub fn cdf(z: f64) -> f64 {
    if z < -8.0 { 0.0 } else if z > 8.0 { 1.0 } else {
        let mut sum = 0.0;
        let mut term = z;
        let mut i = 3;

        while sum + term != sum {
            sum += term;
            term = term * z * z / i as f64;
            i += 2;
        }

        0.5 + sum * pdf(z)
    }
}