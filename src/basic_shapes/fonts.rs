use std::{fs, io};
use std::path::Path;

/// Defines fonts provided with the VisuaLife distribution
///
/// These fonts may be directly embedded in an SVG file
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum VlFont {
    DejaVuSansRegular,
}

impl VlFont {
    /// Returns the local file name of the font file
    pub fn file_name(&self) -> &'static str {
        match self {
            VlFont::DejaVuSansRegular => "./assets/dejavu-sans-webfont.woff2",
        }
    }

    /// Returns the name the font will be registered under
    pub fn font_name(&self) -> &'static str {
        match self {
            VlFont::DejaVuSansRegular => "DejaVuSansRegular",
        }
    }
}

/// Embeds a font provided by the VisuaLife distribution into an SVG document
pub fn embed_vl_font(font: VlFont, font_weight: Option<&str>) -> io::Result<String> {
    embed_font_file(font.font_name(), font.file_name(), font_weight)
}

/// Embed a font file into an SVG string using @font-face and a data: URL.
///
/// User can specify `font_weight`; if `None` was given, the default `"normal"` value will be used.
/// After the font has been defined, one can use `font-family="{font_name}"` in your SVG text.
///
/// # Notes:
/// - Many browsers support this.
/// - Some SVG-to-PDF/PNG pipelines may ignore embedded fonts.
pub fn embed_font_file<P: AsRef<Path>>(font_name: &str, font_path: P, font_weight: Option<&str>) -> io::Result<String> {

    let font_path = font_path.as_ref();
    let bytes = fs::read(font_path)?;

    // base64 crate (v0.22+)
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine as _;
    let b64 = STANDARD.encode(&bytes);

    // Best-effort MIME / format based on extension.
    let (mime, format) = match font_path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .as_deref()
    {
        Some("otf") => ("font/otf", "opentype"),
        Some("ttf") => ("font/ttf", "truetype"),
        Some("woff") => ("font/woff", "woff"),
        Some("woff2") => ("font/woff2", "woff2"),
        _ => ("application/octet-stream", "truetype"),
    };

    let weight = font_weight.unwrap_or("normal");

    // A <style> block that is safe to place in <defs>.
    // CDATA avoids escaping issues in XML.
    let snippet = format!(
        r#"<style><![CDATA[
@font-face {{
  font-family: "{family}";
  src: url("data:{mime};base64,{b64}") format("{format}");
  font-weight: {weight};
}}
]]></style>"#,
        family = font_name,
        mime = mime,
        b64 = b64,
        format = format,
        weight = weight
    );

    return Ok(snippet);
}

