use eframe::egui;
use rand::Rng;

use crate::color;

use super::{pos2, Pos2};

#[derive(Default, Clone, PartialEq)]
pub struct Dot {
    pub pos: Pos2,
    pub color: egui::Color32,
}

impl Dot {
    // TODO: random in square or circle
    pub fn random(mut rng: &mut rand::rngs::ThreadRng) -> Self {
        let x = rng.gen_range(0.0..2.0);
        let y = rng.gen_range(0.0..2.0);
        Self {
            // random point, then normalize
            pos: pos2(x, y) - pos2(1.0, 1.0),
            color: color::gen_random(&mut rng),
        }
    }
}
