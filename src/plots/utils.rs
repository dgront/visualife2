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