/// Converts RGB components to a hexadecimal color string.
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Converts a hexadecimal color string to RGB components.
pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), &'static str> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return Err("Invalid hex color format");
    }

    let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid red component")?;
    let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid green component")?;
    let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid blue component")?;

    Ok((r, g, b))
}

/// Darkens a hexadecimal color by a given fraction.
pub fn darker(color_hex: &str, fraction: f32) -> Result<String, &'static str> {

    if fraction < 0.0 || fraction > 1.0 {
        return Err("Fraction must be between 0.0 and 1.0");
    }
    let (r, g, b) = hex_to_rgb(color_hex)?;

    let new_r = (r as f32 * (1.0 - fraction)).clamp(0.0, 255.0) as u8;
    let new_g = (g as f32 * (1.0 - fraction)).clamp(0.0, 255.0) as u8;
    let new_b = (b as f32 * (1.0 - fraction)).clamp(0.0, 255.0) as u8;

    Ok(rgb_to_hex(new_r, new_g, new_b))
}

/// Darkens a hexadecimal color by a given fraction.
pub fn lighter(color_hex: &str, fraction: f32) -> Result<String, &'static str> {
    if fraction < 0.0 || fraction > 1.0 {
        return Err("Fraction must be between 0.0 and 1.0");
    }

    let (r, g, b) = hex_to_rgb(color_hex)?;

    let new_r = (r as f32 * (1.0 + fraction)).clamp(0.0, 255.0) as u8;
    let new_g = (g as f32 * (1.0 + fraction)).clamp(0.0, 255.0) as u8;
    let new_b = (b as f32 * (1.0 + fraction)).clamp(0.0, 255.0) as u8;

    Ok(rgb_to_hex(new_r, new_g, new_b))
}

/// Mixes two colors by shifting `color1` toward `color2` by `(1.0 - fraction)`.
///
/// `color1` and `color2` should be valid hex color codes, e.g., "#FF0000".
/// `fraction` should be between 0.0 and 1.0.
///
/// Returns a hex string representing the mixed color.
/// # Examples
///
/// ```
/// use visualife::styling::mix_colors;
/// let color = mix_colors("#FF0000", "#0000FF", 0.5).unwrap();
/// assert_eq!(color, "#800080"); // Equal mix of red and blue = purple
///
/// let color = mix_colors("#000000", "#FFFFFF", 0.25).unwrap();
/// assert_eq!(color, "#BFBFBF"); // 25% black + 75% white
///
/// let err = mix_colors("red", "#0000FF", 0.5);
/// assert!(err.is_err());
/// ```
pub fn mix_colors(color1: &str, color2: &str, fraction: f32) -> Result<String, &'static str> {
    let (r1, g1, b1) = hex_to_rgb(color1)?;
    let (r2, g2, b2) = hex_to_rgb(color2)?;

    let mix = |a, b| ((a as f32 * fraction) + (b as f32 * (1.0 - fraction))).round() as u8;

    let r = mix(r1, r2);
    let g = mix(g1, g2);
    let b = mix(b1, b2);

    Ok(rgb_to_hex(r, g, b))
}