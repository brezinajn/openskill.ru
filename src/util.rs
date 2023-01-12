use crate::constants::Constants;
use crate::traits::{Player, Team};

impl Constants {
    pub fn util_c<TEAM: Team<PLAYER>, PLAYER: Player>(&self, teams: &[TEAM]) -> f64 {
        teams.iter()
            .fold(0.0, |acc, team| acc + self.beta_sq() + team.sigma_sq())
    }
}

pub fn util_sum_q<TEAM: Team<PLAYER>, PLAYER: Player>(teams: &[TEAM], c: f64) -> Vec<f64> {
    teams.iter()
        .enumerate()
        .map(|(q_rank, _)| {
            teams.iter()
                .enumerate()
                .filter(|(i_rank, _)| *i_rank > q_rank)
                .map(|(_, team)| (team.mu() + c).exp())
                .sum()
        }).collect()
}

pub fn util_a<TEAM: Team<PLAYER>, PLAYER: Player>(teams: &[TEAM]) -> Vec<f64> {
    teams.iter()
        .enumerate()
        .map(|(i_rank, _)| {
            teams.iter()
                .enumerate()
                .filter(|(q_rank, _)| i_rank == *q_rank)
                .count() as f64
        })
        .collect()
}
