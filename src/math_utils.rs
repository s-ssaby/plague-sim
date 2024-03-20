/* Provides important math functionality */


// generate a random float from 0 to 1 (noninclusive)
pub fn get_random() -> f64 {
    fastrand::f64()
}

pub fn pick_random<I>(collection: I) -> Option<<I as IntoIterator>::Item> where I: IntoIterator , <I as IntoIterator>::IntoIter: ExactSizeIterator{
   fastrand::choice(collection)
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