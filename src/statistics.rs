use crate::constants::Constants;
use crate::gaussian;

impl Constants {
    pub fn v(&self, x: f64, t: f64) -> f64 {
        let xt: f64 = x - t;
        let denom: f64 = gaussian::cdf(xt);

        if denom < self.epsilon { -xt } else { gaussian::pdf(xt) / denom }
    }

    pub fn w(&self, x: f64, t: f64) -> f64 {
        let xt: f64 = x - t;
        let denom: f64 = gaussian::cdf(xt);

        if denom >= self.epsilon {
            self.v(x, t) * (self.v(x, t) + xt)
        } else if x < 0.0 {
            1.0
        } else {
            0.0
        }
    }

    pub fn wt(&self, x: f64, t: f64) -> f64 {
        let xx: f64 = x.abs();
        let b: f64 = gaussian::cdf(t - xx) - gaussian::cdf(-t - xx);

        if b < self.epsilon { 1.0 } else {
            ((t - xx) * gaussian::pdf(t - xx) + (t + xx) * gaussian::pdf(-t - xx)) / b + vt(x, t) * vt(x, t)
        }
    }
}

pub fn vt(x: f64, t: f64) -> f64 {
    let xx: f64 = x.abs();
    let b: f64 = gaussian::cdf(t - xx) - gaussian::cdf(-t - xx);

    if b >= 1e-5 {
        let a: f64 = gaussian::pdf(-t - xx) - gaussian::pdf(t - xx);
        let result: f64 = if x < 0.0 { -a } else { a };

        result / b
    } else if x < 0.0 {
        -x - t
    } else { -x + t }
}

pub fn gamma(ciq: f64, sigma_sq: f64) ->f64 { sigma_sq.sqrt() / ciq }