mod parser;
mod transform;

use std::path::PathBuf;

use eframe::{egui::*};
use egui_file_dialog::FileDialog;
use epaint::*;

use parser::PaintFrame;

use crate::parser::ElementKind;

pub struct EguiSample {
    frame_idx: usize,
    file_dialog: FileDialog,
    selected_file: Option<PathBuf>,
    paint_str: String,
    frames: Vec<PaintFrame>,
    msg: String,
    drag_pos: Option<Pos2>,
    frame_rect: Rect,
}

impl EguiSample {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            frame_idx: 0,
            file_dialog: FileDialog::new(),
            selected_file: None,
            paint_str: format!("# (-20, -20) (250, 300)\nr (100, 100) (200, 200) {{{{rect}}}}\nr (0, 0) (50, 50) {{{{rect2}}}}\n"),
            frames: vec![],
            msg: String::new(),
            drag_pos: None,
            frame_rect: Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0)),
        }
    }
}

impl eframe::App for EguiSample {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}       
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.style_mut(|style| style.interaction.tooltip_delay = 0.0);
        SidePanel::right("here").show(ctx, |ui| {
            let mut idx_i32 = self.frame_idx as i32;
            ui.add(Slider::new(
                    &mut idx_i32,
                    0i32..=((std::cmp::max(self.frames.len(), 1) - 1) as i32)
                
                ).smart_aim(false)
                );
            self.frame_idx = idx_i32 as usize;

            if ui.button("reset view").clicked() && self.frame_idx < self.frames.len() {
                self.frame_rect = self.frames[self.frame_idx].rect;
            }

            if ui.button("select file").clicked() {
                // Open the file dialog to select a file.
                self.file_dialog.select_file();
            }
            ui.label(format!("selected file: {:?}", self.selected_file));

            // Update the dialog
            self.file_dialog.update(ctx);

            if let Some(path) = self.file_dialog.take_selected() {
                self.selected_file = Some(path.to_path_buf());
                match PaintFrame::from_file(path.as_path()) {
                    Ok(frames) => {
                        self.frames = frames;
                        self.frame_idx = 0;
                        if 0 < self.frames.len() {
                            self.frame_rect = self.frames[0].rect.clone();
                        }
                    }
                    Err(e) => {
                        self.msg = format!("{:?}", e);
                    }
                }
            }
                    
            ui.label(format!("{}", self.msg));
        });

        CentralPanel::default().frame(Frame::none().fill(Color32::WHITE)).show(ctx, |ui| {
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
                ui.allocate_painter(ui_size, Sense::drag());
                //ui.allocate_painter(fr_size * max_mul, Sense::drag());

            let to_screen = emath::RectTransform::from_to(
                self.frame_rect,
                Rect::from_center_size(response.rect.center(), fr_size * max_mul),
                //Rect::from_min_max(Pos2::ZERO, (fr_size * max_mul).to_pos2()),
                //response.rect,
                );
            let from_screen = to_screen.inverse();

            for elem in frame.elems.iter() {
                match &elem.shape {
                    ElementKind::Shape(shape) => {
                        painter.add(transform::shape_transform(shape.clone(), &to_screen).unwrap());
                    }
                    ElementKind::Text(text) => {
                        let galley = painter.layout_no_wrap(text.text.clone(), FontId::proportional(text.size * (to_screen.scale().x * to_screen.scale().y).sqrt()), text.color);
                        let rect = galley.rect.clone();
                        //painter.rect_filled(Rect::from_min_max(rect.min + (pointer_pos - rect.max), pointer_pos), 0.0, Color32::WHITE);
                        painter.galley(to_screen * text.pos - rect.size() / 2.0, galley, Color32::PLACEHOLDER);
                    }
                }
            }

            //eprintln!("{:?}", shapes);
            //painter.rect_filled(painter.clip_rect(), 0.0, Color32::WHITE);
            //painter.extend(shapes);

            self.msg = String::new();
            if let Some(pointer_pos) = response.hover_pos() {
                ctx.input(|i| {
                    let zd = i.zoom_delta();
                    let (x, y) = 
                        if i.key_down(Key::Z) { (zd, 1.0) }
                        else if i.key_down(Key::X) { (1.0, zd) }
                        else { (zd, zd) };
                    {
                        let p = from_screen * pointer_pos;
                        self.frame_rect = Rect::from_min_max(
                            p + ((self.frame_rect.min - p) / x ),
                            p + ((self.frame_rect.max - p) / y ),
                        );
                    }
                });
                for h in frame.elems.iter().rev().filter_map(|e| e.hover.as_ref()) {
                    if h.check(pointer_pos, &to_screen) {
                        response = response.on_hover_text_at_pointer(&h.msg.clone());
                        /*
                        let gallary = painter.layout_no_wrap(self.msg.clone(), FontId::proportional(8.0), Color32::BLACK);
                        let rect = gallary.rect.clone();
                        painter.rect_filled(Rect::from_min_max(rect.min + (pointer_pos - rect.max), pointer_pos), 0.0, Color32::WHITE);
                        painter.galley((pointer_pos - rect.max).to_pos2(), gallary);
                        response.mark_changed();
                        */
                        break;
                    }
                }
            }

            if let Some(after) = response.interact_pointer_pos() {
                if let Some(before) = self.drag_pos.take() {
                    self.frame_rect.set_center(self.frame_rect.center() - (from_screen * after - from_screen * before));
                }
                self.drag_pos = Some(after);
            }
            else {
                self.drag_pos = None;
            }
            self.msg = format!("{:?}", self.drag_pos);
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
        //follow_system_theme: false,
        //default_theme: eframe::Theme::Light,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native("egui_sample", options, Box::new(|cc| Ok(Box::new(EguiSample::new(cc))))).unwrap();
}
