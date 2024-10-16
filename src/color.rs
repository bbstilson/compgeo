use eframe::egui;
use rand::Rng;

pub fn gen_random(rng: &mut rand::rngs::ThreadRng) -> egui::Color32 {
    let h = rng.gen_range(0.0..=1.0);
    let s = 0.95;
    let l = 0.7;
    hsl_to_rgb(h, s, l)
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> egui::Color32 {
    let (r, g, b) = if s == 0.0 {
        (l, l, l) // achromatic
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 1.0 / 3.0);
        (r, g, b)
    };

    let (r, g, b) = (
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    );
    egui::Color32::from_rgb(r, g, b)
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let mut t = t;
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}
