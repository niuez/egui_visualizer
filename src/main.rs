mod parser;
mod transform;

use eframe::{egui::*};
use epaint::*;

use parser::PaintFrame;

pub struct EguiSample {
    frame_idx: usize,
    paint_str: String,
    frames: Vec<PaintFrame>,
    msg: String,
}

impl EguiSample {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            frame_idx: 0,
            paint_str: format!("# (-20, -20) (250, 300)\nr (100, 100) (200, 200) {{{{rect}}}}\nr (0, 0) (50, 50) {{{{rect2}}}}\n"),
            frames: PaintFrame::multi_parse("# (-20, -20) (250, 300)\nr (100, 100) (200, 200) {{rect}}\nr (0, 0) (50, 50) {{rect2}}\n")
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
            let mut idx_i32 = self.frame_idx as i32;
            ui.add(Slider::new(
                    &mut idx_i32,
                    0i32..=((std::cmp::max(self.frames.len(), 1) - 1) as i32)
            ));
            self.frame_idx = idx_i32 as usize;
                    
            ui.label(format!("{}", self.msg));

            ScrollArea::vertical()
                .show(ui, |ui| {
                    let text_event = ui.add(TextEdit::multiline(&mut self.paint_str).desired_width(f32::INFINITY));
                    if text_event.changed() {
                        self.frames = PaintFrame::multi_parse(&self.paint_str)
                            .map(|(s, f)| f)
                            .unwrap_or_default();
                    }
                });
        });

        CentralPanel::default().show(ctx, |ui| {
            let default_frame = PaintFrame::default();
            let frame = if self.frame_idx < self.frames.len() { &self.frames[self.frame_idx] } else { &default_frame };
            let ui_size = ui.available_size_before_wrap();
            let fr_size = frame.rect.size();
            let max_mul = {
                let xp = ui_size.x / fr_size.x;
                let yp = ui_size.y / fr_size.y;
                if xp > yp { yp } else { xp }
            };
            let (mut response, painter) =
                ui.allocate_painter(fr_size * max_mul, Sense::hover());

            let to_screen = emath::RectTransform::from_to(
                frame.rect,
                response.rect,
                );
            let from_screen = to_screen.inverse();

            let shapes: Vec<_> = frame.elems.iter()
                .filter_map(|e| {
                    transform::shape_transform(e.shape.clone(), &to_screen)
                })
                .collect();
            painter.extend(shapes);

            self.msg = String::new();
            if let Some(pointer_pos) = response.hover_pos() {
                for h in frame.elems.iter().rev().filter_map(|e| e.hover.as_ref()) {
                    if h.check(pointer_pos, &to_screen) {
                        self.msg = h.msg.clone();
                        
                        let gallary = painter.layout_no_wrap(self.msg.clone(), FontId::proportional(8.0), Color32::BLACK);
                        let rect = gallary.rect.clone();
                        painter.rect_filled(Rect::from_min_max(rect.min + (pointer_pos - rect.max), pointer_pos), 0.0, Color32::WHITE);
                        painter.galley((pointer_pos - rect.max).to_pos2(), gallary);
                        response.mark_changed();
                        break;
                    }
                }
            }
        });
        if ctx.input(|i| i.key_pressed(Key::ArrowRight)) {
            if self.frame_idx + 1 < self.frames.len() {
                self.frame_idx += 1;
            }
            ctx.request_repaint();
        }
        if ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
            if self.frame_idx > 0 {
                self.frame_idx -= 1;
            }
            ctx.request_repaint();
        }
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
