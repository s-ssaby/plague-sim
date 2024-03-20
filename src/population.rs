use std::ops::Add;

#[derive(Debug, Clone, Default, PartialEq, Copy)]
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


    /* Returns all non-dead people in population */
    pub fn get_alive(&self) -> u32 {
        self.healthy + self.infected + self.recovered
    }

    /** Returns total population, including dead */
    pub fn get_total(&self) -> u32 {
        self.dead + self.healthy + self.recovered + self.infected
    }

    // Calculates population resulting from removing a group from this population
    // Errors if group cannot be extracted from this population
    pub fn emigrate(&mut self, group: Self) -> Result<Population, String> {
        if group.healthy > self.healthy {
            Err(format!("Cannot remove {} healthy people from {} healthy people", group.healthy, self.healthy))
        } else if group.dead > self.dead {
            Err(format!("Cannot remove {} dead people from {} dead people", group.dead, self.dead))
        } else if group.recovered > self.recovered {
            Err(format!("Cannot remove {} recovered people from {} recovered people", group.recovered, self.recovered))
        } else if group.infected > self.infected {
            Err(format!("Cannot remove {} infected people from {} infected people", group.infected, self.infected))
        } else {
            let new_healthy = self.healthy - group.healthy;
            let new_dead = self.dead - group.dead;
            let new_recovered = self.recovered - group.recovered;
            let new_infected = self.infected - group.infected;
            Ok(Population { healthy: new_healthy, infected: new_infected, dead: new_dead, recovered: new_recovered })
        }
    }
}

