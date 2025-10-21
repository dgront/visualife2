use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use rand::Rng;
use visualife::styling::{rgb_to_hex, Style, darker};
use visualife::{SvgDrawing};
use visualife::basic_shapes::{SvgElement};


fn main() {
    let draw_width = 1000.0;
    let (n_x, n_y) : ( usize,usize) = (100, 100);
    let box_width = 9.0;
    let mut rng = rand::thread_rng();
    let mut drawing = SvgDrawing::new(draw_width, draw_width);
    let max_noise: f32 = 0.75;

    let start = Instant::now(); // Start timing
    for i in 0..n_x {
        let mut max_drop = i as f32 / 100.0 + 1.0;
        max_drop = max_drop * max_drop * max_drop;

        for j in 0..n_y {
            let fill = rgb_to_hex(255, (i * 255 / n_x) as u8, (j * 255 / n_y) as u8);
            let style = Style::new()
                .stroke(&darker(&fill, 0.3).unwrap())
                .fill(&fill)
                .opacity(rng.gen_range(0.6..=1.0))
                .stroke_width(rng.gen_range(0.25..=3.0));

            let noise_x = rng.gen_range(-max_noise..max_noise);
            let noise_y = rng.gen_range(-max_noise..max_noise) + rng.gen_range(max_drop / 2.0..max_drop);
            let x = i as f32 * draw_width / (n_x as f32) + noise_x;
            let y = j as f32 * draw_width / (n_y as f32) + noise_y;

            let id_str = format!("el_{i}_{j}");

            let mut element = if rng.gen_range(0.0..1.0) < 0.2 {
                let r = box_width / 2.0;
                SvgElement::circle(id_str, x + r, y + r, r)
            } else {
                SvgElement::rect(id_str, x, y, box_width, box_width)
            };

            element.set_style(style);
            drawing.add_element(element);
        }
    }

    let svg_str = drawing.to_svg();

    // ---------- Save to file
    let fname = env::args().nth(1).unwrap_or_else(|| "random_painting.svg".to_string());
    let mut file = File::create(&fname).expect("Failed to create output file");
    file.write_all(svg_str.as_bytes()).expect("Failed to write SVG content");

    eprintln!("SVG saved to {fname}");
    eprintln!("Execution time: {:.3?}", start.elapsed());
}

