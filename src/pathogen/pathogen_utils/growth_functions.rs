use crate::population_types::population::Population;

/// Functions responsible for calculating pathogen spread growth


/// Multiplies infected population by (1 + rate)
/// Also decreases healthy population by same amount infected population was increased
/// If unable to increase infected population by requested amount, increase as much as possible
/// Always rounds down (truncated) new infections calculated
pub fn exponential_growth_truncate(init_pop: Population, rate: f64) -> Population {
    todo!();
}

/// Multiplies infected population by (1 + rate)
/// Also decreases healthy population by same amount infected population was increased
/// If unable to increase infected population by requested amount, increase as much as possible
/// Always rounds down or up to nearest integer for new infections calculated
pub fn exponential_growth_rounded(init_pop: Population, rate: f64) -> Population {
    todo!();
}

/// Grows infected population by a constant amount
/// Also decreased healthy population by same amount infected population was increased
/// If unable to increase infected population by requested amount, increase as much as possible
pub fn linear_growth(init_pop: Population, amount: u32) -> Population {
    todo!()
}