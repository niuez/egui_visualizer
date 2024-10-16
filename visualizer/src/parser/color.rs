use eframe::epaint::ecolor::rgb_from_hsv;
use nom::character::complete::*;
use nom::combinator::*;
use nom::IResult;
use nom::bytes::complete::*;
use nom::sequence::*;
use nom::number::complete::*;
use nom::multi::*;
use nom::branch::alt;
use eframe::egui::*;

const PHI_INV: f32 = 0.618033988749895;

// https://martin.ankerl.com/2009/12/09/how-to-create-random-colors-programmatically/
pub fn parse_tag_color(s: &str) -> IResult<&str, Color32> {
    let (ss, (_, _, _, _, idx, _, _)) = tuple((
            tag("tag"), space0, tag("("), space0,
            nom::character::complete::u64,
            space0, tag(")")
    ))(s)?;
    let h = PHI_INV * idx as f32 * 17.0;
    let s = PHI_INV * idx as f32 * 11.0;
    let h = h - h.floor();
    let s = s - s.floor();
    let [r, g, b] = rgb_from_hsv((h, s * (1.0 - 0.25) + 0.25, 0.95));
    Ok((ss, Rgba::from_rgb(r, g, b).into()))
}

pub fn parse_none_color(s: &str) -> IResult<&str, Color32> {
    let (s, _) = tag("none()")(s)?;
    Ok((s, Color32::TRANSPARENT))
}

pub fn parse_named_color(s: &str) -> IResult<&str, Color32> {
    let (s, (_, n, _)) = tuple((tag("named("), alpha1, tag(")")))(s)?;
    let [r, g, b] = color_name::Color::val().by_string(n.to_owned()).unwrap_or([0, 0, 0]);
    Ok((s, Color32::from_rgb(r, g, b)))
}

pub fn parse_func_color<'a>(name: &'a str, mut func: impl FnMut(f32) -> (u8, u8, u8)) -> impl FnMut(&'a str) -> IResult<&'a str, Color32> {
    move |s| {
        let (s, (_, _, _, t, _, _)) = tuple(( tag(name), tag("("), space0, float, space0, tag(")") ))(s)?;
        let (r, g, b) = func(t);
        Ok((s, Color32::from_rgb(r, g, b)))
    }
}

pub fn parse_color(s: &str) -> IResult<&str, Color32> {
    alt((parse_tag_color, parse_none_color, parse_named_color, 
         parse_func_color("turbo", |t| colorous::TURBO.eval_continuous(t as f64).as_tuple() )
    ))(s)
}
