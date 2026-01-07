use eframe::{App, egui};
use egui_plot::{Plot, PlotPoints, Line};
use rand::Rng;

struct Sierpinski {
    points: Vec<[f64; 2]>,
    current: [f64; 2],
    vertices: [[f64; 2]; 3],
}

impl App for Sierpinski {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let mut rng = rand::rng();

        // generate 100 points per frame
        for _ in 0..100 {
            let v = self.vertices[rng.random_range(0..3)];
            self.current[0] = (self.current[0] + v[0]) / 2.0;
            self.current[1] = (self.current[1] + v[1]) / 2.0;
            self.points.push(self.current);
        }

        // limit memory
        if self.points.len() > 10000 {
            self.points.drain(0..5000);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("sierpinski").show(ui, |plot_ui| {
                plot_ui.line(Line::new("points", PlotPoints::from(self.points.clone())));
            });
        });

        ctx.request_repaint(); // continue updating
    }
}

fn main() -> eframe::Result<()> {
    let app = Sierpinski {
        vertices: [[0.0, 0.0], [1.0, 0.0], [0.5, 0.866]], // equilateral triangle
        current: [0.25, 0.25],
        points: Vec::new(),
    };

    eframe::run_native(
        "Sierpinski Triangle",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
