//! IQR-based entropy outlier detection.
//!
//! Instead of fixed entropy thresholds (which produce false positives when
//! a directory is full of compressed files), this module identifies files
//! whose entropy is statistically anomalous relative to their peers.
//!
//! Uses the Interquartile Range (IQR) method:
//! - Q1 = 25th percentile, Q3 = 75th percentile
//! - IQR = Q3 - Q1
//! - Lower fence = Q1 - 1.5 * IQR
//! - Upper fence = Q3 + 1.5 * IQR
//! - Outliers = values outside the fences

/// Results of IQR-based outlier detection.
#[derive(Debug, Clone)]
pub struct OutlierReport {
    pub q1: f32,
    pub median: f32,
    pub q3: f32,
    pub iqr: f32,
    pub lower_fence: f32,
    pub upper_fence: f32,
    /// Indices into the original entropy array that are outliers.
    pub outlier_indices: Vec<usize>,
}

/// Detect entropy outliers using the IQR method.
/// Returns None if there are fewer than 4 data points.
pub fn detect_outliers(entropies: &[f32]) -> Option<OutlierReport> {
    if entropies.len() < 4 {
        return None;
    }

    let mut sorted: Vec<f32> = entropies.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let q1 = percentile(&sorted, 25.0);
    let median = percentile(&sorted, 50.0);
    let q3 = percentile(&sorted, 75.0);
    let iqr = q3 - q1;

    let lower_fence = q1 - 1.5 * iqr;
    let upper_fence = q3 + 1.5 * iqr;

    let outlier_indices: Vec<usize> = entropies
        .iter()
        .enumerate()
        .filter(|(_, &e)| e < lower_fence || e > upper_fence)
        .map(|(i, _)| i)
        .collect();

    Some(OutlierReport {
        q1,
        median,
        q3,
        iqr,
        lower_fence,
        upper_fence,
        outlier_indices,
    })
}

fn percentile(sorted: &[f32], pct: f32) -> f32 {
    let k = (pct / 100.0) * (sorted.len() - 1) as f32;
    let f = k.floor() as usize;
    let c = k.ceil() as usize;
    if f == c {
        sorted[f]
    } else {
        sorted[f] * (c as f32 - k) + sorted[c] * (k - f as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_outliers_normal_distribution() {
        // Most values clustered around 4.0, with one extreme outlier
        let entropies = vec![3.8, 3.9, 4.0, 4.1, 4.2, 4.0, 3.9, 4.1, 7.9, 4.0];
        let report = detect_outliers(&entropies).unwrap();

        assert!(report.outlier_indices.contains(&8)); // 7.9 is an outlier
        assert!(report.iqr > 0.0);
        assert!(report.upper_fence < 7.9);
    }

    #[test]
    fn test_detect_outliers_too_few() {
        let entropies = vec![1.0, 2.0, 3.0];
        assert!(detect_outliers(&entropies).is_none());
    }

    #[test]
    fn test_detect_outliers_no_outliers() {
        // Uniform data -- no outliers
        let entropies = vec![4.0, 4.1, 4.0, 3.9, 4.0, 4.1, 3.9, 4.0];
        let report = detect_outliers(&entropies).unwrap();
        assert!(report.outlier_indices.is_empty());
    }

    #[test]
    fn test_detect_outliers_both_ends() {
        let entropies = vec![0.1, 4.0, 4.1, 3.9, 4.0, 4.1, 3.9, 4.0, 7.8];
        let report = detect_outliers(&entropies).unwrap();
        // Both 0.1 and 7.8 should be outliers
        assert!(report.outlier_indices.contains(&0)); // 0.1
        assert!(report.outlier_indices.contains(&8)); // 7.8
    }

    #[test]
    fn test_percentile_calculation() {
        let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((percentile(&sorted, 50.0) - 3.0).abs() < f32::EPSILON);
        assert!((percentile(&sorted, 25.0) - 2.0).abs() < f32::EPSILON);
        assert!((percentile(&sorted, 75.0) - 4.0).abs() < f32::EPSILON);
    }
}
