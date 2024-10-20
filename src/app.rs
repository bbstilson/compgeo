use eframe::egui;

use crate::{algorithms, point::Point};

// points
const MAX_NUM_POINTS: usize = 1000;
const DEFAULT_NUM_POINTS: usize = 5;
// radius
const MAX_RADIUS: f32 = 4.0;
const DEFAULT_RADIUS: f32 = 2.00;
// zoom
const MAX_ZOOM: f32 = 1.0;
const DEFAULT_ZOOM: f32 = 0.45;

#[derive(Default)]
pub struct EguiApp {
    app: App,
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.app.ui(ctx, ui);
            });
    }
}

#[derive(PartialEq)]
pub struct App {
    zoom: f32,
    radius: f32,
    num_points: usize,
    points: Vec<Point>,
    vertices: Vec<egui::Pos2>,
    rendered: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            zoom: DEFAULT_ZOOM,
            radius: DEFAULT_RADIUS,
            num_points: DEFAULT_NUM_POINTS,
            points: vec![],
            vertices: vec![],
            rendered: false,
        }
    }
}

impl App {
    fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let painter = egui::Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        );

        ctx.input(|input| {
            if input.key_pressed(egui::Key::Space) {
                self.points = vec![];
                self.rendered = false;
            }
        });

        // self.points = vec![
        //     Point {
        //         color: egui::Color32::RED,
        //         pos: egui::pos2(-1.0, -1.0),
        //     },
        //     Point {
        //         color: egui::Color32::ORANGE,
        //         pos: egui::pos2(1.0, -1.0),
        //     },
        //     Point {
        //         color: egui::Color32::YELLOW,
        //         pos: egui::pos2(0.0, 1.0),
        //     },
        //     Point {
        //         color: egui::Color32::WHITE,
        //         pos: egui::pos2(0.0, 0.0),
        //     },
        // ];

        if self.points.len() < self.num_points {
            let num_to_generate = self.num_points - self.points.len();
            let mut rng = rand::thread_rng();
            let mut points = vec![Point::default(); num_to_generate];
            for i in 0..num_to_generate {
                points[i] = Point::random(&mut rng);
            }
            self.points.append(&mut points);
            self.rendered = false;
        }

        if self.points.len() > self.num_points {
            self.points.truncate(self.num_points);
            self.rendered = false;
        }

        if !self.rendered {
            self.vertices = algorithms::graham_scan(
                &self
                    .points
                    .iter()
                    .map(|p| ("".to_string(), p.pos))
                    .collect::<Vec<_>>(),
            )
            .into_iter()
            .map(|p| p.1)
            .collect();
            self.rendered = true;
        }

        self.paint(&painter);
        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(painter.clip_rect());

        egui::Frame::popup(ui.style())
            .stroke(egui::Stroke::NONE)
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                egui::CollapsingHeader::new("Settings").show(ui, |ui| self.options_ui(ui));
            });
    }

    fn paint(&mut self, painter: &egui::Painter) {
        let mut shapes: Vec<egui::Shape> = Vec::new();

        for Point { pos, color } in &self.points {
            shapes.push(egui::Shape::circle_filled(
                self.to_screen_space(painter, *pos),
                self.radius,
                *color,
            ));
        }

        let vertices = self.vertices.as_slice().windows(2).map(|w| {
            let a = w[0];
            let b = w[1];

            self.draw_line(painter, [a, b], 1.0, egui::Color32::LIGHT_BLUE)
        });

        if !self.vertices.is_empty() {
            shapes.push(self.draw_line(
                painter,
                [
                    *self.vertices.first().unwrap(),
                    *self.vertices.last().unwrap(),
                ],
                1.0,
                egui::Color32::LIGHT_BLUE,
            ));
        }

        self.draw_grid(&painter, &mut shapes);

        painter.extend(vertices);
        painter.extend(shapes);
    }

    fn options_ui(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::Slider::new(&mut self.zoom, 0.0..=MAX_ZOOM).text("zoom"));
        ui.add(egui::Slider::new(&mut self.num_points, 0..=MAX_NUM_POINTS).text("num points"));
        ui.add(egui::Slider::new(&mut self.radius, 0.0..=MAX_RADIUS).text("point size"));

        egui::reset_button(ui, self, "Reset");
    }

    /// Takes a point `p` and converts it to screen space.
    fn to_screen_space(&self, painter: &egui::Painter, p: egui::Pos2) -> egui::Pos2 {
        let rect = painter.clip_rect();
        egui::emath::RectTransform::from_to(
            egui::Rect::from_center_size(egui::Pos2::ZERO, rect.square_proportions() / self.zoom),
            rect,
        ) * egui::pos2(p.x, p.y * -1.0)
    }

    // Debug code. Just draws an X on the middle of the screen.
    #[allow(dead_code)]
    fn draw_grid(&self, painter: &egui::Painter, shapes: &mut Vec<egui::Shape>) {
        let x_axis = self.draw_line(
            painter,
            [egui::pos2(-1.0, 0.0), egui::pos2(1.0, 0.0)],
            1.0,
            egui::Color32::WHITE,
        );
        let y_axis = self.draw_line(
            painter,
            [egui::pos2(0.0, -1.0), egui::pos2(0.0, 1.0)],
            1.0,
            egui::Color32::WHITE,
        );

        // shapes.push(egui::Shape::circle_filled(
        //     self.to_screen_space(painter, egui::pos2(0.0, 0.0)),
        //     5.0,
        //     egui::Color32::RED,
        // ));

        // shapes.push(egui::Shape::circle_filled(
        //     self.to_screen_space(painter, egui::pos2(1.0, 0.0)),
        //     5.0,
        //     egui::Color32::ORANGE,
        // ));

        // shapes.push(egui::Shape::circle_filled(
        //     self.to_screen_space(painter, egui::pos2(0.0, 1.0)),
        //     5.0,
        //     egui::Color32::YELLOW,
        // ));

        // shapes.push(egui::Shape::circle_filled(
        //     self.to_screen_space(painter, egui::pos2(-1.0, 0.0)),
        //     5.0,
        //     egui::Color32::GREEN,
        // ));

        // shapes.push(egui::Shape::circle_filled(
        //     self.to_screen_space(painter, egui::pos2(0.0, -1.0)),
        //     5.0,
        //     egui::Color32::BLUE,
        // ));

        shapes.push(x_axis);
        shapes.push(y_axis);
    }

    fn draw_line(
        &self,
        painter: &egui::Painter,
        points: [egui::Pos2; 2],
        stroke: f32,
        color: egui::Color32,
    ) -> egui::Shape {
        let line = [
            self.to_screen_space(painter, points[0]),
            self.to_screen_space(painter, points[1]),
        ];
        egui::Shape::line_segment(line, (stroke, color))
    }
}
