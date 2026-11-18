//! Statistics for the report: Wilson score intervals and an exact McNemar test.

/// Wilson 95% interval for k successes in n.
pub fn wilson(k: usize, n: usize) -> (f64, f64) {
    if n == 0 {
        return (0.0, 0.0);
    }
    let z = 1.959964f64;
    let n = n as f64;
    let p = k as f64 / n;
    let d = 1.0 + z * z / n;
    let c = (p + z * z / (2.0 * n)) / d;
    let h = z * ((p * (1.0 - p) / n + z * z / (4.0 * n * n)).sqrt()) / d;
    ((c - h).max(0.0), (c + h).min(1.0))
}

/// Exact two-sided McNemar p-value from the discordant counts b and c.
pub fn mcnemar(b: usize, c: usize) -> f64 {
    let n = b + c;
    if n == 0 {
        return 1.0;
    }
    let k = b.min(c);
    let mut p = 0.0;
    let mut coef = 1.0f64; // C(n, 0)
    for i in 0..=k {
        if i > 0 {
            coef = coef * (n - i + 1) as f64 / i as f64;
        }
        p += coef;
    }
    (2.0 * p / 2f64.powi(n as i32)).min(1.0)
}

pub fn round3(x: f64) -> f64 {
    (x * 1000.0).round() / 1000.0
}

#[cfg(test)]
mod tests {
    #[test]
    fn wilson_known_value() {
        let (lo, hi) = super::wilson(6, 15);
        assert!((lo - 0.198).abs() < 0.002 && (hi - 0.643).abs() < 0.002, "{lo} {hi}");
    }
    #[test]
    fn mcnemar_known_value() {
        assert!((super::mcnemar(0, 6) - 0.03125).abs() < 1e-9);
        assert_eq!(super::mcnemar(3, 3), 1.0);
    }
}
