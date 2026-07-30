use std::borrow::Borrow;

use crate::SIGNIFICANT_DIGITS;

/// Generate `n` equally spaced values between `xb` and `xe`.
///
/// If `skip_ends` is `false`, the returned vector includes both endpoints (`xb` and `xe`)
/// whenever `n >= 2`. If `n == 1`, it returns `[xb]`.
///
/// If `skip_ends` is `true`, the returned vector contains `n` points strictly inside
/// the open interval `(xb, xe)` (i.e., both ends are omitted). In that mode, the step
/// is `(xe - xb) / (n + 1)`.
///
/// This function works for increasing and decreasing intervals.
///
/// # Examples
///
/// Include endpoints:
/// ```
/// use visualife::utils::linspace;
/// let v = linspace(5, 0.0, 2.0, false);
/// assert_eq!(v, vec![0.0, 0.5, 1.0, 1.5, 2.0]);
/// ```
///
/// Skip endpoints (open interval):
/// ```
/// use visualife::utils::linspace;
/// let v = linspace(7, 0.0, 2.0, true);
/// let expected = vec![0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75];
/// for (a, b) in v.iter().zip(expected.iter()) {
///     assert!((a - b).abs() < 1e-6);
/// }
/// ```
///
/// Descending interval:
/// ```
/// use visualife::utils::linspace;
/// let v = linspace(5, 2.0, 0.0, false);
/// assert_eq!(v, vec![2.0, 1.5, 1.0, 0.5, 0.0]);
/// ```
pub fn linspace(n: usize, xb: f32, xe: f32, skip_ends: bool) -> Vec<f32> {
    if n == 0 {
        return Vec::new();
    }

    if skip_ends {
        // Points inside (xb, xe)
        let step = (xe - xb) / (n as f32 + 1.0);
        (1..=n)
            .map(|i| xb + step * i as f32)
            .collect()
    } else {
        match n {
            1 => vec![xb],
            _ => {
                let step = (xe - xb) / (n as f32 - 1.0);
                (0..n)
                    .map(|i| xb + step * i as f32)
                    .collect()
            }
        }
    }
}

/// Generate equally spaced values between `xb` and `xe`, separated by `step`
pub fn linspace_by(xb: f32, xe: f32, step: f32) -> Vec<f32> {

    let mut data = vec![xb];
    let mut i = 1;
    while data[i]< xe {
        i+=1;
        data.push(data[i]+step);
    }

    return data
}

/// Returns (n_rows, max_cols) of a 2D iterator without consuming the data.
///
/// It works with slices:
/// ```
/// use visualife::utils::matrix_shape;
/// let r1: &[i32] = &[1, 2, 3];
/// let r2: &[i32] = &[4, 5];
/// let data = vec![r1, r2];
/// assert_eq!(matrix_shape(&data), (2, 3));
/// ```
///
/// ... and with Vec<>. For ragged matrix (rows of different length) provides the maximum size:
/// ```
/// use visualife::utils::matrix_shape;
/// let m = vec![
///     vec![1, 2],
///     vec![3, 4, 5, 6],
///     vec![7],
/// ];
/// assert_eq!(matrix_shape(&m), (3, 4));
pub fn matrix_shape<R, T>(data: &[R]) -> (usize, usize)  where R: AsRef<[T]> {
    let n_rows = data.len();
    let max_cols = data.iter().map(|row| row.as_ref().len()).max().unwrap_or(0);
    (n_rows, max_cols)
}

pub fn min_max(values: &[f32]) -> (f32, f32) {
    let mut iter = values.iter().copied().filter(|v| !v.is_nan());

    let first = iter.next().expect("The provided data slice should not be empty!");
    let mut min_v = first;
    let mut max_v = first;

    for v in iter {
        if v < min_v { min_v = v; }
        if v > max_v { max_v = v; }
    }

    return (min_v, max_v);
}

/// Format a float into a string rounding to ``SIGNIFICANT_DIGITS`` decimal places.
///
/// ``SIGNIFICANT_DIGITS`` is defined in the main ``lib.rs``
pub(crate) fn format_significant<T:Borrow<f32>>(value: T) -> String {

    let x = *value.borrow();

    if x == 0.0 || !x.is_finite() {
        return x.to_string();
    }

    let exponent = x.abs().log10().floor() as i32;
    let decimal_places = SIGNIFICANT_DIGITS as i32 - exponent - 1;

    let formatted = if decimal_places > 0 {
        format!("{:.*}", decimal_places as usize, x)
    } else {
        let scale = 10_f32.powi(-decimal_places);
        format!("{:.0}", (x / scale).round() * scale)
    };

    if formatted.contains('.') {
        formatted.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        formatted
    }
}