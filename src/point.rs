use eframe::egui;
use rand::Rng;

use crate::color;

#[derive(Default, Clone, PartialEq)]
pub struct Point {
    pub vec: egui::Vec2,
    pub color: egui::Color32,
}

impl Point {
    // TODO: random in square or circle
    pub fn random(mut rng: &mut rand::rngs::ThreadRng) -> Self {
        let x = rng.gen_range(0.0..1.0);
        let y = rng.gen_range(0.0..1.0);
        Self {
            // random point, then offset by normalized screen space
            vec: egui::vec2(x, y) - egui::vec2(0.5, 0.5),
            color: color::gen_random(&mut rng),
        }
    }
}
