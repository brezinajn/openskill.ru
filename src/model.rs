use std::borrow::{Borrow, BorrowMut};
use std::fmt::Debug;

use crate::constants::Constants;
use crate::statistics::{gamma, vt};
use crate::traits::{Game, Player, Team};
use crate::util::{util_a, util_sum_q};

impl Constants {
    pub fn placket_luce<GAME: Game<TEAM, PLAYER>, TEAM: Team<PLAYER>, PLAYER: Player>(&self, game: &mut GAME) {
        let teams = game.teams_mut();

        let c = self.util_c(teams);
        let sum_q = util_sum_q(teams, c);
        let a = util_a(teams);

        let teams_len = teams.len();


        for i_rank in 0..teams_len {
            let i_team = teams[i_rank].borrow();

            let omega_set = &mut vec![];
            let delta_set = &mut vec![];

            let i_mu = i_team.mu();

            for q_rank in 0..teams_len {
                if q_rank > i_rank { continue; }

                let quotient = (i_mu / c).exp() / sum_q[q_rank];
                let mu: f64 = if q_rank == i_rank { 1.0 - quotient / a[q_rank] } else { -quotient / a[q_rank] };
                let sigma = (quotient * (1.0 - quotient)) / a[q_rank];

                omega_set.push(mu);
                delta_set.push(sigma);
            }

            let i_sigma_sq = i_team.sigma_sq();
            let gamma = gamma(c, i_sigma_sq);

            let omega_sum: f64 = omega_set.iter().sum();
            let delta_sum: f64 = delta_set.iter().sum();

            let i_omega: f64 = omega_sum * i_sigma_sq / c;
            let i_delta = gamma * delta_sum * i_sigma_sq / c / c;

            let i_team = teams[i_rank].borrow_mut();
            self.update_team(i_team, i_sigma_sq, i_omega, i_delta);
        }
    }

    pub fn thurstone_mosteller_full<GAME: Game<TEAM, PLAYER>, TEAM: Team<PLAYER>, PLAYER: Player>(&self, game: &mut GAME) {
        let teams = game.teams_mut();

        let teams_len = teams.len();

        for i_rank in 0..teams_len {
            let i_team = teams[i_rank].borrow();
            let i_mu = i_team.mu();
            let i_sigma_sq = i_team.sigma_sq();

            let mut i_omega = 0.0;
            let mut i_delta = 0.0;
            for q_rank in 0..teams_len {
                if q_rank == i_rank { continue; }
                let q_team = teams[q_rank].borrow();
                let q_mu = q_team.mu();
                let q_sigma_sq = q_team.sigma_sq();

                let ciq = (i_sigma_sq + q_sigma_sq + self.two_beta_sq()).sqrt();
                let delta_mu = (i_mu - q_mu) / ciq;
                let sig_sq_to_ciq = i_sigma_sq / ciq;
                let gamma = gamma(ciq, teams_len as f64);


                if i_rank == q_rank {
                    i_omega += sig_sq_to_ciq * vt(delta_mu, self.epsilon / ciq);
                    i_delta += ((gamma * sig_sq_to_ciq) / ciq) * self.wt(delta_mu, self.epsilon / ciq);
                } else {
                    let sign = if q_rank > i_rank { 1.0 } else { -1.0 };

                    i_omega += sign * sig_sq_to_ciq * self.v(sign * delta_mu, self.epsilon / ciq);
                    i_delta += ((gamma * sig_sq_to_ciq) / ciq) * self.w(sign * delta_mu, self.epsilon / ciq);

                }
            }

            let i_team = teams[i_rank].borrow_mut();
            self.update_team(i_team, i_sigma_sq, i_omega, i_delta);
        }
    }

    fn update_player<PLAYER: Player>(&self, player: &mut PLAYER, i_sigma_sq: f64, i_omega: f64, i_delta: f64) {
        let sigma_sq = player.sigma_sq();

        player.set_mu(player.mu() + sigma_sq / i_sigma_sq * i_omega);
        player.set_sigma(player.sigma() * f64::max(1.0 - sigma_sq / i_sigma_sq * i_delta, self.epsilon).sqrt())
    }

    fn update_team<PLAYER: Player, TEAM: Team<PLAYER>>(&self, team: &mut TEAM, i_sigma_sq: f64, i_omega: f64, i_delta: f64) {
        team.players_mut()
            .iter_mut()
            .for_each(|player| self.update_player(player, i_sigma_sq, i_omega, i_delta));
    }
}


trait DebugPrint: Debug {
    fn debug(&self) {
        println!("================================================");
        println!("{:#?}", self);
        println!("================================================");
    }
}

impl<T: Debug> DebugPrint for T {}