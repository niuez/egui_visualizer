mod parser;

use eframe::{egui::*};

use parser::PaintFrame;

pub struct EguiSample {
    frame: PaintFrame,
    msg: String,
}

impl EguiSample {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            frame: PaintFrame::parse("r (100, 100) (200, 200) {{rect}}\nr (0, 0) (50, 50) {{rect2}}\n")
                .map(|(s, f)| { println!("{}", s); f })
                .unwrap_or_default(),
            msg: String::new(),
        }
    }
}

impl eframe::App for EguiSample {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}       
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        SidePanel::right("here").show(ctx, |ui| {
            ui.label(format!("{}", self.msg))
        });

        CentralPanel::default().show(ctx, |ui| {
            let (mut response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), Sense::hover());
            println!("{:?}", painter.clip_rect());

            let to_screen = emath::RectTransform::from_to(
                painter.clip_rect(),
                response.rect,
                );
            let from_screen = to_screen.inverse();

            let shapes: Vec<_> = self.frame.elems.iter()
                .map(|e| e.shape.clone())
                .collect();
            painter.extend(shapes);
            //painter.rect(painter.clip_rect() * 0.99, 0.0, Color32::TRANSPARENT, Stroke::new(1.0, Color32::BLACK));

            self.msg = String::new();
            if let Some(pointer_pos) = response.hover_pos() {
                let canvas_pos = from_screen * pointer_pos;
                for h in self.frame.elems.iter().filter_map(|e| e.hover.as_ref()) {
                    if h.check(canvas_pos) {
                        self.msg = h.msg.clone();
                        response.mark_changed();
                    }
                }
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        follow_system_theme: false,
        default_theme: eframe::Theme::Light,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native("egui_sample", options, Box::new(|cc| Box::new(EguiSample::new(cc)))).unwrap();
}
