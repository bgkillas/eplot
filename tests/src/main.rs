use egui::Context;
use rupl::{Complex, Graph, GraphType, Vec3};
use std::fs;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native("eplot", options, Box::new(|_cc| Ok(Box::new(App::new()))))
}

struct App {
    plot: Graph,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.main(ctx);
    }
}

impl App {
    fn new() -> Self {
        /*let plot = Graph::new(
            vec![grab_width3d("data/data12", -8.0, -8.0, 8.0, 8.0)],
            true,
            -8.0,
            8.0,
        );*/
        let mut plot = Graph::new(
            vec![grab_width3d("data/data14", -2.0, -2.0, 2.0, 2.0)],
            false,
            -2.0,
            2.0,
        );
        plot.set_offset(Vec3::new(0.0, 0.0, -2.0));
        Self { plot }
    }
    fn main(&mut self, ctx: &Context) {
        self.plot.update(ctx);
    }
}
fn to_complex(c: &str) -> Complex {
    if !c.contains('i') {
        Complex::Real(c.parse::<f32>().unwrap_or(0.0))
    } else {
        let n = c.starts_with('-');
        let c = if n {
            &c.chars().skip(1).take(c.len() - 2).collect::<String>()
        } else {
            &c.chars().take(c.len() - 1).collect::<String>()
        };
        let r = c.contains('-');
        let l = c
            .split(['-', '+'])
            .map(|c| {
                if c.eq_ignore_ascii_case("in") {
                    f32::INFINITY
                } else if c.eq_ignore_ascii_case("na") {
                    f32::NAN
                } else {
                    c.parse::<f32>().unwrap_or(0.0)
                }
            })
            .collect::<Vec<f32>>();
        let s = if n { -l[0] } else { l[0] };
        if l.len() == 1 {
            Complex::Imag(s)
        } else {
            Complex::Complex(s, if r { -l[1] } else { l[1] })
        }
    }
}
#[allow(dead_code)]
fn grab_width(f: &str, start: f32, end: f32) -> GraphType {
    GraphType::Width(
        fs::read_to_string(f)
            .unwrap()
            .trim()
            .replace(['{', '}'], "")
            .split(',')
            .map(to_complex)
            .collect::<Vec<Complex>>(),
        start,
        end,
    )
}
#[allow(dead_code)]
fn grab_width3d(f: &str, startx: f32, starty: f32, endx: f32, endy: f32) -> GraphType {
    GraphType::Width3D(
        fs::read_to_string(f)
            .unwrap()
            .trim()
            .replace(['{', '}'], "")
            .replace('\n', ",")
            .split(',')
            .map(to_complex)
            .collect::<Vec<Complex>>(),
        startx,
        starty,
        endx,
        endy,
    )
}
#[allow(dead_code)]
fn grab_coord(f: &str) -> GraphType {
    GraphType::Coord(
        fs::read_to_string(f)
            .unwrap()
            .trim()
            .replace(['{', '}'], "")
            .split('\n')
            .map(|c| {
                let a = c.split(',').map(to_complex).collect::<Vec<Complex>>();
                (real(a[0]), a[1])
            })
            .collect::<Vec<(f32, Complex)>>(),
    )
}
#[allow(dead_code)]
fn real(c: Complex) -> f32 {
    match c {
        Complex::Real(y) => y,
        Complex::Imag(_) => 0.0,
        Complex::Complex(y, _) => y,
    }
}
