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
/// use visualife::plots::linspace;
/// let v = linspace(5, 0.0, 2.0, false);
/// assert_eq!(v, vec![0.0, 0.5, 1.0, 1.5, 2.0]);
/// ```
///
/// Skip endpoints (open interval):
/// ```
/// use visualife::plots::linspace;
/// let v = linspace(5, 0.0, 2.0, true);
/// let expected = vec![1.0/3.0, 2.0/3.0, 1.0, 4.0/3.0, 5.0/3.0];
/// for (a, b) in v.iter().zip(expected.iter()) {
///     assert!((a - b).abs() < 1e-6);
/// }
/// ```
///
/// Descending interval:
/// ```
/// use visualife::plots::linspace;
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

/// Returns (n_rows, max_cols) of a 2D iterator without consuming the data.
///
/// It works with slices:
/// ```
/// use visualife::plots::matrix_shape;
/// let r1: &[i32] = &[1, 2, 3];
/// let r2: &[i32] = &[4, 5];
/// let data = vec![r1, r2];
/// assert_eq!(matrix_shape(&data), (2, 3));
/// ```
///
/// ... and with Vec<>. For ragged matrix (rows of different length) provides the maximum size:
/// ```
/// use visualife::plots::matrix_shape;
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