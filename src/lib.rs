use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

pub fn pos(x: f32, y: f32) -> Pos {
    Pos { x, y }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stroke {
    pub color: Color,
    pub width: f32,
}

impl Default for Stroke {
    fn default() -> Self {
        Stroke {
            color: Color::new(0, 0, 0),
            width: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Path {
    pub vp: Vec<Pos>,
    pub fill: Option<Color>,
    pub stroke: Stroke,
}

impl Path {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_vertices(vp: Vec<Pos>) -> Self {
        Self { vp, ..Self::default() }
    }
    pub fn add_pos(mut self, p: Pos) -> Self {
        self.vp.push(p);
        self
    }
    pub fn close(mut self, fill: Color) -> Self {
        self.fill = Some(fill);
        self
    }
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke = Stroke { color, width };
        self
    }
    pub fn element(self) -> Element {
        Element {
            shape: Shape::Path(self),
            msg: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Circle {
    pub center: Pos,
    pub radius: f32,
    pub fill: Option<Color>,
    pub stroke: Option<Stroke>,
}

impl Circle {
    pub fn new(center: Pos, radius: f32) -> Self {
        Self { center, radius, ..Self::default() }
    }
    pub fn close(mut self, fill: Color) -> Self {
        self.fill = Some(fill);
        self
    }
    pub fn stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke = Some(Stroke { color, width });
        self
    }
    pub fn element(self) -> Element {
        Element {
            shape: Shape::Circle(self),
            msg: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Shape {
    Path(Path),
    Circle(Circle),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    pub shape: Shape,
    pub msg: Option<String>,
}

impl Element {
    pub fn with_msg<I: Into<String>>(mut self, msg: I) -> Self {
        self.msg = Some(msg.into());
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
    pub elems: Vec<Element>,
    pub p1: Pos,
    pub p2: Pos,
}

impl Frame {
    pub fn new(p1: Pos, p2: Pos) -> Self {
        Self {
            elems: vec![],
            p1,
            p2,
        }
    }
    pub fn add_element(mut self, elem: Element) -> Self {
        self.elems.push(elem);
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frames {
    pub frames: Vec<Frame>,
}

impl Frames {
    pub fn new() -> Self {
        Self {
            frames: vec![]
        }
    }
    pub fn add_frame(mut self, frame: Frame) -> Self {
        self.frames.push(frame);
        self
    }

    pub fn encode(&self) -> anyhow::Result<Vec<u8>> {
        let encoded: Vec<u8> = bincode::serialize(self)?;
        Ok(encoded)
    }

    pub fn decode(encoded: Vec<u8>) -> anyhow::Result<Self> {
        let decoded: Self = bincode::deserialize(&encoded[..])?;
        Ok(decoded)
    }

    pub fn encode_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> anyhow::Result<()> {
        let writer = std::io::BufWriter::new(std::fs::File::create(path)?);
        bincode::serialize_into(writer, self)?;
        Ok(())
    }

    pub fn decode_from_file<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let reader = std::io::BufReader::new(std::fs::File::open(path)?);
        let res = bincode::deserialize_from(reader)?;
        Ok(res)
    }
}
