use eframe::egui;

use crate::{
    algorithms,
    data::{
        point::Point2,
        pos2,
        simplex::{Point, Simplex, Triangle},
        sphere::{Sphere, Sphere1},
        Dot, Pos2,
    },
};

// points
const MAX_NUM_POINTS: usize = 10_000;
const DEFAULT_NUM_POINTS: usize = 3;
// radius
const MAX_RADIUS: f32 = 4.0;
const DEFAULT_RADIUS: f32 = 1.00;
// zoom
const MAX_ZOOM: f32 = 1.0;
// const DEFAULT_ZOOM: f32 = 0.45;
const DEFAULT_ZOOM: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Algorithm {
    DelaunayTriangulation,
    GrahamScan,
}

impl Into<&str> for Algorithm {
    fn into(self) -> &'static str {
        match self {
            Self::DelaunayTriangulation => "Delaunay Triangulation",
            Self::GrahamScan => "Graham Scan",
        }
    }
}

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
pub struct AppState {
    zoom: f32,
    radius: f32,
    num_points: usize,
    points: Vec<Dot>,
    vertices: Vec<Pos2>,
    spheres: Vec<Sphere1>,
    triangles: Vec<Triangle>,
    rendered: bool,
    algorithm: Algorithm,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            zoom: DEFAULT_ZOOM,
            radius: DEFAULT_RADIUS,
            num_points: DEFAULT_NUM_POINTS,
            points: vec![],
            vertices: vec![],
            spheres: vec![],
            triangles: vec![],
            rendered: false,
            algorithm: Algorithm::GrahamScan,
        }
    }
}

#[derive(Default)]
pub struct App {
    painter: Option<egui::Painter>,
    graph_painter: Option<egui::Painter>,
    state: AppState,
}

