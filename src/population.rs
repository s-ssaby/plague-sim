use std::ops::Add;

#[derive(Debug, Clone, Default, PartialEq)]
/** Represents any group of people */
pub struct Population {
    pub healthy: u32,
    pub infected: u32,
    pub dead: u32,
    pub recovered: u32
}

impl Add for Population {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let tot_healthy = self.healthy + rhs.healthy;
        let tot_infected = self.infected + rhs.infected;
        let tot_dead = self.dead + rhs.dead;
        let tot_recovered = self.recovered + rhs.recovered;
        Population { healthy: tot_healthy, infected: tot_infected, dead: tot_dead, recovered: tot_recovered }
    }

}

impl Population {
    /* Creates a population of healthy people */
    pub fn new (initial_pop: u32) -> Self{
        Self {healthy: initial_pop, dead: 0, recovered: 0, infected: 0}
    }

    // Transports a subpopulation of people out of this population
    // Modifies this population to match resulting population after transportation
    // Errors if group cannot be extracted from this population
    pub fn emigrate(&mut self, group: Self) -> Result<(), String> {
        if group.healthy > self.healthy {
            Err(format!("Cannot remove {} healthy people from {} healthy people", group.healthy, self.healthy))
        } else if group.dead > self.dead {
            Err(format!("Cannot remove {} dead people from {} dead people", group.dead, self.dead))
        } else if group.recovered > self.recovered {
            Err(format!("Cannot remove {} recovered people from {} recovered people", group.recovered, self.recovered))
        } else if group.infected > self.infected {
            Err(format!("Cannot remove {} infected people from {} infected people", group.infected, self.infected))
        } else {
            self.healthy -= group.healthy;
            self.dead -= group.dead;
            self.recovered -= group.recovered;
            self.infected -= group.infected;
            Ok(())
        }
    }
}

