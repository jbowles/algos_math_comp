use std::f32;

pub fn sqrt(v: f32) -> f32 {
    panic!("Your sqrt implementation goes in place of this panic.");
}

#[cfg(test)]
mod tests {
    use std::f32;

    use super::sqrt;

    const TOLERANCE: f32 = 1e-4;

    quickcheck! {
        fn sqrt_test(v: f32) -> bool {
            if v < 0f32 {
                sqrt(v).is_nan()
            } else if v.is_infinite() {
                sqrt(v).is_infinite()
            } else {
                // Check against sqrt from the standard library.
                (sqrt(v) - v.sqrt()).abs() < TOLERANCE
            }
        }
    }
}
