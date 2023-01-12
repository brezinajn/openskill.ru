use std::borrow::{Borrow, BorrowMut};

use crate::traits::{Game, Player, Team};

mod constants;
mod gaussian;
mod statistics;
mod traits;
mod model;
mod util;

#[derive(Debug, Copy, Clone)]
struct PlayerStruct {
    mu: f64,
    sigma: f64,
}

impl PlayerStruct {
    fn default() -> Self {
        Self::new(25.0, 8.3333)
    }
    fn new(mu: f64, sigma: f64) -> Self {
        Self { mu, sigma }
    }
}

impl<'a> TeamStruct<'a> {
    fn new(players: &'a mut [PlayerStruct]) -> Self {
        Self { players }
    }
}

impl<'a> GameStruct<'a> {
    fn new(teams: &'a mut [TeamStruct<'a>]) -> Self { Self { teams } }
}

impl Player for PlayerStruct {
    fn set_mu(&mut self, mu: f64) {
        self.mu = mu
    }
    fn set_sigma(&mut self, sigma: f64) {
        self.sigma = sigma
    }
    fn mu(&self) -> f64 { self.mu }
    fn sigma(&self) -> f64 {
        self.sigma
    }
}


#[derive(Debug)]
struct TeamStruct<'a> {
    players: &'a mut [PlayerStruct],
}

#[derive(Debug)]
struct GameStruct<'a> {
    teams: &'a mut [TeamStruct<'a>],
}

impl<'a> Game<TeamStruct<'a>, PlayerStruct> for GameStruct<'a> {
    fn teams(&self) -> &[TeamStruct<'a>] {
        self.teams.borrow()
    }
    fn teams_mut(&mut self) -> &mut [TeamStruct<'a>] {
        self.teams.borrow_mut()
    }
}

impl<'a> Team<PlayerStruct> for TeamStruct<'a> {
    fn players_mut(&mut self) -> &mut [PlayerStruct] {
        self.players.borrow_mut()
    }
    fn players(&self) -> &[PlayerStruct] {
        self.players.borrow()
    }
}


#[cfg(test)]
mod tests {
    use crate::constants::Constants;

    use super::*;

    #[test]
    fn it_works() {
        let mut players1 = [PlayerStruct::default()];
        let mut players2 = [PlayerStruct::default()];

        let mut teams = [
            TeamStruct::new(players1.borrow_mut()),
            TeamStruct::new(players2.borrow_mut()),
        ];

        let mut game = GameStruct::new(teams.borrow_mut());

        let constants = Constants::default();

        constants.thurstone_mosteller_full(game.borrow_mut());
    }
}
