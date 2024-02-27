// Represents a disease that can spread from person to person

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pathogen {
    pub name: String,
    // probability of transmission when interacting with another person
    infectivity: f32,
    // probability of dying each day
    lethality: f32,
    // probability of recovering each day
    recovery_rate: f32   
}

impl Pathogen {
    pub fn new(name: String, infectivity: f32, lethality: f32, recovery_rate: f32) -> Self {
        Self {name, infectivity, lethality, recovery_rate}
    }

    pub fn get_infectivity(&self) {
        self.infectivity;
    }

    pub fn get_lethality(&self) {
        self.lethality;
    }

    pub fn get_recovery_rate(&self) {
        self.recovery_rate;
    }
}