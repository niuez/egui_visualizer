pub mod color;
use color::*;

use eframe::emath::RectTransform;
use eframe::egui::*;
use eframe::epaint::{CircleShape, PathShape};

#[derive(Debug)]
pub enum HoverCondition {
    Rect(Rect),
    Path(Vec<Pos2>),
    ClosedPath(Vec<Pos2>),
    Circle(Pos2, f32),
}

impl HoverCondition {
    pub fn check(&self, p: Pos2, to_screen: &RectTransform) -> bool {
        match *self {
            Self::Rect(ref rect) => {
                Rect::from_min_max(to_screen * rect.min, to_screen * rect.max).contains(p)
            }
            Self::Path(ref path) => {
                let mut ok = false;
                for i in 1..path.len() {
                    let a1 = to_screen * path[i - 1];
                    let a2 = to_screen * path[i];
                    let dist = 
                        if (a2 - a1).dot(p - a1) < 0.0 || (a1 - a2).dot(p - a2) < 0.0 {
                            std::f32::INFINITY
                        }
                        else {
                            let r = a1 + (a2 - a1).dot(p - a1) / (a2 - a1).length_sq() * (a2 - a1);
                            (r - p).length()
                        };
                    ok |= dist < 10.0;
                }
                ok
            }
            Self::ClosedPath(ref path) => {
                let cross = |p1: Pos2, p2: Pos2, p3: Pos2| {
                    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y) < 0.0
                };
                let mut ok = false;
                for i in 0..path.len() {
                    let b1 = cross(p, to_screen * path[i], to_screen * path[(i + 1) % path.len()]);
                    let b2 = cross(p, to_screen * path[(i + 1) % path.len()], to_screen * path[(i + 2) % path.len()]);
                    let b3 = cross(p, to_screen * path[(i + 2) % path.len()], to_screen * path[i]);
                    if (b1 == b2) && (b2 == b3) {
                        ok = true;
                    }
                }
                ok
            }
            Self::Circle(c, r) => {
                let v = to_screen * c - p;
                (v.x / (r * to_screen.scale().x)).powi(2) + (v.y / (r * to_screen.scale().y)).powi(2) <= 1.0
            }
        }
    }
}

#[derive(Debug)]
pub struct Hover {
    pub hover_cond: HoverCondition,
    pub msg: String,
}

impl Hover {
    pub fn check(&self, p: Pos2, to_screen: &RectTransform) -> bool {
        self.hover_cond.check(p, to_screen)
    }
}

#[derive(Debug)]
pub struct Text {
    pub text: String,
    pub pos: Pos2,
    pub size: f32,
    pub color: Color32,
}

#[derive(Debug)]
pub enum ElementKind {
    Shape(Shape),
    Text(Text),
}

#[derive(Debug)]
pub struct FrameElement {
    pub shape: ElementKind,
    pub hover: Option<Hover>,
}

#[derive(Debug)]
pub struct PaintFrame {
    pub elems: Vec<FrameElement>,
    pub rect: Rect,
}

impl Default for PaintFrame {
    fn default() -> Self {
        PaintFrame {
            elems: Vec::new(),
            rect: Rect::NOTHING,
        }
    }
}

impl PaintFrame {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Vec<Self>> {
        let frames = visualizer_shapes::Frames::decode_from_file(path)?;
        Ok(frames.frames.into_iter().map(|frame| {
            let elems = frame.elems.into_iter().map(|e| {
                //eprintln!("{:?}", e);
                match e.shape {
                    visualizer_shapes::Shape::Path(p) => {
                        let vp = p.vp.into_iter().map(|p| pos2(p.x, p.y)).collect::<Vec<_>>();
                        let closed = p.fill.is_some();
                        FrameElement {
                            shape: ElementKind::Shape(Shape::Path(PathShape {
                                           points: vp.clone(),
                                           closed,
                                           fill: p.fill.map(|f| Color32::from_rgba_unmultiplied(f.r, f.g, f.b, f.a)).unwrap_or(Color32::TRANSPARENT),
                                           stroke: Stroke::new(p.stroke.width, Color32::from_rgba_unmultiplied(p.stroke.color.r, p.stroke.color.g, p.stroke.color.b, p.stroke.color.a)).into(),
                            })),
                            hover: e.msg.map(|msg| Hover { msg, hover_cond: if closed { HoverCondition::ClosedPath(vp) } else { HoverCondition::Path(vp) } })
                        }
                    }
                    visualizer_shapes::Shape::Circle(c) => {
                        FrameElement {
                            shape: ElementKind::Shape(Shape::Circle(CircleShape {
                                center: pos2(c.center.x, c.center.y),
                                radius: c.radius,
                                fill: c.fill.map(|f| Color32::from_rgba_unmultiplied(f.r, f.g, f.b, f.a)).unwrap_or(Color32::TRANSPARENT),
                                stroke: c.stroke.map(|s| Stroke::new(s.width, Color32::from_rgba_unmultiplied(s.color.r, s.color.g, s.color.b, s.color.a))).unwrap_or(Stroke::default()),
                            })),
                            hover: e.msg.map(|msg| Hover { msg, hover_cond: HoverCondition::Circle(pos2(c.center.x, c.center.y), c.radius) })
                        }
                    }
                    visualizer_shapes::Shape::Text(t) => {
                        FrameElement {
                            shape: ElementKind::Text(Text {
                                text: t.text,
                                size: t.size,
                                pos: pos2(t.pos.x, t.pos.y),
                                color: Color32::from_rgba_unmultiplied(t.color.r, t.color.g, t.color.b, t.color.a),
                            }),
                            hover: None,
                        }
                    }
                }
            }).collect::<Vec<_>>();
            PaintFrame {
                elems,
                rect: Rect::from_two_pos(pos2(frame.p1.x, frame.p1.y), pos2(frame.p2.x, frame.p2.y)),
            }
        }).collect())
    }
}