impl App {
    fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.painter = Some(egui::Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        ));

        let height = ui.available_height();
        let width = ui.available_width();
        let midpoint = width * 0.5;
        let half_height = height * 0.5;
        let offset = (self.state.zoom - 1.0) * height * 0.5;
        let tl_rect = egui::pos2(midpoint - (half_height + offset), 0.0 - offset);
        let br_rect = egui::pos2(midpoint + (half_height + offset), height + offset);

        self.graph_painter = Some(egui::Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            egui::Rect {
                min: tl_rect,
                max: br_rect,
            },
        ));

        ctx.input(|input| {
            if input.key_pressed(egui::Key::Space) {
                self.state.points = vec![];
                self.state.rendered = false;
            }
        });

        if self.state.points.len() < self.state.num_points {
            let num_to_generate = self.state.num_points - self.state.points.len();
            let mut rng = rand::thread_rng();
            let mut points = vec![Dot::default(); num_to_generate];
            for i in 0..num_to_generate {
                points[i] = Dot::random(&mut rng);
            }
            self.state.points.append(&mut points);
            self.state.rendered = false;
        }

        if self.state.points.len() > self.state.num_points {
            self.state.points.truncate(self.state.num_points);
            self.state.rendered = false;
        }

        if !self.state.rendered {
            self.state.vertices = algorithms::graham_scan(
                &self.state.points.iter().map(|p| p.pos).collect::<Vec<_>>(),
            );
            self.state.rendered = true;
        }

        // if !self.state.rendered {
        //     let a = Point { x: 0.0, y: 1.0 };
        //     let b = Point { x: 1.0, y: 0.0 };
        //     let c = Point { x: 0.0, y: 0.0 };

        //     let t = Triangle {
        //         vertices: [a, b, c],
        //     };
        //     let c = t.circumscribe().unwrap();
        //     self.state.triangles = vec![t];
        //     self.state.spheres = vec![c];
        //     self.state.rendered = true;
        // }

        self.paint();
        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(self.painter.as_ref().unwrap().clip_rect());

        self.render_menu(ctx);
    }

    fn render_menu(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .resizable([false, false])
            .default_width(280.0)
            .show(ctx, |ui| {
                egui::Grid::new("settings-grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.add(
                            egui::Slider::new(&mut self.state.zoom, 0.0..=MAX_ZOOM).text("zoom"),
                        );
                        ui.end_row();
                        ui.add(
                            egui::Slider::new(&mut self.state.num_points, 0..=MAX_NUM_POINTS)
                                .text("num points"),
                        );
                        ui.end_row();
                        ui.add(
                            egui::Slider::new(&mut self.state.radius, 0.0..=MAX_RADIUS)
                                .text("point size"),
                        );
                        ui.end_row();
                        egui::ComboBox::from_id_salt("algorithm-selection")
                            .selected_text(format!(
                                "{}",
                                <Algorithm as Into<&str>>::into(self.state.algorithm)
                            ))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.state.algorithm,
                                    Algorithm::GrahamScan,
                                    <Algorithm as Into<&str>>::into(Algorithm::GrahamScan),
                                );
                                ui.selectable_value(
                                    &mut self.state.algorithm,
                                    Algorithm::DelaunayTriangulation,
                                    <Algorithm as Into<&str>>::into(
                                        Algorithm::DelaunayTriangulation,
                                    ),
                                );
                            });
                        ui.end_row();
                    });
                egui::reset_button(ui, &mut self.state, "Reset");
            });
    }

    fn paint(&mut self) {
        let mut shapes: Vec<egui::Shape> = Vec::new();

        shapes.append(
            &mut self
                .state
                .points
                .iter()
                .map(|Dot { pos, color }| {
                    egui::Shape::circle_filled(
                        self.to_screen_space(*pos),
                        self.state.radius,
                        *color,
                    )
                })
                .collect(),
        );

        shapes.append(
            &mut self
                .state
                .vertices
                .windows(2)
                .map(|w| {
                    let a = w[0];
                    let b = w[1];
                    self.draw_line([a, b], 1.0, egui::Color32::LIGHT_BLUE)
                })
                .collect(),
        );

        if !self.state.vertices.is_empty() {
            shapes.push(self.draw_line(
                [
                    *self.state.vertices.first().unwrap(),
                    *self.state.vertices.last().unwrap(),
                ],
                1.0,
                egui::Color32::LIGHT_BLUE,
            ));
        }

        shapes.append(
            &mut self
                .state
                .triangles
                .iter()
                .flat_map(|t| self.draw_triangle(t, 1.0, egui::Color32::LIGHT_GRAY))
                .collect(),
        );

        shapes.append(
            &mut self
                .state
                .spheres
                .iter()
                .flat_map(|s| {
                    let Point2 { x, y } = s.center;
                    let center = Pos2 { x, y };
                    [
                        // render the center
                        egui::Shape::circle_filled(
                            self.to_screen_space(pos2(x, y)),
                            2.0,
                            egui::Color32::LIGHT_GREEN,
                        ),
                        self.draw_sphere(s.radius, center, 1.0, egui::Color32::LIGHT_GREEN),
                    ]
                })
                .collect(),
        );
        // shapes.append(&mut self.draw_debug_grid());

        self.add_shapes(shapes);
    }

    fn add_shapes<I>(&mut self, shapes: I)
    where
        I: IntoIterator<Item = egui::Shape>,
    {
        (&mut self.graph_painter.as_ref().unwrap()).extend(shapes)
    }

    #[allow(dead_code)]
    fn draw_debug_grid(&self) -> Vec<egui::Shape> {
        vec![
            // x axis
            self.draw_line([pos2(-1.0, 0.0), pos2(1.0, 0.0)], 1.0, egui::Color32::WHITE),
            // y axis
            self.draw_line([pos2(0.0, -1.0), pos2(0.0, 1.0)], 1.0, egui::Color32::WHITE),
            // center
            egui::Shape::circle_filled(
                self.to_screen_space(pos2(0.0, 0.0)),
                5.0,
                egui::Color32::RED,
            ),
            // unit circle
            self.draw_sphere(1.0, pos2(0.0, 0.0), 1.0, egui::Color32::RED),
            self.draw_sphere(1.0, pos2(1.0, 0.0), 1.0, egui::Color32::ORANGE),
            self.draw_sphere(1.0, pos2(-1.0, 0.0), 1.0, egui::Color32::GREEN),
            self.draw_sphere(1.0, pos2(0.0, 1.0), 1.0, egui::Color32::YELLOW),
            self.draw_sphere(1.0, pos2(0.0, -1.0), 1.0, egui::Color32::BLUE),
            // axis points
            egui::Shape::circle_filled(
                self.to_screen_space(pos2(1.0, 0.0)),
                5.0,
                egui::Color32::ORANGE,
            ),
            egui::Shape::circle_filled(
                self.to_screen_space(pos2(0.0, 1.0)),
                5.0,
                egui::Color32::YELLOW,
            ),
            egui::Shape::circle_filled(
                self.to_screen_space(pos2(-1.0, 0.0)),
                5.0,
                egui::Color32::GREEN,
            ),
            egui::Shape::circle_filled(
                self.to_screen_space(pos2(0.0, -1.0)),
                5.0,
                egui::Color32::BLUE,
            ),
        ]
    }

    fn draw_triangle(&self, t: &Triangle, stroke: f32, color: egui::Color32) -> Vec<egui::Shape> {
        let Triangle { vertices } = t;
        let [a, b, c] = vertices;
        let Point { x: ax, y: ay } = a;
        let Point { x: bx, y: by } = b;
        let Point { x: cx, y: cy } = c;
        let a = pos2(*ax, *ay);
        let b = pos2(*bx, *by);
        let c = pos2(*cx, *cy);
        vec![
            self.draw_line([a, b], stroke, color),
            self.draw_line([b, c], stroke, color),
            self.draw_line([c, a], stroke, color),
        ]
    }

    fn draw_line(&self, points: [Pos2; 2], stroke: f32, color: egui::Color32) -> egui::Shape {
        let line = [
            self.to_screen_space(points[0]),
            self.to_screen_space(points[1]),
        ];
        egui::Shape::line_segment(line, (stroke, color))
    }

    fn draw_sphere(
        &self,
        radius: f32,
        center: Pos2,
        stroke: f32,
        color: egui::Color32,
    ) -> egui::Shape {
        let transformed_center = self.to_screen_space(center);
        let width = self.graph_painter.as_ref().unwrap().clip_rect().width();
        let radius = width * radius * 0.5;
        egui::Shape::circle_stroke(transformed_center, radius, (stroke, color))
    }

    /// Takes a point `p` and converts it to screen space.
    fn to_screen_space(&self, p: Pos2) -> egui::Pos2 {
        debug_assert!(
            p.x <= 1.0 && p.x >= -1.0 && p.y <= 1.0 && p.y >= -1.0,
            "position invariant: {p:?}"
        );
        // !ASSUMPTION!
        // We asssume the graph_painter has an aspect ratio of 1:1. This is converted to
        // a (1,1) vector the center of which is (0.5, 0.5). We want the center to be at
        // the origin (0,0), so we scale the size up by two (2,2), then get the middle.
        let to_rect = self.graph_painter.as_ref().unwrap().clip_rect();
        let size = egui::Vec2 { x: 2.0, y: 2.0 };
        let from_rect = egui::Rect::from_center_size(egui::Pos2::ZERO, size);
        let transform = egui::emath::RectTransform::from_to(from_rect, to_rect);
        transform.transform_pos(egui::pos2(p.x, p.y * -1.0))
    }
}
