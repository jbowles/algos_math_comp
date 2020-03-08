/*
    1/n Σᵢxᵢ
*/
pub fn mean(s: &[f32]) -> f32 {
    let mut sum = 0.0;
    for v in s.iter() {
        sum += *v;
    }
    // (1.0 / (s.len() as f32)) * sum
    sum / (s.len() as f32)
}

/*
    julia: sum((p .- m).^2) / length(p)
    1/n Σᵢ(xᵢ - x̄)
*/
pub fn sample_variance(s: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    let xbar = mean(&s);
    for xi in s.iter() {
        sum += (xi - xbar).powf(2.0);
    }

    // (1.0 / (s.len() as f32)) * sum
    sum / (s.len() as f32)
}

#[cfg(test)]
mod tests {
    use super::{mean, sample_variance};
    use float_cmp::ApproxEq;
    // use float_cmp::ApproxEqRatio;
    // use float_cmp::ApproxEqUlps;
    fn somesamp() -> Vec<f32> {
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
    }

    #[test]
    fn find_mean() {
        //4.363_636_5
        let res = mean(&somesamp());
        assert!(4.3636.approx_eq(res, (0.00006, 2)));
        // assert!((res).approx_eq_ulps(&4.363_636_5, 2));
    }

    #[test]
    fn find_sample_variance() {
        //4.049_587
        let res = sample_variance(somesamp());
        assert!(4.0495.approx_eq(res, (0.00009, 2)));
        // assert!((res).approx_eq_ulps(&4.049_587, 2));
    }
}
