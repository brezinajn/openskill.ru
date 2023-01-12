
pub trait Player {
    fn set_mu(&mut self, mu: f64);
    fn set_sigma(&mut self, sigma: f64);

    fn mu(&self) -> f64;
    fn sigma(&self) -> f64;

    fn sigma_sq(&self) -> f64 {
        let sigma = self.sigma();
        sigma * sigma
    }
}

pub trait Team<P: Player> {
    fn players_mut(&mut self) -> &mut [P];
    fn players(&self) -> &[P];

    fn mu(&self)-> f64{
        self.players()
            .iter()
            .fold(0.0, |acc, elem| acc + elem.mu())
    }

    fn sigma(&self)-> f64{
        self.players()
            .iter()
            .fold(0.0, |acc, elem| acc + elem.sigma())
    }

    fn sigma_sq(&self)-> f64{
        self.players()
            .iter()
            .fold(0.0, |acc, elem| acc + elem.sigma_sq())
    }
}

pub trait Game<T: Team<P>, P: Player> {
    fn teams(&self) -> &[T];
    fn teams_mut(&mut self) -> &mut [T];
}
