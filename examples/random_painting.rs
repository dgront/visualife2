use rand::Rng;
use visualife::styling::{rgb_to_hex, Style, darker};
use visualife::{SvgDrawing};
use visualife::basic_shapes::{SvgElement, ElementID};

fn main() {
    let draw_width = 1000.0;
    let (n_x, n_y) = (100, 100);
    let box_width = 9.0;
    let mut rng = rand::thread_rng();
    let mut drawing = SvgDrawing::new(draw_width, draw_width);
    let max_noise: f32 = 0.5;
    for i in 0..n_x {
        let mut max_drop = i as f32 / 100.0 + 1.0;
        max_drop = max_drop * max_drop * max_drop;
        for j in 0..n_y {
            let fill = rgb_to_hex(255, i * 255 / n_x, j * 255 / n_y);
            let style = Style::new()
                .stroke(&darker(&fill, 0.3).ok().unwrap())
                .fill(&fill)
                .opacity(rng.gen_range(0.6..=1.0))
                .stroke_width(rng.gen_range(0.5..=2.0));
            let style_id = drawing.styles_mut().add_style(style);
            let noise_x = rng.gen_range(-max_noise..max_noise);
            let noise_y = rng.gen_range(-max_noise..max_noise) + rng.gen_range(max_drop/2.0..max_drop);
            let x = i as f32 * draw_width / (n_x as f32) + noise_x;
            let y = j as f32 * draw_width / (n_y as f32) + noise_y;
            let id = ElementID::from(format!("el_{i}_{j}"));
            drawing.styles_mut().style_element(style_id, &id);
            if rng.gen_range(0.0..1.0) < 0.1 {
                let r = box_width / 2.0;
                drawing.add_element(SvgElement::Circle { id, cx: x + r, cy: y + r, r });
            } else {
                let rect = SvgElement::Rect { id, x, y, width: box_width, height: box_width };
                // let angle = rng.gen_range(0.0..=i as f32 * j as f32 * 60.0 / (n_x * n_y) as f32);
                // rect.angle = angle;
                drawing.add_element(rect);
            }
        }
    }

    drawing.draw();
}
