pub mod color;
use color::*;

use eframe::emath::RectTransform;
use eframe::epaint::RectShape;
use nom::character::complete::*;
use nom::combinator::*;
use nom::IResult;
use nom::bytes::complete::*;
use nom::sequence::*;
use nom::number::complete::*;
use nom::multi::*;
use nom::branch::alt;
use eframe::egui::*;

#[derive(Debug)]
pub enum HoverCondition {
    Rect(Rect),
    Path(Vec<Pos2>),
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
pub struct FrameElement {
    pub shape: Shape,
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
    pub fn multi_parse(s: &str) -> IResult<&str, Vec<Self>> {
        let (s, res) = many0(Self::parse)(s)?;
        Ok(( s, res ))
    }
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (_, _, p1, _, p2, _, _)) = tuple((
                tag("# "), space0, parse_pos2, space0, parse_pos2, space0, newline
        ))(s)?;
        let (s, elems) = many0(
            map(
                tuple(( parse_element, space0, newline)),
                |(e, _, _)| e
            )
        )(s)?;
        let frame = PaintFrame {
            elems,
            rect: Rect::from_two_pos(p1, p2),
        };
        Ok(( s, frame ))
    }
}

enum ParseMsg {
    Any(char),
    End,
}

fn parsemsg_any(s: &str) -> IResult<&str, ParseMsg> {
    let (s, c) = anychar(s)?;
    Ok(( s, ParseMsg::Any(c) ))
}
fn parsemsg_end(s: &str) -> IResult<&str, ParseMsg> {
    let (s, _) = tag("}}")(s)?;
    Ok(( s, ParseMsg::End ))
}

fn parsemsg(s: &str) -> IResult<&str, String> {
    let (mut s, _) = tag("{{")(s)?;
    let mut msg = String::new();
    loop {
        let (ss, inline) = alt((parsemsg_end, parsemsg_any,))(s)?;
        s = ss;
        match inline {
            ParseMsg::Any(c) => {
                msg.push(c);
            }
            ParseMsg::End => {
                break;
            }
        }
    }
    Ok(( s, msg ))
}

fn parse_pos2(s: &str) -> IResult<&str, Pos2> {
    let (s, (_, _, x, _, _, _, y, _, _))
        = tuple((tag("("), space0, float, space0, tag(","), space0, float, space0, tag(")")))(s)?;
    Ok((s, pos2(x, y)))
}

fn parse_vec_pos2(s: &str) -> IResult<&str, Vec<Pos2>> {
    let sp = tuple((space0, tag(","), space0));
    let (s, (_, _, vp, _, _)) =
        tuple((
            tag("["), space0,
            separated_list0(sp, parse_pos2),
            space0, tag("]")
        ))(s)?;
    Ok((s, vp))
}

fn parse_path(s: &str) -> IResult<&str, FrameElement> {
    let (s, (_, _, vp, _, msg)) = tuple(( tag("p "), space0, parse_vec_pos2, space0, opt(parsemsg) ))(s)?;
    let elem = FrameElement {
        shape: Shape::line(vp.clone(), Stroke::new(1.0, Color32::BLACK)),
        hover: msg.map(|msg| Hover { msg, hover_cond: HoverCondition::Path(vp) })
    };
    Ok(( s, elem ))
}

fn parse_rect(s: &str) -> IResult<&str, FrameElement> {
    let (s, (_, _, p1, _, p2, _, msg)) = tuple(( tag("r"), space0, parse_pos2, space0, parse_pos2, space0, opt(parsemsg) ))(s)?;
    let rect = Rect::from_two_pos(p1, p2);
    let elem = FrameElement {
        shape: Shape::Rect(RectShape::new(rect.clone(), 0.0, Color32::TRANSPARENT, Stroke::new(1.0, Color32::BLACK))),
        hover: msg.map(|msg| Hover { msg, hover_cond: HoverCondition::Rect(rect.clone()) })
    };
    Ok(( s, elem ))
}

fn parse_rect_fill(s: &str) -> IResult<&str, FrameElement> {
    let (s, (_, _, p1, _, p2, _, c, _, msg)) = tuple(( tag("rf"), space0, parse_pos2, space0, parse_pos2, space0, parse_color, space0, opt(parsemsg) ))(s)?;
    let rect = Rect::from_two_pos(p1, p2);
    let elem = FrameElement {
        shape: Shape::Rect(RectShape::new(rect.clone(), 0.0, c, Stroke::NONE)),
        hover: msg.map(|msg| Hover { msg, hover_cond: HoverCondition::Rect(rect.clone()) })
    };
    Ok(( s, elem ))
}

fn parse_element(s: &str) -> IResult<&str, FrameElement> {
    let (s, e) = alt((
            parse_path,
            parse_rect,
            parse_rect_fill,
    ))(s)?;
    Ok(( s, e ))
}
