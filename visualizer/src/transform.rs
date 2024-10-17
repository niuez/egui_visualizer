use eframe::{egui::*, emath::RectTransform};
use epaint::*;

pub fn shape_transform(shape: Shape, to_screen: &RectTransform) -> Option<Shape> {
    match shape {
        Shape::Path(path) => {
            Some(Shape::Path(PathShape {
                points: path.points.iter().map(|p| to_screen * p.clone()).collect(),
                ..path
            }))
        }
        Shape::Rect(rect) => {
            Some(Shape::Rect(RectShape {
                rect: Rect::from_min_max(to_screen * rect.rect.min, to_screen * rect.rect.max),
                ..rect
            }))
        }
        Shape::Circle(circle) => {
            Some(Shape::Circle(CircleShape {
                center: to_screen * circle.center,
                radius: to_screen.scale().length() * circle.radius,
                ..circle
            }))
        }
        _ => unreachable!(),
    }
}
