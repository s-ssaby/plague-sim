use rand_distr::{Binomial, Distribution};
use rand::{rngs::ThreadRng, Rng};


/* Provides important math functionality */


// generate a random float from 0 to 1 (noninclusive)
pub fn get_random() -> f64 {
    fastrand::f64()
}

pub fn pick_random<I>(collection: I) -> Option<<I as IntoIterator>::Item> where I: IntoIterator , <I as IntoIterator>::IntoIter: ExactSizeIterator{
   fastrand::choice(collection)
}

/// Returns how many trials succeeded given a trial amount and a success rate according to a binomial distribution
pub fn binomial_sample(trials: u32, success_rate: f64) -> u32 {
    let distr = Binomial::new(trials.into(), success_rate).unwrap();
    let mut generator = rand::thread_rng();
    distr.sample(&mut generator).try_into().unwrap()
}

/// Rounds down or up to nearest integer randomly
/// 
/// Probababilities based on distance from integers
/// 
/// Notes:
/// * Input float x and x + 1 must lie within range of representable u32 integers
/// # Example
/// ```
/// use functionality::math_utils::probabilistic_round;
/// // Should always evaluate to 1
/// assert_eq!(probabilistic_round(1.0), Ok(1));
/// 
/// // Should always evaluate to 1 or 2, each one 50% of the time
/// let value = probabilistic_round(1.5);
/// assert!(value == Ok(1) || value == Ok(2));
/// 
/// // Should always evaluate to 10 25% of the time, or 11 75% of the time
/// let value2 = probabilistic_round(10.750000001);
/// assert!(value2 == Ok(10) || value2 == Ok(11));
/// 
/// // Negative numbers should fail, but not zero
/// assert!(probabilistic_round(-1.0).is_err());
/// assert!(probabilistic_round(-10.0).is_err());
/// assert!(probabilistic_round(-0.0).is_ok());
/// assert!(probabilistic_round(0.0).is_ok());
/// 
/// // Too big numbers should fail
/// assert!(probabilistic_round(4294967295.0).is_err());
/// assert!(probabilistic_round(4294967295.1).is_err());
/// assert!(probabilistic_round(4294967296.0).is_err());
/// assert!(probabilistic_round(5294967295.0).is_err());
/// ```
pub fn probabilistic_round(x: f32) -> Result<u32, String> {
    // x and x + 1 must be in range representable by u32 numbers
    if x < 0.0 || x >= 4294967295.0 {
        Err(format!("Cannot probabilistically round a value of {}", x))
    } else {
        let rounded_down = x as u32;
        let fraction_part = x - rounded_down as f32;
        if (get_random() as f32) < fraction_part {
            Ok(rounded_down + 1)
        } else {
            Ok(rounded_down)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math_utils;
    #[test]
    fn pick_random() {
        let values = [1, 2, 3, 4];
        let rand_val = math_utils::pick_random(&values).unwrap();
        assert!((1..=4).contains(rand_val));

        assert_eq!(values.len(), 4);
    }
    
}