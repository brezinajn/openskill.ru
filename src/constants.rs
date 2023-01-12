use std::borrow::BorrowMut;

#[derive(Copy, Clone, Debug)]
pub struct Constants {
    pub z: i32,
    pub mu: f64,
    pub sigma: f64,
    pub beta: f64,
    pub epsilon: f64,
}

impl Constants {
    fn new(
        z: i32,
        mu: f64,
        sigma: f64,
        beta: f64,
        epsilon: f64,
    ) -> Self {
        Self { z, mu, sigma, beta, epsilon }
    }

    pub fn default() -> Constants {
        const Z: i32 = 3;
        const MU: f64 = 25.0;
        const SIGMA: f64 = MU / Z as f64;

        Self::new(
            Z,
            MU,
            SIGMA,
            SIGMA / 2.0,
            0.0001,
        )
    }

    pub fn build<F>(f: F) -> Self where F: FnOnce(&mut Self) {
        let mut a = Self::default();

        f(a.borrow_mut());

        a
    }

    pub fn beta_sq(&self) -> f64 {
        self.beta * self.beta
    }

    pub fn two_beta_sq(&self) -> f64 {
        self.beta_sq() * 2.0
    }
}
