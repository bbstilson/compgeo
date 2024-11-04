use eframe::egui;
use rand::Rng;

use crate::color;

#[derive(Default, Clone, PartialEq)]
pub struct Dot {
    pub pos: egui::Pos2,
    pub color: egui::Color32,
}

impl Dot {
    // TODO: random in square or circle
    pub fn random(mut rng: &mut rand::rngs::ThreadRng) -> Self {
        let x = rng.gen_range(0.0..2.0);
        let y = rng.gen_range(0.0..2.0);
        Self {
            // random point, then normalize
            pos: (egui::pos2(x, y) - egui::pos2(1.0, 1.0)).to_pos2(),
            color: color::gen_random(&mut rng),
        }
    }
}
