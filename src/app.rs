use eframe::egui;

use crate::{algorithms, point::Point};

// points
const MAX_POINTS: usize = 1000;
// radius
const MAX_RADIUS: f32 = 3.0;
const DEFAULT_RADIUS: f32 = 1.00;
// zoom
const MAX_ZOOM: f32 = 1.0;
const DEFAULT_ZOOM: f32 = 0.75;

#[derive(Default)]
pub struct EguiApp {
    app: App,
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| {
                self.app.ui(ui);
            });
    }
}

#[derive(PartialEq)]
pub struct App {
    zoom: f32,
    radius: f32,
    num_points: usize,
    points: Vec<Point>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            zoom: DEFAULT_ZOOM,
            radius: DEFAULT_RADIUS,
            num_points: 10,
            points: vec![],
        }
    }
}

impl App {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let painter = egui::Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        );

        if self.points.len() < self.num_points {
            let num_to_generate = self.num_points - self.points.len();
            let mut rng = rand::thread_rng();
            let mut points = vec![Point::default(); num_to_generate];
            for i in 0..num_to_generate {
                points[i] = Point::random(&mut rng);
            }
            self.points.append(&mut points);
        }

        if self.points.len() > self.num_points {
            self.points.truncate(self.num_points);
        }

        // TODO: don't do this on every frame
        let vertices =
            algorithms::grahams_scan(&self.points.iter().map(|p| p.vec).collect::<Vec<_>>());

        if !vertices.is_empty() {
            let mut shapes = vertices
                .as_slice()
                .windows(2)
                .map(|w| {
                    let a = w[0];
                    let b = w[1];

                    draw_line(
                        self.screen_space(&painter),
                        [a.to_pos2(), b.to_pos2()],
                        self.radius,
                        egui::Color32::LIGHT_BLUE,
                    )
                })
                .collect::<Vec<_>>();

            shapes.push(draw_line(
                self.screen_space(&painter),
                [
                    vertices.first().unwrap().to_pos2(),
                    vertices.last().unwrap().to_pos2(),
                ],
                self.radius,
                egui::Color32::LIGHT_BLUE,
            ));

            painter.extend(shapes);
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

        for Point { vec, color } in &self.points {
            shapes.push(egui::Shape::circle_filled(
                self.screen_space(painter) * vec.to_pos2(),
                self.radius,
                *color,
            ));
        }
        // draw_x(self.radius, &mut shapes, self.screen_space(painter));

        painter.extend(shapes);
    }

    fn options_ui(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::Slider::new(&mut self.zoom, 0.0..=MAX_ZOOM).text("zoom"));
        ui.add(egui::Slider::new(&mut self.num_points, 0..=MAX_POINTS).text("num points"));
        ui.add(egui::Slider::new(&mut self.radius, 0.0..=MAX_RADIUS).text("point size"));

        egui::reset_button(ui, self, "Reset");
    }

    fn screen_space(&self, painter: &egui::Painter) -> egui::emath::RectTransform {
        let rect = painter.clip_rect();
        egui::emath::RectTransform::from_to(
            egui::Rect::from_center_size(egui::Pos2::ZERO, rect.square_proportions() / self.zoom),
            rect,
        )
    }
}

fn draw_line(
    to_screen: egui::emath::RectTransform,
    points: [egui::Pos2; 2],
    stroke: f32,
    color: egui::Color32,
) -> egui::Shape {
    let line = [to_screen * points[0], to_screen * points[1]];
    egui::Shape::line_segment(line, (stroke, color))
}

// Debug code. Just draws an X on the middle of the screen.
#[allow(dead_code)]
fn draw_x(stroke: f32, shapes: &mut Vec<egui::Shape>, to_screen: egui::emath::RectTransform) {
    let center = egui::pos2(0.0, 0.0);
    let end = center + egui::Vec2::new(0.5, 0.5);
    let a = draw_line(to_screen, [center, end], stroke, egui::Color32::LIGHT_BLUE);

    let center = egui::pos2(0.0, 0.0);
    let end = center + egui::Vec2::new(0.5, -0.5);
    let b = draw_line(to_screen, [center, end], stroke, egui::Color32::LIGHT_GREEN);

    let center = egui::pos2(0.0, 0.0);
    let end = center + egui::Vec2::new(-0.5, 0.5);
    let c = draw_line(to_screen, [center, end], stroke, egui::Color32::LIGHT_RED);

    let center = egui::pos2(0.0, 0.0);
    let end = center + egui::Vec2::new(-0.5, -0.5);
    let d = draw_line(
        to_screen,
        [center, end],
        stroke,
        egui::Color32::LIGHT_YELLOW,
    );
    shapes.push(a);
    shapes.push(b);
    shapes.push(c);
    shapes.push(d);
}
