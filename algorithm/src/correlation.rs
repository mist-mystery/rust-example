pub fn correlation_coefficient(p: &[(f64, f64)]) -> f64 {
    let n = p.len() as f64;
    if n == 0.0 {
        return 0.0; // Avoid division by zero
    }

    let sum_x: f64 = p.iter().map(|&(x, _)| x).sum();
    let sum_y: f64 = p.iter().map(|&(_, y)| y).sum();
    let sum_x2: f64 = p.iter().map(|&(x, _)| x * x).sum();
    let sum_y2: f64 = p.iter().map(|&(_, y)| y * y).sum();
    let sum_xy: f64 = p.iter().map(|&(x, y)| x * y).sum();

    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();

    if denominator == 0.0 {
        return 0.0; // Avoid division by zero
    }

    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_correlation_coefficient() {
        let data = vec![(1.0, 2.0), (2.0, 3.0), (3.0, 5.0), (4.0, 4.0)];
        let result = correlation_coefficient(&data);
        assert_eq!(result, 0.8);
    }

    #[test]
    fn test_correlation_coefficient_empty() {
        let data: Vec<(f64, f64)> = vec![];
        let result = correlation_coefficient(&data);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_correlation_coefficient_single_point() {
        let data = vec![(1.0, 2.0)];
        let result = correlation_coefficient(&data);
        assert_eq!(result, 0.0); // Single point should return 0.0
    }

    #[test]
    fn test_correlation_coefficient_zero_variance() {
        let data = vec![(1.0, 2.0), (1.0, 3.0), (1.0, 4.0)];
        let result = correlation_coefficient(&data);
        assert_eq!(result, 0.0); // Zero variance in x should return 0.0
    }
}
